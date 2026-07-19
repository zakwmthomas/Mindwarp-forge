#!/usr/bin/env python3
"""Strictly validate C4 external evidence; only the CLI can grant attestation_verified."""
from __future__ import annotations
import argparse, base64, hashlib, json, shutil, subprocess
from datetime import datetime
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
PATHS=ROOT/"tools/fixtures/c4-hierarchy-history-receipt/bounded-paths.txt"
LOCAL=ROOT/"docs/canonical-system/G1_C4_LOCAL_PLATFORM_OBSERVATIONS.json"

def strict_load(path):
    def pairs(items):
        out={}
        for key,value in items:
            if key in out: raise ValueError(f"duplicate JSON key: {key}")
            out[key]=value
        return out
    raw=Path(path).read_bytes()
    if len(raw)>1_048_576: raise ValueError("JSON input exceeds 1 MiB")
    return json.loads(raw.decode("utf-8-sig"),object_pairs_hook=pairs,parse_float=lambda _:(_ for _ in ()).throw(ValueError("floats forbidden")),parse_constant=lambda _:(_ for _ in ()).throw(ValueError("non-finite values forbidden")))
def canonical(v): return json.dumps(v,sort_keys=True,separators=(",",":"),ensure_ascii=True).encode()
def digest(v): return hashlib.sha256(canonical(v)).hexdigest()
def exact_keys(value,keys,label):
    if set(value)!=set(keys): raise ValueError(f"{label} fields changed")
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
    rows=[]
    for relative in PATHS.read_text(encoding="utf-8").splitlines():
        if relative:
            blob=subprocess.check_output(["git","rev-parse",f"{commit}:{relative}"],cwd=ROOT,text=True).strip();rows.append(f"{relative}:{blob}")
    return hashlib.sha256("\n".join(rows).encode()).hexdigest()
def tree_manifest(commit):
    raw=subprocess.check_output(["git","ls-tree","-r","--full-tree",commit],cwd=ROOT).decode().rstrip()
    return hashlib.sha256(raw.encode()).hexdigest()
