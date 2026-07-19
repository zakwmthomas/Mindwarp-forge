#!/usr/bin/env python3
"""Hostile fixtures for the authority-negative C4 external receipt importer."""
from __future__ import annotations
import base64, copy, hashlib, importlib.util, json, tempfile
from pathlib import Path

MODULE=Path(__file__).with_name("verify-g1-c4-external-receipt.py")
spec=importlib.util.spec_from_file_location("c4verify",MODULE);v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
def b64(raw): return base64.b64encode(raw).decode()
def hashed(value,name): value[name]=v.digest(value);return value
def fixtures():
    payload=b"c4-independent-payload";line=payload.hex().encode()+b"\n";empty=b""
    request={"schema_version":1,"protocol_id":"G1-C4-INDEPENDENT-PLATFORM-V1","request_id":"g1-c4-"+("c"*16),"challenge":"c"*64,"created_utc":"2026-07-19T00:00:00Z",
      "source":{"repository_id":"mindwarp-forge","source_commit":"a"*40,"tracked_tree_manifest_sha256":"b"*64,"bounded_source_manifest_sha256":"c"*64,"fixture_manifest_sha256":"d"*64,"fixture_lock_sha256":"e"*64,"dependency_graph_sha256":"1"*64},
      "semantic":{"receipt_id":"G1-C4-HIERARCHY-HISTORY","expected_sha256":hashlib.sha256(payload).hexdigest(),"encoding":"lowercase_hex","max_decoded_bytes":65536},
      "toolchain":{"rustc_release":"1.97.0","rustc_commit":"2d8144b7880597b6e6d3dfd63a9a9efae3f533d3","cargo_release":"1.97.0"},
      "allowed_platforms":[{"target":"x86_64-unknown-linux-gnu","os":"linux","architecture":"x86_64","pointer_width":64,"endian":"little"},{"target":"x86_64-apple-darwin","os":"macos","architecture":"x86_64","pointer_width":64,"endian":"little"},{"target":"aarch64-apple-darwin","os":"macos","architecture":"aarch64","pointer_width":64,"endian":"little"}],
      "execution_policy":{"provider":"github-actions","hosted_runner_required":True,"native_execution_required":True,"compile_only_forbidden":True,"process_count":2,"concurrent":True,"direct_executable":True,"clean_before_after":True},
      "provenance_policy":{"repository":"x/y","workflow_path":".github/workflows/g1-c4-independent-platform.yml","signer_workflow":"x/y/.github/workflows/g1-c4-independent-platform.yml"},
      "reference_platform":{"os":"windows","architecture":"x86_64","receipt_id":"G1-C4-LOCAL-PLATFORM-OBSERVATIONS"},
      "authority":{"evidence_only":True,"promotion_authority":False,"activate_c5":False,"repository_mutation":False}}
    hashed(request,"request_sha256")
    execution=lambda n,p,start,end:{"ordinal":n,"launch_id":f"l{n}","pid":p,"started_utc":start,"ended_utc":end,"argv":["/tmp/target/c4-hierarchy-history-receipt","--start-at-unix-ms","1"],"exit_code":0,"stdout_base64":b64(line),"stdout_sha256":hashlib.sha256(line).hexdigest(),"stderr_base64":b64(empty),"stderr_sha256":hashlib.sha256(empty).hexdigest(),"executable_sha256":"f"*64}
    result={"schema_version":1,"protocol_id":request["protocol_id"],"request_id":request["request_id"],"request_sha256":request["request_sha256"],"challenge":request["challenge"],"result_id":"x",
      "source":dict(request["source"],clean_before=True,clean_after=True),"platform":copy.deepcopy(request["allowed_platforms"][0]),
      "provenance":{"kind":"provider_hosted_ci","provider":"github-actions","hosted_runner":True,"run_id":"1","run_attempt":"1","job_id":"j","job_url":"https://github/x/actions/runs/1","repository":"x/y","workflow_ref":"x/y/.github/workflows/g1-c4-independent-platform.yml@refs/heads/main","runner_image":"ubuntu24","attestation_required":True},
      "toolchain":{"rustc_vv_base64":b64(b"rustc\ncommit-hash: 2d8144b7880597b6e6d3dfd63a9a9efae3f533d3\nhost: x86_64-unknown-linux-gnu\nrelease: 1.97.0\n"),"rustc_vv_sha256":hashlib.sha256(b"rustc\ncommit-hash: 2d8144b7880597b6e6d3dfd63a9a9efae3f533d3\nhost: x86_64-unknown-linux-gnu\nrelease: 1.97.0\n").hexdigest(),"cargo_v_base64":b64(b"cargo 1.97.0 (fixture)\n"),"cargo_v_sha256":hashlib.sha256(b"cargo 1.97.0 (fixture)\n").hexdigest(),"rustc_executable_sha256":"2"*64,"cargo_executable_sha256":"3"*64},
      "environment":{"cargo_net_offline":True,"rustflags":"","rustc":"","rustc_wrapper":"","cargo_build_target":"","cargo_encoded_rustflags":"","cargo_config_present":False,"forbidden_build_env_present":False},
      "build":{"argv":["cargo","build","--release","--locked","--offline","--manifest-path","/repo/tools/fixtures/c4-hierarchy-history-receipt/Cargo.toml"],"exit_code":0,"stdout_sha256":"0"*64,"stderr_sha256":"0"*64,"executable_sha256_before":"f"*64,"executable_sha256_after":"f"*64},
      "self_test":{"argv":["/tmp/target/c4-hierarchy-history-receipt","--self-test"],"exit_code":0,"stdout_base64":b64(b"C4 semantic receipt self-test passed: 8 receipt hostiles, 74-ID registry, exact C2+C3A replay and authority-negative bytes.\n"),"stdout_sha256":hashlib.sha256(b"C4 semantic receipt self-test passed: 8 receipt hostiles, 74-ID registry, exact C2+C3A replay and authority-negative bytes.\n").hexdigest(),"stderr_base64":b64(empty),"stderr_sha256":hashlib.sha256(empty).hexdigest()},
      "executions":[execution(1,11,"2026-07-19T00:00:00Z","2026-07-19T00:00:02Z"),execution(2,12,"2026-07-19T00:00:01Z","2026-07-19T00:00:03Z")],"claims":{"evidence_only":True,"promotion_authority":False,"activate_c5":False,"repository_mutation":False,"independent_platform_execution_claimed":True}}
    hashed(result,"result_sha256");return request,result
