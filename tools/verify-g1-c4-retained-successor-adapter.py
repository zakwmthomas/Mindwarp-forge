#!/usr/bin/env python3
"""Strict live-route half of the retained C4 successor adapter."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any


EXPECTED_AUTHORITY = (
    "Owner-authorized bounded C5 significance/scheduler implementation and "
    "capability-free closure proof only. Exact dependency C4. Frozen candidate "
    "G1_C5_CLOSURE_READINESS.md. No C3B, C6, C7, broad G1 closure, runtime "
    "controllers, runtime executors, cache mutation, storage mutation, product "
    "weights, AI generation, rendering implementation, filesystem, network, "
    "process, Companion, Greenfield, visual assets or Kernel mutation."
)
EXPECTED_RECORDED_AUTHORITY = (
    "Owner-authorized recorded C5 significance/scheduler capability-free closure evidence "
    "only. Exact dependency C4. C5 remains the sole master-program cursor pending a separate "
    "C6 transition. No C3B, C6 activation, C7, broad G1 closure, runtime controllers, runtime "
    "executors, cache mutation, storage mutation, product weights, AI generation, rendering "
    "implementation, filesystem, network, process, Companion, Greenfield, visual assets, "
    "promotion authority or Kernel mutation."
)
EXPECTED_C6_AUTHORITY = (
    "Owner-authorized C6 semantic/construction and organism-ecology reconciliation and capability-free "
    "readiness only. Exact dependencies verified C4 and C5. Retain corrected C6 prerequisite evidence "
    "as non-closure evidence. No C6 implementation source, C3B, C7, broad G1 closure, runtime, product "
    "ontology or vocabulary, solver or AI generation, geometry, assets, animation, renderer, visual-quality "
    "claim, physiology or content constants, filesystem, network, process, Companion, Greenfield, promotion "
    "authority or Kernel mutation."
)
EXPECTED_C4_RUN = "run-bc2154f73f6243239910ac30bc3b1994"
REQUIRED_RECEIPTS = {
    f"registered-full-gate:{EXPECTED_C4_RUN}:passed",
    "receipt:G1-C4-CLOSURE:recorded",
    "receipt:c5-semantic-sha256:88e2be61586e728613fe2c7bf5b947074459fc5f63d6e5f13d4f4648e64624eb",
    "external:c5-github-run-29678602236:hosted-linux-attested:passed",
    "receipt:c5-independent-result-sha256:4dd77d3b16927644af2c9bb1b74f76e1dd7cc279a09a8297d10738a0efce1bf4",
    "independent-review:c5-portability-receipt:accepted",
    "focused:c5-proofreceipt-integration:forge-desktop-43:passed",
    "independent-review:c5-proofreceipt-integration:accepted",
    "registered-full-gate:run-71ef6dfd6e2945ab9745c85f3dcf4d6b:passed",
    "owner-route:c5-reconciliation-readiness:authorized",
    "owner-authorization:c5-frozen-implementation-candidate:released",
    "gate:c5-local-hardened:90-tests:passed",
    "gate:c5-hostile-registry:92-of-92:mapped",
    "gate:c5-closure-readiness:92-hostiles:frozen",
    "gate:c5-readiness-route-hostiles:passed",
}


def strict_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def reject_float(_: str) -> Any:
    raise ValueError("floating-point JSON values are forbidden")


def reject_constant(_: str) -> Any:
    raise ValueError("non-finite JSON values are forbidden")


def load_strict(path: Path) -> dict[str, Any]:
    value = json.loads(
        path.read_text(encoding="utf-8-sig"),
        object_pairs_hook=strict_object,
        parse_float=reject_float,
        parse_constant=reject_constant,
    )
    if type(value) is not dict:
        raise ValueError(f"{path.name} must contain one JSON object")
    return value


def exact_string(record: dict[str, Any], key: str, expected: str) -> None:
    value = record.get(key)
    if type(value) is not str or value != expected:
        raise ValueError(f"live checkpoint {key} is not the exact admitted value")


def exact_string_list(record: dict[str, Any], key: str) -> list[str]:
    value = record.get(key)
    if type(value) is not list or any(type(item) is not str for item in value):
        raise ValueError(f"{key} must be an exact string array")
    if len(value) != len(set(value)):
        raise ValueError(f"{key} contains duplicate values")
    return value


def one_item(items: list[Any], item_id: str) -> dict[str, Any]:
    matches = [item for item in items if type(item) is dict and item.get("id") == item_id]
    if len(matches) != 1:
        raise ValueError(f"master program must contain exactly one {item_id} item")
    return matches[0]


def verify(checkpoint_path: Path, program_path: Path, observation_path: Path) -> None:
    checkpoint = load_strict(checkpoint_path)
    program = load_strict(program_path)

    c6 = checkpoint.get("batch_id") == "G1-C6-SEMANTIC-CONSTRUCTION-ORGANISM-ECOLOGY-READINESS-V1"
    exact_string(checkpoint, "batch_id", "G1-C6-SEMANTIC-CONSTRUCTION-ORGANISM-ECOLOGY-READINESS-V1" if c6 else "G1-C5-SIGNIFICANCE-SCHEDULER-CLOSURE-V1")
    exact_string(checkpoint, "master_program_item", "C6" if c6 else "C5")
    state = checkpoint.get("state")
    substage = checkpoint.get("substage_id")
    authority = checkpoint.get("authority_lane")
    full_gate = (
        type(state) is str and state == "executing"
        and type(substage) is str and substage == "c5-full-gate-route-reconciliation"
        and type(authority) is str and authority == EXPECTED_AUTHORITY
    )
    recorded = (
        type(state) is str and state == "recorded"
        and type(substage) is str and substage == "c5-registered-closure-recorded"
        and type(authority) is str and authority == EXPECTED_RECORDED_AUTHORITY
    )
    c6_readiness = (
        type(state) is str and state == "executing"
        and type(substage) is str and substage == "c6-reconciliation-readiness"
        and type(authority) is str and authority == EXPECTED_C6_AUTHORITY
    )
    if not (full_gate or recorded or c6_readiness):
        raise ValueError("live checkpoint state/substage/authority tuple is not exact")

    receipts = exact_string_list(checkpoint, "verification_receipts")
    missing = sorted(REQUIRED_RECEIPTS.difference(receipts))
    if missing:
        raise ValueError(f"live checkpoint is missing retained evidence: {missing[0]}")
    if recorded and "receipt:G1-C5-CLOSURE:recorded" not in receipts:
        raise ValueError("recorded C5 route is missing its closure receipt")
    if c6_readiness:
        for receipt in ("receipt:G1-C5-CLOSURE:recorded", "owner-route:c6-reconciliation-readiness:authorized", "transition:c5-verified-c6-readiness-activated:recorded"):
            if receipt not in receipts:
                raise ValueError(f"C6 readiness route is missing retained evidence: {receipt}")

    items = program.get("items")
    if type(items) is not list or any(type(item) is not dict for item in items):
        raise ValueError("master program items must be an object array")
    ids = [item.get("id") for item in items]
    if any(type(item_id) is not str for item_id in ids) or len(ids) != len(set(ids)):
        raise ValueError("master program item IDs must be unique strings")

    c4 = one_item(items, "C4")
    c5 = one_item(items, "C5")
    for record, item_id, state, status in (
        (c4, "C4", "verified", "complete"),
        (c5, "C5", "verified" if c6_readiness else "executing", "complete" if c6_readiness else "active"),
    ):
        if record.get("state") != state or record.get("status") != status:
            raise ValueError(f"{item_id} state/status is not exact")
    if exact_string_list(c4, "depends_on") != ["C2", "C3A"]:
        raise ValueError("C4 dependency order is not exact")
    if exact_string_list(c5, "depends_on") != ["C4"]:
        raise ValueError("C5 dependency is not exact")
    if recorded or c6_readiness:
        if c5.get("gate") != "recorded":
            raise ValueError("recorded C5 route lost its recorded master gate")
        if "G1_C5_CLOSURE_RESULT.md" not in exact_string_list(c5, "sources"):
            raise ValueError("recorded C5 route lost its closure result source")
        c6 = one_item(items, "C6")
        expected_c6 = ("executing", "active") if c6_readiness else ("proposed", "gated")
        if (c6.get("state"), c6.get("status")) != expected_c6:
            raise ValueError("C6 successor state drifted")
    if "G1_C4_CLOSURE_RESULT.md" not in exact_string_list(c4, "sources"):
        raise ValueError("C4 closure source is missing")
    proof = c4.get("proof")
    proof_runs = [] if type(proof) is not str else re.findall(
        r"(?<![0-9a-f])run-[0-9a-f]{32}(?![0-9a-f])", proof
    )
    if proof_runs != [EXPECTED_C4_RUN]:
        raise ValueError("C4 proof does not bind the exact retained full-gate run")

    active = [
        item.get("id")
        for item in items
        if item.get("state") == "executing" and item.get("status") == "active"
    ]
    if active != (["C6"] if c6_readiness else ["C5"]):
        raise ValueError("successor must be the sole executing active program item")

    observation = load_strict(observation_path)
    expected_observation_keys = {
        "schema_version", "receipt_id", "semantic_receipt_sha256", "source_commit",
        "tracked_tree_manifest_sha256", "bounded_source_manifest_sha256", "rustc", "cargo",
        "observations", "independent_second_platform_execution", "promotion_authority",
    }
    if set(observation) != expected_observation_keys:
        raise ValueError("retained C4 observation root fields are not exact")
    expected_observation = {
        "schema_version": 1,
        "receipt_id": "G1-C4-LOCAL-PLATFORM-OBSERVATIONS",
        "semantic_receipt_sha256": "263a7c274c5bbfb5a48f0a7ccf3462eb35ddc7c96c1c92ff01d8ef37a40f6996",
        "source_commit": "17f39f7018de8a02c8292bcde0fafa2bf58fc7d4",
        "tracked_tree_manifest_sha256": "3b0bb235e5a3159ecba5b4d34acfd6157725773af79584e3b801283b8442e4dd",
        "bounded_source_manifest_sha256": "b31d92b7ebf36540a64249192fca96d83537c01932ededf84c94e1a886acc108",
        "independent_second_platform_execution": False,
        "promotion_authority": False,
    }
    for key, expected in expected_observation.items():
        value = observation.get(key)
        if type(value) is not type(expected) or value != expected:
            raise ValueError(f"retained C4 observation {key} is not exact")
    if type(observation.get("rustc")) is not str or type(observation.get("cargo")) is not str:
        raise ValueError("retained C4 observation toolchain fields are malformed")
    if type(observation.get("observations")) is not list or len(observation["observations"]) != 3:
        raise ValueError("retained C4 observation rows are not exact")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--checkpoint", required=True, type=Path)
    parser.add_argument("--program", required=True, type=Path)
    parser.add_argument("--observation", required=True, type=Path)
    args = parser.parse_args()
    verify(args.checkpoint, args.program, args.observation)
    print("Current C4 successor route verified: exact tuple, authority, dependencies and retained receipts.")


if __name__ == "__main__":
    main()
