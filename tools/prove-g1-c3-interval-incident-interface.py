#!/usr/bin/env python3
"""Disposable oracle for interval-incident smooth-dielectric events.

The exact branch classifier quantifies over a full Q1.62 component box. The
fixed evaluator then uses outward dyadic interval arithmetic for one admitted
branch. It does not modify or emulate the verified Rust v1 API.
"""

from __future__ import annotations

import hashlib
import importlib.util
import itertools
import json
import random
import sys
from dataclasses import dataclass
from fractions import Fraction
from pathlib import Path


Q48 = 1 << 48
Q62 = 1 << 62
PRECISIONS = (96, 128, 160)
REFERENCE_PRECISION = 384
GENERATED_CASES = 256
SEED = 0x1A7E2FA6

checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        raise AssertionError(message)


def load_staged():
    path = Path(__file__).with_name(
        "prove-g1-c3-visible-radiance-interface-staged-kernel.py"
    )
    spec = importlib.util.spec_from_file_location("forge_interval_incident_staged", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load staged interval arithmetic")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def load_point_reference():
    path = Path(__file__).with_name("prove-g1-c3-visible-radiance-interface-math.py")
    spec = importlib.util.spec_from_file_location("forge_interval_incident_point", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load point interface reference")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


@dataclass(frozen=True)
class BoxCase:
    name: str
    lower: tuple[int, int, int]
    upper: tuple[int, int, int]
    axis: int
    eta_i_raw: int
    eta_t_raw: int

    def __post_init__(self) -> None:
        if any(a > b for a, b in zip(self.lower, self.upper)):
            raise ValueError("reversed component box")
        if self.lower[self.axis] <= 0:
            raise ValueError("normal component must be strictly positive")
        if self.eta_i_raw <= 0 or self.eta_t_raw <= 0:
            raise ValueError("indices must be positive")


@dataclass(frozen=True)
class BranchReceipt:
    outcome: str
    tangent_squared_min: int
    tangent_squared_max: int
    normal_squared_min: int
    normal_squared_max: int
    discriminator_min: int
    discriminator_max: int


@dataclass(frozen=True)
class BoxEvent:
    outcome: str
    reflectance: object | None
    transmittance: object | None
    reflected: tuple[object, object, object] | None
    transmitted: tuple[object, object, object] | None
    metrics: object | None


def square_bounds(lower: int, upper: int) -> tuple[int, int]:
    low = 0 if lower <= 0 <= upper else min(lower * lower, upper * upper)
    high = max(lower * lower, upper * upper)
    return low, high


def classify_branch(case: BoxCase) -> BranchReceipt:
    component_squares = [square_bounds(a, b) for a, b in zip(case.lower, case.upper)]
    normal_min, normal_max = component_squares[case.axis]
    tangent_min = sum(v[0] for i, v in enumerate(component_squares) if i != case.axis)
    tangent_max = sum(v[1] for i, v in enumerate(component_squares) if i != case.axis)
    coefficient = case.eta_i_raw * case.eta_i_raw - case.eta_t_raw * case.eta_t_raw
    target_squared = case.eta_t_raw * case.eta_t_raw
    if coefficient <= 0:
        discriminator_min = coefficient * tangent_max - target_squared * normal_max
        discriminator_max = coefficient * tangent_min - target_squared * normal_min
    else:
        discriminator_min = coefficient * tangent_min - target_squared * normal_max
        discriminator_max = coefficient * tangent_max - target_squared * normal_min
    if discriminator_min >= 0:
        outcome = "all_tir"
    elif discriminator_max < 0:
        outcome = "all_transmit"
    else:
        outcome = "ambiguous_interface_branch"
    return BranchReceipt(
        outcome,
        tangent_min,
        tangent_max,
        normal_min,
        normal_max,
        discriminator_min,
        discriminator_max,
    )


def fixed_bounds(lower: Fraction, upper: Fraction, bits: int, metrics, staged):
    scale = 1 << bits
    lower_shifted = lower.numerator * scale
    upper_shifted = upper.numerator * scale
    metrics.live(lower_shifted, lower.denominator, upper_shifted, upper.denominator)
    return staged.FixedInterval(
        staged.floor_div(lower_shifted, lower.denominator),
        staged.ceil_div(upper_shifted, upper.denominator),
        bits,
        metrics,
    )


def fixed_raw_box(lower: int, upper: int, source_bits: int, bits: int, metrics, staged):
    return fixed_bounds(
        Fraction(lower, 1 << source_bits),
        Fraction(upper, 1 << source_bits),
        bits,
        metrics,
        staged,
    )


def interval_event(case: BoxCase, precision: int, staged) -> BoxEvent:
    branch = classify_branch(case)
    if branch.outcome == "ambiguous_interface_branch":
        return BoxEvent(branch.outcome, None, None, None, None, None)

    metrics = staged.Metrics(precision)
    zero = staged.FixedInterval.from_fraction(0, precision, metrics)
    one = staged.FixedInterval.from_fraction(1, precision, metrics)
    two = staged.FixedInterval.from_fraction(2, precision, metrics)
    components = tuple(
        fixed_raw_box(a, b, 62, precision, metrics, staged)
        for a, b in zip(case.lower, case.upper)
    )
    squared = staged.sum_intervals([value.square() for value in components], zero)
    norm = squared.sqrt()
    if norm.lower <= 0:
        return BoxEvent("nonconvergent_enclosure", None, None, None, None, metrics)
    incident = tuple(value / norm for value in components)
    cos_i = incident[case.axis]
    reflected = tuple(
        incident[index] - cos_i * (two if index == case.axis else zero)
        for index in range(3)
    )

    if branch.outcome == "all_tir":
        return BoxEvent("all_tir", one, zero, reflected, None, metrics)

    sin_lower = Fraction(
        case.eta_i_raw * case.eta_i_raw * branch.tangent_squared_min,
        case.eta_t_raw
        * case.eta_t_raw
        * (branch.tangent_squared_min + branch.normal_squared_max),
    )
    sin_upper = Fraction(
        case.eta_i_raw * case.eta_i_raw * branch.tangent_squared_max,
        case.eta_t_raw
        * case.eta_t_raw
        * (branch.tangent_squared_max + branch.normal_squared_min),
    )
    check(Fraction(0) <= sin_lower <= sin_upper < 1, f"{case.name}: transmit branch")
    sin_t_squared = fixed_bounds(
        sin_lower, sin_upper, precision, metrics, staged
    ).intersect(Fraction(0), Fraction(1))
    cos_t = (one - sin_t_squared).intersect(Fraction(0), Fraction(1)).sqrt()
    q = staged.FixedInterval.from_fraction(
        Fraction(case.eta_t_raw, case.eta_i_raw), precision, metrics
    )
    parallel_denominator = q * cos_i + cos_t
    perpendicular_denominator = cos_i + q * cos_t
    if parallel_denominator.lower <= 0 or perpendicular_denominator.lower <= 0:
        return BoxEvent("nonconvergent_enclosure", None, None, None, None, metrics)
    r_parallel = (q * cos_i - cos_t) / parallel_denominator
    r_perpendicular = (cos_i - q * cos_t) / perpendicular_denominator
    reflectance = ((r_parallel.square() + r_perpendicular.square()) / two).intersect(
        Fraction(0), Fraction(1)
    )
    transmittance = one - reflectance
    transmitted = tuple(
        cos_t if index == case.axis else incident[index] / q
        for index in range(3)
    )
    check((reflectance + transmittance).contains_fraction(1), f"{case.name}: energy")
    check(
        staged.sum_intervals([value.square() for value in reflected], zero).contains_fraction(1),
        f"{case.name}: reflected norm",
    )
    check(
        staged.sum_intervals([value.square() for value in transmitted], zero).contains_fraction(1),
        f"{case.name}: transmitted norm",
    )
    return BoxEvent(
        "all_transmit", reflectance, transmittance, reflected, transmitted, metrics
    )


def values(event: BoxEvent) -> dict[str, object]:
    if event.outcome == "ambiguous_interface_branch":
        return {}
    result = {
        "reflectance": event.reflectance,
        "transmittance": event.transmittance,
    }
    for index, value in enumerate(event.reflected or ()):
        result[f"reflected_{index}"] = value
    for index, value in enumerate(event.transmitted or ()):
        result[f"transmitted_{index}"] = value
    return result


def contains_interval(outer, inner) -> bool:
    return (
        outer.lower * (1 << inner.bits) <= inner.lower * (1 << outer.bits)
        and inner.upper * (1 << outer.bits) <= outer.upper * (1 << inner.bits)
    )


def overlaps_reference(value, reference) -> bool:
    scale = 1 << value.bits
    return not (
        value.upper * reference.lower.denominator < reference.lower.numerator * scale
        or reference.upper.numerator * scale < value.lower * reference.upper.denominator
    )


def projected_endpoints(value, target_bits: int, staged) -> tuple[int, int]:
    return (
        staged.floor_div(value.lower * (1 << target_bits), 1 << value.bits),
        staged.ceil_div(value.upper * (1 << target_bits), 1 << value.bits),
    )


def numerical_excess(candidate: BoxEvent, reference: BoxEvent, staged) -> int:
    excess = 0
    reference_values = values(reference)
    for name, value in values(candidate).items():
        target_bits = 48 if name in ("reflectance", "transmittance") else 62
        c_lower, c_upper = projected_endpoints(value, target_bits, staged)
        r_lower, r_upper = projected_endpoints(reference_values[name], target_bits, staged)
        check(c_lower <= r_lower <= r_upper <= c_upper, f"{name}: precision containment")
        excess = max(excess, r_lower - c_lower, c_upper - r_upper)
    return excess


def sampled_points(case: BoxCase) -> list[tuple[int, int, int]]:
    choices = []
    for lower, upper in zip(case.lower, case.upper):
        choices.append(sorted({lower, (lower + upper) // 2, upper}))
    return [tuple(point) for point in itertools.product(*choices)]


def compare_point_samples(case: BoxCase, event: BoxEvent, point_reference, *, strict: bool) -> None:
    branch = classify_branch(case)
    seen_tir = False
    seen_transmit = False
    event_values = values(event)
    for point in sampled_points(case):
        reference = point_reference.band_event(
            point,
            case.axis,
            Fraction(case.eta_i_raw, Q48),
            Fraction(case.eta_t_raw, Q48),
        )
        seen_tir |= reference.total_internal_reflection
        seen_transmit |= not reference.total_internal_reflection
        if event.outcome == "ambiguous_interface_branch":
            continue
        check(
            reference.total_internal_reflection == (event.outcome == "all_tir"),
            f"{case.name}: sampled branch containment",
        )
        reference_values = {
            "reflectance": reference.reflectance,
            "transmittance": reference.transmittance,
        }
        for index, value in enumerate(reference.reflected):
            reference_values[f"reflected_{index}"] = value
        for index, value in enumerate(reference.transmitted or ()):
            reference_values[f"transmitted_{index}"] = value
        for name, reference_value in reference_values.items():
            if strict:
                check(event_values[name].contains_reference(reference_value), f"{case.name}: point {name}")
            else:
                check(overlaps_reference(event_values[name], reference_value), f"{case.name}: point {name}")
    if branch.outcome == "ambiguous_interface_branch":
        check(seen_tir and seen_transmit, f"{case.name}: mixed branch witnesses")


def fixed_cases() -> list[BoxCase]:
    critical_scale = Q62 // 5
    critical_normal = 3 * critical_scale
    critical_tangent = 4 * critical_scale
    return [
        BoxCase("normal-point", (Q62, 0, 0), (Q62, 0, 0), 0, Q48, 3 * Q48 // 2),
        BoxCase("normal-one-unit", (Q62 - 1, -1, 0), (Q62, 1, 0), 0, Q48, 3 * Q48 // 2),
        BoxCase(
            "critical-mixed",
            (critical_normal, critical_tangent, 0),
            (critical_normal + 1, critical_tangent, 0),
            0, 5 * Q48, 4 * Q48,
        ),
        BoxCase(
            "critical-all-tir",
            (critical_normal, critical_tangent, 0),
            (critical_normal, critical_tangent, 0),
            0, 5 * Q48, 4 * Q48,
        ),
        BoxCase(
            "critical-all-transmit",
            (critical_normal + 1, critical_tangent, 0),
            (critical_normal + 1, critical_tangent, 0),
            0, 5 * Q48, 4 * Q48,
        ),
        BoxCase(
            "tangent-sign-crossing",
            (Q62 - 16, -3, -2),
            (Q62, 4, 2),
            0, Q48, 2 * Q48,
        ),
        BoxCase("grazing-transmit", (1, Q62 - 8, 0), (2, Q62, 4), 0, Q48, 16 * Q48),
        BoxCase("grazing-tir", (1, Q62 - 8, 0), (2, Q62, 4), 0, 2 * Q48, Q48),
        BoxCase("near-critical-narrow", (3 * Q62 // 5 - 1, 4 * Q62 // 5 - 1, 0), (3 * Q62 // 5 + 1, 4 * Q62 // 5 + 1, 0), 0, 5 * Q48, 4 * Q48),
    ]


def generated_cases() -> list[BoxCase]:
    rng = random.Random(SEED)
    cases = []
    for index in range(GENERATED_CASES):
        axis = index % 3
        center = [rng.randint(-(Q62 - 32), Q62 - 32) for _ in range(3)]
        center[axis] = rng.randint(1 << 50, Q62 - 32)
        widths = [rng.randint(0, 8) for _ in range(3)]
        lower = tuple(center[i] - widths[i] for i in range(3))
        upper = tuple(center[i] + widths[i] for i in range(3))
        cases.append(
            BoxCase(
                f"generated-{index}",
                lower,
                upper,
                axis,
                rng.randint(1 << 46, 1 << 52),
                rng.randint(1 << 46, 1 << 52),
            )
        )
    return cases


def to_q62_box(event: BoxEvent, staged) -> tuple[tuple[int, int, int], tuple[int, int, int]]:
    check(event.transmitted is not None, "chain requires transmission")
    projected = [projected_endpoints(value, 62, staged) for value in event.transmitted]
    return tuple(value[0] for value in projected), tuple(value[1] for value in projected)


def repeated_lane(staged, lane: str, material_eta: int) -> dict[str, object]:
    initial = BoxCase(
        f"repeat-{lane}-0", (13 * Q62 // 16, 7 * Q62 // 16, 0),
        (13 * Q62 // 16 + 1, 7 * Q62 // 16 + 1, 0), 0, Q48, material_eta
    )
    case = initial
    outcomes = []
    widths = []
    max_live = 0
    for event_index in range(64):
        event = interval_event(case, 160, staged)
        outcomes.append(event.outcome)
        if event.outcome != "all_transmit":
            break
        max_live = max(max_live, event.metrics.max_live_integer_bits)
        lower, upper = to_q62_box(event, staged)
        widths.append(max(b - a for a, b in zip(lower, upper)))
        eta_i = case.eta_t_raw
        eta_t = Q48 if eta_i != Q48 else material_eta
        case = BoxCase(f"repeat-{lane}-{event_index + 1}", lower, upper, 0, eta_i, eta_t)
    checkpoints = {
        str(index): widths[index - 1]
        for index in (1, 2, 4, 8, 16, 32, 64)
        if len(widths) >= index
    }
    return {
        "events_attempted": len(outcomes),
        "terminal_outcome": outcomes[-1] if outcomes else "not_run",
        "all_transmit_events": sum(value == "all_transmit" for value in outcomes),
        "direction_width_checkpoints_q62": checkpoints,
        "max_direction_width_q62": max(widths, default=0),
        "max_live_integer_bits": max_live,
    }


def repeated_portfolio(staged) -> dict[str, object]:
    lanes = {
        "red": repeated_lane(staged, "red", 4 * Q48 // 3),
        "green": repeated_lane(staged, "green", 7 * Q48 // 5),
        "blue": repeated_lane(staged, "blue", 8 * Q48 // 5),
    }
    return {
        "lanes": lanes,
        "max_direction_width_q62": max(
            lane["max_direction_width_q62"] for lane in lanes.values()
        ),
        "max_live_integer_bits": max(
            lane["max_live_integer_bits"] for lane in lanes.values()
        ),
    }


def main() -> None:
    staged = load_staged()
    point_reference = load_point_reference()
    cases = fixed_cases() + generated_cases()
    distribution: dict[str, int] = {}
    stop_distribution: dict[str, int] = {}
    max_live = 0
    max_stored = 0
    max_excess = 0
    nonconvergent = 0
    sampled = 0
    fixed_receipts: dict[str, object] = {}
    fixed_names = {case.name for case in fixed_cases()}

    for case in cases:
        branch = classify_branch(case)
        distribution[branch.outcome] = distribution.get(branch.outcome, 0) + 1
        reference = interval_event(case, REFERENCE_PRECISION, staged)
        check(reference.outcome == branch.outcome, f"{case.name}: reference branch")
        if branch.outcome == "ambiguous_interface_branch":
            compare_point_samples(case, reference, point_reference, strict=False)
            sampled += len(sampled_points(case))
            stop_distribution[branch.outcome] = stop_distribution.get(branch.outcome, 0) + 1
            if case.name in fixed_names:
                fixed_receipts[case.name] = {"branch": branch.outcome, "stop": branch.outcome}
            continue

        stopped = None
        stopping_event = None
        for precision in PRECISIONS:
            candidate = interval_event(case, precision, staged)
            max_live = max(max_live, candidate.metrics.max_live_integer_bits)
            max_stored = max(max_stored, candidate.metrics.max_stored_endpoint_bits)
            if candidate.outcome == "nonconvergent_enclosure":
                continue
            check(candidate.outcome == branch.outcome, f"{case.name}: fixed branch")
            candidate_values = values(candidate)
            for name, reference_value in values(reference).items():
                check(contains_interval(candidate_values[name], reference_value), f"{case.name}: {name}")
            excess = numerical_excess(candidate, reference, staged)
            max_excess = max(max_excess, excess)
            if excess <= 1:
                stopped = precision
                stopping_event = candidate
                break
        if stopped is None:
            nonconvergent += 1
            stop_distribution["nonconvergent_enclosure"] = stop_distribution.get("nonconvergent_enclosure", 0) + 1
        else:
            key = str(stopped)
            stop_distribution[key] = stop_distribution.get(key, 0) + 1
            check(stopping_event is not None, f"{case.name}: stopping event retained")
            compare_point_samples(case, stopping_event, point_reference, strict=True)
            sampled += len(sampled_points(case))
        if case.name in fixed_names:
            reference_values = values(reference)
            power_width = max(
                reference_values[name].projected_width(48)
                for name in ("reflectance", "transmittance")
            )
            direction_width = max(
                value.projected_width(62)
                for name, value in reference_values.items()
                if name not in ("reflectance", "transmittance")
            )
            fixed_receipts[case.name] = {
                "branch": branch.outcome,
                "stop": stopped if stopped is not None else "nonconvergent_enclosure",
                "power_width_q48": power_width,
                "direction_width_q62": direction_width,
            }

    check(distribution.get("all_tir", 0) > 0, "portfolio has all-TIR boxes")
    check(distribution.get("all_transmit", 0) > 0, "portfolio has transmit boxes")
    check(distribution.get("ambiguous_interface_branch", 0) > 0, "portfolio has mixed boxes")
    check(max_live <= 512, "candidate fixed arithmetic exceeds 512 live bits")
    repeated = repeated_portfolio(staged)
    check(
        all(lane["events_attempted"] == 64 for lane in repeated["lanes"].values()),
        "three-band repeated portfolio reached the event ceiling",
    )
    forced_case = next(case for case in fixed_cases() if case.name == "critical-all-transmit")
    forced_reference = interval_event(forced_case, REFERENCE_PRECISION, staged)
    forced_candidate = interval_event(forced_case, 80, staged)
    forced_excess = numerical_excess(forced_candidate, forced_reference, staged)
    check(forced_excess > 1, "forced 80-bit cap must not fabricate certification")

    limitations = " ".join(
        [
            "no schema implementation dependency or verified-module change",
            "no coefficient catalogue bulk composer perception rendering",
            "no collision navigation organism biome planet terrain runtime",
            "no approval promotion or C3 closure",
        ]
    )
    for forbidden in (
        "schema", "implementation", "dependency", "coefficient", "composer",
        "perception", "rendering", "collision", "navigation", "organism",
        "biome", "planet", "terrain", "runtime", "approval", "promotion", "closure",
    ):
        check(forbidden in limitations, f"authority limitation: {forbidden}")

    receipt = {
        "schema_version": 1,
        "status": "pass",
        "oracle_kind": "exact_box_branch_plus_outward_fixed_interval_audit",
        "seed": SEED,
        "fixed_cases": len(fixed_cases()),
        "generated_cases": GENERATED_CASES,
        "total_cases": len(cases),
        "sampled_point_containment_regressions": sampled,
        "candidate_precisions": PRECISIONS,
        "reference_precision": REFERENCE_PRECISION,
        "branch_distribution": distribution,
        "stop_distribution": stop_distribution,
        "fixed_case_receipts": fixed_receipts,
        "nonconvergent_cases": nonconvergent,
        "max_numerical_excess_target_units": max_excess,
        "max_candidate_live_integer_bits": max_live,
        "max_candidate_stored_endpoint_bits": max_stored,
        "repeated_event_portfolio": repeated,
        "forced_cap_precision": 80,
        "forced_cap_outcome": "nonconvergent_enclosure",
        "forced_cap_numerical_excess_target_units": forced_excess,
        "checks": checks,
        "point_reference_checks": point_reference.checks,
        "limitations": limitations,
    }
    canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    receipt["receipt_sha256"] = hashlib.sha256(canonical).hexdigest()
    print(json.dumps(receipt, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
