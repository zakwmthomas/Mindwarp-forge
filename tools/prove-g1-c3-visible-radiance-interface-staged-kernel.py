#!/usr/bin/env python3
"""Disposable counterexample oracle for the staged interface numerical kernel."""

from __future__ import annotations

import importlib.util
import json
import random
import sys
from dataclasses import dataclass
from fractions import Fraction
from math import gcd, isqrt
from pathlib import Path


REQUIRED_PRECISIONS = (72, 80, 96, 128)
SENSITIVITY_PRECISIONS = (160, 192, 256, 384)
PRECISIONS = REQUIRED_PRECISIONS + SENSITIVITY_PRECISIONS
POWER_BITS = 48
DIRECTION_BITS = 62
Q48 = 1 << 48
MAX_SQUARED_DELTA = (1 << 128) - 1
GENERATED_CASES = 1024
SEED = 0xF0A6E17

checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        raise AssertionError(message)


def floor_div(numerator: int, denominator: int) -> int:
    if denominator == 0:
        raise ZeroDivisionError("zero denominator")
    if denominator < 0:
        numerator = -numerator
        denominator = -denominator
    return numerator // denominator


def ceil_div(numerator: int, denominator: int) -> int:
    return -floor_div(-numerator, denominator)


@dataclass
class Metrics:
    precision: int
    max_live_integer_bits: int = 0
    max_stored_endpoint_bits: int = 0
    max_power_width: int = 0
    max_direction_width: int = 0
    max_power_case: str = ""
    max_direction_case: str = ""
    theorem_intersections: int = 0
    transmitted: int = 0
    tir: int = 0

    def live(self, *values: int) -> None:
        for value in values:
            self.max_live_integer_bits = max(
                self.max_live_integer_bits, abs(value).bit_length()
            )

    def stored(self, *values: int) -> None:
        for value in values:
            self.max_stored_endpoint_bits = max(
                self.max_stored_endpoint_bits, abs(value).bit_length()
            )


