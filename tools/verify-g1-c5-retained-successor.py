#!/usr/bin/env python3
"""Fail-closed classifier for the exact retained C5 portability successor."""

from __future__ import annotations

import argparse
import hashlib
import subprocess
from pathlib import Path


SOURCE_COMMIT = "9e48dd117c2b22b62bd31dba15c10c3a9bf4b100"
SOURCE_MANIFEST_SHA256 = "9430bc530ba39403803a05fd99a9bc5c257472c2f320921ca242b51344947ecb"
SUCCESSOR_MANIFEST_SHA256 = "e8e79816668d1f49167a895c068de61ad363784483534c184462066948eb1658"
PATH_LIST = "tools/fixtures/c5-significance-scheduler-receipt/bounded-paths.txt"

EXPECTED_PATHS = (
    "Cargo.toml",
    "Cargo.lock",
    ".gitignore",
    "contracts/significance-scheduler-contract.md",
    "crates/significance-scheduler/Cargo.toml",
    "crates/significance-scheduler/src/lib.rs",
    "crates/significance-scheduler/src/significance.rs",
    "crates/significance-scheduler/src/scheduler.rs",
    "crates/significance-scheduler/src/closure.rs",
    "crates/significance-scheduler/src/proof.rs",
    "crates/significance-scheduler/tests/multi_domain_consumer_fidelity.rs",
    "crates/significance-scheduler/tests/eight_domain_scheduler_closure.rs",
    "crates/significance-scheduler/tests/c5_contract_hostiles.rs",
    "crates/significance-scheduler/tests/c5_scheduler_hostiles.rs",
    "crates/significance-scheduler/tests/c5_residency_trace_authority_hostiles.rs",
    "crates/significance-scheduler/tests/c5_pressure_simulation.rs",
    "docs/canonical-system/SIGNIFICANCE_SCHEDULER_DESIGN_GATE.md",
    "docs/canonical-system/G1_C5_MULTI_DOMAIN_FIDELITY_RESULT.md",
    "docs/canonical-system/G1_C5_CLOSURE_READINESS.md",
    "docs/canonical-system/G1_C5_LOCAL_IMPLEMENTATION_CANDIDATE.md",
    "docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION_PROTOCOL.md",
    "tools/fixtures/c5-significance-scheduler-receipt/main.rs",
    "tools/fixtures/c5-significance-scheduler-receipt/Cargo.toml",
    "tools/fixtures/c5-significance-scheduler-receipt/Cargo.lock",
    PATH_LIST,
    "tools/test-g1-c5-closure-readiness.ps1",
    "tools/verify-g1-c5-significance-scheduler-implementation.ps1",
    "tools/test-g1-c5-portability-classifier.ps1",
    "tools/verify-g1-c5-platform-observation-receipt.ps1",
    "tools/generate-g1-c5-external-request.py",
    "tools/run-g1-c5-external-receipt.py",
    "tools/verify-g1-c5-external-receipt.py",
    "tools/test-g1-c5-external-receipt.py",
    "tools/verify-g1-c5-independent-platform-result.py",
    "tools/verify-g1-c5-independent-platform-result.ps1",
    "tools/test-g1-c5-independent-platform-result.py",
    "tools/test-g1-c5-independent-platform-result.ps1",
    "tools/verify.ps1",
    ".github/workflows/g1-c5-independent-platform.yml",
)

ALLOWED_TRANSITIONS = {
    "docs/canonical-system/G1_C5_LOCAL_IMPLEMENTATION_CANDIDATE.md": (
        "124910f83296f424ee66073e13ae1b10ff3298d4",
        "82ad7a0a213b36d003c2eac72a3ce29f77d6a66b",
    ),
    "docs/canonical-system/G1_C5_INDEPENDENT_PLATFORM_EXECUTION_PROTOCOL.md": (
        "088dd5f397fb58e3b7876400b71a0bb80ef9d53c",
        "1d35f00e0612710bae78d4f48ffad6e30fa2715d",
    ),
    "tools/verify.ps1": (
        "81a19541b5fa0d025718cc8976f2b281cc536a6e",
        "852ff5114c39e6a5389429000b78c9b3858931a3",
    ),
}


def git(root: Path, *args: str) -> str:
    completed = subprocess.run(
        ["git", "-C", str(root), *args],
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode:
        raise ValueError(completed.stderr.strip() or "git command failed")
    return completed.stdout.strip()


def manifest(rows: list[tuple[str, str]]) -> str:
    payload = "\n".join(f"{path}:{blob}" for path, blob in rows).encode()
    return hashlib.sha256(payload).hexdigest()


def verify_maps(
    paths: tuple[str, ...], source: dict[str, str], successor: dict[str, str]
) -> tuple[str, str]:
    if paths != EXPECTED_PATHS or len(paths) != 39 or len(set(paths)) != 39:
        raise ValueError("C5 bounded path set or order is not the exact frozen 39-path list")
    if set(source) != set(EXPECTED_PATHS) or set(successor) != set(EXPECTED_PATHS):
        raise ValueError("C5 source or successor blob map is incomplete")

    changed: list[str] = []
    for path in EXPECTED_PATHS:
        old = source[path]
        new = successor[path]
        if path in ALLOWED_TRANSITIONS:
            expected_old, expected_new = ALLOWED_TRANSITIONS[path]
            if (old, new) != (expected_old, expected_new):
                raise ValueError(f"C5 classified evidence tuple drifted: {path}")
            changed.append(path)
        elif old != new:
            raise ValueError(f"C5 bounded proof blob drifted outside classification: {path}")
    if tuple(changed) != tuple(ALLOWED_TRANSITIONS):
        raise ValueError("C5 classified evidence path set is not exact")

    source_sha = manifest([(path, source[path]) for path in EXPECTED_PATHS])
    successor_sha = manifest([(path, successor[path]) for path in EXPECTED_PATHS])
    if source_sha != SOURCE_MANIFEST_SHA256:
        raise ValueError("C5 retained source manifest digest drifted")
    if successor_sha != SUCCESSOR_MANIFEST_SHA256:
        raise ValueError("C5 successor manifest digest drifted")
    return source_sha, successor_sha


def verify(root: Path) -> tuple[str, str]:
    if git(root, "rev-parse", SOURCE_COMMIT) != SOURCE_COMMIT:
        raise ValueError("C5 retained source commit is unavailable or ambiguous")
    raw_paths = git(root, "show", f"{SOURCE_COMMIT}:{PATH_LIST}").splitlines()
    paths = tuple(raw_paths)
    source = {
        path: git(root, "rev-parse", f"{SOURCE_COMMIT}:{path}") for path in paths
    }
    successor: dict[str, str] = {}
    for path in paths:
        full = root / path
        if not full.is_file():
            raise ValueError(f"C5 successor bounded path is missing: {path}")
        successor[path] = git(root, "hash-object", "--", str(full))
    return verify_maps(paths, source, successor)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    source, successor = verify(args.root.resolve())
    print(
        "C5 retained successor verified: exact 39-path set, 36 unchanged blobs, "
        f"two evidence-only transitions plus one exact closure-orchestration transition, source {source}, successor {successor}."
    )


if __name__ == "__main__":
    main()
