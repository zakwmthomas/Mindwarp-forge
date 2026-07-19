#!/usr/bin/env python3
"""Replay and verify the retained C4 GitHub attestation package."""
from __future__ import annotations
import argparse,base64,hashlib,importlib.util,json,shutil,subprocess,tempfile
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
IMPORTER_PATH=Path(__file__).with_name("verify-g1-c4-external-receipt.py")
spec=importlib.util.spec_from_file_location("c4_external_importer",IMPORTER_PATH);v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
CANONICAL=ROOT/"docs/canonical-system/G1_C4_INDEPENDENT_PLATFORM_EXECUTION.json"

def load_retained(path):
    raw=v.bounded_read(path,9_437_184,"retained receipt")
    def pairs(items):
        out={}
        for key,value in items:
            if key in out: raise ValueError(f"duplicate JSON key: {key}")
            out[key]=value
        return out
    value=json.loads(raw.decode("utf-8"),object_pairs_hook=pairs,parse_float=lambda _:(_ for _ in ()).throw(ValueError("floats forbidden")),parse_constant=lambda _:(_ for _ in ()).throw(ValueError("non-finite values forbidden")))
    if raw!=(json.dumps(value,sort_keys=True,indent=2)+"\n").encode(): raise ValueError("retained receipt JSON is not canonical")
    return value
def decode_exact(value,claimed,label,max_bytes):
    if not isinstance(value,str) or not isinstance(claimed,str) or len(claimed)!=64: raise ValueError(f"{label} encoding or hash malformed")
    raw=base64.b64decode(value,validate=True)
    if len(raw)>max_bytes or hashlib.sha256(raw).hexdigest()!=claimed: raise ValueError(f"{label} bytes or hash changed")
    return raw
def check_source(request,local):
    commit=request["source"]["source_commit"]
    if v.tree_manifest(commit)!=local["tracked_tree_manifest_sha256"] or v.source_manifest(commit)!=local["bounded_source_manifest_sha256"]: raise ValueError("retained source Git objects changed")
    manifest,lock,dependency=v.source_artifact_bindings(commit)
    if (manifest,lock,dependency)!=(request["source"]["fixture_manifest_sha256"],request["source"]["fixture_lock_sha256"],request["source"]["dependency_graph_sha256"]): raise ValueError("retained fixture, lock or dependency graph changed")
    completed=subprocess.run(["git","merge-base","--is-ancestor",commit,"HEAD"],cwd=ROOT)
    if completed.returncode: raise ValueError("retained source commit is not an ancestor of HEAD")
def verify_retained(receipt,*,local=None,runner=subprocess.run,gh_path=None,source_checker=check_source,verify_git=True):
    keys=["schema_version","receipt_id","protocol_id","status","classification","request_sha256","result_sha256","source_commit","semantic_receipt_sha256","repository","signer_workflow","platform","hosted_runner","request_json_base64","request_json_sha256","result_json_base64","result_json_sha256","attestation_bundle_base64","attestation_bundle_sha256","authority","receipt_sha256"]
    v.exact_keys(receipt,keys,"retained receipt")
    claimed=receipt["receipt_sha256"];unsigned=dict(receipt);unsigned.pop("receipt_sha256")
    if not isinstance(claimed,str) or len(claimed)!=64 or v.digest(unsigned)!=claimed: raise ValueError("retained receipt self hash changed")
    request_raw=decode_exact(receipt["request_json_base64"],receipt["request_json_sha256"],"request JSON",1_048_576)
    result_raw=decode_exact(receipt["result_json_base64"],receipt["result_json_sha256"],"result JSON",1_048_576)
    bundle_raw=decode_exact(receipt["attestation_bundle_base64"],receipt["attestation_bundle_sha256"],"attestation bundle",4_194_304)
    with tempfile.TemporaryDirectory(prefix="forge-c4-retained-") as td:
        request_path=Path(td)/"request.json";result_path=Path(td)/"result.json";bundle_path=Path(td)/"bundle.jsonl"
        request_path.write_bytes(request_raw);result_path.write_bytes(result_raw);bundle_path.write_bytes(bundle_raw)
        request=v.strict_load(request_path);result=v.strict_load(result_path)
        local=local if local is not None else v.strict_load(v.LOCAL)
        v.validate_frozen_request(request,local,False,verify_git=verify_git)
        derived=v.validate(request,result,True)
        source=request["source"];repo=request["provenance_policy"]["repository"];signer=f"{repo}/.github/workflows/g1-c4-independent-platform.yml"
        if local["independent_second_platform_execution"] is not False or local["promotion_authority"] is not False: raise ValueError("local reference receipt gained authority")
        if source["source_commit"]!=local["source_commit"] or source["tracked_tree_manifest_sha256"]!=local["tracked_tree_manifest_sha256"] or source["bounded_source_manifest_sha256"]!=local["bounded_source_manifest_sha256"] or request["semantic"]["expected_sha256"]!=local["semantic_receipt_sha256"]: raise ValueError("retained package is not bound to the local observation")
        if verify_git: source_checker(request,local)
        summary={"schema_version":1,"receipt_id":"G1-C4-INDEPENDENT-PLATFORM-EXECUTION","protocol_id":"G1-C4-INDEPENDENT-PLATFORM-V1","status":derived["status"],"classification":derived["classification"],"request_sha256":derived["request_sha256"],"result_sha256":derived["result_sha256"],"source_commit":derived["source_commit"],"semantic_receipt_sha256":derived["semantic_receipt_sha256"],"repository":repo,"signer_workflow":signer,"platform":result["platform"],"hosted_runner":True,"authority":{"evidence_only":True,"promotion_authority":False,"activate_c5":False,"repository_mutation":False}}
        for key,value in summary.items():
            if receipt[key]!=value: raise ValueError(f"retained summary changed: {key}")
        gh_path=gh_path or shutil.which("gh")
        if not gh_path: raise ValueError("GitHub CLI is required to replay the retained attestation")
        command=[gh_path,"attestation","verify",str(result_path),"--repo",repo,"--bundle",str(bundle_path),"--deny-self-hosted-runners","--source-digest",source["source_commit"],"--signer-workflow",signer,"--signer-digest",source["source_commit"],"--format","json"]
        replay=runner(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        if replay.returncode: raise ValueError(f"retained attestation replay failed: {replay.stderr.strip()}")
    return receipt
def main():
    ap=argparse.ArgumentParser();ap.add_argument("--receipt",default=str(CANONICAL));args=ap.parse_args()
    if Path(args.receipt).resolve()!=CANONICAL.resolve(): raise SystemExit("retained receipt path is fixed by protocol")
    if not CANONICAL.is_file(): raise SystemExit("C4 independent-platform receipt is missing")
    receipt=verify_retained(load_retained(CANONICAL))
    print(f"C4 retained independent-platform attestation replay verified: {receipt['result_sha256']}")
if __name__=="__main__": main()
