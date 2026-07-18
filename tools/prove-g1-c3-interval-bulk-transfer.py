#!/usr/bin/env python3
"""Disposable exact oracle for one-band conditional interval bulk transfer.

This imports the retained cell-step and exponential proof harnesses as
independent reference components. It is not production code, a coefficient
source, a path composer, or a visibility result.
"""
from decimal import Decimal, localcontext
from fractions import Fraction as F
from importlib.util import module_from_spec, spec_from_file_location
from itertools import product
from math import isqrt
from pathlib import Path
import hashlib
import json
import random

ROOT = Path(__file__).resolve().parent


def load(name, path):
    spec = spec_from_file_location(name, path)
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


cell = load("interval_cell_step_oracle", ROOT / "prove-g1-c3-interval-optical-cell-step.py")
bulk = load("bulk_transfer_oracle", ROOT / "prove-g1-c3-visible-radiance-math.py")

S = 1 << 160
Q48 = 1 << 48
Q64 = 1 << 64
MAX_BITS = 0


def track(*values):
    global MAX_BITS
    for value in values:
        MAX_BITS = max(MAX_BITS, abs(value).bit_length())


def ceil_div(n, d):
    assert n >= 0 and d > 0
    track(n, d)
    return (n + d - 1) // d


def square_interval(bounds):
    lo, hi = bounds
    maximum = max(lo * lo, hi * hi)
    minimum = 0 if lo <= 0 <= hi else min(lo * lo, hi * hi)
    track(minimum, maximum)
    return minimum, maximum


def norm_bounds(direction):
    squares = [square_interval(axis) for axis in direction]
    lo2 = sum(value[0] for value in squares)
    hi2 = sum(value[1] for value in squares)
    track(lo2, hi2)
    lo = isqrt(lo2)
    hi_floor = isqrt(hi2)
    hi = hi_floor if hi_floor * hi_floor == hi2 else hi_floor + 1
    return lo, hi


def speed_time_length_bounds(direction, time_box):
    speed_lo, speed_hi = norm_bounds(direction)
    lo_product = speed_lo * time_box[0]
    hi_product = speed_hi * time_box[1]
    track(lo_product, hi_product)
    return lo_product // S, ceil_div(hi_product, S)


def displacement_length_bounds(point, hit_point):
    squared = []
    for start, end in zip(point, hit_point):
        delta = (end[0] - start[1], end[1] - start[0])
        squared.append(square_interval(delta))
    lo2 = sum(value[0] for value in squared)
    hi2 = sum(value[1] for value in squared)
    track(lo2, hi2)
    lo = isqrt(lo2)
    hi_floor = isqrt(hi2)
    hi = hi_floor if hi_floor * hi_floor == hi2 else hi_floor + 1
    return lo, hi


def length_bounds(point, direction, step_result):
    speed_time = speed_time_length_bounds(direction, step_result["time_box"])
    displacement = displacement_length_bounds(point, step_result["next_point_box"])
    intersection = (max(speed_time[0], displacement[0]),
                    min(speed_time[1], displacement[1]))
    assert intersection[0] <= intersection[1]
    return intersection, speed_time, displacement


def optical_depth_bounds(length_box, coefficient_q16_48):
    lo_product = length_box[0] * coefficient_q16_48
    hi_product = length_box[1] * coefficient_q16_48
    track(lo_product, hi_product)
    # Q160 length * Q48 coefficient -> Q64.64 by dividing by 2^144.
    return lo_product >> 144, ceil_div(hi_product, 1 << 144)


def transmission_bounds(tau_box):
    lower_q64, _ = bulk.exp_neg_q64_bounds(F(tau_box[1], Q64))
    _, upper_q64 = bulk.exp_neg_q64_bounds(F(tau_box[0], Q64))
    return (
        bulk.floor_mul_div(lower_q64, Q48, Q64),
        bulk.ceil_mul_div(upper_q64, Q48, Q64),
    )


