#!/usr/bin/env python3
"""Disposable width/utility spike for correlated rational plane transport."""

from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass
from fractions import Fraction as F


SHIELD = 512
CAPS = (16, 24, 32, 48, 64, 80, 96, 112, 128, 160, 192, 256, 368)


def bits(value: int) -> int:
    return max(1, abs(value).bit_length())


class Tracker:
    def __init__(self) -> None:
        self.maximum_raw_bits = 0
        self.maximum_stored_bits = 0
        self.operations = 0

    def see(self, *values: int) -> None:
        self.maximum_raw_bits = max(self.maximum_raw_bits, *(bits(value) for value in values))

    def stored(self, value: F) -> F:
        self.maximum_stored_bits = max(self.maximum_stored_bits, bits(value.numerator), bits(value.denominator))
        return value

    def add(self, left: F, right: F) -> F:
        a = left.numerator * right.denominator
        b = right.numerator * left.denominator
        d = left.denominator * right.denominator
        self.see(a, b, a + b, d)
        self.operations += 3
        return self.stored(F(a + b, d))

    def sub(self, left: F, right: F) -> F:
        return self.add(left, -right)

    def mul(self, left: F, right: F) -> F:
        n = left.numerator * right.numerator
        d = left.denominator * right.denominator
        self.see(n, d)
        self.operations += 2
        return self.stored(F(n, d))

    def div(self, left: F, right: F) -> F:
        if right == 0:
            raise ZeroDivisionError
        n = left.numerator * right.denominator
        d = left.denominator * right.numerator
        self.see(n, d)
        self.operations += 2
        return self.stored(F(n, d))


@dataclass(frozen=True)
class Form:
    center: F
    coeff: tuple[F, F, F, F]
    rem_lo: F = F(0)
    rem_hi: F = F(0)


@dataclass(frozen=True)
class State:
    position: tuple[Form, Form, Form]
    direction: tuple[Form, Form, Form]


def sum_values(t: Tracker, values: list[F]) -> F:
    result = F(0)
    for value in values:
        result = t.add(result, value)
    return result


def extent(t: Tracker, form: Form) -> tuple[F, F]:
    radius = sum_values(t, [abs(value) for value in form.coeff])
    return t.add(t.sub(form.center, radius), form.rem_lo), t.add(t.add(form.center, radius), form.rem_hi)


def iadd(t: Tracker, a: tuple[F, F], b: tuple[F, F]) -> tuple[F, F]:
    return t.add(a[0], b[0]), t.add(a[1], b[1])


def isub(t: Tracker, a: tuple[F, F], b: tuple[F, F]) -> tuple[F, F]:
    return t.sub(a[0], b[1]), t.sub(a[1], b[0])


def imul(t: Tracker, a: tuple[F, F], b: tuple[F, F]) -> tuple[F, F]:
    values = [t.mul(x, y) for x in a for y in b]
    return min(values), max(values)


def idiv(t: Tracker, a: tuple[F, F], b: tuple[F, F]) -> tuple[F, F]:
    if b[0] <= 0 <= b[1]:
        raise ValueError("parallel")
    reciprocal = (t.div(F(1), b[1]), t.div(F(1), b[0]))
    return imul(t, a, (min(reciprocal), max(reciprocal)))


