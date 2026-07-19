#!/usr/bin/env python3
"""Strictly validate C5 external evidence; only the CLI can grant attestation_verified."""
from __future__ import annotations
import argparse, base64, hashlib, json, re, shutil, stat, subprocess, tempfile, tomllib
from datetime import datetime
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
PATHS=ROOT/"tools/fixtures/c5-significance-scheduler-receipt/bounded-paths.txt"
LOCAL=ROOT/"docs/canonical-system/G1_C5_LOCAL_PLATFORM_OBSERVATIONS.json"

def strict_load_bytes(raw):
    def pairs(items):
        out={}
        for key,value in items:
            if key in out: raise ValueError(f"duplicate JSON key: {key}")
            out[key]=value
        return out
    if len(raw)>1_048_576: raise ValueError("JSON input exceeds 1 MiB")
    return json.loads(raw.decode("utf-8-sig"),object_pairs_hook=pairs,parse_float=lambda _:(_ for _ in ()).throw(ValueError("floats forbidden")),parse_constant=lambda _:(_ for _ in ()).throw(ValueError("non-finite values forbidden")))
def bounded_read(path,max_bytes,label):
    path=Path(path)
    if path.is_symlink(): raise ValueError(f"{label} may not be a symlink")
    info=path.stat()
    if not stat.S_ISREG(info.st_mode) or info.st_size>max_bytes: raise ValueError(f"{label} is missing, non-regular or oversized")
    raw=path.read_bytes()
    if len(raw)!=info.st_size or len(raw)>max_bytes: raise ValueError(f"{label} changed while being read")
    return raw
def strict_load(path): return strict_load_bytes(bounded_read(path,1_048_576,"JSON input"))
def canonical(v): return json.dumps(v,sort_keys=True,separators=(",",":"),ensure_ascii=True).encode()
def digest(v): return hashlib.sha256(canonical(v)).hexdigest()
def exact_keys(value,keys,label):
    if set(value)!=set(keys): raise ValueError(f"{label} fields changed")
def exact_value(actual,expected):
    if type(actual) is not type(expected): return False
    if isinstance(expected,dict): return set(actual)==set(expected) and all(exact_value(actual[k],v) for k,v in expected.items())
    if isinstance(expected,list): return len(actual)==len(expected) and all(exact_value(a,b) for a,b in zip(actual,expected))
    return actual==expected
def absolute_path(value): return isinstance(value,str) and (value.startswith("/") or Path(value).is_absolute())
def unhash(value,name):
    copy=dict(value);claimed=copy.pop(name)
    if not isinstance(claimed,str) or len(claimed)!=64 or digest(copy)!=claimed: raise ValueError(f"{name} mismatch")
    return claimed
def decoded(field,hash_field):
    if not isinstance(field,str) or len(field)>1_398_104: raise ValueError("encoded output exceeds 1 MiB")
    raw=base64.b64decode(field,validate=True)
    if hashlib.sha256(raw).hexdigest()!=hash_field: raise ValueError("raw output hash mismatch")
    return raw
def source_manifest(commit):
    raw=subprocess.check_output(["git","show",f"{commit}:tools/fixtures/c5-significance-scheduler-receipt/bounded-paths.txt"],cwd=ROOT).decode()
    paths=raw.splitlines()
    if not paths or len(paths)!=len(set(paths)) or "tools/fixtures/c5-significance-scheduler-receipt/bounded-paths.txt" not in paths: raise ValueError("committed bounded path list is empty, duplicate or not self-binding")
    rows=[]
    for relative in paths:
        if not relative or "\\" in relative or relative.startswith("/") or any(part in (".","..","") for part in relative.split("/")): raise ValueError("committed bounded path is noncanonical")
        blob=subprocess.check_output(["git","rev-parse",f"{commit}:{relative}"],cwd=ROOT,text=True).strip();rows.append(f"{relative}:{blob}")
    return hashlib.sha256("\n".join(rows).encode()).hexdigest()
def canonical_tree_bytes(raw):
    raw=raw.replace(b"\r\n",b"\n")
    if b"\r" in raw or not raw.endswith(b"\n"): raise ValueError("git ls-tree output is not canonical line-delimited data")
    return raw[:-1]
def tree_manifest(commit):
    raw=subprocess.check_output(["git","-c","core.quotePath=true","ls-tree","-r","--full-tree",commit],cwd=ROOT)
    return hashlib.sha256(canonical_tree_bytes(raw)).hexdigest()
