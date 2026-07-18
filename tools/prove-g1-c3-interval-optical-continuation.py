#!/usr/bin/env python3
"""Exact counterexamples for interval optical continuation.

This is deliberately a falsification oracle, not a continuous-domain solver.
Every reported vector is evaluated with Fraction arithmetic.  It proves that
representative rays and correlation-erasing boxes are unsafe; it does not prove
that any surviving representation is sufficient for implementation.
"""
from fractions import Fraction as F
import hashlib
import json


def frac(x):
    return [x.numerator, x.denominator]


def next_face(point, direction):
    x, y = point
    dx, dy = direction
    candidates = []
    if dx > 0:
        candidates.append(((F(1) - x) / dx, "x_max"))
    elif dx < 0:
        candidates.append(((F(0) - x) / dx, "x_min"))
    if dy > 0:
        candidates.append(((F(1) - y) / dy, "y_max"))
    elif dy < 0:
        candidates.append(((F(0) - y) / dy, "y_min"))
    candidates = [(t, face) for t, face in candidates if t >= 0]
    first = min(t for t, _ in candidates)
    faces = sorted(face for t, face in candidates if t == first)
    return {"t": frac(first), "faces": faces, "positive_progress": first > 0}


def tir(normal, tangent, eta_i, eta_t):
    total = normal * normal + tangent * tangent
    return tangent * tangent * eta_i * eta_i >= total * eta_t * eta_t


def main():
    n = 1 << 62
    point = (F(1, 2), F(1, 2))
    left = next_face(point, (F(n + 1), F(n)))
    right = next_face(point, (F(n), F(n + 1)))
    midpoint = next_face(point, (F(2 * n + 1, 2), F(2 * n + 1, 2)))

    correlated_a = {"point": (F(1, 2), F(0)), "direction": (F(1), F(1))}
    correlated_b = {"point": (F(0), F(1, 2)), "direction": (F(2), F(1, 2))}
    impossible_cross = {"point": correlated_b["point"], "direction": correlated_a["direction"]}

    below = tir(F(3), F(2), F(3, 2), F(1))
    above = tir(F(2), F(2), F(3, 2), F(1))

    vectors = {
        "one_q1_62_unit_face_reversal": {
            "direction_a": [n + 1, n], "result_a": left,
            "direction_b": [n, n + 1], "result_b": right,
            "midpoint_representative": midpoint,
            "disposition": "whole_enclosure_is_ambiguous_next_face",
        },
        "correlation_erasure": {
            "correlated_state_a": next_face(correlated_a["point"], correlated_a["direction"]),
            "correlated_state_b": next_face(correlated_b["point"], correlated_b["direction"]),
            "independent_box_impossible_cross": next_face(impossible_cross["point"], impossible_cross["direction"]),
            "disposition": "axis_box_can_fabricate_a_face_not_reached_by_any_correlated_state",
        },
        "near_parallel_progress": {
            "result": next_face(point, (F(1), F(1, 1 << 100))),
            "disposition": "positive_progress_is_exact_but_small_components_require_wide_division",
        },
        "prior_face_zero_progress": {
            "result": next_face((F(1), F(1, 2)), (F(1), F(0))),
            "disposition": "zero_time_same_face_must_not_be_nudged_forward",
        },
        "critical_branch_ambiguity": {
            "eta_i": frac(F(3, 2)), "eta_t": frac(F(1)),
            "below_vector": [3, 2], "below_is_tir": below,
            "above_vector": [2, 2], "above_is_tir": above,
            "disposition": "incident_enclosure_spanning_both_vectors_has_ambiguous_interface_branch",
        },
    }
    assert left["faces"] == ["x_max"] and right["faces"] == ["y_max"]
    assert midpoint["faces"] == ["x_max", "y_max"]
    assert next_face(correlated_a["point"], correlated_a["direction"])["faces"] == ["x_max"]
    assert next_face(correlated_b["point"], correlated_b["direction"])["faces"] == ["x_max"]
    assert next_face(impossible_cross["point"], impossible_cross["direction"])["faces"] == ["y_max"]
    assert not below and above
    canonical = json.dumps(vectors, sort_keys=True, separators=(",", ":")).encode()
    print(json.dumps({
        "schema_version": 1,
        "oracle_kind": "exact_fraction_counterexamples_not_continuous_proof",
        "vectors": vectors,
        "vectors_sha256": hashlib.sha256(canonical).hexdigest(),
    }, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
