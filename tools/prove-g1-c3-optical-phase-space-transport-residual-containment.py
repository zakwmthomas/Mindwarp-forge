#!/usr/bin/env python3
"""Independent exact oracle for the integrated residual-containment repair."""

from __future__ import annotations

import hashlib
import itertools
import json
from fractions import Fraction as F


D = (1 << 64) - 1
S = 1 << 32
SMALL = D // 16384


def form(center: int, coefficient: int, rem: tuple[int, int]) -> dict[str, object]:
    return {
        "center": center,
        "coeff": (coefficient, coefficient - 1, coefficient - 2, coefficient - 3),
        "rem": rem,
    }


P = (
    form(D // 5, SMALL, (-7, 11)),
    form(D // 5, SMALL, (-5, 13)),
    form(D // 5, SMALL, (-3, 17)),
)
V = (
    form(9 * D // 10, SMALL // 8, (-2, 3)),
    form(3 * D // 5, SMALL // 8, (-3, 5)),
    form(D // 3, SMALL // 8, (-5, 7)),
)


def extent(value: dict[str, object]) -> tuple[int, int]:
    radius = sum(abs(item) for item in value["coeff"])
    return value["center"] - radius + value["rem"][0], value["center"] + radius + value["rem"][1]


def affine(value: dict[str, object], remainder_variable: int) -> list[int]:
    result = [0] * 9
    result[0] = value["center"]
    result[1:5] = value["coeff"]
    result[5 + remainder_variable] = 1
    return result


def multiply(left: list[int], right: list[int]) -> dict[tuple[int, int], int]:
    result: dict[tuple[int, int], int] = {}
    for i, a in enumerate(left):
        for j, b in enumerate(right):
            degree = tuple(sorted(index - 1 for index in (i, j) if index))
            result[degree] = result.get(degree, 0) + a * b
    return result


def scale(poly: dict[tuple[int, int], int], factor: int) -> dict[tuple[int, int], int]:
    return {term: value * factor for term, value in poly.items()}


def combine(
    left: dict[tuple[int, int], int],
    right: dict[tuple[int, int], int],
    sign: int,
) -> dict[tuple[int, int], int]:
    result = dict(left)
    for term, value in right.items():
        result[term] = result.get(term, 0) + sign * value
    return result


def term_bounds(coefficient: int, variables: tuple[int, ...], ranges: list[tuple[int, int]]) -> tuple[int, int]:
    values = [coefficient]
    for variable in variables:
        values = [value * endpoint for value in values for endpoint in ranges[variable]]
    return min(values), max(values)


def polynomial_bounds(poly: dict[tuple[int, int], int], ranges: list[tuple[int, int]]) -> tuple[int, int]:
    lower = 0
    upper = 0
    for variables, coefficient in poly.items():
        term_lower, term_upper = term_bounds(coefficient, variables, ranges)
        lower += term_lower
        upper += term_upper
    return lower, upper


def interval_product(left: tuple[F, F], right: tuple[F, F]) -> tuple[F, F]:
    values = [a * b for a in left for b in right]
    return min(values), max(values)


def direct_tangential(axis: int, index: int, height_raw: int) -> tuple[tuple[F, F], tuple[F, ...], tuple[F, F], tuple[F, F]]:
    p_axis = P[axis]
    v_axis = V[axis]
    position = P[index]
    direction = V[index]
    a0 = height_raw * D - p_axis["center"] * S
    b = v_axis["center"]
    center_n = position["center"] * S * b + a0 * direction["center"]
    center = F(center_n, D * S * b)
    coefficient_numerators = []
    for symbol in range(4):
        dt = -p_axis["coeff"][symbol] * b * S - a0 * v_axis["coeff"][symbol]
        coefficient_numerators.append(
            position["coeff"][symbol] * S * b * b
            + dt * direction["center"]
            + a0 * direction["coeff"][symbol] * b
        )
    coefficients = tuple(F(value, D * S * b * b) for value in coefficient_numerators)

    p_axis_affine = affine(p_axis, 0)
    v_axis_affine = affine(v_axis, 1)
    position_affine = affine(position, 2)
    direction_affine = affine(direction, 3)
    a_affine = [0] * 9
    a_affine[0] = a0
    a_affine[1:5] = [-value * S for value in p_axis["coeff"]]
    a_affine[5] = -S
    exact_numerator = combine(
        scale(multiply(position_affine, v_axis_affine), S),
        multiply(a_affine, direction_affine),
        1,
    )
    exact_scaled = scale(exact_numerator, b * b)
    linearized = [0] * 9
    linearized[0] = center_n * b
    linearized[1:5] = coefficient_numerators
    residual_poly = combine(exact_scaled, multiply(linearized, v_axis_affine), -1)
    ranges = [(-1, 1)] * 4 + [p_axis["rem"], v_axis["rem"], position["rem"], direction["rem"]]
    residual_n = polynomial_bounds(residual_poly, ranges)
    velocity = extent(v_axis)
    candidates = [F(numerator, D * S * endpoint * b * b) for numerator in residual_n for endpoint in velocity]
    remainder = min(candidates), max(candidates)
    radius = sum(abs(value) for value in coefficients)
    direct = center - radius + remainder[0], center + radius + remainder[1]

    position_extent = tuple(F(value, D) for value in extent(position))
    axis_position_extent = tuple(F(value, D) for value in extent(p_axis))
    axis_velocity_extent = tuple(F(value, D) for value in velocity)
    direction_extent = tuple(F(value, D) for value in extent(direction))
    time = (
        (F(height_raw, S) - axis_position_extent[1]) / axis_velocity_extent[1],
        (F(height_raw, S) - axis_position_extent[0]) / axis_velocity_extent[0],
    )
    propagated = interval_product(time, direction_extent)
    physical = position_extent[0] + propagated[0], position_extent[1] + propagated[1]
    return direct, coefficients, remainder, physical


checked_points = 0
containment_axes = 0
for index in (1, 2):
    direct, coefficients, remainder, physical = direct_tangential(0, index, S)
    if not physical[0] <= direct[0] <= direct[1] <= physical[1]:
        raise AssertionError("tight residual escaped physical face enclosure")
    containment_axes += 1
    for symbols in itertools.product((F(-1), F(-1, 2), F(0), F(1, 2), F(1)), repeat=4):
        for remainders in itertools.product(*[(F(a), F(b)) for a, b in (P[0]["rem"], V[0]["rem"], P[index]["rem"], V[index]["rem"])]):
            point = [F(item["center"] + sum(c * u for c, u in zip(item["coeff"], symbols)) + remainders[2 if axis == index else 0], D) for axis, item in enumerate(P)]
            velocity = [F(item["center"] + sum(c * u for c, u in zip(item["coeff"], symbols)) + remainders[3 if axis == index else 1], D) for axis, item in enumerate(V)]
            exact = point[index] + (F(1) - point[0]) * velocity[index] / velocity[0]
            affine_value = F(P[index]["center"] * S * V[0]["center"] + (S * D - P[0]["center"] * S) * V[index]["center"], D * S * V[0]["center"])
            affine_value += sum(coefficient * symbol for coefficient, symbol in zip(coefficients, symbols))
            residual_value = exact - affine_value
            if not remainder[0] <= residual_value <= remainder[1]:
                raise AssertionError("quadratic residual falsifier escaped")
            checked_points += 1

receipt = {
    "oracle": "optical_phase_space_transport_residual_containment_v1",
    "input_bits": 64,
    "shared_symbol_grid": 5,
    "declared_remainder_corners": 16,
    "residual_falsifiers": checked_points,
    "physical_face_containment_axes": containment_axes,
    "residual_model": "exact_quadratic_termwise_interval",
    "residual_endpoint_denominator": "D*S*V_endpoint*b^2",
    "storage_shield_bits": 512,
    "live_shield_bits": 490,
    "authority_effect": "none_evidence_only",
}
receipt["receipt_sha256"] = hashlib.sha256(
    json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
).hexdigest()
print(json.dumps(receipt, sort_keys=True, indent=2))