@dataclass(frozen=True)
class FixedInterval:
    lower: int
    upper: int
    bits: int
    metrics: Metrics

    def __post_init__(self) -> None:
        if self.lower > self.upper:
            raise ValueError("reversed fixed interval")
        self.metrics.stored(self.lower, self.upper)

    @property
    def scale(self) -> int:
        return 1 << self.bits

    @classmethod
    def from_fraction(
        cls, value: int | Fraction, bits: int, metrics: Metrics
    ) -> "FixedInterval":
        exact = Fraction(value)
        shifted = exact.numerator << bits
        metrics.live(shifted, exact.denominator)
        return cls(
            floor_div(shifted, exact.denominator),
            ceil_div(shifted, exact.denominator),
            bits,
            metrics,
        )

    def compatible(self, other: "FixedInterval") -> None:
        if self.bits != other.bits or self.metrics is not other.metrics:
            raise ValueError("incompatible fixed intervals")

    def __add__(self, other: "FixedInterval") -> "FixedInterval":
        self.compatible(other)
        return FixedInterval(
            self.lower + other.lower,
            self.upper + other.upper,
            self.bits,
            self.metrics,
        )

    def __sub__(self, other: "FixedInterval") -> "FixedInterval":
        self.compatible(other)
        return FixedInterval(
            self.lower - other.upper,
            self.upper - other.lower,
            self.bits,
            self.metrics,
        )

    def __mul__(self, other: "FixedInterval") -> "FixedInterval":
        self.compatible(other)
        products = (
            self.lower * other.lower,
            self.lower * other.upper,
            self.upper * other.lower,
            self.upper * other.upper,
        )
        self.metrics.live(*products)
        return FixedInterval(
            floor_div(min(products), self.scale),
            ceil_div(max(products), self.scale),
            self.bits,
            self.metrics,
        )

    def __truediv__(self, other: "FixedInterval") -> "FixedInterval":
        self.compatible(other)
        if other.lower <= 0 <= other.upper:
            raise ZeroDivisionError("fixed denominator interval contains zero")
        candidates: list[tuple[int, int]] = []
        for numerator in (self.lower, self.upper):
            for denominator in (other.lower, other.upper):
                scaled_numerator = numerator * self.scale
                self.metrics.live(scaled_numerator, denominator)
                candidates.append((scaled_numerator, denominator))
        return FixedInterval(
            min(floor_div(n, d) for n, d in candidates),
            max(ceil_div(n, d) for n, d in candidates),
            self.bits,
            self.metrics,
        )

    def square(self) -> "FixedInterval":
        if self.lower <= 0 <= self.upper:
            product = max(self.lower * self.lower, self.upper * self.upper)
            self.metrics.live(product)
            return FixedInterval(
                0, ceil_div(product, self.scale), self.bits, self.metrics
            )
        return self * self

    def sqrt(self) -> "FixedInterval":
        if self.lower < 0:
            raise ValueError("negative fixed square root")
        lower_radicand = self.lower * self.scale
        upper_radicand = self.upper * self.scale
        self.metrics.live(lower_radicand, upper_radicand)
        lower = isqrt(lower_radicand)
        upper_floor = isqrt(upper_radicand)
        upper = upper_floor if upper_floor * upper_floor == upper_radicand else upper_floor + 1
        return FixedInterval(lower, upper, self.bits, self.metrics)

    def intersect(self, lower: Fraction, upper: Fraction) -> "FixedInterval":
        lower_raw = ceil_div(lower.numerator << self.bits, lower.denominator)
        upper_raw = floor_div(upper.numerator << self.bits, upper.denominator)
        result = FixedInterval(
            max(self.lower, lower_raw),
            min(self.upper, upper_raw),
            self.bits,
            self.metrics,
        )
        self.metrics.theorem_intersections += 1
        return result

    def contains_fraction(self, value: int | Fraction) -> bool:
        exact = Fraction(value)
        return (
            self.lower * exact.denominator
            <= exact.numerator * self.scale
            <= self.upper * exact.denominator
        )

    def contains_reference(self, reference) -> bool:
        return self.contains_fraction(reference.lower) and self.contains_fraction(
            reference.upper
        )

    def projected_width(self, bits: int) -> int:
        target_scale = 1 << bits
        lower = floor_div(self.lower * target_scale, self.scale)
        upper = ceil_div(self.upper * target_scale, self.scale)
        return upper - lower


def sum_intervals(values: list[FixedInterval], zero: FixedInterval) -> FixedInterval:
    result = zero
    for value in values:
        result = result + value
    return result


@dataclass(frozen=True)
class Case:
    name: str
    delta: tuple[int, int, int]
    axis: int
    eta_i_raw: int
    eta_t_raw: int


@dataclass(frozen=True)
class TirReceipt:
    tir: bool
    pre_left_bits: int
    pre_right_bits: int
    post_left_bits: int
    post_right_bits: int


def exact_tir(case: Case) -> TirReceipt:
    squared = sum(component * component for component in case.delta)
    normal = case.delta[case.axis]
    check(0 < squared <= MAX_SQUARED_DELTA, f"{case.name}: admitted squared delta")
    check(normal > 0, f"{case.name}: positive oriented normal")
    tangent_squared = squared - normal * normal

    pre_left = tangent_squared * case.eta_i_raw * case.eta_i_raw
    pre_right = squared * case.eta_t_raw * case.eta_t_raw

    ratio_gcd = gcd(case.eta_i_raw, case.eta_t_raw)
    eta_i = case.eta_i_raw // ratio_gcd
    eta_t = case.eta_t_raw // ratio_gcd
    geometry_gcd = gcd(tangent_squared, squared)
    tangent_reduced = tangent_squared // geometry_gcd
    squared_reduced = squared // geometry_gcd
    post_left = tangent_reduced * eta_i * eta_i
    post_right = squared_reduced * eta_t * eta_t
    check(
        (pre_left >= pre_right) == (post_left >= post_right),
        f"{case.name}: cancellation changed TIR classification",
    )
    return TirReceipt(
        pre_left >= pre_right,
        pre_left.bit_length(),
        pre_right.bit_length(),
        post_left.bit_length(),
        post_right.bit_length(),
    )


