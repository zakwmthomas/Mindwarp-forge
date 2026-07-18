#!/usr/bin/env python3
"""Disposable exact/fixed-160 oracle for one 3D interval optical cell step.

The candidate uses outward dyadic interval arithmetic at 160 fractional bits.
Python Fraction evaluates admitted corner witnesses independently.  The oracle
selects a face only when its entire outward time interval is strictly before
the lower bound of every competing possible face.  It is not production code
and does not perform bulk transfer, interface optics, or path composition.
"""
from fractions import Fraction as F
from itertools import product
import hashlib
import json
import random

F_BITS = 160
S = 1 << F_BITS
Q62 = 1 << 62
Q32 = 1 << 32
AXES = "xyz"
MAX_BITS = 0


def track(value):
    global MAX_BITS
    MAX_BITS = max(MAX_BITS, abs(value).bit_length())
    return value


def ceil_div(n, d):
    assert n >= 0 and d > 0
    track(n)
    track(d)
    return (n + d - 1) // d


def ceil_signed(n, d):
    assert d > 0
    return -((-n) // d)


def mul_interval(a, b):
    values = [track(x * y) for x in a for y in b]
    return (min(values) // S, ceil_signed(max(values), S))


def add_interval(a, b):
    return (track(a[0] + b[0]), track(a[1] + b[1]))


def q62_box(value, extra_units=0):
    scaled = value * Q62
    lo = scaled.numerator // scaled.denominator - extra_units
    hi = ceil_signed(scaled.numerator, scaled.denominator) + extra_units
    return (lo << (F_BITS - 62), hi << (F_BITS - 62))


def q32_box(value, extra_units=0):
    scaled = value * Q32
    lo = scaled.numerator // scaled.denominator - extra_units
    hi = ceil_signed(scaled.numerator, scaled.denominator) + extra_units
    return (lo << (F_BITS - 32), hi << (F_BITS - 32))


def face_time_candidates(point_box, direction_box, cell):
    candidates = []
    for axis in range(3):
        p_lo, p_hi = point_box[axis]
        d_lo, d_hi = direction_box[axis]
        lower_face = cell[axis] * S
        upper_face = (cell[axis] + 1) * S
        assert lower_face <= p_lo <= p_hi <= upper_face

        if d_hi > 0:
            n_lo = upper_face - p_hi
            n_hi = upper_face - p_lo
            t_lo = track(n_lo * S) // d_hi
            t_hi = ceil_div(track(n_hi * S), d_lo) if d_lo > 0 else None
            candidates.append({"face": f"{AXES[axis]}+", "axis": axis,
                               "sign": 1, "lo": t_lo, "hi": t_hi})
        if d_lo < 0:
            n_lo = p_lo - lower_face
            n_hi = p_hi - lower_face
            speed_max = -d_lo
            speed_min = -d_hi if d_hi < 0 else None
            t_lo = track(n_lo * S) // speed_max
            t_hi = ceil_div(track(n_hi * S), speed_min) if speed_min else None
            candidates.append({"face": f"{AXES[axis]}-", "axis": axis,
                               "sign": -1, "lo": t_lo, "hi": t_hi})
    return candidates


def step(point_box, direction_box, cell, extent=(128, 128, 128), unavailable=frozenset()):
    candidates = face_time_candidates(point_box, direction_box, cell)
    if not candidates or any(c["lo"] == 0 for c in candidates):
        return {"kind": "no_forward_progress", "candidates": candidates}

    winners = []
    for candidate in candidates:
        if candidate["hi"] is None:
            continue
        if all(candidate is other or candidate["hi"] < other["lo"] for other in candidates):
            winners.append(candidate)
    if len(winners) != 1:
        return {"kind": "ambiguous_next_face", "candidates": candidates}

    winner = winners[0]
    t_box = (winner["lo"], winner["hi"])
    next_point = []
    for axis in range(3):
        if axis == winner["axis"]:
            boundary = (cell[axis] + (1 if winner["sign"] > 0 else 0)) * S
            next_point.append((boundary, boundary))
        else:
            propagated = add_interval(point_box[axis], mul_interval(direction_box[axis], t_box))
            cell_bounds = (cell[axis] * S, (cell[axis] + 1) * S)
            clipped = (max(propagated[0], cell_bounds[0]), min(propagated[1], cell_bounds[1]))
            assert clipped[0] <= clipped[1]
            next_point.append(clipped)

    next_cell = list(cell)
    next_cell[winner["axis"]] += winner["sign"]
    next_cell = tuple(next_cell)
    base = {"face": winner["face"], "time_box": t_box,
            "next_point_box": tuple(next_point), "next_cell": next_cell}
    if any(v < 0 or v >= extent[i] for i, v in enumerate(next_cell)):
        return {"kind": "outer_domain_exit", **base}
    if next_cell in unavailable:
        return {"kind": "unavailable_neighbor", **base}
    return {"kind": "certified_next_face", **base}


def exact_face(point, direction, cell):
    times = []
    for axis in range(3):
        if direction[axis] > 0:
            times.append(((F(cell[axis] + 1) - point[axis]) / direction[axis], f"{AXES[axis]}+"))
        elif direction[axis] < 0:
            times.append(((F(cell[axis]) - point[axis]) / direction[axis], f"{AXES[axis]}-"))
    if not times:
        return None, []
    t = min(x[0] for x in times)
    return t, sorted(face for value, face in times if value == t)


def raw_fraction(value):
    return F(value, S)


def verify_corner_containment(point_box, direction_box, cell, result):
    if result["kind"] not in ("certified_next_face", "outer_domain_exit", "unavailable_neighbor"):
        return 0
    checked = 0
    for p_raw in product(*[(a[0], a[1]) for a in point_box]):
        for d_raw in product(*[(a[0], a[1]) for a in direction_box]):
            point = tuple(raw_fraction(v) for v in p_raw)
            direction = tuple(raw_fraction(v) for v in d_raw)
            t, faces = exact_face(point, direction, cell)
            assert faces == [result["face"]]
            t_raw = t * S
            assert result["time_box"][0] <= t_raw <= result["time_box"][1]
            hit = tuple(point[i] + direction[i] * t for i in range(3))
            for axis in range(3):
                lo, hi = result["next_point_box"][axis]
                assert F(lo, S) <= hit[axis] <= F(hi, S)
            checked += 1
    return checked


def widths(box):
    return [hi - lo for lo, hi in box]


def compact(result):
    value = {"kind": result["kind"]}
    for key in ("face", "next_cell"):
        if key in result:
            value[key] = result[key]
    if "time_box" in result:
        value["time_width_q160"] = result["time_box"][1] - result["time_box"][0]
        value["point_width_q160"] = widths(result["next_point_box"])
    return value


def named_cases():
    centre = tuple(q32_box(F(1, 2)) for _ in range(3))
    zero = q62_box(F(0))
    one = q62_box(F(1))
    q = F(3, 4)
    tie = q62_box(q)
    one_unit_lo = (tie[0], tie[0])
    one_unit_hi = (tie[0] + (1 << 98), tie[0] + (1 << 98))
    tiny = (1 << 98, 1 << 98)
    cases = {
        "normal_x": (centre, (one, zero, zero), (0, 0, 0), {}, "certified_next_face"),
        "one_unit_x_wins": (centre, (one_unit_hi, one_unit_lo, zero), (0, 0, 0), {}, "certified_next_face"),
        "one_unit_y_wins": (centre, (one_unit_lo, one_unit_hi, zero), (0, 0, 0), {}, "certified_next_face"),
        "box_spans_reversal": (centre, ((one_unit_lo[0], one_unit_hi[1]), (one_unit_lo[0], one_unit_hi[1]), zero), (0, 0, 0), {}, "ambiguous_next_face"),
        "exact_xy_corner": (centre, (tie, tie, zero), (0, 0, 0), {}, "ambiguous_next_face"),
        "near_parallel_q62_minimum": (centre, (one, tiny, zero), (0, 0, 0), {}, "certified_next_face"),
        "zero_straddling_competitor": (centre, (one, (-tiny[0], tiny[0]), zero), (0, 0, 0), {}, "certified_next_face"),
        "prior_face_zero_progress": (((S, S), centre[1], centre[2]), (one, zero, zero), (0, 0, 0), {}, "no_forward_progress"),
        "outer_exit": (centre, (one, zero, zero), (0, 0, 0), {"extent": (1, 1, 1)}, "outer_domain_exit"),
        "unavailable_neighbor": (centre, (one, zero, zero), (0, 0, 0), {"unavailable": frozenset({(1, 0, 0)})}, "unavailable_neighbor"),
        "correlation_erasure_box": (
            ((0, S // 2), (0, S // 2), centre[2]),
            ((S, 2 * S), (S // 2, S), zero),
            (0, 0, 0), {}, "ambiguous_next_face"),
        "maximum_coordinate_cell": (
            (q32_box(F(65534) + F(1, 2)), centre[1], centre[2]),
            (one, tiny, zero), (65534, 0, 0), {"extent": (65536, 1, 1)},
            "certified_next_face"),
    }
    output = {}
    corner_checks = 0
    for name, (p, d, cell, kwargs, expected) in cases.items():
        result = step(p, d, cell, **kwargs)
        assert result["kind"] == expected, (name, result)
        corner_checks += verify_corner_containment(p, d, cell, result)
        output[name] = compact(result)
    return output, corner_checks


def generated_portfolio():
    rng = random.Random(0xC3115EED)
    counts = {"certified_next_face": 0, "ambiguous_next_face": 0,
              "no_forward_progress": 0}
    corner_checks = 0
    false_ambiguity_controls = 0
    triples = [(F(12, 13), F(4, 13), F(3, 13)),
               (F(4, 13), F(12, 13), F(3, 13)),
               (F(3, 13), F(4, 13), F(12, 13))]
    for index in range(256):
        cell = (1, 1, 1)
        p = tuple(q32_box(F(1) + F(rng.randint(4, 12), 16), 1) for _ in range(3))
        if index < 224:
            base = triples[index % 3]
            signs = tuple(-1 if ((index >> axis) & 1) else 1 for axis in range(3))
            d = tuple(q62_box(base[axis] * signs[axis], 1) for axis in range(3))
        else:
            # Deliberately near a face-time tie: sound boxes should usually
            # refuse to select even when some sampled correlated states agree.
            a = F(9, 13)
            delta = F((index % 4) - 1, 1 << 62)
            d = (q62_box(a + delta, 1), q62_box(a - delta, 1), q62_box(F(1, 13), 1))
        result = step(p, d, cell)
        counts[result["kind"]] += 1
        corner_checks += verify_corner_containment(p, d, cell, result)
        if result["kind"] == "ambiguous_next_face":
            corner_faces = set()
            for pp in product(*[(x[0], x[1]) for x in p]):
                for dd in product(*[(x[0], x[1]) for x in d]):
                    _, faces = exact_face(tuple(raw_fraction(v) for v in pp),
                                          tuple(raw_fraction(v) for v in dd), cell)
                    corner_faces.add(tuple(faces))
            if len(corner_faces) == 1:
                false_ambiguity_controls += 1
    return counts, corner_checks, false_ambiguity_controls


def repeated_portfolio():
    directions = {
        "red": (q62_box(F(1)), q62_box(F(1, 1 << 62)), q62_box(F(0))),
        "green": (q62_box(F(1)), q62_box(F(2, 1 << 62)), q62_box(F(1, 1 << 62))),
        "blue": (q62_box(F(1)), q62_box(F(3, 1 << 62)), q62_box(F(-1, 1 << 62))),
        "widened": (q62_box(F(1), 1), q62_box(F(1, 1 << 20), 1), q62_box(F(1, 1 << 21), 1)),
    }
    receipts = {}
    for name, direction in directions.items():
        point = (q32_box(F(0)), q32_box(F(1, 2)), q32_box(F(1, 2)))
        cell = (0, 0, 0)
        max_point_width = 0
        total_corner_checks = 0
        for _ in range(64):
            result = step(point, direction, cell, extent=(65, 2, 2))
            assert result["kind"] == "certified_next_face" and result["face"] == "x+", (name, result)
            total_corner_checks += verify_corner_containment(point, direction, cell, result)
            max_point_width = max(max_point_width, *widths(result["next_point_box"]))
            point = result["next_point_box"]
            cell = result["next_cell"]
        receipts[name] = {"steps": 64, "final_cell": cell,
                          "maximum_point_width_q160": max_point_width,
                          "corner_checks": total_corner_checks}
    return receipts


def main():
    named, named_checks = named_cases()
    generated, generated_checks, false_ambiguity = generated_portfolio()
    repeated = repeated_portfolio()
    receipt = {
        "schema_version": 1,
        "oracle_kind": "fixed160_outward_candidate_with_independent_fraction_corner_checks",
        "fractional_bits": F_BITS,
        "named_cases": named,
        "generated_256": {
            "outcomes": generated,
            "certified_corner_containment_checks": generated_checks,
            "ambiguous_cases_with_one_sampled_corner_face": false_ambiguity,
        },
        "repeated_64_step_lanes": repeated,
        "named_corner_containment_checks": named_checks,
        "maximum_observed_live_bits": MAX_BITS,
        "interpretation": {
            "certification": "whole_box_strict_face_order_only",
            "axis_box_ambiguity": "sound_typed_result_not_numerical_failure",
            "point_lineage": "must_replay_from_exact_initial_face_or_prior_cell_step",
            "endpoint_relation": "belongs_to_later_ordering_consumer_not_one_cell_step",
            "next_dependency": "implementation_readiness_audit_not_interval_bulk_or_composer",
        },
    }
    canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    output = {**receipt, "receipt_sha256": hashlib.sha256(canonical).hexdigest()}
    print(json.dumps(output, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
