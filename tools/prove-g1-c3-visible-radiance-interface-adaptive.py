#!/usr/bin/env python3
"""Disposable oracle for bounded adaptive interface interval refinement."""

from __future__ import annotations

import importlib.util
import json
import sys
from dataclasses import dataclass
from fractions import Fraction
from math import isqrt
from pathlib import Path


ADAPTIVE_LADDER = (96, 128, 160, 192, 256, 384)
REJECTED_BASELINES = (72, 80)
POWER_BITS = 48
DIRECTION_BITS = 62

checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        raise AssertionError(message)


def load_staged_module():
    path = Path(__file__).with_name(
        "prove-g1-c3-visible-radiance-interface-staged-kernel.py"
    )
    spec = importlib.util.spec_from_file_location("forge_interface_staged", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load staged interface oracle")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    check(module.REQUIRED_PRECISIONS == (72, 80, 96, 128), "staged baseline drift")
    return module


def floor_fraction(value: Fraction) -> int:
    return value.numerator // value.denominator


def ceil_fraction(value: Fraction) -> int:
    return -((-value.numerator) // value.denominator)


@dataclass(frozen=True)
class RationalInterval:
    lower: Fraction
    upper: Fraction

    def __post_init__(self) -> None:
        if self.lower > self.upper:
            raise ValueError("reversed rational interval")

    @classmethod
    def from_fixed(cls, value) -> "RationalInterval":
        scale = 1 << value.bits
        return cls(Fraction(value.lower, scale), Fraction(value.upper, scale))

    def intersect(self, other: "RationalInterval") -> "RationalInterval":
        return RationalInterval(max(self.lower, other.lower), min(self.upper, other.upper))

    def contains_reference(self, reference) -> bool:
        return self.lower <= reference.lower and reference.upper <= self.upper

    def projected_width(self, bits: int) -> int:
        scale = 1 << bits
        return ceil_fraction(self.upper * scale) - floor_fraction(self.lower * scale)


def event_intervals(event) -> dict[str, RationalInterval]:
    values = {
        "reflectance": RationalInterval.from_fixed(event.reflectance),
        "transmittance": RationalInterval.from_fixed(event.transmittance),
    }
    for index, component in enumerate(event.reflected):
        values[f"reflected_{index}"] = RationalInterval.from_fixed(component)
    if event.transmitted is not None:
        for index, component in enumerate(event.transmitted):
            values[f"transmitted_{index}"] = RationalInterval.from_fixed(component)
    return values


def reference_intervals(event) -> dict[str, object]:
    values = {
        "reflectance": event.reflectance,
        "transmittance": event.transmittance,
    }
    for index, component in enumerate(event.reflected):
        values[f"reflected_{index}"] = component
    if event.transmitted is not None:
        for index, component in enumerate(event.transmitted):
            values[f"transmitted_{index}"] = component
    return values


def all_outputs_certified(values: dict[str, RationalInterval]) -> bool:
    for name, value in values.items():
        target = POWER_BITS if name in ("reflectance", "transmittance") else DIRECTION_BITS
        if value.projected_width(target) > 1:
            return False
    return True


def squared_path(case) -> int:
    return sum(component * component for component in case.delta)


def perfect_square_root(case) -> int | None:
    squared = squared_path(case)
    root = isqrt(squared)
    return root if root * root == squared else None


def exact_fast_path_kind(case, staged) -> str | None:
    tangent_squared = squared_path(case) - case.delta[case.axis] ** 2
    if tangent_squared == 0:
        return "normal_incidence"
    root = perfect_square_root(case)
    if root is not None and case.eta_i_raw == case.eta_t_raw:
        return "index_matched_perfect_square"
    if root is not None and staged.exact_tir(case).tir:
        return "tir_perfect_square"
    return None


def exact_fast_path_event(case, staged, kind: str):
    metrics = staged.Metrics(96)
    bits = metrics.precision
    zero = staged.FixedInterval.from_fraction(0, bits, metrics)
    one = staged.FixedInterval.from_fraction(1, bits, metrics)

    if kind == "normal_incidence":
        eta_i = Fraction(case.eta_i_raw)
        eta_t = Fraction(case.eta_t_raw)
        reflectance_exact = ((eta_t - eta_i) / (eta_t + eta_i)) ** 2
        reflectance = staged.FixedInterval.from_fraction(reflectance_exact, bits, metrics)
        transmittance = one - reflectance
        reflected = tuple(
            staged.FixedInterval.from_fraction(-1 if i == case.axis else 0, bits, metrics)
            for i in range(3)
        )
        transmitted = tuple(
            staged.FixedInterval.from_fraction(1 if i == case.axis else 0, bits, metrics)
            for i in range(3)
        )
        return staged.StagedEvent(False, reflectance, transmittance, reflected, transmitted), metrics

    root = perfect_square_root(case)
    check(root is not None, f"{case.name}: fast path requires perfect square")
    incident = tuple(
        staged.FixedInterval.from_fraction(Fraction(component, root), bits, metrics)
        for component in case.delta
    )
    reflected = tuple(
        staged.FixedInterval.from_fraction(
            Fraction(-case.delta[i], root) if i == case.axis else Fraction(case.delta[i], root),
            bits,
            metrics,
        )
        for i in range(3)
    )
    if kind == "index_matched_perfect_square":
        return staged.StagedEvent(False, zero, one, reflected, incident), metrics
    if kind == "tir_perfect_square":
        return staged.StagedEvent(True, one, zero, reflected, None), metrics
    raise AssertionError(f"unknown exact fast path: {kind}")


def assert_fast_path_equivalence(case, event, reference, kind: str, staged) -> None:
    check(event.tir == reference.total_internal_reflection, f"{case.name}: fast-path branch")
    if kind == "normal_incidence":
        eta_i = Fraction(case.eta_i_raw)
        eta_t = Fraction(case.eta_t_raw)
        reflectance = ((eta_t - eta_i) / (eta_t + eta_i)) ** 2
        reflected = tuple(Fraction(-1 if i == case.axis else 0) for i in range(3))
        transmitted = tuple(Fraction(1 if i == case.axis else 0) for i in range(3))
    else:
        root = perfect_square_root(case)
        check(root is not None, f"{case.name}: exact root for equivalence")
        reflectance = Fraction(1 if kind == "tir_perfect_square" else 0)
        reflected = tuple(
            Fraction(-case.delta[i], root)
            if i == case.axis
            else Fraction(case.delta[i], root)
            for i in range(3)
        )
        transmitted = (
            None
            if kind == "tir_perfect_square"
            else tuple(Fraction(component, root) for component in case.delta)
        )
    transmittance = 1 - reflectance
    check(event.reflectance.contains_fraction(reflectance), f"{case.name}: fast reflectance")
    check(reference.reflectance.contains(reflectance), f"{case.name}: reference reflectance")
    check(event.transmittance.contains_fraction(transmittance), f"{case.name}: fast transmittance")
    check(reference.transmittance.contains(transmittance), f"{case.name}: reference transmittance")
    for index, exact in enumerate(reflected):
        check(event.reflected[index].contains_fraction(exact), f"{case.name}: fast reflected {index}")
        check(reference.reflected[index].contains(exact), f"{case.name}: reference reflected {index}")
    if transmitted is None:
        check(event.transmitted is None and reference.transmitted is None, f"{case.name}: fast no transmission")
    else:
        check(event.transmitted is not None and reference.transmitted is not None, f"{case.name}: fast transmission")
        for index, exact in enumerate(transmitted):
            check(event.transmitted[index].contains_fraction(exact), f"{case.name}: fast transmitted {index}")
            check(reference.transmitted[index].contains(exact), f"{case.name}: reference transmitted {index}")


def assert_structural_zeros(case, event) -> None:
    for index, component in enumerate(case.delta):
        if index != case.axis and component == 0:
            reflected = event.reflected[index]
            check(reflected.lower == 0 == reflected.upper, f"{case.name}: reflected zero drift")
            if event.transmitted is not None:
                transmitted = event.transmitted[index]
                check(
                    transmitted.lower == 0 == transmitted.upper,
                    f"{case.name}: transmitted zero drift",
                )


@dataclass
class AdaptiveReceipt:
    outcome: str
    stop_level: int | None
    evaluations: int
    fractional_bit_work: int
    max_live_bits: int
    max_stored_bits: int
    intersections: int
    fast_path: str | None


def adaptive_event(case, reference, staged, ladder=ADAPTIVE_LADDER) -> AdaptiveReceipt:
    kind = exact_fast_path_kind(case, staged)
    if kind is not None:
        event, metrics = exact_fast_path_event(case, staged, kind)
        assert_fast_path_equivalence(case, event, reference, kind, staged)
        assert_structural_zeros(case, event)
        check(all_outputs_certified(event_intervals(event)), f"{case.name}: fast path width")
        return AdaptiveReceipt(
            "known",
            None,
            0,
            0,
            metrics.max_live_integer_bits,
            metrics.max_stored_endpoint_bits,
            0,
            kind,
        )

    retained: dict[str, RationalInterval] | None = None
    reference_values = reference_intervals(reference)
    evaluations = 0
    work = 0
    max_live = 0
    max_stored = 0
    intersections = 0
    for precision in ladder:
        metrics = staged.Metrics(precision)
        event = staged.staged_event(case, metrics)
        staged.compare_event(case, event, reference)
        assert_structural_zeros(case, event)
        fresh = event_intervals(event)
        evaluations += 1
        work += precision
        max_live = max(max_live, metrics.max_live_integer_bits)
        max_stored = max(max_stored, metrics.max_stored_endpoint_bits)
        if retained is None:
            retained = fresh
        else:
            previous = retained
            check(previous.keys() == fresh.keys(), f"{case.name}: adaptive shape drift")
            retained = {}
            for name in previous:
                combined = previous[name].intersect(fresh[name])
                check(
                    previous[name].lower <= combined.lower
                    and combined.upper <= previous[name].upper,
                    f"{case.name}: retained interval widened",
                )
                retained[name] = combined
                intersections += 1
        for name, value in retained.items():
            check(
                value.contains_reference(reference_values[name]),
                f"{case.name}: adaptive reference containment {name}",
            )
        if all_outputs_certified(retained):
            return AdaptiveReceipt(
                "known",
                precision,
                evaluations,
                work,
                max_live,
                max_stored,
                intersections,
                None,
            )

    return AdaptiveReceipt(
        "nonconvergent_enclosure",
        None,
        evaluations,
        work,
        max_live,
        max_stored,
        intersections,
        None,
    )


def extra_cases(staged) -> list:
    return [
        staged.Case("normal-neighbor", (7, 1, 0), 0, staged.Q48, 3 * staged.Q48 // 2),
        staged.Case("matched-neighbor", (4, 3, 0), 0, 7 * staged.Q48 // 5, 7 * staged.Q48 // 5 + 1),
        staged.Case("perfect-square-neighbor", (4, 3, 1), 0, 7 * staged.Q48 // 5, 7 * staged.Q48 // 5),
        staged.Case("tir-perfect-neighbor", (4, 3, 0), 0, 5 * staged.Q48, 4 * staged.Q48 + 1),
    ]


def verify_negative_neighbors(cases: list, staged) -> None:
    by_name = {case.name: case for case in cases}
    check(exact_fast_path_kind(by_name["normal"], staged) == "normal_incidence", "normal fast path")
    check(exact_fast_path_kind(by_name["normal-neighbor"], staged) != "normal_incidence", "normal neighbor")
    check(exact_fast_path_kind(by_name["matched"], staged) == "index_matched_perfect_square", "matched fast path")
    check(exact_fast_path_kind(by_name["matched-neighbor"], staged) != "index_matched_perfect_square", "matched neighbor")
    check(perfect_square_root(by_name["matched"]) == 5, "perfect-square positive")
    check(perfect_square_root(by_name["perfect-square-neighbor"]) is None, "perfect-square neighbor")
    check(exact_fast_path_kind(by_name["critical"], staged) == "tir_perfect_square", "TIR perfect fast path")
    check(exact_fast_path_kind(by_name["tir-perfect-neighbor"], staged) != "tir_perfect_square", "TIR neighbor")


def main() -> None:
    staged = load_staged_module()
    reference_module = staged.load_reference()
    cases = staged.fixed_cases() + extra_cases(staged) + staged.generated_cases()
    verify_negative_neighbors(cases, staged)

    references = {
        case.name: reference_module.band_event(
            case.delta,
            case.axis,
            Fraction(case.eta_i_raw, staged.Q48),
            Fraction(case.eta_t_raw, staged.Q48),
        )
        for case in cases
    }

    distribution: dict[str, int] = {}
    fast_paths: dict[str, int] = {}
    nonconvergent = 0
    total_evaluations = 0
    total_work = 0
    max_live = 0
    max_stored = 0
    total_intersections = 0
    general_cases = 0
    fixed_160_supported = 0
    fixed_384_supported = 0
    fixed_160_work = 0
    fixed_384_work = 0

    for case in cases:
        receipt = adaptive_event(case, references[case.name], staged)
        max_live = max(max_live, receipt.max_live_bits)
        max_stored = max(max_stored, receipt.max_stored_bits)
        total_evaluations += receipt.evaluations
        total_work += receipt.fractional_bit_work
        total_intersections += receipt.intersections
        if receipt.fast_path is not None:
            fast_paths[receipt.fast_path] = fast_paths.get(receipt.fast_path, 0) + 1
            distribution["exact_fast_path"] = distribution.get("exact_fast_path", 0) + 1
            continue
        general_cases += 1
        if receipt.outcome == "nonconvergent_enclosure":
            nonconvergent += 1
            distribution["nonconvergent_enclosure"] = distribution.get("nonconvergent_enclosure", 0) + 1
        else:
            key = str(receipt.stop_level)
            distribution[key] = distribution.get(key, 0) + 1

        for precision in (160, 384):
            metrics = staged.Metrics(precision)
            event = staged.staged_event(case, metrics)
            staged.compare_event(case, event, references[case.name])
            supported = all_outputs_certified(event_intervals(event))
            if precision == 160:
                fixed_160_supported += int(supported)
                fixed_160_work += 160
            else:
                fixed_384_supported += int(supported)
                fixed_384_work += 384

    hostile = next(case for case in cases if case.name == "coprime-wide-transmit")
    forced_cap = adaptive_event(hostile, references[hostile.name], staged, ladder=(96, 128))
    check(forced_cap.outcome == "nonconvergent_enclosure", "forced cap must be typed nonconvergence")
    check(forced_cap.evaluations == 2, "forced cap must terminate at declared level")

    check(nonconvergent == 0, "main adaptive portfolio contains nonconvergent cases")
    check(sum(distribution.values()) == len(cases), "adaptive distribution total")
    check(fixed_384_supported == general_cases, "fixed 384 baseline must support portfolio")
    check(total_evaluations <= general_cases * len(ADAPTIVE_LADDER), "adaptive evaluation ceiling")
    check(total_work < fixed_384_work, "adaptive work must beat fixed 384 baseline")

    limitations = " ".join(
        [
            "no schema implementation dependency coefficient catalogue",
            "no downstream refractive path perception rendering",
            "no passage navigation biome sphere planet terrain runtime",
            "no approval promotion C3 closure",
        ]
    )
    for forbidden in (
        "schema", "implementation", "dependency", "coefficient", "path",
        "perception", "rendering", "passage", "navigation", "biome", "sphere",
        "planet", "terrain", "runtime", "approval", "promotion", "closure",
    ):
        check(forbidden in limitations, f"authority limitation {forbidden}")

    print(
        json.dumps(
            {
                "status": "pass",
                "adaptive_ladder": ADAPTIVE_LADDER,
                "rejected_baselines": REJECTED_BASELINES,
                "total_cases": len(cases),
                "generated_cases": staged.GENERATED_CASES,
                "checks": checks,
                "staged_dependency_checks": staged.checks,
                "stop_distribution": distribution,
                "exact_fast_path_distribution": fast_paths,
                "main_nonconvergent_cases": nonconvergent,
                "forced_cap_outcome": forced_cap.outcome,
                "forced_cap_evaluations": forced_cap.evaluations,
                "adaptive_staged_evaluations": total_evaluations,
                "adaptive_fractional_bit_work_units": total_work,
                "fixed_160_supported_general_cases": fixed_160_supported,
                "fixed_160_fractional_bit_work_units": fixed_160_work,
                "fixed_384_supported_general_cases": fixed_384_supported,
                "fixed_384_fractional_bit_work_units": fixed_384_work,
                "max_adaptive_live_integer_bits": max_live,
                "max_adaptive_stored_endpoint_bits": max_stored,
                "cross_level_intersections": total_intersections,
                "reference": "existing 384-bit arbitrary-precision interface oracle",
                "limitations": limitations,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
