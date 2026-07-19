#!/usr/bin/env python3
"""Generate one fresh authority-negative C5 independent-run request."""
from __future__ import annotations
import argparse, hashlib, json, re, secrets, subprocess, tomllib
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LOCAL = ROOT / "docs/canonical-system/G1_C5_LOCAL_PLATFORM_OBSERVATIONS.json"
PATHS = ROOT / "tools/fixtures/c5-significance-scheduler-receipt/bounded-paths.txt"

def run(*args: str) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()

def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode()

def digest(value: object) -> str:
    return hashlib.sha256(canonical(value)).hexdigest()

def canonical_tree_bytes(raw: bytes) -> bytes:
    raw = raw.replace(b"\r\n", b"\n")
    if b"\r" in raw or not raw.endswith(b"\n"):
        raise ValueError("git ls-tree output is not canonical line-delimited data")
    return raw[:-1]

def tree_sha(commit: str) -> str:
    tree = subprocess.check_output(["git", "-c", "core.quotePath=true", "ls-tree", "-r", "--full-tree", commit], cwd=ROOT)
    return hashlib.sha256(canonical_tree_bytes(tree)).hexdigest()

def manifest_sha(commit: str) -> str:
    rows=[]
    for relative in PATHS.read_text(encoding="utf-8").splitlines():
        if relative:
            rows.append(f"{relative}:{run('git','rev-parse',f'{commit}:{relative}')}")
    return hashlib.sha256("\n".join(rows).encode()).hexdigest()
def dependency_sha() -> str:
    lock=tomllib.loads((ROOT/'tools/fixtures/c5-significance-scheduler-receipt/Cargo.lock').read_text(encoding='utf-8'))
    rows=[{"name":p["name"],"version":p["version"],"source":p.get("source","path"),"checksum":p.get("checksum",""),"dependencies":sorted(p.get("dependencies",[]))} for p in lock["package"]]
    return hashlib.sha256(canonical(sorted(rows,key=lambda p:(p["name"],p["version"],p["source"])))).hexdigest()

def main() -> None:
    parser=argparse.ArgumentParser()
    parser.add_argument("--output", required=True)
    parser.add_argument("--github-repository",required=True,help="Exact owner/repository expected to host the attested workflow")
    args=parser.parse_args()
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+",args.github_repository): raise SystemExit("invalid GitHub repository identity")
    local=json.loads(LOCAL.read_text(encoding="utf-8-sig"))
    commit=local["source_commit"]
    if run("git","status","--porcelain"):
        raise SystemExit("C5 external request requires a clean tree")
    if run("git","merge-base","--is-ancestor",commit,"HEAD") != "":
        raise SystemExit("unexpected merge-base output")
    bounded=manifest_sha(commit)
    if bounded != local["bounded_source_manifest_sha256"]:
        raise SystemExit("local receipt does not bind the external-run source surface")
    tracked=tree_sha(commit)
    if tracked != local["tracked_tree_manifest_sha256"]:
        raise SystemExit("local receipt does not bind the external-run tracked tree")
    challenge=secrets.token_hex(32)
    request={
      "schema_version":1,"protocol_id":"G1-C5-INDEPENDENT-PLATFORM-V1",
      "request_id":f"g1-c5-{challenge[:16]}","challenge":challenge,
      "created_utc":datetime.now(timezone.utc).isoformat().replace("+00:00","Z"),
      "source":{"repository_id":"mindwarp-forge","source_commit":commit,
        "tracked_tree_manifest_sha256":tracked,"bounded_source_manifest_sha256":bounded,
        "fixture_manifest_sha256":hashlib.sha256((ROOT/'tools/fixtures/c5-significance-scheduler-receipt/Cargo.toml').read_bytes()).hexdigest(),
        "fixture_lock_sha256":hashlib.sha256((ROOT/'tools/fixtures/c5-significance-scheduler-receipt/Cargo.lock').read_bytes()).hexdigest(),"dependency_graph_sha256":dependency_sha()},
      "semantic":{"receipt_id":"G1-C5-SIGNIFICANCE-SCHEDULER","expected_sha256":local["semantic_receipt_sha256"],"encoding":"lowercase_hex","max_decoded_bytes":65536},
      "toolchain":{"rustc_release":"1.97.0","rustc_commit":"2d8144b7880597b6e6d3dfd63a9a9efae3f533d3","cargo_release":"1.97.0"},
      "allowed_platforms":[
        {"target":"x86_64-unknown-linux-gnu","os":"linux","architecture":"x86_64","pointer_width":64,"endian":"little"},
        {"target":"x86_64-apple-darwin","os":"macos","architecture":"x86_64","pointer_width":64,"endian":"little"},
        {"target":"aarch64-apple-darwin","os":"macos","architecture":"aarch64","pointer_width":64,"endian":"little"}],
      "execution_policy":{"provider":"github-actions","hosted_runner_required":True,"native_execution_required":True,"compile_only_forbidden":True,"process_count":2,"concurrent":True,"direct_executable":True,"clean_before_after":True},
      "provenance_policy":{"repository":args.github_repository,"workflow_path":".github/workflows/g1-c5-independent-platform.yml","signer_workflow":f"{args.github_repository}/.github/workflows/g1-c5-independent-platform.yml"},
      "reference_platform":{"os":"windows","architecture":"x86_64","receipt_id":"G1-C5-LOCAL-PLATFORM-OBSERVATIONS"},
      "authority":{"evidence_only":True,"promotion_authority":False,"activate_c6":False,"repository_mutation":False}}
    request["request_sha256"]=digest(request)
    output=Path(args.output);output.parent.mkdir(parents=True,exist_ok=True)
    output.write_text(json.dumps(request,sort_keys=True,indent=2)+"\n",encoding="utf-8")
    print(f"C5 external request generated: {request['request_id']} {request['request_sha256']}")
if __name__ == "__main__": main()
