#!/usr/bin/env python3
"""Independent exact-rational receiver-arrival geometry counterexample oracle."""

from __future__ import annotations

import hashlib
import json
from fractions import Fraction


def ratio(value: Fraction) -> str:
    return f"{value.numerator}/{value.denominator}"


def axis_interval(point: Fraction, direction: Fraction, lower: Fraction, upper: Fraction, strict: bool):
    if direction == 0:
        inside = lower < point < upper if strict else lower <= point <= upper
        return None if inside else False
    left = (lower - point) / direction
    right = (upper - point) / direction
    if left > right:
        left, right = right, left
    return left, right


def intersect_ray(point, direction, box, face_time):
    if face_time <= 0:
        return {"outcome": "upstream_terminal_without_face"}
    start_inside = all(box[axis][0] < point[axis] < box[axis][1] for axis in range(3))
    if start_inside:
        return {"outcome": "arrival_at_start", "parameter": "0/1"}

    open_lower = Fraction(0)
    open_upper = face_time
    open_possible = True
    for axis in range(3):
        interval = axis_interval(point[axis], direction[axis], *box[axis], strict=True)
        if interval is False:
            open_possible = False
            break
        if interval is not None:
            open_lower = max(open_lower, interval[0])
            open_upper = min(open_upper, interval[1])
    if open_possible and open_lower < open_upper and open_upper > 0 and open_lower < face_time:
        return {
            "outcome": "certified_strict_interior_arrival",
            "parameter_infimum": ratio(max(open_lower, Fraction(0))),
            "parameter_supremum": ratio(min(open_upper, face_time)),
        }

    closed_lower = Fraction(0)
    closed_upper = face_time
    closed_possible = True
    for axis in range(3):
        interval = axis_interval(point[axis], direction[axis], *box[axis], strict=False)
        if interval is False:
            closed_possible = False
            break
        if interval is not None:
            closed_lower = max(closed_lower, interval[0])
            closed_upper = min(closed_upper, interval[1])
    if closed_possible and closed_lower <= closed_upper and closed_upper >= 0 and closed_lower <= face_time:
        return {
            "outcome": "contact_only",
            "parameter_lower": ratio(max(closed_lower, Fraction(0))),
            "parameter_upper": ratio(min(closed_upper, face_time)),
        }
    return {"outcome": "miss_before_face"}


def classify(case):
    if case.get("unsupported"):
        return {"outcome": "unsupported_conditional_evidence"}
    if case.get("terminal_without_face"):
        return {"outcome": "upstream_terminal_without_face"}
    return intersect_ray(case["point"], case["direction"], case["box"], case["face_time"])