@dataclass(frozen=True)
class StagedEvent:
    tir: bool
    reflectance: FixedInterval
    transmittance: FixedInterval
    reflected: tuple[FixedInterval, FixedInterval, FixedInterval]
    transmitted: tuple[FixedInterval, FixedInterval, FixedInterval] | None


def staged_event(case: Case, metrics: Metrics) -> StagedEvent:
    bits = metrics.precision
    zero = FixedInterval.from_fraction(0, bits, metrics)
    one = FixedInterval.from_fraction(1, bits, metrics)
    two = FixedInterval.from_fraction(2, bits, metrics)
    squared = sum(component * component for component in case.delta)
    normal = case.delta[case.axis]
    tangent_squared = squared - normal * normal
    tir_receipt = exact_tir(case)

    sqrt_squared = FixedInterval.from_fraction(squared, bits, metrics).sqrt()
    incident = tuple(
        FixedInterval.from_fraction(component, bits, metrics) / sqrt_squared
        for component in case.delta
    )
    cos_i = FixedInterval.from_fraction(normal, bits, metrics) / sqrt_squared
    reflected = tuple(
        incident[index]
        - cos_i
        * (two if index == case.axis else zero)
        for index in range(3)
    )

    if tir_receipt.tir:
        metrics.tir += 1
        return StagedEvent(True, one, zero, reflected, None)

    metrics.transmitted += 1
    sin_t_exact = Fraction(
        tangent_squared * case.eta_i_raw * case.eta_i_raw,
        squared * case.eta_t_raw * case.eta_t_raw,
    )
    sin_t_squared = FixedInterval.from_fraction(sin_t_exact, bits, metrics).intersect(
        Fraction(0), Fraction(1)
    )
    cos_t = (one - sin_t_squared).intersect(Fraction(0), Fraction(1)).sqrt()
    q = FixedInterval.from_fraction(
        Fraction(case.eta_t_raw, case.eta_i_raw), bits, metrics
    )

    r_parallel = (q * cos_i - cos_t) / (q * cos_i + cos_t)
    r_perpendicular = (cos_i - q * cos_t) / (cos_i + q * cos_t)
    raw_reflectance = (r_parallel.square() + r_perpendicular.square()) / two
    reflectance = raw_reflectance.intersect(Fraction(0), Fraction(1))
    transmittance = one - reflectance

    tangent = tuple(
        incident[index]
        - cos_i
        * (one if index == case.axis else zero)
        for index in range(3)
    )
    transmitted = tuple(
        tangent[index] / q
        + cos_t
        * (one if index == case.axis else zero)
        for index in range(3)
    )

    check((reflectance + transmittance).contains_fraction(1), f"{case.name}: staged energy")
    check(
        sum_intervals([component.square() for component in reflected], zero).contains_fraction(1),
        f"{case.name}: staged reflected unit vector",
    )
    check(
        sum_intervals([component.square() for component in transmitted], zero).contains_fraction(1),
        f"{case.name}: staged transmitted unit vector",
    )
    tangent_result = sum_intervals(
        [transmitted[index].square() for index in range(3) if index != case.axis],
        zero,
    )
    check(tangent_result.contains_fraction(sin_t_exact), f"{case.name}: staged Snell")
    return StagedEvent(False, reflectance, transmittance, reflected, transmitted)


