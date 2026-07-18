#!/usr/bin/env python3
"""Code-free readiness oracle for a bounded calibrated source-energy owner."""

from __future__ import annotations

import copy
import hashlib
import json
import math
from fractions import Fraction
from typing import Callable


AXES = ("u0", "u1", "u2", "u3")
SIDES = ("lower", "upper")
RESOLUTIONS = ("resolved_leaf", "unresolved_within_cell")
U128_MAX = (1 << 128) - 1
MAX_DIGITS = 39
MAX_DEPTH = 12
MAX_FRONTIER = 64
MAX_DIRECTIVES = 63
MAX_INPUT_BYTES = 128 * 1024
MAX_RESULT_BYTES = 256 * 1024
MAX_LIVE_BYTES = 4 * 1024 * 1024
MAX_ENERGY_BITS = 385
CELL_CAP = 32 * 1024
SPLIT_RECEIPT_CAP = 64 * 1024
SUBJECT_DOMAIN = b"mindwarp.calibrated-source-energy-distribution.subject.v1"
CELL_DOMAIN = b"mindwarp.calibrated-source-energy-distribution.disposable-cell.v1"
ALLOCATION_DOMAIN = b"mindwarp.calibrated-source-energy-distribution.allocation.v1"
SPLIT_DOMAIN = b"mindwarp.calibrated-source-energy-distribution.energy-split.v1"
DISTRIBUTION_DOMAIN = b"mindwarp.calibrated-source-energy-distribution.distribution.v1"


