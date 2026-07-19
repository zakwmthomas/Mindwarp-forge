#!/usr/bin/env python3
"""Produce a C4 external execution result on a GitHub-hosted native runner."""
from __future__ import annotations
import argparse, base64, hashlib, json, os, platform, shutil, subprocess, time, tomllib, uuid
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=ROOT/"tools/fixtures/c4-hierarchy-history-receipt/Cargo.toml"
PATHS=ROOT/"tools/fixtures/c4-hierarchy-history-receipt/bounded-paths.txt"
def canonical(v): return json.dumps(v,sort_keys=True,separators=(",",":"),ensure_ascii=True).encode()
def digest(v): return hashlib.sha256(canonical(v)).hexdigest()
def file_sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def command(*args,env=None): return subprocess.run(args,cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=False)
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
def manifest_sha(commit):
    rows=[]
    for relative in PATHS.read_text(encoding="utf-8").splitlines():
        if relative:
            blob=subprocess.check_output(["git","rev-parse",f"{commit}:{relative}"],cwd=ROOT,text=True).strip()
            rows.append(f"{relative}:{blob}")
    return hashlib.sha256("\n".join(rows).encode()).hexdigest()
def dependency_sha():
    lock=tomllib.loads((ROOT/"tools/fixtures/c4-hierarchy-history-receipt/Cargo.lock").read_text(encoding="utf-8"))
    rows=[{"name":p["name"],"version":p["version"],"source":p.get("source","path"),"checksum":p.get("checksum",""),"dependencies":sorted(p.get("dependencies",[]))} for p in lock["package"]]
    return hashlib.sha256(canonical(sorted(rows,key=lambda p:(p["name"],p["version"],p["source"])))).hexdigest()
def captured(proc,ordinal,argv,started,executable_sha256):
    stdout,stderr=proc.communicate();ended=datetime.now(timezone.utc).isoformat().replace("+00:00","Z")
    return {"ordinal":ordinal,"launch_id":str(uuid.uuid4()),"pid":proc.pid,"started_utc":started,"ended_utc":ended,"argv":argv,"exit_code":proc.returncode,
      "stdout_base64":base64.b64encode(stdout).decode(),"stdout_sha256":hashlib.sha256(stdout).hexdigest(),
      "stderr_base64":base64.b64encode(stderr).decode(),"stderr_sha256":hashlib.sha256(stderr).hexdigest(),"executable_sha256":executable_sha256}