def source_artifact_bindings(commit):
    manifest=subprocess.check_output(["git","show",f"{commit}:tools/fixtures/c5-significance-scheduler-receipt/Cargo.toml"],cwd=ROOT)
    lock=subprocess.check_output(["git","show",f"{commit}:tools/fixtures/c5-significance-scheduler-receipt/Cargo.lock"],cwd=ROOT)
    packages=tomllib.loads(lock.decode())["package"]
    rows=[{"name":p["name"],"version":p["version"],"source":p.get("source","path"),"checksum":p.get("checksum",""),"dependencies":sorted(p.get("dependencies",[]))} for p in packages]
    dependency=hashlib.sha256(canonical(sorted(rows,key=lambda p:(p["name"],p["version"],p["source"])))).hexdigest()
    return hashlib.sha256(manifest).hexdigest(),hashlib.sha256(lock).hexdigest(),dependency
def validate_frozen_request(request,local,freshness_required,verify_git=True):
    expected_platforms=[{"target":"x86_64-unknown-linux-gnu","os":"linux","architecture":"x86_64","pointer_width":64,"endian":"little"},{"target":"x86_64-apple-darwin","os":"macos","architecture":"x86_64","pointer_width":64,"endian":"little"},{"target":"aarch64-apple-darwin","os":"macos","architecture":"aarch64","pointer_width":64,"endian":"little"}]
    expected_policy={"provider":"github-actions","hosted_runner_required":True,"native_execution_required":True,"compile_only_forbidden":True,"process_count":2,"concurrent":True,"direct_executable":True,"clean_before_after":True}
    source=request.get("source",{});repo=request.get("provenance_policy",{}).get("repository","");commit=source.get("source_commit","")
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+",repo): raise ValueError("request repository identity is malformed")
    challenge=request.get("challenge");request_id=request.get("request_id")
    if not isinstance(challenge,str) or not re.fullmatch(r"[0-9a-f]{64}",challenge) or request_id!=f"g1-c5-{challenge[:16]}": raise ValueError("request challenge or identity is malformed")
    if type(request.get("schema_version")) is not int or request.get("schema_version")!=1 or request.get("protocol_id")!="G1-C5-INDEPENDENT-PLATFORM-V1" or source.get("repository_id")!="mindwarp-forge" or not exact_value(request.get("semantic"),{"receipt_id":"G1-C5-SIGNIFICANCE-SCHEDULER","expected_sha256":local["semantic_receipt_sha256"],"encoding":"lowercase_hex","max_decoded_bytes":65536}) or not exact_value(request.get("toolchain"),{"rustc_release":"1.97.0","rustc_commit":"2d8144b7880597b6e6d3dfd63a9a9efae3f533d3","cargo_release":"1.97.0"}) or not exact_value(request.get("allowed_platforms"),expected_platforms) or not exact_value(request.get("execution_policy"),expected_policy) or not exact_value(request.get("provenance_policy"),{"repository":repo,"workflow_path":".github/workflows/g1-c5-independent-platform.yml","signer_workflow":f"{repo}/.github/workflows/g1-c5-independent-platform.yml"}) or not exact_value(request.get("reference_platform"),{"os":"windows","architecture":"x86_64","receipt_id":"G1-C5-LOCAL-PLATFORM-OBSERVATIONS"}) or not exact_value(request.get("authority"),{"evidence_only":True,"promotion_authority":False,"activate_c6":False,"repository_mutation":False}): raise ValueError("request is not the frozen C5 external protocol")
    if commit!=local["source_commit"] or source.get("tracked_tree_manifest_sha256")!=local["tracked_tree_manifest_sha256"] or source.get("bounded_source_manifest_sha256")!=local["bounded_source_manifest_sha256"]: raise ValueError("request is not bound to the retained local C5 receipt")
    if freshness_required:
        created=datetime.fromisoformat(request["created_utc"].replace("Z","+00:00"));now=datetime.now(created.tzinfo)
        if created.tzinfo is None or not (0<=(now-created).total_seconds()<=86400): raise ValueError("request challenge is outside the 24-hour freshness window")
    if verify_git:
        if tree_manifest(commit)!=source.get("tracked_tree_manifest_sha256") or source_manifest(commit)!=source.get("bounded_source_manifest_sha256"): raise ValueError("request source provenance does not match the local repository")
        fixture_manifest,fixture_lock,dependency_graph=source_artifact_bindings(commit)
        if (fixture_manifest,fixture_lock,dependency_graph)!=(source.get("fixture_manifest_sha256"),source.get("fixture_lock_sha256"),source.get("dependency_graph_sha256")): raise ValueError("fixture, lock or dependency binding changed")