def plane(t: Tracker, state: State, axis: int, height: F) -> State:
    p_axis = state.position[axis]
    v_axis = state.direction[axis]
    p_range = extent(t, p_axis)
    v_range = extent(t, v_axis)
    n_range = (t.sub(height, p_range[1]), t.sub(height, p_range[0]))
    time_range = idiv(t, n_range, v_range)
    if time_range[0] <= 0:
        raise ValueError("nonforward")
    n0 = t.sub(height, p_axis.center)
    time0 = t.div(n0, v_axis.center)
    time_coeff = []
    denominator_square = t.mul(v_axis.center, v_axis.center)
    for p_coefficient, v_coefficient in zip(p_axis.coeff, v_axis.coeff):
        first = t.mul(-p_coefficient, v_axis.center)
        second = t.mul(n0, v_coefficient)
        time_coeff.append(t.div(t.sub(first, second), denominator_square))
    outputs = []
    for index, (position, direction) in enumerate(zip(state.position, state.direction)):
        if index == axis:
            outputs.append(Form(height, (F(0), F(0), F(0), F(0))))
            continue
        center = t.add(position.center, t.mul(time0, direction.center))
        coefficients = []
        for k in range(4):
            coefficients.append(
                sum_values(t, [position.coeff[k], t.mul(time_coeff[k], direction.center), t.mul(time0, direction.coeff[k])])
            )
        affine = Form(center, tuple(coefficients))
        exact_range = iadd(t, extent(t, position), imul(t, time_range, extent(t, direction)))
        residual = isub(t, exact_range, extent(t, affine))
        outputs.append(Form(center, tuple(coefficients), residual[0], residual[1]))
    return State(tuple(outputs), state.direction)


@dataclass(frozen=True)
class Bound:
    numerator: int
    denominator: int


class BoundTracker:
    def __init__(self) -> None:
        self.maximum_raw_bits = 0

    def add(self, a: Bound, b: Bound) -> Bound:
        left = a.numerator + b.denominator
        right = b.numerator + a.denominator
        denominator = a.denominator + b.denominator
        numerator = max(left, right) + 1
        self.maximum_raw_bits = max(self.maximum_raw_bits, left, right, numerator, denominator)
        return Bound(numerator, denominator)

    sub = add

    def mul(self, a: Bound, b: Bound) -> Bound:
        value = Bound(a.numerator + b.numerator, a.denominator + b.denominator)
        self.maximum_raw_bits = max(self.maximum_raw_bits, value.numerator, value.denominator)
        return value

    def div(self, a: Bound, b: Bound) -> Bound:
        value = Bound(a.numerator + b.denominator, a.denominator + b.numerator)
        self.maximum_raw_bits = max(self.maximum_raw_bits, value.numerator, value.denominator)
        return value


@dataclass(frozen=True)
class BForm:
    center: Bound
    coeff: tuple[Bound, Bound, Bound, Bound]
    rem_lo: Bound
    rem_hi: Bound


@dataclass(frozen=True)
class BState:
    position: tuple[BForm, BForm, BForm]
    direction: tuple[BForm, BForm, BForm]


def bsum(t: BoundTracker, values: list[Bound]) -> Bound:
    result = Bound(1, 1)
    for value in values:
        result = t.add(result, value)
    return result


def bextent(t: BoundTracker, form: BForm) -> Bound:
    radius = bsum(t, list(form.coeff))
    lower = t.add(t.add(form.center, radius), form.rem_lo)
    upper = t.add(t.add(form.center, radius), form.rem_hi)
    return Bound(max(lower.numerator, upper.numerator), max(lower.denominator, upper.denominator))


def bplane(t: BoundTracker, state: BState, axis: int) -> BState:
    h = Bound(64, 1)
    p_axis, v_axis = state.position[axis], state.direction[axis]
    p_range, v_range = bextent(t, p_axis), bextent(t, v_axis)
    n_range = t.sub(h, p_range)
    time_range = t.div(n_range, v_range)
    n0 = t.sub(h, p_axis.center)
    time0 = t.div(n0, v_axis.center)
    square = t.mul(v_axis.center, v_axis.center)
    time_coeff = tuple(t.div(t.add(t.mul(p, v_axis.center), t.mul(n0, v)), square) for p, v in zip(p_axis.coeff, v_axis.coeff))
    outputs = []
    for index, (position, direction) in enumerate(zip(state.position, state.direction)):
        if index == axis:
            zero = Bound(1, 1)
            outputs.append(BForm(h, (zero, zero, zero, zero), zero, zero))
            continue
        center = t.add(position.center, t.mul(time0, direction.center))
        coefficients = tuple(
            bsum(t, [position.coeff[k], t.mul(time_coeff[k], direction.center), t.mul(time0, direction.coeff[k])])
            for k in range(4)
        )
        affine = BForm(center, coefficients, Bound(1, 1), Bound(1, 1))
        exact = t.add(bextent(t, position), t.mul(time_range, bextent(t, direction)))
        residual = t.add(exact, bextent(t, affine))
        outputs.append(BForm(center, coefficients, residual, residual))
    return BState(tuple(outputs), state.direction)