def reseal(value,name): value.pop(name,None);hashed(value,name)
def reject(name,mutate,attested=True):
    request,result=fixtures();mutate(request,result);reseal(request,"request_sha256");result["request_sha256"]=request["request_sha256"];reseal(result,"result_sha256")
    local={"source_commit":request["source"]["source_commit"],"tracked_tree_manifest_sha256":request["source"]["tracked_tree_manifest_sha256"],"bounded_source_manifest_sha256":request["source"]["bounded_source_manifest_sha256"],"semantic_receipt_sha256":request["semantic"]["expected_sha256"]}
    try:v.validate_frozen_request(request,local,False,verify_git=False);v.validate(request,result,attested)
    except Exception:return
    raise AssertionError(f"hostile admitted: {name}")
def main():
    request,result=fixtures();local={"source_commit":request["source"]["source_commit"],"tracked_tree_manifest_sha256":request["source"]["tracked_tree_manifest_sha256"],"bounded_source_manifest_sha256":request["source"]["bounded_source_manifest_sha256"],"semantic_receipt_sha256":request["semantic"]["expected_sha256"]};v.validate_frozen_request(request,local,False,verify_git=False);assert v.validate(request,result,True)["classification"]=="independent_platform_execution"
    cases={
      "independence.self-asserted-host":lambda q,r:r["provenance"].update(hosted_runner=False),
      "independence.self-hosted-provider":lambda q,r:r["provenance"].update(kind="self_hosted_ci"),
      "execution.compile-only":lambda q,r:r["claims"].update(independent_platform_execution_claimed=False),
      "execution.echo-known-receipt":lambda q,r:r["executions"][0].update(stdout_base64=b64(b"00\n")),
      "execution.single-launch-replayed":lambda q,r:r["executions"][1].update(pid=11),
      "execution.sequential-masquerade":lambda q,r:r["executions"][1].update(started_utc="2026-07-19T00:00:03Z",ended_utc="2026-07-19T00:00:04Z"),
      "replay.old-envelope-new-challenge":lambda q,r:r.update(challenge="d"*64),
      "request.empty-challenge":lambda q,r:q.update(challenge="",request_id="g1-c4-"),
      "request.short-challenge":lambda q,r:q.update(challenge="c"*62,request_id="g1-c4-"+("c"*16)),
      "request.uppercase-challenge":lambda q,r:q.update(challenge="C"*64,request_id="g1-c4-"+("C"*16)),
      "request.nonhex-challenge":lambda q,r:q.update(challenge="z"*64,request_id="g1-c4-"+("z"*16)),
      "request.challenge-type":lambda q,r:q.update(challenge=7,request_id="g1-c4-7"),
      "source.dirty-tracked":lambda q,r:r["source"].update(clean_before=False),
      "source.commit-tree-mismatch":lambda q,r:r["source"].update(source_commit="9"*40),
      "dependency.lock-drift":lambda q,r:r["source"].update(fixture_lock_sha256="9"*64),
      "runner.workflow-drift":lambda q,r:r["provenance"].update(workflow_ref="x/y/.github/workflows/other.yml@main"),
      "request.signer-workflow-drift":lambda q,r:q["provenance_policy"].update(signer_workflow="x/y/.github/workflows/other.yml"),
      "toolchain.version-drift":lambda q,r:q["toolchain"].update(rustc_release="1.96.0"),
      "runner.environment-drift":lambda q,r:r["environment"].update(rustflags="-Ctarget-cpu=native"),
      "artifact.swap-after-hash":lambda q,r:r["build"].update(executable_sha256_after="9"*64),
      "artifact.run-hash-drift":lambda q,r:r["executions"][0].update(executable_sha256="9"*64),
      "target.claim-runtime-mismatch":lambda q,r:r["platform"].update(target="aarch64-linux-android"),
      "target.same-platform":lambda q,r:r["platform"].update(os="windows"),
      "receipt.type-coercion":lambda q,r:r["executions"][0].update(pid="11"),
      "receipt.unknown-field":lambda q,r:r.update(extra=True),
      "receipt.nested-unknown-field":lambda q,r:r["build"].update(extra=True),
      "receipt.boolean-pid":lambda q,r:r["executions"][0].update(pid=True),
      "runner.argv-extra":lambda q,r:r["build"]["argv"].append("--features=x"),
      "request.compile-only-admission":lambda q,r:q["execution_policy"].update(compile_only_forbidden=False),
      "request.authority-flip":lambda q,r:q["authority"].update(promotion_authority=True),
      "attestation.bad-signature":lambda q,r:None,
      "authority.promotion-flip":lambda q,r:r["claims"].update(promotion_authority=True),
      "authority.c5-flip":lambda q,r:r["claims"].update(activate_c5=True),
      "output.stderr":lambda q,r:r["executions"][0].update(stderr_base64=b64(b"bad"),stderr_sha256=hashlib.sha256(b"bad").hexdigest()),
      "output.extra-line":lambda q,r:r["executions"][0].update(stdout_base64=b64(b"00\n00\n"),stdout_sha256=hashlib.sha256(b"00\n00\n").hexdigest())}
    for name,mutation in cases.items():reject(name,mutation,attested=name!="attestation.bad-signature")
    with tempfile.TemporaryDirectory() as td:
        path=Path(td)/"duplicate.json";path.write_text('{"a":1,"a":2}',encoding="utf-8")
        try:v.strict_load(path);raise AssertionError("receipt.duplicate-key admitted")
        except ValueError:pass
    workflow=(Path(__file__).resolve().parents[1]/".github/workflows/g1-c4-independent-platform.yml").read_text(encoding="utf-8")
    for required in ("actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5","actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02","actions/download-artifact@d3f86a106a0bac45b974a628896c90dbdf5c8093","actions/attest@f7c74d28b9d84cb8768d0b8ca14a4bac6ef463e6","artifact-metadata: write","${{ steps.attest.outputs.bundle-path }}","REQUEST_BASE64: ${{ inputs.request_base64 }}","printf '%s' \"$REQUEST_BASE64\""):
        if required not in workflow:raise AssertionError(f"workflow control missing: {required}")
    if "printf '%s' '${{ inputs.request_base64 }}'" in workflow:raise AssertionError("workflow input shell injection returned")
    verifier=MODULE.read_text(encoding="utf-8")
    for required in ("--deny-self-hosted-runners","--source-digest","--signer-workflow","--signer-digest"):
        if required not in verifier:raise AssertionError(f"attestation policy missing: {required}")
    if "--custom-trusted-root" in verifier:raise AssertionError("caller-selected trust root returned")
    print(f"C4 external receipt importer verified: {len(cases)+1} hostile cases and pinned workflow controls pass.")
if __name__=="__main__":main()
