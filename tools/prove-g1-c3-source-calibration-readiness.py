#!/usr/bin/env python3
"""Code-free falsification oracle for source-calibration readiness."""

from __future__ import annotations

import copy
import hashlib
import json
import math
from typing import Callable


BANDS = ("blue", "green", "red")
U128_MAX = (1 << 128) - 1
MAX_DECIMAL_DIGITS = 39
MAX_INPUT_BYTES = 16 * 1024
MAX_RESULT_BYTES = 32 * 1024
MAX_AGGREGATE_BYTES = 64 * 1024
BASIS_DOMAIN = b"mindwarp.calibrated-spectral-time-basis.basis.v1"
TIME_DOMAIN = b"mindwarp.calibrated-spectral-time-basis.legacy-time-commitment.v1"
LEGACY_BAND_TIME_DOMAIN = b"mindwarp.optical-phase-space.band-time.v1"


def canonical_json(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def digest(domain: bytes, value: object) -> bytes:
    return hashlib.sha256(domain + b"\0" + canonical_json(value)).digest()


def reject(action: Callable[[], object]) -> bool:
    try:
        action()
    except ValueError:
        return True
    return False


def require_exact_keys(value: object, keys: set[str], label: str) -> dict[str, object]:
    if not isinstance(value, dict) or set(value) != keys:
        raise ValueError(f"{label} fields")
    return value


def parse_u128_decimal(text: object, *, positive: bool) -> int:
    if not isinstance(text, str) or not text or not text.isascii() or not text.isdigit():
        raise ValueError("unsigned decimal")
    if len(text) > MAX_DECIMAL_DIGITS or (len(text) > 1 and text[0] == "0"):
        raise ValueError("decimal bounds or alias")
    value = int(text)
    if value > U128_MAX or (positive and value == 0):
        raise ValueError("u128 range")
    return value


def parse_rational(value: object, *, positive: bool) -> tuple[int, int]:
    record = require_exact_keys(value, {"numerator", "denominator"}, "rational")
    numerator = parse_u128_decimal(record["numerator"], positive=positive)
    denominator = parse_u128_decimal(record["denominator"], positive=True)
    if math.gcd(numerator, denominator) != 1:
        raise ValueError("non-reduced rational")
    return numerator, denominator


def compare_rational(left: tuple[int, int], right: tuple[int, int]) -> int:
    a = left[0] * right[1]
    b = right[0] * left[1]
    return (a > b) - (a < b)


def parse_id(value: object) -> list[int]:
    if (
        not isinstance(value, list)
        or len(value) != 32
        or any(not isinstance(item, int) or isinstance(item, bool) or item < 0 or item > 255 for item in value)
        or all(item == 0 for item in value)
    ):
        raise ValueError("nonzero 32-byte identity")
    return value


def validate_input(value: object) -> dict[str, object]:
    if len(canonical_json(value)) > MAX_INPUT_BYTES:
        raise ValueError("input byte ceiling")
    record = require_exact_keys(
        value,
        {
            "basis_version",
            "calibration_provenance_id",
            "quantity_kind",
            "schema_version",
            "spectral_coordinate",
            "spectral_intervals",
            "spectral_weighting",
            "time_cell",
            "unit",
        },
        "input",
    )
    if record["schema_version"] != 1:
        raise ValueError("schema version")
    if (
        not isinstance(record["basis_version"], int)
        or isinstance(record["basis_version"], bool)
        or record["basis_version"] < 1
        or record["basis_version"] > (1 << 32) - 1
    ):
        raise ValueError("basis version")
    if record["quantity_kind"] != "radiant_energy" or record["unit"] != "joule":
        raise ValueError("quantity")
    if record["spectral_coordinate"] != "vacuum_wavelength_metre":
        raise ValueError("spectral coordinate")
    if record["spectral_weighting"] != "unit_energy_integral":
        raise ValueError("spectral weighting")
    parse_id(record["calibration_provenance_id"])

    intervals = record["spectral_intervals"]
    if not isinstance(intervals, list) or len(intervals) != 3:
        raise ValueError("exactly three intervals")
    previous_upper: tuple[int, int] | None = None
    for expected_band, item in zip(BANDS, intervals):
        interval = require_exact_keys(item, {"band", "lower", "upper"}, "interval")
        if interval["band"] != expected_band:
            raise ValueError("band order")
        lower = parse_rational(interval["lower"], positive=False)
        upper = parse_rational(interval["upper"], positive=True)
        if compare_rational(lower, upper) >= 0:
            raise ValueError("empty or reversed interval")
        if previous_upper is not None and compare_rational(previous_upper, lower) != 0:
            raise ValueError("spectral gap or overlap")
        previous_upper = upper

    time_cell = require_exact_keys(
        record["time_cell"],
        {"clock_origin_id", "end_tick", "seconds_per_tick", "start_tick"},
        "time cell",
    )
    parse_id(time_cell["clock_origin_id"])
    start = time_cell["start_tick"]
    end = time_cell["end_tick"]
    if (
        not isinstance(start, int)
        or isinstance(start, bool)
        or not isinstance(end, int)
        or isinstance(end, bool)
        or start < 0
        or end > (1 << 64) - 1
        or start >= end
    ):
        raise ValueError("half-open u64 tick cell")
    parse_rational(time_cell["seconds_per_tick"], positive=True)
    return record


def compile_basis(value: object, supplied: dict[str, object] | None = None) -> dict[str, object]:
    record = validate_input(value)
    basis_id = digest(BASIS_DOMAIN, record)
    time_basis_id = digest(TIME_DOMAIN, [*basis_id])
    band_time_ids = {
        band: digest(LEGACY_BAND_TIME_DOMAIN, [band, [*time_basis_id]]) for band in BANDS
    }
    result = {
        "authority_effect": "none_evidence_only",
        "calibrated_basis_id": [*basis_id],
        "derived_legacy_band_time_ids": {band: [*band_time_ids[band]] for band in BANDS},
        "derived_legacy_time_basis_id": [*time_basis_id],
        "input": record,
        "limitations": "no source allocation transport applicability spatial calibration detector visibility runtime promotion or C3 closure",
        "schema_version": 1,
    }
    if supplied is not None and supplied != {
        "calibrated_basis_id": result["calibrated_basis_id"],
        "derived_legacy_band_time_ids": result["derived_legacy_band_time_ids"],
        "derived_legacy_time_basis_id": result["derived_legacy_time_basis_id"],
    }:
        raise ValueError("caller-supplied derived identity mismatch")
    result_bytes = len(canonical_json(result))
    if result_bytes > MAX_RESULT_BYTES or len(canonical_json(record)) + result_bytes > MAX_AGGREGATE_BYTES:
        raise ValueError("result or aggregate byte ceiling")
    return result


def rational(numerator: str, denominator: str) -> dict[str, str]:
    return {"numerator": numerator, "denominator": denominator}


def fixture() -> dict[str, object]:
    return {
        "basis_version": 1,
        "calibration_provenance_id": [17] * 32,
        "quantity_kind": "radiant_energy",
        "schema_version": 1,
        "spectral_coordinate": "vacuum_wavelength_metre",
        "spectral_intervals": [
            {"band": "blue", "lower": rational("1", "2500000"), "upper": rational("1", "2000000")},
            {"band": "green", "lower": rational("1", "2000000"), "upper": rational("3", "5000000")},
            {"band": "red", "lower": rational("3", "5000000"), "upper": rational("7", "10000000")},
        ],
        "spectral_weighting": "unit_energy_integral",
        "time_cell": {
            "clock_origin_id": [34] * 32,
            "end_tick": 116,
            "seconds_per_tick": rational("1", "1000"),
            "start_tick": 100,
        },
        "unit": "joule",
    }


def changed(base: dict[str, object], path: tuple[object, ...], value: object) -> dict[str, object]:
    result = copy.deepcopy(base)
    target: object = result
    for part in path[:-1]:
        target = target[part]  # type: ignore[index]
    target[path[-1]] = value  # type: ignore[index]
    return result


def main() -> None:
    base = fixture()
    first = compile_basis(base)
    second = compile_basis(copy.deepcopy(base))
    assert canonical_json(first) == canonical_json(second)

    supplied = {
        "calibrated_basis_id": first["calibrated_basis_id"],
        "derived_legacy_band_time_ids": first["derived_legacy_band_time_ids"],
        "derived_legacy_time_basis_id": first["derived_legacy_time_basis_id"],
    }
    assert compile_basis(base, supplied) == first

    distinct = (
        changed(base, ("basis_version",), 2),
        changed(base, ("calibration_provenance_id",), [18] * 32),
        changed(base, ("spectral_intervals", 0, "lower"), rational("1", "3000000")),
        changed(base, ("time_cell", "end_tick"), 117),
        changed(base, ("time_cell", "clock_origin_id"), [35] * 32),
        changed(base, ("time_cell", "seconds_per_tick"), rational("1", "2000")),
    )
    distinct_results = [compile_basis(item) for item in distinct]
    assert all(item["calibrated_basis_id"] != first["calibrated_basis_id"] for item in distinct_results)
    assert all(item["derived_legacy_time_basis_id"] != first["derived_legacy_time_basis_id"] for item in distinct_results)

    bad_supplied = copy.deepcopy(supplied)
    bad_supplied["calibrated_basis_id"] = [99] * 32
    oversized = copy.deepcopy(base)
    oversized["unknown_padding"] = "x" * MAX_INPUT_BYTES
    hostile = (
        lambda: compile_basis(changed(base, ("schema_version",), 2)),
        lambda: compile_basis(changed(base, ("basis_version",), 0)),
        lambda: compile_basis(changed(base, ("quantity_kind",), "radiant_power")),
        lambda: compile_basis(changed(base, ("unit",), "watt")),
        lambda: compile_basis(changed(base, ("spectral_coordinate",), "frequency_hertz")),
        lambda: compile_basis(changed(base, ("spectral_weighting",), "detector_response")),
        lambda: compile_basis(changed(base, ("calibration_provenance_id",), [0] * 32)),
        lambda: compile_basis(changed(base, ("calibration_provenance_id",), [1] * 31)),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "lower", "numerator"), "-1")),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "lower", "numerator"), "+1")),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "lower", "numerator"), "01")),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "lower", "denominator"), "0")),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "lower"), rational("2", "4"))),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "lower", "numerator"), str(U128_MAX + 1))),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "lower"), {"numerator": "1", "denominator": "2500000", "alias": 1})),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "upper"), rational("1", "2500000"))),
        lambda: compile_basis(changed(base, ("spectral_intervals", 1, "lower"), rational("11", "20000000"))),
        lambda: compile_basis(changed(base, ("spectral_intervals", 1, "lower"), rational("9", "20000000"))),
        lambda: compile_basis(changed(base, ("spectral_intervals", 0, "band"), "red")),
        lambda: compile_basis(changed(base, ("spectral_intervals",), base["spectral_intervals"][:2])),
        lambda: compile_basis(changed(base, ("time_cell", "clock_origin_id"), [0] * 32)),
        lambda: compile_basis(changed(base, ("time_cell", "start_tick"), -1)),
        lambda: compile_basis(changed(base, ("time_cell", "end_tick"), 100)),
        lambda: compile_basis(changed(base, ("time_cell", "end_tick"), 1 << 64)),
        lambda: compile_basis(changed(base, ("time_cell", "seconds_per_tick"), rational("0", "1"))),
        lambda: compile_basis(changed(base, ("time_cell", "seconds_per_tick"), rational("1", "0"))),
        lambda: compile_basis(changed(base, ("time_cell",), {**base["time_cell"], "unknown": True})),
        lambda: compile_basis({**base, "unknown": True}),
        lambda: compile_basis(oversized),
        lambda: compile_basis(base, bad_supplied),
    )
    hostile_rejections = sum(reject(case) for case in hostile)
    assert hostile_rejections == len(hostile)

    input_bytes = len(canonical_json(base))
    result_bytes = len(canonical_json(first))
    receipt_evidence = {
        "basis_id": first["calibrated_basis_id"],
        "band_time_ids": first["derived_legacy_band_time_ids"],
        "input_bytes": input_bytes,
        "result_bytes": result_bytes,
        "time_basis_id": first["derived_legacy_time_basis_id"],
    }
    receipt = {
        "aggregate_cap_bytes": MAX_AGGREGATE_BYTES,
        "candidate": "calibrated-spectral-time-basis",
        "checksum": hashlib.sha256(canonical_json(receipt_evidence)).hexdigest(),
        "derived_legacy_band_time_ids": 3,
        "distinct_identity_substitutions": len(distinct),
        "hostile_rejections": hostile_rejections,
        "identity_graph": "physical_descriptor_to_basis_to_legacy_time_to_unchanged_v1_band_time",
        "input_cap_bytes": MAX_INPUT_BYTES,
        "observed_input_bytes": input_bytes,
        "observed_result_bytes": result_bytes,
        "production_artifacts": "none",
        "result_cap_bytes": MAX_RESULT_BYTES,
        "status": "pass_ready_for_owner_decision_only",
        "transport_applicability": "blocked_separately",
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
