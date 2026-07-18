#!/usr/bin/env python3
"""Disposable exact oracle for the bounded local dielectric interface design."""

from __future__ import annotations

import json
import os
import random
from dataclasses import dataclass
from fractions import Fraction
from math import isqrt


ROOT_BITS = int(os.environ.get("FORGE_INTERFACE_ROOT_BITS", "384"))
POWER_BITS = 48
DIRECTION_BITS = 62
GENERATED_CASES = 1024
SEED = 0xF0A6E17
Q48 = 1 << 48
MAX_SQUARED_DELTA = (1 << 128) - 1

max_numerator_bits = 0
max_denominator_bits = 0
max_power_width = 0
max_direction_width = 0
max_raw_reflectance_overshoot = Fraction(0)
checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        raise AssertionError(message)


def track(value: Fraction) -> Fraction:
    global max_numerator_bits, max_denominator_bits
    max_numerator_bits = max(max_numerator_bits, abs(value.numerator).bit_length())
    max_denominator_bits = max(max_denominator_bits, value.denominator.bit_length())
    return value


@dataclass(frozen=True)
class Interval:
    lower: Fraction
    upper: Fraction

    def __post_init__(self) -> None:
        track(self.lower)
        track(self.upper)
        if self.lower > self.upper:
            raise ValueError("reversed interval")

    @staticmethod
    def exact(value: int | Fraction) -> "Interval":
        f = track(Fraction(value))
        return Interval(f, f)

    def __add__(self, other: "Interval") -> "Interval":
        return Interval(self.lower + other.lower, self.upper + other.upper)

    def __sub__(self, other: "Interval") -> "Interval":
        return Interval(self.lower - other.upper, self.upper - other.lower)

    def __mul__(self, other: "Interval") -> "Interval":
        products = (
            self.lower * other.lower,
            self.lower * other.upper,
            self.upper * other.lower,
            self.upper * other.upper,
        )
        return Interval(min(products), max(products))

    def reciprocal(self) -> "Interval":
        if self.lower <= 0 <= self.upper:
            raise ZeroDivisionError("interval contains zero")
        return Interval(Fraction(1, 1) / self.upper, Fraction(1, 1) / self.lower)

    def __truediv__(self, other: "Interval") -> "Interval":
        return self * other.reciprocal()

    def contains(self, value: int | Fraction) -> bool:
        f = Fraction(value)
        return self.lower <= f <= self.upper


def square(value: Interval) -> Interval:
    if value.lower <= 0 <= value.upper:
        return Interval(Fraction(0), max(value.lower * value.lower, value.upper * value.upper))
    endpoints = (value.lower * value.lower, value.upper * value.upper)
    return Interval(min(endpoints), max(endpoints))


def sqrt_interval(value: Fraction, bits: int = ROOT_BITS) -> Interval:
    if value < 0:
        raise ValueError("negative square root")
    scaled_numerator = value.numerator << (2 * bits)
    scaled_floor = scaled_numerator // value.denominator
    lower_raw = isqrt(scaled_floor)
    while (lower_raw + 1) * (lower_raw + 1) * value.denominator <= scaled_numerator:
        lower_raw += 1
    exact = lower_raw * lower_raw * value.denominator == scaled_numerator
    upper_raw = lower_raw if exact else lower_raw + 1
    scale = 1 << bits
    result = Interval(Fraction(lower_raw, scale), Fraction(upper_raw, scale))
    check(square(result).lower <= value <= square(result).upper, "sqrt enclosure")
    return result


def floor_fraction(value: Fraction) -> int:
    return value.numerator // value.denominator