def validate(request,result,attestation_verified):
    exact_keys(request,["schema_version","protocol_id","request_id","challenge","created_utc","source","semantic","toolchain","allowed_platforms","execution_policy","provenance_policy","reference_platform","authority","request_sha256"],"request")
    request_sha=unhash(request,"request_sha256")
    exact_keys(request["source"],["repository_id","source_commit","tracked_tree_manifest_sha256","bounded_source_manifest_sha256","fixture_manifest_sha256","fixture_lock_sha256","dependency_graph_sha256"],"request source")
    exact_keys(request["semantic"],["receipt_id","expected_sha256","encoding","max_decoded_bytes"],"request semantic")
    exact_keys(request["toolchain"],["rustc_release","rustc_commit","cargo_release"],"request toolchain")
    exact_keys(request["execution_policy"],["provider","hosted_runner_required","native_execution_required","compile_only_forbidden","process_count","concurrent","direct_executable","clean_before_after"],"request execution policy")
    exact_keys(request["provenance_policy"],["repository","workflow_path","signer_workflow"],"request provenance policy")
    exact_keys(request["reference_platform"],["os","architecture","receipt_id"],"request reference platform")
    exact_keys(request["authority"],["evidence_only","promotion_authority","activate_c6","repository_mutation"],"request authority")
    for platform_row in request["allowed_platforms"]: exact_keys(platform_row,["target","os","architecture","pointer_width","endian"],"allowed platform")
    repo=request["provenance_policy"]["repository"];expected_signer=f"{repo}/.github/workflows/g1-c5-independent-platform.yml"
    if type(request["schema_version"]) is not int or request["schema_version"]!=1 or request["protocol_id"]!="G1-C5-INDEPENDENT-PLATFORM-V1" or not exact_value(request["execution_policy"],{"provider":"github-actions","hosted_runner_required":True,"native_execution_required":True,"compile_only_forbidden":True,"process_count":2,"concurrent":True,"direct_executable":True,"clean_before_after":True}) or not exact_value(request["provenance_policy"],{"repository":repo,"workflow_path":".github/workflows/g1-c5-independent-platform.yml","signer_workflow":expected_signer}) or not exact_value(request["reference_platform"],{"os":"windows","architecture":"x86_64","receipt_id":"G1-C5-LOCAL-PLATFORM-OBSERVATIONS"}) or not exact_value(request["authority"],{"evidence_only":True,"promotion_authority":False,"activate_c6":False,"repository_mutation":False}): raise ValueError("request protocol, signer or authority policy changed")
    exact_keys(result,["schema_version","protocol_id","request_id","request_sha256","challenge","result_id","source","platform","provenance","toolchain","environment","build","self_test","executions","claims","result_sha256"],"result")
    result_sha=unhash(result,"result_sha256")
    for key in ("schema_version","protocol_id","request_id","challenge"):
        if result[key]!=request[key]: raise ValueError(f"request/result {key} mismatch")
    if result["request_sha256"]!=request_sha: raise ValueError("request binding mismatch")
    source=dict(result["source"]);clean_before=source.pop("clean_before",None);clean_after=source.pop("clean_after",None)
    if source!=request["source"] or clean_before is not True or clean_after is not True: raise ValueError("source binding or cleanliness mismatch")
    if not any(exact_value(result["platform"],row) for row in request["allowed_platforms"]) or result["platform"]["os"]==request["reference_platform"]["os"]: raise ValueError("platform is not independently diverse")
    exact_keys(result["platform"],["target","os","architecture","pointer_width","endian"],"result platform")
    p=result["provenance"];exact_keys(p,["kind","provider","hosted_runner","run_id","run_attempt","job_id","job_url","repository","workflow_ref","runner_image","attestation_required"],"provenance")
    if p["kind"]!="provider_hosted_ci" or p["provider"]!="github-actions" or p["hosted_runner"] is not True or p["attestation_required"] is not True or p["repository"]!=request["provenance_policy"]["repository"] or not attestation_verified: raise ValueError("provider attestation is not verified")
    if "/.github/workflows/g1-c5-independent-platform.yml@" not in p["workflow_ref"]: raise ValueError("workflow identity drift")
    claims=result["claims"]
    if not exact_value(claims,{"evidence_only":True,"promotion_authority":False,"activate_c6":False,"repository_mutation":False,"independent_platform_execution_claimed":True}): raise ValueError("authority claim changed")
    build=result["build"]
    exact_keys(build,["argv","exit_code","stdout_sha256","stderr_sha256","executable_sha256_before","executable_sha256_after"],"build")
    if type(build["exit_code"]) is not int or build["exit_code"]!=0 or build["executable_sha256_before"]!=build["executable_sha256_after"] or len(build["argv"])!=7 or build["argv"][:6] != ["cargo","build","--release","--locked","--offline","--manifest-path"] or not absolute_path(build["argv"][6]) or not build["argv"][6].replace('\\','/').endswith('/tools/fixtures/c5-significance-scheduler-receipt/Cargo.toml'): raise ValueError("build or executable stability failed")
    self_test=result["self_test"]
    exact_keys(self_test,["argv","exit_code","stdout_base64","stdout_sha256","stderr_base64","stderr_sha256"],"self test")
    expected_self=b"C5 semantic receipt self-test passed: strict 38-field CBOR, 8 receipt hostiles, 10 pressure transcripts, 92-ID registry, and 10 authority-negative flags.\n"
    if type(self_test["exit_code"]) is not int or self_test["exit_code"]!=0 or len(self_test["argv"])!=2 or self_test["argv"][1]!="--self-test" or not absolute_path(self_test["argv"][0]) or not self_test["argv"][0].replace('\\','/').endswith('/c5-significance-scheduler-receipt') or decoded(self_test["stdout_base64"],self_test["stdout_sha256"])!=expected_self or decoded(self_test["stderr_base64"],self_test["stderr_sha256"]): raise ValueError("self-test evidence failed")
    toolchain=result["toolchain"];exact_keys(toolchain,["rustc_vv_base64","rustc_vv_sha256","cargo_v_base64","cargo_v_sha256","rustc_executable_sha256","cargo_executable_sha256"],"result toolchain");rustc_text=decoded(toolchain["rustc_vv_base64"],toolchain["rustc_vv_sha256"]).decode();cargo_text=decoded(toolchain["cargo_v_base64"],toolchain["cargo_v_sha256"]).decode()
    if any(not isinstance(toolchain[key],str) or len(toolchain[key])!=64 for key in ("rustc_executable_sha256","cargo_executable_sha256")): raise ValueError("toolchain executable hash malformed")
    if not exact_value(result["environment"],{"cargo_net_offline":True,"rustflags":"","rustc":"","rustc_wrapper":"","cargo_build_target":"","cargo_encoded_rustflags":"","cargo_config_present":False,"forbidden_build_env_present":False}): raise ValueError("unapproved execution environment")
    if f"host: {result['platform']['target']}" not in rustc_text or f"release: {request['toolchain']['rustc_release']}" not in rustc_text or f"commit-hash: {request['toolchain']['rustc_commit']}" not in rustc_text or f"cargo {request['toolchain']['cargo_release']} " not in cargo_text: raise ValueError("toolchain identity or native target drift")
    executions=result["executions"]
    if not isinstance(executions,list) or len(executions)!=2 or len({v["pid"] for v in executions})!=2 or len({v["launch_id"] for v in executions})!=2: raise ValueError("two unique launches required")
    payloads=[];intervals=[]
    for index,entry in enumerate(executions,1):
        exact_keys(entry,["ordinal","launch_id","pid","started_utc","ended_utc","argv","exit_code","stdout_base64","stdout_sha256","stderr_base64","stderr_sha256","executable_sha256"],"execution")
        if type(entry["ordinal"]) is not int or entry["ordinal"]!=index or type(entry["pid"]) is not int or entry["pid"]<=0 or type(entry["exit_code"]) is not int or entry["exit_code"]!=0 or entry["executable_sha256"]!=build["executable_sha256_before"]: raise ValueError("execution identity, artifact or exit failed")
        if len(entry["argv"])!=3 or entry["argv"][0]!=self_test["argv"][0] or entry["argv"][1]!="--start-at-unix-ms" or entry["argv"][2]!=executions[0]["argv"][2]: raise ValueError("direct executable argv drift")
        intervals.append((datetime.fromisoformat(entry["started_utc"].replace("Z","+00:00")),datetime.fromisoformat(entry["ended_utc"].replace("Z","+00:00"))))
        if intervals[-1][0]>=intervals[-1][1]: raise ValueError("execution timestamps are invalid")
        stdout=decoded(entry["stdout_base64"],entry["stdout_sha256"]);stderr=decoded(entry["stderr_base64"],entry["stderr_sha256"])
        if stderr or stdout not in (stdout.strip()+b"\n",stdout.strip()+b"\r\n") or len(stdout.strip())%2 or any(c not in b"0123456789abcdef" for c in stdout.strip()): raise ValueError("execution output framing failed")
        payloads.append(bytes.fromhex(stdout.strip().decode()))
    if max(intervals[0][0],intervals[1][0])>=min(intervals[0][1],intervals[1][1]): raise ValueError("launches did not overlap")
    if payloads[0]!=payloads[1] or hashlib.sha256(payloads[0]).hexdigest()!=request["semantic"]["expected_sha256"] or len(payloads[0])>request["semantic"]["max_decoded_bytes"]: raise ValueError("semantic receipt mismatch")
    return {"status":"independence_verified","classification":"independent_platform_execution","request_sha256":request_sha,"result_sha256":result_sha,"source_commit":request["source"]["source_commit"],"semantic_receipt_sha256":request["semantic"]["expected_sha256"],"promotion_authority":False,"activate_c6":False}