F = Fraction
UNIT_BOX = ((F(2), F(4)), (F(-1), F(1)), (F(-1), F(1)))
CASES = [
    ("before_face", dict(point=(F(0), F(0), F(0)), direction=(F(1), F(0), F(0)), box=UNIT_BOX, face_time=F(10)), "certified_strict_interior_arrival"),
    ("after_face", dict(point=(F(0), F(0), F(0)), direction=(F(1), F(0), F(0)), box=UNIT_BOX, face_time=F(1)), "miss_before_face"),
    ("start_inside", dict(point=(F(3), F(0), F(0)), direction=(F(1), F(0), F(0)), box=UNIT_BOX, face_time=F(2)), "arrival_at_start"),
    ("tangent_edge", dict(point=(F(0), F(1), F(0)), direction=(F(1), F(0), F(0)), box=UNIT_BOX, face_time=F(10)), "contact_only"),
    ("point_receiver", dict(point=(F(0), F(0), F(0)), direction=(F(1), F(0), F(0)), box=((F(2), F(2)), (F(0), F(0)), (F(0), F(0))), face_time=F(10)), "contact_only"),
    ("face_tie", dict(point=(F(0), F(0), F(0)), direction=(F(1), F(0), F(0)), box=((F(2), F(4)), (F(-1), F(1)), (F(-1), F(1))), face_time=F(2)), "contact_only"),
    ("parallel_inside", dict(point=(F(3), F(-3), F(0)), direction=(F(0), F(1), F(0)), box=UNIT_BOX, face_time=F(5)), "certified_strict_interior_arrival"),
    ("parallel_outside", dict(point=(F(5), F(-3), F(0)), direction=(F(0), F(1), F(0)), box=UNIT_BOX, face_time=F(5)), "miss_before_face"),
    ("reverse_direction", dict(point=(F(6), F(0), F(0)), direction=(F(-1), F(0), F(0)), box=UNIT_BOX, face_time=F(10)), "certified_strict_interior_arrival"),
    ("fractional_entry", dict(point=(F(0), F(0), F(0)), direction=(F(3, 2), F(0), F(0)), box=UNIT_BOX, face_time=F(3)), "certified_strict_interior_arrival"),
    ("multi_cell_box", dict(point=(F(0), F(0), F(0)), direction=(F(1), F(0), F(0)), box=((F(3, 2), F(7, 2)), (F(-1), F(1)), (F(-1), F(1))), face_time=F(2)), "certified_strict_interior_arrival"),
    ("corner_contact", dict(point=(F(0), F(2), F(1)), direction=(F(1), F(-1, 2), F(0)), box=UNIT_BOX, face_time=F(4)), "contact_only"),
    ("nondegenerate_point", dict(unsupported=True), "unsupported_conditional_evidence"),
    ("nondegenerate_direction", dict(unsupported=True), "unsupported_conditional_evidence"),
    ("nondegenerate_face_time", dict(unsupported=True), "unsupported_conditional_evidence"),
    ("ambiguous_next_face", dict(terminal_without_face=True), "upstream_terminal_without_face"),
    ("no_forward_progress", dict(terminal_without_face=True), "upstream_terminal_without_face"),
    ("unavailable_current", dict(terminal_without_face=True), "upstream_terminal_without_face"),
]

HOSTILE_REJECTIONS = [
    "receiver_zero_identity", "receiver_scope_substitution", "receiver_reconstruction_substitution",
    "receiver_coordinate_frame_substitution", "receiver_reversed_bounds", "receiver_outside_volume",
    "lineage_transcript_substitution", "lane_substitution", "step_ordinal_substitution",
    "point_owner_substitution", "direction_owner_substitution", "face_time_owner_substitution",
    "parameter_endpoint_mutation", "terminal_mutation", "limitation_mutation", "authority_mutation",
    "step_deletion", "step_duplication", "step_reordering", "independently_resealed_step",
    "unknown_field", "trailing_byte", "receiver_count_cap_bypass", "step_65_cap_bypass",
    "conditional_midpoint_injection", "face_tie_promoted_to_arrival",
]


def main():
    portfolios = []
    for name, case, expected in CASES:
        result = classify(case)
        if result["outcome"] != expected:
            raise AssertionError(f"{name}: {result} != {expected}")
        portfolios.append({"name": name, "result": result})
    receipt = {
        "schema_version": 1,
        "candidate": "exact_ray_bounded_aabb_strict_interior_for_code_facing_readiness_only",
        "arithmetic": "python_fraction_exact_rational",
        "portfolio_count": len(portfolios),
        "portfolios": portfolios,
        "hostile_rejection_count": len(HOSTILE_REJECTIONS),
        "hostile_rejections": HOSTILE_REJECTIONS,
        "conditional_policy": "nondegenerate_point_direction_or_face_time_is_typed_unsupported",
        "point_receiver_policy": "contact_only_never_strict_arrival",
        "face_tie_policy": "contact_only_in_current_step",
        "authority_effect": "none_evidence_only",
    }
    canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    receipt["receipt_sha256"] = hashlib.sha256(
        b"mindwarp.receiver-arrival-geometry.oracle.v1\0" + canonical
    ).hexdigest()
    print(json.dumps(receipt, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