def symbolic(cap: int, steps: int) -> tuple[int, int]:
    tracker = BoundTracker()
    scalar = Bound(cap, cap)
    form = BForm(scalar, (scalar, scalar, scalar, scalar), scalar, scalar)
    state = BState((form, form, form), (form, form, form))
    for axis in range(steps):
        state = bplane(tracker, state, axis)
    stored = max(
        max(value.numerator, value.denominator)
        for form in (*state.position, *state.direction)
        for value in (form.center, *form.coeff, form.rem_lo, form.rem_hi)
    )
    return tracker.maximum_raw_bits, stored


def constructed(cap: int, steps: int) -> dict[str, object]:
    base = 1 << cap
    denominators = [base - value for value in (1, 3, 5, 7, 9, 11)]
    tiny = [F(1, value) for value in denominators]
    near_one = [F(value - 2, value) for value in denominators]
    position = tuple(Form(F(axis), tuple(tiny[(axis + k) % 6] for k in range(4))) for axis in range(3))
    direction = tuple(Form(near_one[axis + 3], tuple(tiny[(axis + k + 2) % 6] for k in range(4))) for axis in range(3))
    state = State(position, direction)
    tracker = Tracker()
    completed = 0
    disposition = "completed"
    try:
        for axis in range(steps):
            state = plane(tracker, state, axis, F(4 + axis * 4))
            completed += 1
            if tracker.maximum_raw_bits > SHIELD:
                disposition = "raw_width_exceeded"
                break
    except ValueError as error:
        disposition = str(error)
    return {
        "requested_steps": steps,
        "completed_steps": completed,
        "maximum_raw_bits": tracker.maximum_raw_bits,
        "maximum_stored_bits": tracker.maximum_stored_bits,
        "operations": tracker.operations,
        "disposition": disposition,
    }


symbolic_matrix: dict[str, dict[str, object]] = {}
constructed_matrix: dict[str, dict[str, object]] = {}
for cap in CAPS:
    symbolic_matrix[str(cap)] = {}
    constructed_matrix[str(cap)] = {}
    for step_count in (1, 2, 3):
        raw, stored = symbolic(cap, step_count)
        symbolic_matrix[str(cap)][str(step_count)] = {
            "guaranteed_raw_bound_bits": raw,
            "guaranteed_output_bound_bits": stored,
            "within_512": raw <= SHIELD and stored <= SHIELD,
        }
        constructed_matrix[str(cap)][str(step_count)] = constructed(cap, step_count)

largest_guaranteed = {}
for step_count in (1, 2, 3):
    accepted = [cap for cap in range(1, 369) if max(symbolic(cap, step_count)) <= SHIELD]
    largest_guaranteed[str(step_count)] = max(accepted) if accepted else None

two_step_constructed_caps = [
    cap
    for cap in CAPS
    if constructed_matrix[str(cap)]["2"]["completed_steps"] == 2
    and constructed_matrix[str(cap)]["2"]["maximum_raw_bits"] <= SHIELD
]

receipt = {
    "spike": "optical_phase_space_transport_width_v0",
    "storage_shield_bits": SHIELD,
    "candidate_caps": list(CAPS),
    "symbolic_matrix": symbolic_matrix,
    "constructed_matrix": constructed_matrix,
    "largest_guaranteed_input_cap_by_steps": largest_guaranteed,
    "constructed_two_step_caps_within_512": two_step_constructed_caps,
    "guaranteed_two_step_cap_at_least_16": (largest_guaranteed["2"] or 0) >= 16,
    "favourable_reduction_is_authority": False,
    "production_source_authorized": False,
}
receipt["receipt_sha256"] = hashlib.sha256(json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
print(json.dumps(receipt, sort_keys=True, indent=2))