def build_retained_receipt(request,result,request_raw,result_raw,bundle_raw):
    derived=validate(request,result,True);repo=request["provenance_policy"]["repository"]
    receipt={
      "schema_version":1,"receipt_id":"G1-C5-INDEPENDENT-PLATFORM-EXECUTION","protocol_id":"G1-C5-INDEPENDENT-PLATFORM-V1",
      "status":derived["status"],"classification":derived["classification"],"request_sha256":derived["request_sha256"],"result_sha256":derived["result_sha256"],
      "source_commit":derived["source_commit"],"semantic_receipt_sha256":derived["semantic_receipt_sha256"],"repository":repo,
      "signer_workflow":f"{repo}/.github/workflows/g1-c5-independent-platform.yml","platform":result["platform"],"hosted_runner":True,
      "request_json_base64":base64.b64encode(request_raw).decode(),"request_json_sha256":hashlib.sha256(request_raw).hexdigest(),
      "result_json_base64":base64.b64encode(result_raw).decode(),"result_json_sha256":hashlib.sha256(result_raw).hexdigest(),
      "attestation_bundle_base64":base64.b64encode(bundle_raw).decode(),"attestation_bundle_sha256":hashlib.sha256(bundle_raw).hexdigest(),
      "authority":{"evidence_only":True,"promotion_authority":False,"activate_c6":False,"repository_mutation":False}}
    receipt["receipt_sha256"]=digest(receipt)
    return receipt