def canonical_json(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def digest(domain: bytes, value: object) -> str:
    return hashlib.sha256(domain + b"\0" + canonical_json(value)).hexdigest()


def reject(action: Callable[[], object]) -> bool:
    try:
        action()
    except ValueError:
        return True
    return False


def exact_keys(value: object, keys: set[str], label: str) -> dict[str, object]:
    if not isinstance(value, dict) or set(value) != keys:
        raise ValueError(f"{label} fields")
    return value


def parse_u128(text: object, *, positive: bool) -> int:
    if not isinstance(text, str) or not text or not text.isascii() or not text.isdigit():
        raise ValueError("unsigned decimal")
    if len(text) > MAX_DIGITS or (len(text) > 1 and text[0] == "0"):
        raise ValueError("decimal alias or size")
    value = int(text)
    if value > U128_MAX or (positive and value == 0):
        raise ValueError("u128 range")
    return value


def parse_energy(value: object, *, positive: bool = False) -> tuple[int, int]:
    record = exact_keys(value, {"denominator", "numerator"}, "energy")
    numerator = parse_u128(record["numerator"], positive=positive)
    denominator = parse_u128(record["denominator"], positive=True)
    if math.gcd(numerator, denominator) != 1:
        raise ValueError("noncanonical energy")
    return numerator, denominator


def energy(numerator: int, denominator: int = 1) -> dict[str, str]:
    divisor = math.gcd(numerator, denominator)
    return {"denominator": str(denominator // divisor), "numerator": str(numerator // divisor)}


def parse_id(value: object) -> str:
    if not isinstance(value, str) or len(value) != 64:
        raise ValueError("identity")
    try:
        raw = bytes.fromhex(value)
    except ValueError as error:
        raise ValueError("identity") from error
    if not any(raw):
        raise ValueError("zero identity")
    return value


def path_key(path: tuple[tuple[str, str], ...]) -> tuple[tuple[int, int], ...]:
    return tuple((AXES.index(axis), SIDES.index(side)) for axis, side in path)


def cell(root_id: str, reconstruction_id: str, path: tuple[tuple[str, str], ...], measure: Fraction) -> dict[str, object]:
    if len(path) > MAX_DEPTH:
        raise ValueError("depth limit")
    payload = {
        "measure": energy(measure.numerator, measure.denominator),
        "path": [{"axis": axis, "side": side} for axis, side in path],
        "reconstruction_id": reconstruction_id,
        "root_id": root_id,
    }
    return {**payload, "cell_id": digest(CELL_DOMAIN, payload)}


def subject(query: dict[str, object]) -> str:
    calibration = exact_keys(query["calibration"], {"band", "basis_id", "band_time_id", "calibration_provenance_id"}, "calibration")
    root = exact_keys(query["root"], {"measure", "reconstruction_id", "root_id", "scope_id", "source_id", "source_revision"}, "root")
    for key in ("basis_id", "band_time_id", "calibration_provenance_id"):
        parse_id(calibration[key])
    for key in ("reconstruction_id", "root_id", "scope_id", "source_id"):
        parse_id(root[key])
    if calibration["band"] not in ("blue", "green", "red"):
        raise ValueError("band")
    revision = root["source_revision"]
    if not isinstance(revision, int) or isinstance(revision, bool) or revision < 1 or revision > (1 << 32) - 1:
        raise ValueError("revision")
    source_provenance_id = parse_id(query["source_provenance_id"])
    if source_provenance_id == calibration["calibration_provenance_id"]:
        raise ValueError("provenance conflation")
    parse_energy(root["measure"], positive=True)
    payload = {
        "band": calibration["band"],
        "band_time_id": calibration["band_time_id"],
        "basis_id": calibration["basis_id"],
        "reconstruction_id": root["reconstruction_id"],
        "root_id": root["root_id"],
        "scope_id": root["scope_id"],
        "source_id": root["source_id"],
        "source_provenance_id": source_provenance_id,
        "source_revision": revision,
    }
    return digest(SUBJECT_DOMAIN, payload)


def allocation(subject_id: str, owned_cell: dict[str, object], joules: object, resolution: object) -> dict[str, object]:
    amount = parse_energy(joules)
    if resolution not in RESOLUTIONS:
        raise ValueError("resolution")
    payload = {
        "cell_id": owned_cell["cell_id"],
        "joules": energy(*amount),
        "measure": owned_cell["measure"],
        "path": owned_cell["path"],
        "resolution": resolution,
        "subject_id": subject_id,
    }
    return {**payload, "allocation_id": digest(ALLOCATION_DOMAIN, payload)}


def prove_energy_split(parent: object, lower: object, upper: object) -> int:
    a, b = parse_energy(parent)
    c, d = parse_energy(lower)
    e, f = parse_energy(upper)
    left = a * d * f
    right = b * (c * f + e * d)
    live_bits = max(left.bit_length(), right.bit_length())
    if live_bits > MAX_ENERGY_BITS:
        raise ValueError("energy arithmetic shield")
    if left != right:
        raise ValueError("energy conservation")
    return live_bits


def compile_distribution(value: object) -> dict[str, object]:
    if len(canonical_json(value)) > MAX_INPUT_BYTES:
        raise ValueError("input byte ceiling")
    query = exact_keys(value, {"calibration", "directives", "root", "root_joules", "schema_version", "source_provenance_id"}, "query")
    if query["schema_version"] != 1:
        raise ValueError("schema version")
    directives = query["directives"]
    if not isinstance(directives, list) or len(directives) > MAX_DIRECTIVES:
        raise ValueError("directive ceiling")
    subject_id = subject(query)
    root = query["root"]
    root_measure = Fraction(*parse_energy(root["measure"], positive=True))
    root_cell = cell(root["root_id"], root["reconstruction_id"], (), root_measure)
    root_allocation = allocation(subject_id, root_cell, query["root_joules"], "unresolved_within_cell")
    frontier: dict[str, tuple[dict[str, object], dict[str, object]]] = {
        root_allocation["allocation_id"]: (root_cell, root_allocation)
    }
    split_receipts: list[dict[str, object]] = []
    max_live_bits = 0
    for directive_value in directives:
        directive = exact_keys(directive_value, {"axis", "lower_joules", "lower_resolution", "parent_allocation_id", "upper_joules", "upper_resolution"}, "directive")
        parent_id = parse_id(directive["parent_allocation_id"])
        if parent_id not in frontier:
            raise ValueError("non-frontier parent")
        axis = directive["axis"]
        if axis not in AXES:
            raise ValueError("axis")
        parent_cell, parent_allocation = frontier[parent_id]
        path = tuple((step["axis"], step["side"]) for step in parent_cell["path"])
        if len(path) == MAX_DEPTH:
            raise ValueError("depth limit")
        max_live_bits = max(max_live_bits, prove_energy_split(parent_allocation["joules"], directive["lower_joules"], directive["upper_joules"]))
        parent_measure = Fraction(*parse_energy(parent_cell["measure"], positive=True))
        lower_cell = cell(root["root_id"], root["reconstruction_id"], path + ((axis, "lower"),), parent_measure / 2)
        upper_cell = cell(root["root_id"], root["reconstruction_id"], path + ((axis, "upper"),), parent_measure / 2)
        lower = allocation(subject_id, lower_cell, directive["lower_joules"], directive["lower_resolution"])
        upper = allocation(subject_id, upper_cell, directive["upper_joules"], directive["upper_resolution"])
        split_payload = {
            "children": [lower["allocation_id"], upper["allocation_id"]],
            "parent_allocation_id": parent_id,
            "phase_split_id": digest(SPLIT_DOMAIN, {"axis": axis, "children": [lower_cell["cell_id"], upper_cell["cell_id"]], "parent": parent_cell["cell_id"]}),
            "subject_id": subject_id,
        }
        receipt = {**split_payload, "energy_split_id": digest(SPLIT_DOMAIN, split_payload)}
        del frontier[parent_id]
        frontier[lower["allocation_id"]] = (lower_cell, lower)
        frontier[upper["allocation_id"]] = (upper_cell, upper)
        split_receipts.append(receipt)
        if len(frontier) > MAX_FRONTIER:
            raise ValueError("frontier ceiling")
    ordered = sorted((item[1] for item in frontier.values()), key=lambda item: path_key(tuple((step["axis"], step["side"]) for step in item["path"])))
    total_measure = sum((Fraction(*parse_energy(item["measure"], positive=True)) for item in ordered), Fraction())
    total_energy = sum((Fraction(*parse_energy(item["joules"])) for item in ordered), Fraction())
    if total_measure != root_measure or total_energy != Fraction(*parse_energy(query["root_joules"])):
        raise ValueError("root conservation")
    distribution_payload = {
        "energy_split_ids": [item["energy_split_id"] for item in split_receipts],
        "frontier_allocation_ids": [item["allocation_id"] for item in ordered],
        "root_allocation_id": root_allocation["allocation_id"],
        "subject_id": subject_id,
    }
    result = {
        "authority_effect": "none_evidence_only",
        "distribution_id": digest(DISTRIBUTION_DOMAIN, distribution_payload),
        "energy_splits": split_receipts,
        "frontier": ordered,
        "limitations": "no density transport applicability received energy detector visibility runtime promotion or C3 closure",
        "maximum_energy_live_bits": max_live_bits,
        "root_allocation": root_allocation,
        "schema_version": 1,
        "subject_id": subject_id,
    }
    result_bytes = len(canonical_json(result))
    live_bytes = len(frontier) * CELL_CAP + SPLIT_RECEIPT_CAP + len(canonical_json(query)) + result_bytes
    if result_bytes > MAX_RESULT_BYTES or live_bytes > MAX_LIVE_BYTES:
        raise ValueError("result or live byte ceiling")
    result["resource_receipt"] = {
        "frontier_atoms": len(frontier),
        "input_bytes": len(canonical_json(query)),
        "live_canonical_bytes_upper_bound": live_bytes,
        "result_bytes_before_receipt": result_bytes,
        "split_directives": len(directives),
    }
    return result


def fixture() -> dict[str, object]:
    return {
        "calibration": {
            "band": "green",
            "band_time_id": "22" * 32,
            "basis_id": "11" * 32,
            "calibration_provenance_id": "33" * 32,
        },
        "directives": [],
        "root": {
            "measure": energy(3, 2),
            "reconstruction_id": "44" * 32,
            "root_id": "55" * 32,
            "scope_id": "66" * 32,
            "source_id": "77" * 32,
            "source_revision": 1,
        },
        "root_joules": energy(64),
        "schema_version": 1,
        "source_provenance_id": "88" * 32,
    }


def directive(parent: str, axis: str, parent_energy: Fraction, lower_resolution: str = "resolved_leaf", upper_resolution: str = "resolved_leaf") -> dict[str, object]:
    lower = parent_energy / 3
    upper = parent_energy - lower
    return {
        "axis": axis,
        "lower_joules": energy(lower.numerator, lower.denominator),
        "lower_resolution": lower_resolution,
        "parent_allocation_id": parent,
        "upper_joules": energy(upper.numerator, upper.denominator),
        "upper_resolution": upper_resolution,
    }


def build_balanced(target: int) -> tuple[dict[str, object], dict[str, object]]:
    query = fixture()
    result = compile_distribution(query)
    while len(result["frontier"]) < target:
        parent = min(result["frontier"], key=lambda item: len(item["path"]))
        depth = len(parent["path"])
        query["directives"].append(directive(parent["allocation_id"], AXES[depth % len(AXES)], Fraction(*parse_energy(parent["joules"]))))
        result = compile_distribution(query)
    return query, result


def changed(base: dict[str, object], path: tuple[object, ...], value: object) -> dict[str, object]:
    result = copy.deepcopy(base)
    target: object = result
    for part in path[:-1]:
        target = target[part]  # type: ignore[index]
    target[path[-1]] = value  # type: ignore[index]
    return result


def main() -> None:
    root_only = compile_distribution(fixture())
    portfolios = [build_balanced(count) for count in (4, 16, 64)]
    query64, result64 = portfolios[-1]
    assert canonical_json(result64) == canonical_json(compile_distribution(copy.deepcopy(query64)))

    skew = fixture()
    skew_result = compile_distribution(skew)
    for depth in range(MAX_DEPTH):
        parent = skew_result["frontier"][-1]
        skew["directives"].append(directive(parent["allocation_id"], AXES[depth % 4], Fraction(*parse_energy(parent["joules"])), "unresolved_within_cell", "resolved_leaf"))
        skew_result = compile_distribution(skew)
    assert len(skew_result["frontier"]) == 13 and max(len(item["path"]) for item in skew_result["frontier"]) == MAX_DEPTH

    axis_base = fixture()
    axis_root = compile_distribution(axis_base)["frontier"][0]
    axis_base["directives"].append(directive(axis_root["allocation_id"], "u0", Fraction(*parse_energy(axis_root["joules"]))))
    axis_change = copy.deepcopy(axis_base)
    axis_change["directives"][0]["axis"] = "u3"
    assert compile_distribution(axis_change)["distribution_id"] != compile_distribution(axis_base)["distribution_id"]

    base = portfolios[0][0]
    hostile: list[Callable[[], object]] = [
        lambda: compile_distribution(changed(base, ("schema_version",), 2)),
        lambda: compile_distribution(changed(base, ("source_provenance_id",), "00" * 32)),
        lambda: compile_distribution(changed(base, ("source_provenance_id",), "33" * 32)),
        lambda: compile_distribution(changed(base, ("root", "source_revision"), 0)),
        lambda: compile_distribution(changed(base, ("calibration", "band"), "infrared")),
        lambda: compile_distribution(changed(base, ("root", "measure"), energy(0))),
        lambda: compile_distribution(changed(base, ("root_joules",), {"numerator": "01", "denominator": "1"})),
        lambda: compile_distribution(changed(base, ("root_joules",), {"numerator": "2", "denominator": "2"})),
        lambda: compile_distribution(changed(base, ("root_joules",), {"numerator": str(U128_MAX + 1), "denominator": "1"})),
        lambda: compile_distribution({**base, "unknown": True}),
        lambda: compile_distribution(changed(base, ("directives", 0, "axis"), "u4")),
        lambda: compile_distribution(changed(base, ("directives", 0, "parent_allocation_id"), "99" * 32)),
        lambda: compile_distribution(changed(base, ("directives", 0, "lower_resolution"), "zero")),
        lambda: compile_distribution(changed(base, ("directives", 0, "lower_joules"), energy(99))),
        lambda: compile_distribution(changed(base, ("directives", 0, "lower_joules"), {"numerator": "-1", "denominator": "1"})),
        lambda: compile_distribution(changed(base, ("directives", 0), {**base["directives"][0], "child_id": "aa" * 32})),
        lambda: compile_distribution({**base, "directives": base["directives"] + [copy.deepcopy(base["directives"][0])]}),
        lambda: compile_distribution({**query64, "directives": query64["directives"] + [copy.deepcopy(query64["directives"][-1])]}),
        lambda: compile_distribution({**fixture(), "padding": "x" * MAX_INPUT_BYTES}),
    ]
    hostile_rejections = sum(reject(case) for case in hostile)
    assert hostile_rejections == len(hostile)
    assert root_only["frontier"][0]["resolution"] == "unresolved_within_cell"
    assert result64["resource_receipt"]["frontier_atoms"] == 64
    assert result64["resource_receipt"]["split_directives"] == 63
    assert result64["resource_receipt"]["live_canonical_bytes_upper_bound"] < MAX_LIVE_BYTES

    evidence = {
        "axis_substitution_changes_identity": True,
        "balanced_distribution_ids": [result["distribution_id"] for _, result in portfolios],
        "hostile_rejections": hostile_rejections,
        "maximum_energy_live_bits": max(result["maximum_energy_live_bits"] for _, result in portfolios),
        "maximum_observed_input_bytes": max(result["resource_receipt"]["input_bytes"] for _, result in portfolios),
        "maximum_observed_live_upper_bound": result64["resource_receipt"]["live_canonical_bytes_upper_bound"],
        "maximum_observed_result_bytes": max(result["resource_receipt"]["result_bytes_before_receipt"] for _, result in portfolios),
        "skew_depth": MAX_DEPTH,
    }
    receipt = {
        "axis_bearing_replay": "compact_parent_allocation_plus_upstream_axis",
        "candidate": "calibrated-source-energy-distribution",
        "checksum": hashlib.sha256(canonical_json(evidence)).hexdigest(),
        "energy_live_bit_cap": MAX_ENERGY_BITS,
        "frontier_cap": MAX_FRONTIER,
        "hostile_rejections": hostile_rejections,
        "input_cap_bytes": MAX_INPUT_BYTES,
        "live_cap_bytes": MAX_LIVE_BYTES,
        "maximum_observed_input_bytes": evidence["maximum_observed_input_bytes"],
        "maximum_observed_live_upper_bound": evidence["maximum_observed_live_upper_bound"],
        "maximum_observed_result_bytes": evidence["maximum_observed_result_bytes"],
        "portfolios": [1, 4, 16, 64, 13],
        "production_artifacts": "none",
        "result_cap_bytes": MAX_RESULT_BYTES,
        "split_directive_cap": MAX_DIRECTIVES,
        "status": "pass_ready_for_owner_decision_only",
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