def load_reference():
    path = Path(__file__).with_name("prove-g1-c3-visible-radiance-interface-math.py")
    spec = importlib.util.spec_from_file_location("forge_interface_exact_reference", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load independent interface reference")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    check(module.ROOT_BITS >= 384, "independent reference precision")
    return module


def fixed_cases() -> list[Case]:
    cases = [
        Case("normal", (7, 0, 0), 0, Q48, 3 * Q48 // 2),
        Case("matched", (4, 3, 0), 0, 7 * Q48 // 5, 7 * Q48 // 5),
        Case("critical", (3, 4, 0), 0, 5 * Q48, 4 * Q48),
        Case("above-critical", (3, 4, 0), 0, 5 * Q48, 4 * Q48 - 1),
        Case("below-critical", (3, 4, 0), 0, 5 * Q48, 4 * Q48 + 1),
        Case("grazing-transmit", (1, 1 << 40, 3), 0, Q48, 16 * Q48),
        Case("reverse-normal", (9, 0, 0), 0, 3 * Q48 // 2, Q48),
        Case("dispersive-red", (13, 8, -5), 0, 3 * Q48 // 2, 4 * Q48 // 3),
        Case("dispersive-green", (13, 8, -5), 0, 3 * Q48 // 2, 7 * Q48 // 5),
        Case("dispersive-blue", (13, 8, -5), 0, 3 * Q48 // 2, 8 * Q48 // 5),
    ]
    for axis in range(3):
        delta = [0, 0, 0]
        delta[axis] = 3
        delta[(axis + 1) % 3] = 4
        cases.extend(
            [
                Case(f"critical-axis-{axis}", tuple(delta), axis, 5 * Q48, 4 * Q48),
                Case(
                    f"critical-axis-{axis}-below",
                    tuple(delta),
                    axis,
                    5 * Q48,
                    4 * Q48 + 1,
                ),
                Case(
                    f"critical-axis-{axis}-above",
                    tuple(delta),
                    axis,
                    5 * Q48,
                    4 * Q48 - 1,
                ),
            ]
        )
    hostile_tangent = (1 << 64) - 100
    cases.extend(
        [
            Case(
                "coprime-wide-tir",
                (1, hostile_tangent, 0),
                0,
                (1 << 52) - 1,
                (1 << 52) - 3,
            ),
            Case(
                "coprime-wide-transmit",
                (1, hostile_tangent, 0),
                0,
                (1 << 52) - 3,
                (1 << 52) - 1,
            ),
        ]
    )
    return cases


def generated_cases() -> list[Case]:
    rng = random.Random(SEED)
    cases: list[Case] = []
    for index in range(GENERATED_CASES):
        axis = index % 3
        delta = [rng.randint(-(1 << 20), 1 << 20) for _ in range(3)]
        delta[axis] = rng.randint(1, 1 << 20)
        cases.append(
            Case(
                f"generated-{index}",
                tuple(delta),
                axis,
                rng.randint(1 << 46, 1 << 52),
                rng.randint(1 << 46, 1 << 52),
            )
        )
    return cases


def compare_event(case: Case, staged: StagedEvent, reference) -> None:
    receipt = exact_tir(case)
    check(staged.tir == receipt.tir, f"{case.name}: staged/exact TIR mismatch")
    check(staged.tir == reference.total_internal_reflection, f"{case.name}: reference TIR mismatch")
    check(staged.reflectance.contains_reference(reference.reflectance), f"{case.name}: reflectance containment")
    check(staged.transmittance.contains_reference(reference.transmittance), f"{case.name}: transmittance containment")
    for index in range(3):
        check(
            staged.reflected[index].contains_reference(reference.reflected[index]),
            f"{case.name}: reflected component {index} containment",
        )
    if staged.tir:
        check(staged.transmitted is None and reference.transmitted is None, f"{case.name}: no TIR transmission")
    else:
        check(staged.transmitted is not None and reference.transmitted is not None, f"{case.name}: transmission present")
        for index in range(3):
            check(
                staged.transmitted[index].contains_reference(reference.transmitted[index]),
                f"{case.name}: transmitted component {index} containment",
            )


def main() -> None:
    reference_module = load_reference()
    cases = fixed_cases() + generated_cases()
    references = {}
    max_pre_product_bits = 0
    max_post_product_bits = 0
    critical_neighborhood_cases = 0
    for case in cases:
        receipt = exact_tir(case)
        max_pre_product_bits = max(
            max_pre_product_bits, receipt.pre_left_bits, receipt.pre_right_bits
        )
        max_post_product_bits = max(
            max_post_product_bits, receipt.post_left_bits, receipt.post_right_bits
        )
        if "critical" in case.name:
            critical_neighborhood_cases += 1
        references[case.name] = reference_module.band_event(
            case.delta,
            case.axis,
            Fraction(case.eta_i_raw, Q48),
            Fraction(case.eta_t_raw, Q48),
        )

    precision_results = []
    supported_precisions = []
    for precision in PRECISIONS:
        metrics = Metrics(precision)
        for case in cases:
            staged = staged_event(case, metrics)
            compare_event(case, staged, references[case.name])
            case_power_width = max(
                staged.reflectance.projected_width(POWER_BITS),
                staged.transmittance.projected_width(POWER_BITS),
            )
            if case_power_width > metrics.max_power_width:
                metrics.max_power_width = case_power_width
                metrics.max_power_case = case.name
            for component in staged.reflected:
                width = component.projected_width(DIRECTION_BITS)
                if width > metrics.max_direction_width:
                    metrics.max_direction_width = width
                    metrics.max_direction_case = case.name + ":reflected"
            if staged.transmitted is not None:
                for component in staged.transmitted:
                    width = component.projected_width(DIRECTION_BITS)
                    if width > metrics.max_direction_width:
                        metrics.max_direction_width = width
                        metrics.max_direction_case = case.name + ":transmitted"

        derived_live_ceiling = max(2 * precision + 132, precision + 232)
        check(
            metrics.max_live_integer_bits <= derived_live_ceiling,
            f"{precision}: live integer {metrics.max_live_integer_bits} exceeded derived ceiling {derived_live_ceiling}",
        )
        supported = metrics.max_power_width <= 1 and metrics.max_direction_width <= 1
        if supported:
            supported_precisions.append(precision)
        precision_results.append(
            {
                "fractional_bits": precision,
                "status": "supported_in_portfolio" if supported else "rejected_by_width",
                "max_power_projection_width_units": metrics.max_power_width,
                "max_power_case": metrics.max_power_case,
                "max_direction_projection_width_units": metrics.max_direction_width,
                "max_direction_case": metrics.max_direction_case,
                "max_live_integer_bits": metrics.max_live_integer_bits,
                "derived_live_ceiling_bits": derived_live_ceiling,
                "max_stored_endpoint_bits": metrics.max_stored_endpoint_bits,
                "theorem_intersections": metrics.theorem_intersections,
                "transmitted_cases": metrics.transmitted,
                "total_internal_reflection_cases": metrics.tir,
            }
        )

    check(max_post_product_bits <= 232, "post-cancellation TIR product exceeded design ceiling")
    check(max_post_product_bits > 128, "hostile portfolio failed to exercise wider-than-u128 TIR")

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
                "status": "pass_with_required_precision_counterexample",
                "reference": "existing 384-bit arbitrary-precision interface oracle",
                "seed": SEED,
                "fixed_and_hostile_cases": len(fixed_cases()),
                "generated_cases": GENERATED_CASES,
                "critical_neighborhood_cases": critical_neighborhood_cases,
                "total_cases": len(cases),
                "checks": checks,
                "max_pre_cancellation_tir_product_bits": max_pre_product_bits,
                "max_post_cancellation_tir_product_bits": max_post_product_bits,
                "required_precisions": REQUIRED_PRECISIONS,
                "sensitivity_precisions": SENSITIVITY_PRECISIONS,
                "supported_precisions_in_portfolio": supported_precisions,
                "precision_results": precision_results,
                "exploratory_ior_domain": "exact Q16.48 values in [1/4,16]; computational stress range only",
                "limitations": limitations,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
