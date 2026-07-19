#!/usr/bin/env python3
"""Hostile fixtures for the exact C5 retained-successor classifier."""

from __future__ import annotations

import importlib.util
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "c5_successor", ROOT / "tools/verify-g1-c5-retained-successor.py"
)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def must_fail(paths, source, successor, label: str) -> None:
    try:
        MODULE.verify_maps(tuple(paths), dict(source), dict(successor))
    except ValueError:
        return
    raise AssertionError(f"hostile admitted: {label}")


def main() -> None:
    paths = MODULE.EXPECTED_PATHS
    source = {
        path: MODULE.git(ROOT, "rev-parse", f"{MODULE.SOURCE_COMMIT}:{path}")
        for path in paths
    }
    successor = dict(source)
    for path, (_, new) in MODULE.ALLOWED_TRANSITIONS.items():
        successor[path] = new

    MODULE.verify_maps(paths, source, successor)
    MODULE.verify(ROOT)

    must_fail(paths[:-1], source, successor, "missing path")
    must_fail(paths + ("forged",), source, successor, "extra path")
    reordered = list(paths)
    reordered[0], reordered[1] = reordered[1], reordered[0]
    must_fail(reordered, source, successor, "reordered path")

    changed = dict(successor)
    unchanged_path = "crates/significance-scheduler/src/closure.rs"
    changed[unchanged_path] = "f" * 40
    must_fail(paths, source, changed, "unclassified bounded drift")

    for path, (old, new) in MODULE.ALLOWED_TRANSITIONS.items():
        changed = dict(successor)
        changed[path] = old
        must_fail(paths, source, changed, f"missing transition {path}")
        changed = dict(successor)
        changed[path] = "e" * 40
        must_fail(paths, source, changed, f"wildcard transition {path}")
        forged_source = dict(source)
        forged_source[path] = new
        must_fail(paths, forged_source, successor, f"crossed tuple {path}")

    crossed = dict(successor)
    left, right = tuple(MODULE.ALLOWED_TRANSITIONS)[:2]
    crossed[left], crossed[right] = crossed[right], crossed[left]
    must_fail(paths, source, crossed, "crossed successor blobs")

    incomplete = dict(successor)
    incomplete.pop(paths[0])
    must_fail(paths, source, incomplete, "incomplete successor map")

    print(
        "C5 retained-successor hostiles passed: exact path order, missing/extra, "
        "unclassified drift, wildcard, crossed and incomplete tuples reject."
    )


if __name__ == "__main__":
    main()