def validate(request,result,attestation_verified):
    exact_keys(request,["schema_version","protocol_id","request_id","challenge","created_utc","source","semantic","toolchain","allowed_platforms","execution_policy","provenance_policy","reference_platform","authority","request_sha256"],"request")
    request_sha=unhash(request,"request_sha256")
    exact_keys(request["source"],["repository_id","source_commit","tracked_tree_manifest_sha256","bounded_source_manifest_sha256","fixture_manifest_sha256","fixture_lock_sha256","dependency_graph_sha256"],"request source")
    exact_keys(request["semantic"],["receipt_id","expected_sha256","encoding","max_decoded_bytes"],"request semantic")
    exact_keys(request["toolchain"],["rustc_release","rustc_commit","cargo_release"],"request toolchain")
    exact_keys(request["execution_policy"],["provider","hosted_runner_required","native_execution_required","compile_only_forbidden","process_count","concurrent","direct_executable","clean_before_after"],"request execution policy")
    exact_keys(request["provenance_policy"],["repository","workflow_path","signer_workflow"],"request provenance policy")
    exact_keys(request["reference_platform"],["os","architecture","receipt_id"],"request reference platform")
    exact_keys(request["authority"],["evidence_only","promotion_authority","activate_c5","repository_mutation"],"request authority")
    for platform_row in request["allowed_platforms"]: exact_keys(platform_row,["target","os","architecture","pointer_width","endian"],"allowed platform")
    repo=request["provenance_policy"]["repository"];expected_signer=f"{repo}/.github/workflows/g1-c4-independent-platform.yml"
    if request["schema_version"]!=1 or request["protocol_id"]!="G1-C4-INDEPENDENT-PLATFORM-V1" or request["execution_policy"]!={"provider":"github-actions","hosted_runner_required":True,"native_execution_required":True,"compile_only_forbidden":True,"process_count":2,"concurrent":True,"direct_executable":True,"clean_before_after":True} or request["provenance_policy"]!={"repository":repo,"workflow_path":".github/workflows/g1-c4-independent-platform.yml","signer_workflow":expected_signer} or request["reference_platform"]!={"os":"windows","architecture":"x86_64","receipt_id":"G1-C4-LOCAL-PLATFORM-OBSERVATIONS"} or request["authority"]!={"evidence_only":True,"promotion_authority":False,"activate_c5":False,"repository_mutation":False}: raise ValueError("request protocol, signer or authority policy changed")
    exact_keys(result,["schema_version","protocol_id","request_id","request_sha256","challenge","result_id","source","platform","provenance","toolchain","environment","build","self_test","executions","claims","result_sha256"],"result")
    result_sha=unhash(result,"result_sha256")
    for key in ("schema_version","protocol_id","request_id","challenge"):
        if result[key]!=request[key]: raise ValueError(f"request/result {key} mismatch")
    if result["request_sha256"]!=request_sha: raise ValueError("request binding mismatch")
    source=dict(result["source"]);clean_before=source.pop("clean_before",None);clean_after=source.pop("clean_after",None)
    if source!=request["source"] or clean_before is not True or clean_after is not True: raise ValueError("source binding or cleanliness mismatch")
    if result["platform"] not in request["allowed_platforms"] or result["platform"]["os"]==request["reference_platform"]["os"]: raise ValueError("platform is not independently diverse")
    exact_keys(result["platform"],["target","os","architecture","pointer_width","endian"],"result platform")
    p=result["provenance"];exact_keys(p,["kind","provider","hosted_runner","run_id","run_attempt","job_id","job_url","repository","workflow_ref","runner_image","attestation_required"],"provenance")
    if p["kind"]!="provider_hosted_ci" or p["provider"]!="github-actions" or p["hosted_runner"] is not True or p["attestation_required"] is not True or p["repository"]!=request["provenance_policy"]["repository"] or not attestation_verified: raise ValueError("provider attestation is not verified")
    if "/.github/workflows/g1-c4-independent-platform.yml@" not in p["workflow_ref"]: raise ValueError("workflow identity drift")
    claims=result["claims"]
    if claims!={"evidence_only":True,"promotion_authority":False,"activate_c5":False,"repository_mutation":False,"independent_platform_execution_claimed":True}: raise ValueError("authority claim changed")
    build=result["build"]
    exact_keys(build,["argv","exit_code","stdout_sha256","stderr_sha256","executable_sha256_before","executable_sha256_after"],"build")
    if build["exit_code"]!=0 or build["executable_sha256_before"]!=build["executable_sha256_after"] or len(build["argv"])!=7 or build["argv"][:6] != ["cargo","build","--release","--locked","--offline","--manifest-path"] or not absolute_path(build["argv"][6]) or not build["argv"][6].replace('\\','/').endswith('/tools/fixtures/c4-hierarchy-history-receipt/Cargo.toml'): raise ValueError("build or executable stability failed")
    self_test=result["self_test"]
    exact_keys(self_test,["argv","exit_code","stdout_base64","stdout_sha256","stderr_base64","stderr_sha256"],"self test")
    expected_self=b"C4 semantic receipt self-test passed: 8 receipt hostiles, 74-ID registry, exact C2+C3A replay and authority-negative bytes.\n"
    if self_test["exit_code"]!=0 or len(self_test["argv"])!=2 or self_test["argv"][1]!="--self-test" or not absolute_path(self_test["argv"][0]) or not self_test["argv"][0].replace('\\','/').endswith('/c4-hierarchy-history-receipt') or decoded(self_test["stdout_base64"],self_test["stdout_sha256"])!=expected_self or decoded(self_test["stderr_base64"],self_test["stderr_sha256"]): raise ValueError("self-test evidence failed")
    toolchain=result["toolchain"];exact_keys(toolchain,["rustc_vv_base64","rustc_vv_sha256","cargo_v_base64","cargo_v_sha256","rustc_executable_sha256","cargo_executable_sha256"],"result toolchain");rustc_text=decoded(toolchain["rustc_vv_base64"],toolchain["rustc_vv_sha256"]).decode();cargo_text=decoded(toolchain["cargo_v_base64"],toolchain["cargo_v_sha256"]).decode()
    if any(not isinstance(toolchain[key],str) or len(toolchain[key])!=64 for key in ("rustc_executable_sha256","cargo_executable_sha256")): raise ValueError("toolchain executable hash malformed")
    if result["environment"]!={"cargo_net_offline":True,"rustflags":"","rustc":"","rustc_wrapper":"","cargo_build_target":"","cargo_encoded_rustflags":"","cargo_config_present":False,"forbidden_build_env_present":False}: raise ValueError("unapproved execution environment")
    if f"host: {result['platform']['target']}" not in rustc_text or f"release: {request['toolchain']['rustc_release']}" not in rustc_text or f"commit-hash: {request['toolchain']['rustc_commit']}" not in rustc_text or f"cargo {request['toolchain']['cargo_release']} " not in cargo_text: raise ValueError("toolchain identity or native target drift")
    executions=result["executions"]
    if not isinstance(executions,list) or len(executions)!=2 or len({v["pid"] for v in executions})!=2 or len({v["launch_id"] for v in executions})!=2: raise ValueError("two unique launches required")
    payloads=[];intervals=[]
    for index,entry in enumerate(executions,1):
        exact_keys(entry,["ordinal","launch_id","pid","started_utc","ended_utc","argv","exit_code","stdout_base64","stdout_sha256","stderr_base64","stderr_sha256","executable_sha256"],"execution")
        if entry["ordinal"]!=index or type(entry["pid"]) is not int or entry["pid"]<=0 or entry["exit_code"]!=0 or entry["executable_sha256"]!=build["executable_sha256_before"]: raise ValueError("execution identity, artifact or exit failed")
        if len(entry["argv"])!=3 or entry["argv"][0]!=self_test["argv"][0] or entry["argv"][1]!="--start-at-unix-ms" or entry["argv"][2]!=executions[0]["argv"][2]: raise ValueError("direct executable argv drift")
        intervals.append((datetime.fromisoformat(entry["started_utc"].replace("Z","+00:00")),datetime.fromisoformat(entry["ended_utc"].replace("Z","+00:00"))))
        if intervals[-1][0]>=intervals[-1][1]: raise ValueError("execution timestamps are invalid")
        stdout=decoded(entry["stdout_base64"],entry["stdout_sha256"]);stderr=decoded(entry["stderr_base64"],entry["stderr_sha256"])
        if stderr or stdout not in (stdout.strip()+b"\n",stdout.strip()+b"\r\n") or len(stdout.strip())%2 or any(c not in b"0123456789abcdef" for c in stdout.strip()): raise ValueError("execution output framing failed")
        payloads.append(bytes.fromhex(stdout.strip().decode()))
    if max(intervals[0][0],intervals[1][0])>=min(intervals[0][1],intervals[1][1]): raise ValueError("launches did not overlap")
    if payloads[0]!=payloads[1] or hashlib.sha256(payloads[0]).hexdigest()!=request["semantic"]["expected_sha256"] or len(payloads[0])>request["semantic"]["max_decoded_bytes"]: raise ValueError("semantic receipt mismatch")
    return {"status":"independence_verified","classification":"independent_platform_execution","request_sha256":request_sha,"result_sha256":result_sha,"source_commit":request["source"]["source_commit"],"semantic_receipt_sha256":request["semantic"]["expected_sha256"],"promotion_authority":False,"activate_c5":False}
