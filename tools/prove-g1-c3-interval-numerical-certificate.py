#!/usr/bin/env python3
"""Disposable production-decision audit for interval-incident enclosures.

The production rule below deliberately has no reference-precision argument.
The retained 384-bit evaluation is used only after the decision as test truth.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import sys
from pathlib import Path


PRODUCTION_PRECISION = 160
REFERENCE_PRECISION = 384


def load_interval_oracle():
    path = Path(__file__).with_name("prove-g1-c3-interval-incident-interface.py")
    spec = importlib.util.spec_from_file_location("forge_interval_certificate_source", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load retained interval oracle")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def production_decision(case, interval):
    """Decide using admitted case data and fixed 160-bit work only."""
    branch = interval.classify_branch(case)
    if branch.outcome == "ambiguous_interface_branch":
        return {"outcome": branch.outcome, "event": None}
    event = interval.interval_event(case, PRODUCTION_PRECISION, interval.load_staged())
    if event.outcome == "nonconvergent_enclosure":
        return {"outcome": event.outcome, "event": None}
    if event.outcome != branch.outcome:
        raise AssertionError(f"{case.name}: exact branch and evaluator disagree")
    return {"outcome": "bounded_enclosure", "branch": branch.outcome, "event": event}


def main() -> None:
    interval = load_interval_oracle()
    staged = interval.load_staged()
    cases = interval.fixed_cases() + interval.generated_cases()
    checks = 0
    bounded = 0
    ambiguous = 0
    nonconvergent = 0
    max_excess = 0
    max_live = 0
    max_stored = 0

    for case in cases:
        decision = production_decision(case, interval)
        reference = interval.interval_event(case, REFERENCE_PRECISION, staged)
        checks += 1
        if decision["outcome"] == "ambiguous_interface_branch":
            assert reference.outcome == "ambiguous_interface_branch"
            ambiguous += 1
            checks += 1
            continue
        if decision["outcome"] == "nonconvergent_enclosure":
            nonconvergent += 1
            continue
        event = decision["event"]
        assert event is not None
        assert event.outcome == reference.outcome
        checks += 1
        for name, reference_value in interval.values(reference).items():
            assert interval.contains_interval(interval.values(event)[name], reference_value)
            checks += 1
        max_excess = max(max_excess, interval.numerical_excess(event, reference, staged))
        max_live = max(max_live, event.metrics.max_live_integer_bits)
        max_stored = max(max_stored, event.metrics.max_stored_endpoint_bits)
        bounded += 1

    forced_case = next(
        case for case in interval.fixed_cases() if case.name == "critical-all-transmit"
    )
    forced_raw = interval.interval_event(forced_case, 80, staged)
    forced_reference = interval.interval_event(forced_case, REFERENCE_PRECISION, staged)
    forced_excess = interval.numerical_excess(forced_raw, forced_reference, staged)
    fixed_decision = production_decision(forced_case, interval)
    checks += 3
    assert forced_raw.outcome == "all_transmit"
    assert forced_excess > 1
    assert fixed_decision["outcome"] == "bounded_enclosure"

    repeated = interval.repeated_portfolio(staged)
    assert all(
        lane["events_attempted"] == 64 and lane["all_transmit_events"] == 64
        for lane in repeated["lanes"].values()
    )
    checks += 3

    # For Q1.62 inputs, squared component extrema need at most 126 bits.
    # Q16.48 index squares plus that geometry need at most 232 bits. The
    # retained fixed evaluator's two widest operation families are therefore
    # bounded by F+232 and 2F+132, matching the existing checked-kernel guard.
    derived_live_bits = max(PRODUCTION_PRECISION + 232, 2 * PRODUCTION_PRECISION + 132)
    assert derived_live_bits == 452 < 512
    assert max_live <= derived_live_bits
    assert repeated["max_live_integer_bits"] <= derived_live_bits
    checks += 3

    strategies = {
        "analytic_padding_budget": {
            "status": "not_selected",
            "reason": "no retained operation-by-operation proof yet separates dependency width from rounding padding",
        },
        "widened_shadow_relation": {
            "status": "not_selected",
            "reason": "no proved shadow construction yet bounds the limiting enclosure rather than merely another finite-precision enclosure",
        },
        "fixed_160_sound_enclosure": {
            "status": "supported_for_readiness_redesign",
            "production_rule": "exact whole-box branch then one fixed outward 160-bit evaluation",
            "known_semantics": "bounded_enclosure without a numerical-tightness claim",
            "nonconvergent_semantics": "the fixed evaluator cannot form a finite enclosure at its declared cap",
        },
    }

    receipt = {
        "schema_version": 1,
        "status": "pass",
        "oracle_kind": "production_only_interval_decision_audit",
        "production_precision": PRODUCTION_PRECISION,
        "reference_precision_test_only": REFERENCE_PRECISION,
        "strategies": strategies,
        "total_cases": len(cases),
        "bounded_enclosures": bounded,
        "ambiguous_interface_branches": ambiguous,
        "production_nonconvergent": nonconvergent,
        "max_160_vs_384_numerical_excess_target_units": max_excess,
        "max_production_live_integer_bits": max_live,
        "max_production_stored_endpoint_bits": max_stored,
        "derived_maximum_live_bits": derived_live_bits,
        "storage_bits": 512,
        "forced_80_raw_evaluator_outcome": forced_raw.outcome,
        "forced_80_reference_excess_target_units": forced_excess,
        "forced_case_fixed_160_disposition": fixed_decision["outcome"],
        "repeated_event_portfolio": repeated,
        "checks": checks,
        "limitations": [
            "fixed 160 is a soundness contract and makes no endpoint-tightness promise",
            "384-bit evaluation is external test truth and never production work",
            "schema provenance codec allocation and platform cost remain readiness gates",
            "no composer perception rendering collision navigation organism biome planet terrain runtime promotion or C3 closure",
        ],
    }
    canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    receipt["receipt_sha256"] = hashlib.sha256(canonical).hexdigest()
    print(json.dumps(receipt, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