def main():
    ap=argparse.ArgumentParser();ap.add_argument("--request",required=True);ap.add_argument("--output",required=True);args=ap.parse_args()
    req=strict_load(args.request);claimed=req.pop("request_sha256")
    if digest(req)!=claimed: raise SystemExit("request hash mismatch")
    req["request_sha256"]=claimed
    required={"GITHUB_ACTIONS":"true","RUNNER_ENVIRONMENT":"github-hosted"}
    for key,value in required.items():
        if os.environ.get(key)!=value: raise SystemExit(f"nonqualifying provider environment: {key}")
    commit=subprocess.check_output(["git","rev-parse","HEAD"],cwd=ROOT,text=True).strip()
    if commit!=req["source"]["source_commit"] or os.environ.get("GITHUB_SHA")!=commit: raise SystemExit("source commit mismatch")
    if os.environ.get("GITHUB_REPOSITORY")!=req["provenance_policy"]["repository"] or req["provenance_policy"]["workflow_path"] not in os.environ.get("GITHUB_WORKFLOW_REF",""): raise SystemExit("provider repository or workflow identity mismatch")
    if subprocess.check_output(["git","status","--porcelain","--untracked-files=all"],cwd=ROOT,text=True).strip(): raise SystemExit("dirty source before run")
    created=datetime.fromisoformat(req["created_utc"].replace("Z","+00:00"));age=(datetime.now(timezone.utc)-created).total_seconds()
    if age<0 or age>86400: raise SystemExit("request challenge is outside the 24-hour freshness window")
    if manifest_sha(commit)!=req["source"]["bounded_source_manifest_sha256"]: raise SystemExit("bounded source mismatch")
    forbidden_exact=("RUSTFLAGS","RUSTC","RUSTC_WRAPPER","CARGO_BUILD_TARGET","CARGO_ENCODED_RUSTFLAGS","RUSTDOCFLAGS","CC","CFLAGS","LDFLAGS")
    forbidden_prefix=("CARGO_TARGET_","CARGO_PROFILE_","CC_","CFLAGS_","LDFLAGS_")
    if any(os.environ.get(key) for key in forbidden_exact) or any(value for key,value in os.environ.items() if key.startswith(forbidden_prefix)): raise SystemExit("unapproved Rust build environment")
    configs=[ROOT/".cargo/config",ROOT/".cargo/config.toml",Path.home()/".cargo/config",Path.home()/".cargo/config.toml"]
    if any(path.exists() for path in configs): raise SystemExit("unapproved Cargo configuration")
    env=os.environ.copy();target_dir=Path(os.environ.get("RUNNER_TEMP","/tmp"))/"forge-c4-independent-target";env["CARGO_TARGET_DIR"]=str(target_dir);env["FORGE_ROOT"]=str(ROOT)
    if dependency_sha()!=req["source"]["dependency_graph_sha256"]: raise SystemExit("resolved dependency graph drift")
    build_argv=["cargo","build","--release","--locked","--offline","--manifest-path",str(MANIFEST)]
    build=command(*build_argv,env=env)
    if build.returncode: raise SystemExit(build.stderr.decode(errors="replace"))
    exe=target_dir/"release"/("c4-hierarchy-history-receipt.exe" if os.name=="nt" else "c4-hierarchy-history-receipt")
    before=file_sha(exe);self_argv=[str(exe),"--self-test"];self_test=command(*self_argv,env=env)
    if self_test.returncode: raise SystemExit("fixture self-test failed")
    start_at=int(time.time()*1000)+1500;run_argv=[str(exe),"--start-at-unix-ms",str(start_at)]
    started=datetime.now(timezone.utc).isoformat().replace("+00:00","Z");p1=subprocess.Popen(run_argv,cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    started2=datetime.now(timezone.utc).isoformat().replace("+00:00","Z");p2=subprocess.Popen(run_argv,cwd=ROOT,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    if p1.poll() is not None or p2.poll() is not None: raise SystemExit("both native processes were not simultaneously live before barrier release")
    with ThreadPoolExecutor(max_workers=2) as pool:
        futures=[pool.submit(captured,p1,1,run_argv,started,before),pool.submit(captured,p2,2,run_argv,started2,before)]
        executions=[future.result() for future in futures]
    after=file_sha(exe)
    rustc=command("rustc","-vV",env=env);cargo=command("cargo","-V",env=env);cfg=command("rustc","--print","cfg",env=env).stdout.decode()
    rustc_path=shutil.which("rustc");cargo_path=shutil.which("cargo");rustc_exe=file_sha(rustc_path);cargo_exe=file_sha(cargo_path)
    host=next(line.split(": ",1)[1] for line in rustc.stdout.decode().splitlines() if line.startswith("host: "))
    rustc_text=rustc.stdout.decode();cargo_text=cargo.stdout.decode()
    if f"release: {req['toolchain']['rustc_release']}" not in rustc_text or f"commit-hash: {req['toolchain']['rustc_commit']}" not in rustc_text or f"cargo {req['toolchain']['cargo_release']} " not in cargo_text: raise SystemExit("toolchain identity drift")
    os_name={"Linux":"linux","Darwin":"macos"}.get(platform.system(),platform.system().lower());arch=platform.machine().lower().replace("amd64","x86_64").replace("arm64","aarch64")
    pointer=int(next(line.split('"')[1] for line in cfg.splitlines() if line.startswith('target_pointer_width=')));endian=next(line.split('"')[1] for line in cfg.splitlines() if line.startswith('target_endian='))
    allowed={json.dumps(v,sort_keys=True) for v in req["allowed_platforms"]};actual={"target":host,"os":os_name,"architecture":arch,"pointer_width":pointer,"endian":endian}
    if json.dumps(actual,sort_keys=True) not in allowed: raise SystemExit(f"platform not allowed: {actual}")
    if subprocess.check_output(["git","status","--porcelain","--untracked-files=all"],cwd=ROOT,text=True).strip(): raise SystemExit("dirty source after run")
    result={"schema_version":1,"protocol_id":req["protocol_id"],"request_id":req["request_id"],"request_sha256":claimed,"challenge":req["challenge"],"result_id":str(uuid.uuid4()),
      "source":dict(req["source"],clean_before=True,clean_after=True),"platform":actual,
      "provenance":{"kind":"provider_hosted_ci","provider":"github-actions","hosted_runner":True,"run_id":os.environ["GITHUB_RUN_ID"],"run_attempt":os.environ["GITHUB_RUN_ATTEMPT"],"job_id":os.environ["GITHUB_JOB"],"job_url":f"{os.environ['GITHUB_SERVER_URL']}/{os.environ['GITHUB_REPOSITORY']}/actions/runs/{os.environ['GITHUB_RUN_ID']}","repository":os.environ["GITHUB_REPOSITORY"],"workflow_ref":os.environ["GITHUB_WORKFLOW_REF"],"runner_image":os.environ.get("ImageOS","unknown"),"attestation_required":True},
      "toolchain":{"rustc_vv_base64":base64.b64encode(rustc.stdout).decode(),"rustc_vv_sha256":hashlib.sha256(rustc.stdout).hexdigest(),"cargo_v_base64":base64.b64encode(cargo.stdout).decode(),"cargo_v_sha256":hashlib.sha256(cargo.stdout).hexdigest(),"rustc_executable_sha256":rustc_exe,"cargo_executable_sha256":cargo_exe},
      "environment":{"cargo_net_offline":env.get("CARGO_NET_OFFLINE")=="true","rustflags":"","rustc":"","rustc_wrapper":"","cargo_build_target":"","cargo_encoded_rustflags":"","cargo_config_present":False,"forbidden_build_env_present":False},
      "build":{"argv":build_argv,"exit_code":build.returncode,"stdout_sha256":hashlib.sha256(build.stdout).hexdigest(),"stderr_sha256":hashlib.sha256(build.stderr).hexdigest(),"executable_sha256_before":before,"executable_sha256_after":after},
      "self_test":{"argv":self_argv,"exit_code":self_test.returncode,"stdout_base64":base64.b64encode(self_test.stdout).decode(),"stdout_sha256":hashlib.sha256(self_test.stdout).hexdigest(),"stderr_base64":base64.b64encode(self_test.stderr).decode(),"stderr_sha256":hashlib.sha256(self_test.stderr).hexdigest()},
      "executions":executions,"claims":{"evidence_only":True,"promotion_authority":False,"activate_c5":False,"repository_mutation":False,"independent_platform_execution_claimed":True}}
    result["result_sha256"]=digest(result);out=Path(args.output);out.parent.mkdir(parents=True,exist_ok=True);out.write_text(json.dumps(result,sort_keys=True,indent=2)+"\n",encoding="utf-8")
    print(f"C4 external result produced: {result['result_id']} {result['result_sha256']}")
if __name__=="__main__": main()