def main():
    ap=argparse.ArgumentParser();ap.add_argument("--request",required=True);ap.add_argument("--result",required=True);ap.add_argument("--bundle",required=True);ap.add_argument("--output",default=str(ROOT/"docs/canonical-system/G1_C4_INDEPENDENT_PLATFORM_EXECUTION.json"));args=ap.parse_args()
    canonical_output=(ROOT/"docs/canonical-system/G1_C4_INDEPENDENT_PLATFORM_EXECUTION.json").resolve()
    if Path(args.output).resolve()!=canonical_output: raise SystemExit("derived receipt output path is fixed by protocol")
    if canonical_output.exists(): raise SystemExit("refusing duplicate import over the canonical derived receipt")
    req=strict_load(args.request);res=strict_load(args.result);source=req.get("source",{});commit=source.get("source_commit","");local=strict_load(LOCAL)
    expected_platforms=[{"target":"x86_64-unknown-linux-gnu","os":"linux","architecture":"x86_64","pointer_width":64,"endian":"little"},{"target":"x86_64-apple-darwin","os":"macos","architecture":"x86_64","pointer_width":64,"endian":"little"},{"target":"aarch64-apple-darwin","os":"macos","architecture":"aarch64","pointer_width":64,"endian":"little"}]
    expected_policy={"provider":"github-actions","hosted_runner_required":True,"native_execution_required":True,"compile_only_forbidden":True,"process_count":2,"concurrent":True,"direct_executable":True,"clean_before_after":True}
    if req.get("schema_version")!=1 or req.get("protocol_id")!="G1-C4-INDEPENDENT-PLATFORM-V1" or req.get("semantic")!={"receipt_id":"G1-C4-HIERARCHY-HISTORY","expected_sha256":local["semantic_receipt_sha256"],"encoding":"lowercase_hex","max_decoded_bytes":65536} or req.get("toolchain")!={"rustc_release":"1.97.0","rustc_commit":"2d8144b7880597b6e6d3dfd63a9a9efae3f533d3","cargo_release":"1.97.0"} or req.get("allowed_platforms")!=expected_platforms or req.get("execution_policy")!=expected_policy or req.get("reference_platform")!={"os":"windows","architecture":"x86_64","receipt_id":"G1-C4-LOCAL-PLATFORM-OBSERVATIONS"} or req.get("authority")!={"evidence_only":True,"promotion_authority":False,"activate_c5":False,"repository_mutation":False}: raise SystemExit("request policy is not the frozen C4 external protocol")
    if source.get("source_commit")!=local["source_commit"] or source.get("bounded_source_manifest_sha256")!=local["bounded_source_manifest_sha256"]: raise SystemExit("request is not bound to the retained local C4 receipt")
    created=datetime.fromisoformat(req["created_utc"].replace("Z","+00:00"));now=datetime.now(created.tzinfo)
    if created.tzinfo is None or not (0<=(now-created).total_seconds()<=86400): raise SystemExit("request challenge is outside the 24-hour freshness window")
    if tree_manifest(commit)!=source.get("tracked_tree_manifest_sha256") or source_manifest(commit)!=source.get("bounded_source_manifest_sha256"): raise SystemExit("request source provenance does not match the local repository")
    if hashlib.sha256((ROOT/"tools/fixtures/c4-hierarchy-history-receipt/Cargo.toml").read_bytes()).hexdigest()!=source.get("fixture_manifest_sha256") or hashlib.sha256((ROOT/"tools/fixtures/c4-hierarchy-history-receipt/Cargo.lock").read_bytes()).hexdigest()!=source.get("fixture_lock_sha256"): raise SystemExit("fixture or lock binding changed")
    gh=shutil.which("gh")
    if not gh: raise SystemExit("GitHub CLI is required for cryptographic attestation verification")
    repo=res.get("provenance",{}).get("repository","")
    repo=req["provenance_policy"]["repository"];signer=f"{repo}/.github/workflows/g1-c4-independent-platform.yml"
    command=[gh,"attestation","verify",args.result,"--repo",repo,"--bundle",args.bundle,"--deny-self-hosted-runners","--source-digest",commit,"--signer-workflow",signer,"--signer-digest",commit,"--format","json"]
    verified=subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    if verified.returncode: raise SystemExit(f"attestation verification failed: {verified.stderr.strip()}")
    receipt=validate(req,res,True);receipt["attestation_verification_sha256"]=hashlib.sha256(verified.stdout.encode()).hexdigest()
    out=Path(args.output);out.parent.mkdir(parents=True,exist_ok=True);out.write_text(json.dumps(receipt,sort_keys=True,indent=2)+"\n",encoding="utf-8")
    print(f"C4 independent platform evidence verified: {receipt['result_sha256']}")
if __name__=="__main__": main()