def main():
    ap=argparse.ArgumentParser();ap.add_argument("--request",required=True);ap.add_argument("--result",required=True);ap.add_argument("--bundle",required=True);ap.add_argument("--output",default=str(ROOT/"docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json"));args=ap.parse_args()
    canonical_output=(ROOT/"docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json").resolve()
    if Path(args.output).resolve()!=canonical_output: raise SystemExit("derived receipt output path is fixed by protocol")
    request_raw=bounded_read(args.request,1_048_576,"request JSON");result_raw=bounded_read(args.result,1_048_576,"result JSON");bundle_raw=bounded_read(args.bundle,4_194_304,"attestation bundle")
    req=strict_load_bytes(request_raw);res=strict_load_bytes(result_raw);source=req.get("source",{});commit=source.get("source_commit","");local=strict_load(LOCAL)
    validate_frozen_request(req,local,True)
    gh=shutil.which("gh")
    if not gh: raise SystemExit("GitHub CLI is required for cryptographic attestation verification")
    repo=res.get("provenance",{}).get("repository","")
    repo=req["provenance_policy"]["repository"];signer=f"{repo}/.github/workflows/g1-c5-independent-platform.yml"
    with tempfile.TemporaryDirectory(prefix="forge-c5-import-") as td:
        request_snapshot=Path(td)/"request.json";result_snapshot=Path(td)/"result.json";bundle_snapshot=Path(td)/"bundle.jsonl"
        request_snapshot.write_bytes(request_raw);result_snapshot.write_bytes(result_raw);bundle_snapshot.write_bytes(bundle_raw)
        command=[gh,"attestation","verify",str(result_snapshot),"--repo",repo,"--bundle",str(bundle_snapshot),"--deny-self-hosted-runners","--source-digest",commit,"--signer-workflow",signer,"--signer-digest",commit,"--format","json"]
        verified=subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    if verified.returncode: raise SystemExit(f"attestation verification failed: {verified.stderr.strip()}")
    receipt=build_retained_receipt(req,res,request_raw,result_raw,bundle_raw)
    out=Path(args.output);out.parent.mkdir(parents=True,exist_ok=True)
    try:
        with out.open("x",encoding="utf-8",newline="\n") as handle: handle.write(json.dumps(receipt,sort_keys=True,indent=2)+"\n")
    except FileExistsError: raise SystemExit("refusing duplicate import over the canonical derived receipt")
    print(f"C5 independent platform evidence verified: {receipt['result_sha256']}")
if __name__=="__main__": main()