def candidate(point, direction, current_cell, coefficient=Q48 // 4,
              extent=(128, 128, 128), current_evidence="substance",
              unavailable=frozenset(), opaque=False):
    result = cell.step(point, direction, current_cell, extent=extent,
                       unavailable=unavailable)
    if result["kind"] not in ("certified_next_face", "outer_domain_exit",
                               "unavailable_neighbor"):
        return {"kind": result["kind"], "cell_step": result}
    if current_evidence == "unavailable":
        return {"kind": "unavailable_current_cell", "cell_step": result}
    base = {"cell_step": result, "terminal_after_span": result["kind"]}
    if current_evidence == "vacuum" or coefficient == 0:
        lengths, speed_time, displacement = length_bounds(point, direction, result)
        return {"kind": "vacuum_or_zero_identity", "length_box": lengths,
                "speed_time_length_box": speed_time, "displacement_length_box": displacement,
                "transmission_q0_48": (Q48, Q48), **base}
    lengths, speed_time, displacement = length_bounds(point, direction, result)
    if opaque:
        return {"kind": "opaque", "length_box": lengths,
                "speed_time_length_box": speed_time, "displacement_length_box": displacement,
                "transmission_q0_48": (0, 0), **base}
    tau = optical_depth_bounds(lengths, coefficient)
    transmission = transmission_bounds(tau)
    return {"kind": "finite", "length_box": lengths,
            "speed_time_length_box": speed_time, "displacement_length_box": displacement,
            "optical_depth_q64_64": tau, "transmission_q0_48": transmission,
            **base}


def decimal(value):
    return Decimal(value.numerator) / Decimal(value.denominator)


def verify_witnesses(point, direction, current_cell, result, coefficient):
    if result["kind"] not in ("finite", "vacuum_or_zero_identity", "opaque"):
        return 0
    step_result = result["cell_step"]
    checks = 0
    for p_raw in product(*point):
        for d_raw in product(*direction):
            p = tuple(F(value, S) for value in p_raw)
            d = tuple(F(value, S) for value in d_raw)
            time, faces = cell.exact_face(p, d, current_cell)
            assert faces == [step_result["face"]]
            speed_squared = sum(value * value for value in d)
            exact_length_squared = speed_squared * time * time
            lo, hi = result["length_box"]
            assert F(lo * lo, S * S) <= exact_length_squared <= F(hi * hi, S * S)
            if result["kind"] == "finite":
                with localcontext() as context:
                    context.prec = 140
                    true_length = decimal(speed_squared).sqrt() * decimal(time)
                    true_tau = true_length * Decimal(coefficient) / Decimal(Q48)
                    tau_lo, tau_hi = result["optical_depth_q64_64"]
                    assert Decimal(tau_lo) / Decimal(Q64) <= true_tau <= Decimal(tau_hi) / Decimal(Q64)
                    truth = (-true_tau).exp() * Decimal(Q48)
                    tr_lo, tr_hi = result["transmission_q0_48"]
                    assert Decimal(tr_lo) <= truth <= Decimal(tr_hi)
            checks += 1
    return checks


def compact(result):
    output = {"kind": result["kind"]}
    for key in ("length_box", "speed_time_length_box", "displacement_length_box",
                "optical_depth_q64_64", "transmission_q0_48", "terminal_after_span"):
        if key in result:
            output[key] = result[key]
    if "cell_step" in result:
        output["cell_step_kind"] = result["cell_step"]["kind"]
        if "face" in result["cell_step"]:
            output["face"] = result["cell_step"]["face"]
    return output