def ceil_fraction(value: Fraction) -> int:
    return -((-value.numerator) // value.denominator)


def projected_width(value: Interval, bits: int) -> int:
    scale = 1 << bits
    return ceil_fraction(value.upper * scale) - floor_fraction(value.lower * scale)


def intersect(value: Interval, lower: Fraction, upper: Fraction) -> Interval:
    result = Interval(max(value.lower, lower), min(value.upper, upper))
    check(result.lower <= result.upper, "theorem intersection is nonempty")
    return result


def classify_geometry(
    before: tuple[int, int, int],
    after: tuple[int, int, int],
    before_medium: str,
    after_medium: str,
    *,
    positive_on_both_sides: bool = True,
    lane_ambiguous: bool = False,
) -> str:
    if lane_ambiguous:
        return "ambiguous_boundary_lane"
    if not positive_on_both_sides or before_medium == after_medium:
        return "no_interface_event"
    differences = [after[i] - before[i] for i in range(3)]
    changed = [d for d in differences if d != 0]
    if len(changed) != 1 or abs(changed[0]) != 1:
        return "ambiguous_interface_geometry"
    return "unique_face_event"


@dataclass(frozen=True)
class BandEvent:
    total_internal_reflection: bool
    reflectance: Interval
    transmittance: Interval
    reflected: tuple[Interval, Interval, Interval]
    transmitted: tuple[Interval, Interval, Interval] | None


def band_event(delta: tuple[int, int, int], axis: int, eta_i: Fraction, eta_t: Fraction) -> BandEvent:
    global max_power_width, max_direction_width, max_raw_reflectance_overshoot
    check(eta_i > 0 and eta_t > 0, "positive refractive indices")
    squared = sum(component * component for component in delta)
    check(0 < squared <= MAX_SQUARED_DELTA, "admitted squared path delta")
    normal_component = delta[axis]
    check(normal_component > 0, "oriented normal faces target medium")

    squared_f = track(Fraction(squared))
    cos_i_squared = track(Fraction(normal_component * normal_component, squared))
    sin_i_squared = track(Fraction(1) - cos_i_squared)
    relative = track(eta_t / eta_i)
    sin_t_squared = track(sin_i_squared / (relative * relative))

    sqrt_squared = sqrt_interval(squared_f)
    incident = tuple(Interval.exact(component) / sqrt_squared for component in delta)
    cos_i = Interval.exact(normal_component) / sqrt_squared
    reflected = tuple(
        incident[i] - (cos_i * Interval.exact(2 if i == axis else 0)) for i in range(3)
    )

    if sin_t_squared >= 1:
        reflectance = Interval.exact(1)
        transmittance = Interval.exact(0)
        max_power_width = max(max_power_width, projected_width(reflectance, POWER_BITS))
        return BandEvent(True, reflectance, transmittance, reflected, None)

    cos_t = sqrt_interval(Fraction(1) - sin_t_squared)
    q = Interval.exact(relative)
    r_parallel = (q * cos_i - cos_t) / (q * cos_i + cos_t)
    r_perpendicular = (cos_i - q * cos_t) / (cos_i + q * cos_t)
    raw_reflectance = (square(r_parallel) + square(r_perpendicular)) / Interval.exact(2)
    if raw_reflectance.upper > 1:
        max_raw_reflectance_overshoot = max(
            max_raw_reflectance_overshoot, raw_reflectance.upper - 1
        )
    reflectance = intersect(raw_reflectance, Fraction(0), Fraction(1))
    transmittance = Interval(Fraction(1) - reflectance.upper, Fraction(1) - reflectance.lower)

    tangent = tuple(
        incident[i] - (cos_i * Interval.exact(1 if i == axis else 0)) for i in range(3)
    )
    transmitted = tuple(
        tangent[i] / q + (cos_t * Interval.exact(1 if i == axis else 0))
        for i in range(3)
    )

    check((reflectance + transmittance).contains(1), "energy enclosure")
    check(sum_intervals(square(component) for component in reflected).contains(1), "reflected unit vector")
    check(sum_intervals(square(component) for component in transmitted).contains(1), "transmitted unit vector")
    transmitted_tangent_squared = sum_intervals(
        square(transmitted[i]) for i in range(3) if i != axis
    )
    check(transmitted_tangent_squared.contains(sin_t_squared), "Snell tangent invariant")

    max_power_width = max(
        max_power_width,
        projected_width(reflectance, POWER_BITS),
        projected_width(transmittance, POWER_BITS),
    )
    for component in reflected + transmitted:
        max_direction_width = max(max_direction_width, projected_width(component, DIRECTION_BITS))
    return BandEvent(False, reflectance, transmittance, reflected, transmitted)


def sum_intervals(values) -> Interval:
    total = Interval.exact(0)
    for value in values:
        total = total + value
    return total


def fixed_cases() -> None:
    check(classify_geometry((0, 0, 0), (1, 0, 0), "air", "glass") == "unique_face_event", "face")
    check(classify_geometry((0, 0, 0), (1, 1, 0), "air", "glass") == "ambiguous_interface_geometry", "edge")
    check(classify_geometry((0, 0, 0), (1, 1, 1), "air", "glass") == "ambiguous_interface_geometry", "vertex")
    check(classify_geometry((0, 0, 0), (1, 0, 0), "air", "air") == "no_interface_event", "subdivision")
    check(classify_geometry((0, 0, 0), (1, 0, 0), "air", "glass", positive_on_both_sides=False) == "no_interface_event", "point contact")
    check(classify_geometry((0, 0, 0), (1, 0, 0), "air", "glass", lane_ambiguous=True) == "ambiguous_boundary_lane", "lane")

    normal = band_event((7, 0, 0), 0, Fraction(1), Fraction(3, 2))
    check(not normal.total_internal_reflection, "normal incidence transmits")
    check(normal.reflectance.contains(Fraction(1, 25)), "normal analytic reflectance")
    check(normal.transmitted is not None and normal.transmitted[0].contains(1), "normal direction unchanged")

    matched = band_event((4, 3, 0), 0, Fraction(7, 5), Fraction(7, 5))
    check(matched.reflectance.contains(0), "index matched reflection")
    check(matched.transmittance.contains(1), "index matched transmission")

    critical = band_event((3, 4, 0), 0, Fraction(5, 4), Fraction(1))
    check(critical.total_internal_reflection, "critical equality is TIR")
    above = band_event((2, 4, 0), 0, Fraction(5, 4), Fraction(1))
    check(above.total_internal_reflection, "above critical is TIR")
    below = band_event((4, 3, 0), 0, Fraction(5, 4), Fraction(1))
    check(not below.total_internal_reflection, "below critical transmits")

    reverse_normal = band_event((9, 0, 0), 0, Fraction(3, 2), Fraction(1))
    check(intervals_overlap(normal.reflectance, reverse_normal.reflectance), "normal reciprocity")


def intervals_overlap(a: Interval, b: Interval) -> bool:
    return max(a.lower, b.lower) <= min(a.upper, b.upper)


def generated_cases() -> tuple[int, int]:
    rng = random.Random(SEED)
    transmitted = 0
    tir = 0
    for index in range(GENERATED_CASES):
        axis = index % 3
        delta = [rng.randint(-(1 << 20), 1 << 20) for _ in range(3)]
        delta[axis] = rng.randint(1, 1 << 20)
        if delta == [0, 0, 0]:
            delta[axis] = 1
        eta_i = Fraction(rng.randint(1 << 46, 1 << 52), Q48)
        eta_t = Fraction(rng.randint(1 << 46, 1 << 52), Q48)
        event = band_event(tuple(delta), axis, eta_i, eta_t)
        if event.total_internal_reflection:
            tir += 1
        else:
            transmitted += 1
            check(event.transmitted is not None, "transmitted direction present")
        check(event.reflectance.lower >= 0 and event.reflectance.upper <= 1, "power theorem bounds")
        check(event.transmittance.lower >= 0 and event.transmittance.upper <= 1, "transmission theorem bounds")
    return transmitted, tir


def main() -> None:
    fixed_cases()
    transmitted, tir = generated_cases()
    limitations = " ".join(
        [
            "no schema or implementation",
            "no real coefficient catalogue",
            "no downstream refractive path",
            "no perception or rendering",
            "no passage or navigation",
            "no biome planet terrain runtime approval promotion",
        ]
    )
    for forbidden in (
        "schema", "coefficient", "path", "perception", "rendering", "passage",
        "biome", "planet", "terrain", "runtime", "approval", "promotion",
    ):
        check(forbidden in limitations, f"authority limitation {forbidden}")

    print(
        json.dumps(
            {
                "status": "pass",
                "seed": SEED,
                "generated_cases": GENERATED_CASES,
                "transmitted_cases": transmitted,
                "total_internal_reflection_cases": tir,
                "checks": checks,
                "root_interval_bits": ROOT_BITS,
                "power_projection_bits": POWER_BITS,
                "direction_projection_bits": DIRECTION_BITS,
                "max_power_projection_width_units": max_power_width,
                "max_direction_projection_width_units": max_direction_width,
                "max_intermediate_numerator_bits": max_numerator_bits,
                "max_intermediate_denominator_bits": max_denominator_bits,
                "max_raw_reflectance_overshoot_numerator": max_raw_reflectance_overshoot.numerator,
                "max_raw_reflectance_overshoot_denominator": max_raw_reflectance_overshoot.denominator,
                "exploratory_ior_domain": "exact Q16.48 values in [1/4,16]; computational stress range only",
                "limitations": limitations,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