def named_cases():
    centre = tuple(cell.q32_box(F(1, 2)) for _ in range(3))
    zero = cell.q62_box(F(0))
    one = cell.q62_box(F(1))
    tiny = (1 << 98, 1 << 98)
    cases = {
        "finite_axis": (centre, (one, zero, zero), {}, Q48 // 4, "finite"),
        "finite_nonsquare_norm": (centre, (cell.q62_box(F(3, 4)), cell.q62_box(F(1, 2)), zero), {}, Q48 // 3, "finite"),
        "minimum_positive_component": (centre, (one, tiny, zero), {}, Q48 - 1, "finite"),
        "zero_coefficient": (centre, (one, zero, zero), {}, 0, "vacuum_or_zero_identity"),
        "vacuum": (centre, (one, zero, zero), {"current_evidence": "vacuum"}, Q48 // 2, "vacuum_or_zero_identity"),
        "opaque": (centre, (one, zero, zero), {"opaque": True}, Q48, "opaque"),
        "unavailable_current": (centre, (one, zero, zero), {"current_evidence": "unavailable"}, Q48, "unavailable_current_cell"),
        "outer_after_span": (centre, (one, zero, zero), {"extent": (1, 1, 1)}, Q48 // 2, "finite"),
        "unavailable_after_span": (centre, (one, zero, zero), {"unavailable": frozenset({(1, 0, 0)})}, Q48 // 2, "finite"),
        "ambiguous_geometry": (centre, (one, one, zero), {}, Q48 // 2, "ambiguous_next_face"),
        "no_progress": (centre, (zero, zero, zero), {}, Q48 // 2, "no_forward_progress"),
    }
    output = {}
    checks = 0
    for name, (point, direction, kwargs, coefficient, expected) in cases.items():
        result = candidate(point, direction, (0, 0, 0), coefficient, **kwargs)
        assert result["kind"] == expected, (name, result)
        checks += verify_witnesses(point, direction, (0, 0, 0), result, coefficient)
        output[name] = compact(result)
    return output, checks


def generated_portfolio():
    rng = random.Random(0xB017160)
    counts = {"finite": 0, "ambiguous_next_face": 0, "no_forward_progress": 0}
    checks = 0
    maximum_length_width = 0
    maximum_transmission_width = 0
    for index in range(256):
        point = tuple(cell.q32_box(F(rng.randrange(1, 7), 8), rng.randrange(0, 2)) for _ in range(3))
        if index % 8:
            major = index % 3
            raw = []
            for axis in range(3):
                base = F(7, 8) if axis == major else F(rng.randrange(-2, 3), 16)
                raw.append(cell.q62_box(base, 1))
            direction = tuple(raw)
        else:
            tie = cell.q62_box(F(3, 4), 1)
            direction = (tie, tie, cell.q62_box(F(0)))
        coefficient = rng.randrange(0, Q48)
        result = candidate(point, direction, (0, 0, 0), coefficient)
        counts[result["kind"]] += 1
        checks += verify_witnesses(point, direction, (0, 0, 0), result, coefficient)
        if "length_box" in result:
            maximum_length_width = max(maximum_length_width, result["length_box"][1] - result["length_box"][0])
        if "transmission_q0_48" in result:
            maximum_transmission_width = max(maximum_transmission_width, result["transmission_q0_48"][1] - result["transmission_q0_48"][0])
    return counts, checks, maximum_length_width, maximum_transmission_width


def repeated_lanes():
    directions = {
        "red": (cell.q62_box(F(1)), cell.q62_box(F(1, 1 << 62)), cell.q62_box(F(0))),
        "green": (cell.q62_box(F(1)), cell.q62_box(F(2, 1 << 62)), cell.q62_box(F(1, 1 << 62))),
        "blue": (cell.q62_box(F(1)), cell.q62_box(F(3, 1 << 62)), cell.q62_box(F(-1, 1 << 62))),
        "widened": (cell.q62_box(F(1), 1), cell.q62_box(F(1, 1 << 20), 1), cell.q62_box(F(1, 1 << 21), 1)),
    }
    receipts = {}
    for lane_index, (name, direction) in enumerate(directions.items()):
        point = (cell.q32_box(F(0)), cell.q32_box(F(1, 2)), cell.q32_box(F(1, 2)))
        current = (0, 0, 0)
        maximum_length_width = 0
        maximum_transmission_width = 0
        checks = 0
        for step_index in range(64):
            coefficient = (Q48 // 128) * (lane_index + 1)
            result = candidate(point, direction, current, coefficient, extent=(65, 2, 2))
            assert result["kind"] == "finite" and result["cell_step"]["face"] == "x+"
            checks += verify_witnesses(point, direction, current, result, coefficient)
            maximum_length_width = max(maximum_length_width, result["length_box"][1] - result["length_box"][0])
            maximum_transmission_width = max(maximum_transmission_width, result["transmission_q0_48"][1] - result["transmission_q0_48"][0])
            point = result["cell_step"]["next_point_box"]
            current = result["cell_step"]["next_cell"]
        receipts[name] = {"steps": 64, "final_cell": current,
                          "maximum_length_width_q160": maximum_length_width,
                          "maximum_transmission_width_q0_48": maximum_transmission_width,
                          "corner_checks": checks}
    return receipts


def main():
    named, named_checks = named_cases()
    generated, generated_checks, length_width, transmission_width = generated_portfolio()
    repeated = repeated_lanes()
    receipt = {
        "schema_version": 1,
        "oracle_kind": "one_band_speed_norm_times_certified_time_with_exact_correlated_witnesses",
        "fractional_bits": 160,
        "named_cases": named,
        "named_corner_checks": named_checks,
        "generated_256": {"outcomes": generated, "corner_checks": generated_checks,
                          "maximum_length_width_q160": length_width,
                          "maximum_transmission_width_q0_48": transmission_width},
        "repeated_64_step_lanes": repeated,
        "maximum_observed_live_bits": MAX_BITS,
        "interpretation": {
            "spectral_geometry": "one_band_per_cell_step_after_dispersion",
            "length": "intersection_of_speed_time_and_start_hit_displacement_enclosures",
            "terminal_neighbor": "current_cell_transfer_retained_before_outer_or_unavailable_neighbor",
            "lineage": "local_conditional_only_composer_must_bind_ordered_receipts",
            "arithmetic": "third_private_signed512_copy_requires_explicit_consolidation_disposition",
        },
    }
    canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    print(json.dumps({**receipt, "receipt_sha256": hashlib.sha256(canonical).hexdigest()}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
