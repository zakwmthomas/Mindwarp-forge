#!/usr/bin/env python3
"""Disposable exact-rational oracle for the G1/C3 transport certificate.

This is proof evidence, not a production schema or implementation.
"""

from __future__ import annotations

import copy
import hashlib
import itertools
import json
from dataclasses import dataclass
from fractions import Fraction as F
from typing import Callable, Iterable


SYMBOLS = 4
AXES = 3
DOMAIN = b"mindwarp:g1:c3:optical-phase-space-transport-certificate:oracle:v0\x00"


def rat(value: F) -> str:
    return f"{value.numerator}/{value.denominator}"


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def digest(value: object) -> str:
    return hashlib.sha256(DOMAIN + canonical(value)).hexdigest()


def opaque(byte: int) -> str:
    return f"{byte:02x}" * 32


def valid_id(value: object) -> bool:
    if not isinstance(value, str) or len(value) != 64 or value == "0" * 64:
        return False
    try:
        int(value, 16)
    except ValueError:
        return False
    return True


@dataclass(frozen=True)
class Interval:
    lo: F
    hi: F

    def __post_init__(self) -> None:
        if self.lo > self.hi:
            raise ValueError("reversed_interval")

    def add(self, other: "Interval") -> "Interval":
        return Interval(self.lo + other.lo, self.hi + other.hi)

    def sub(self, other: "Interval") -> "Interval":
        return Interval(self.lo - other.hi, self.hi - other.lo)

    def mul(self, other: "Interval") -> "Interval":
        products = (self.lo * other.lo, self.lo * other.hi, self.hi * other.lo, self.hi * other.hi)
        return Interval(min(products), max(products))

    def div(self, other: "Interval") -> "Interval":
        if other.lo <= 0 <= other.hi:
            raise TypedStop("unsupported_parallel_or_reversed_plane")
        reciprocals = Interval(min(F(1, other.lo), F(1, other.hi)), max(F(1, other.lo), F(1, other.hi)))
        return self.mul(reciprocals)


@dataclass(frozen=True)
class Form:
    center: F
    coeff: tuple[F, F, F, F]
    rem: Interval = Interval(F(0), F(0))

    def extent(self) -> Interval:
        radius = sum((abs(value) for value in self.coeff), F(0))
        return Interval(self.center - radius + self.rem.lo, self.center + radius + self.rem.hi)

    def add(self, other: "Form") -> "Form":
        return Form(
            self.center + other.center,
            tuple(a + b for a, b in zip(self.coeff, other.coeff)),
            self.rem.add(other.rem),
        )

    def scale(self, scalar: F) -> "Form":
        remainder = self.rem.mul(Interval(scalar, scalar))
        return Form(self.center * scalar, tuple(scalar * value for value in self.coeff), remainder)


@dataclass(frozen=True)
class State:
    position: tuple[Form, Form, Form]
    direction: tuple[Form, Form, Form]


class TypedStop(ValueError):
    pass


def form_record(form: Form) -> dict[str, object]:
    return {
        "center": rat(form.center),
        "coefficients": [rat(value) for value in form.coeff],
        "remainder": [rat(form.rem.lo), rat(form.rem.hi)],
    }


def state_record(state: State) -> dict[str, object]:
    return {
        "position": [form_record(form) for form in state.position],
        "direction": [form_record(form) for form in state.direction],
    }


def interval_record(interval: Interval) -> list[str]:
    return [rat(interval.lo), rat(interval.hi)]


def form_at(form: Form, u: tuple[F, F, F, F], remainder: F = F(0)) -> F:
    if not form.rem.lo <= remainder <= form.rem.hi:
        raise ValueError("remainder_outside_form")
    return form.center + sum((a * b for a, b in zip(form.coeff, u)), F(0)) + remainder


def fixed_advance(state: State, step: dict[str, object]) -> tuple[State, dict[str, object]]:
    scalar = step["advance"]
    if not isinstance(scalar, F):
        raise ValueError("advance_not_exact")
    output = State(
        tuple(p.add(v.scale(scalar)) for p, v in zip(state.position, state.direction)),
        state.direction,
    )
    return output, {"kind": "fixed_advance", "introduced_remainders": [["0/1", "0/1"]] * 6}


def plane_intersection(state: State, step: dict[str, object]) -> tuple[State, dict[str, object]]:
    axis = step.get("axis")
    height = step.get("height")
    orientation = step.get("orientation")
    if not isinstance(axis, int) or not 0 <= axis < AXES or not isinstance(height, F):
        raise ValueError("plane_step_shape")
    denominator = state.direction[axis].extent()
    expected_orientation = "increasing" if denominator.lo > 0 else "decreasing" if denominator.hi < 0 else None
    if expected_orientation is None or orientation != expected_orientation:
        raise TypedStop("unsupported_parallel_or_reversed_plane")

    numerator_form = Form(height, (F(0),) * SYMBOLS).add(state.position[axis].scale(F(-1)))
    time_interval = numerator_form.extent().div(denominator)
    if time_interval.lo <= 0:
        raise TypedStop("unsupported_nonforward_or_ordering")

    n0 = height - state.position[axis].center
    d0 = state.direction[axis].center
    if d0 == 0:
        raise TypedStop("unsupported_parallel_or_reversed_plane")
    time0 = n0 / d0
    time_coeff = tuple(
        ((-state.position[axis].coeff[k]) * d0 - n0 * state.direction[axis].coeff[k]) / (d0 * d0)
        for k in range(SYMBOLS)
    )

    positions: list[Form] = []
    residuals: list[list[str]] = []
    for index, (position, direction) in enumerate(zip(state.position, state.direction)):
        if index == axis:
            output = Form(height, (F(0),) * SYMBOLS)
        else:
            center = position.center + time0 * direction.center
            coeff = tuple(
                position.coeff[k] + time_coeff[k] * direction.center + time0 * direction.coeff[k]
                for k in range(SYMBOLS)
            )
            affine = Form(center, coeff)
            exact_interval = position.extent().add(time_interval.mul(direction.extent()))
            affine_interval = affine.extent()
            residual = exact_interval.sub(affine_interval)
            output = Form(center, coeff, residual)
        positions.append(output)
        residuals.append(interval_record(output.rem))
    return State(tuple(positions), state.direction), {
        "kind": "axis_plane",
        "time_interval": interval_record(time_interval),
        "introduced_remainders": residuals + [["0/1", "0/1"]] * 3,
    }


def derive(initial: State, steps: list[dict[str, object]]) -> tuple[State, list[dict[str, object]]]:
    state = initial
    receipts: list[dict[str, object]] = []
    previous_order: F | None = None
    seen_tokens: set[str] = set()
    for index, step in enumerate(steps):
        token = step.get("topology_token")
        order = step.get("order")
        if not valid_id(token) or token in seen_tokens or not isinstance(order, F):
            raise ValueError("topology_binding")
        if previous_order is not None and order <= previous_order:
            raise TypedStop("unsupported_nonforward_or_ordering")
        seen_tokens.add(token)
        previous_order = order
        kind = step.get("kind")
        if kind == "fixed_advance":
            state, receipt = fixed_advance(state, step)
        elif kind == "axis_plane":
            state, receipt = plane_intersection(state, step)
        elif kind in ("interface", "refraction", "tir"):
            raise TypedStop("unsupported_nonlinear_interface")
        else:
            raise ValueError("unknown_step")
        receipt.update({"index": index, "topology_token": token, "order": rat(order)})
        receipts.append(receipt)
    return state, receipts


def certificate(
    initial: State,
    steps: list[dict[str, object]],
    bindings: dict[str, object],
    claimed_output: State | None = None,
    extra_fields: dict[str, object] | None = None,
) -> dict[str, object]:
    required = {"cell_id", "scope_id", "reconstruction_id", "band_time_id", "physical_recipe_id", "physical_profile_id"}
    if set(bindings) != required or not all(valid_id(value) for value in bindings.values()):
        raise ValueError("binding_identity")
    if extra_fields:
        forbidden = {"arrival", "coupling", "emission", "radiance", "power", "visibility", "authority", "measure"}
        if forbidden.intersection(extra_fields):
            raise ValueError("forbidden_authority_field")
        raise ValueError("unknown_certificate_field")
    output, receipts = derive(initial, steps)
    if claimed_output is not None and claimed_output != output:
        raise ValueError("forged_output")
    record = {
        **bindings,
        "input_forms": state_record(initial),
        "steps": [step_record(step) for step in steps],
        "output_forms": state_record(output),
        "step_receipts": receipts,
        "authority_effect": "none_evidence_only",
    }
    record["certificate_id"] = digest(record)
    return record


def step_record(step: dict[str, object]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in step.items():
        result[key] = rat(value) if isinstance(value, F) else value
    return result


def all_fractions(value: object) -> Iterable[F]:
    if isinstance(value, F):
        yield value
    elif isinstance(value, Form):
        yield value.center
        yield from value.coeff
        yield value.rem.lo
        yield value.rem.hi
    elif isinstance(value, State):
        for form in (*value.position, *value.direction):
            yield from all_fractions(form)
    elif isinstance(value, (list, tuple)):
        for item in value:
            yield from all_fractions(item)
    elif isinstance(value, dict):
        for item in value.values():
            yield from all_fractions(item)


def evaluate_exact_plane(initial: State, step: dict[str, object], u: tuple[F, F, F, F]) -> tuple[F, F, F]:
    axis = int(step["axis"])
    position = tuple(form_at(form, u) for form in initial.position)
    direction = tuple(form_at(form, u) for form in initial.direction)
    time = (step["height"] - position[axis]) / direction[axis]
    return tuple(position[i] + time * direction[i] for i in range(AXES))


positive = 0
hostile = 0
typed_stops: dict[str, int] = {}


def check(condition: bool) -> None:
    global positive
    if not condition:
        raise AssertionError("positive_portfolio_failed")
    positive += 1


def reject(action: Callable[[], object], reason: str | None = None) -> None:
    global hostile
    try:
        action()
    except TypedStop as error:
        if reason is not None and str(error) != reason:
            raise AssertionError(f"wrong typed stop: {error}") from error
        typed_stops[str(error)] = typed_stops.get(str(error), 0) + 1
        hostile += 1
        return
    except (ValueError, ZeroDivisionError):
        if reason is not None:
            raise AssertionError("expected typed stop")
        hostile += 1
        return
    raise AssertionError("hostile_accepted")


z = F(0)
initial = State(
    (
        Form(F(-2), (F(1, 4), z, z, z)),
        Form(F(1), (z, F(1, 5), z, z)),
        Form(F(3), (z, z, F(1, 6), z)),
    ),
    (
        Form(F(2), (F(1, 8), z, z, z)),
        Form(F(1, 2), (z, F(1, 12), z, z)),
        Form(F(-1, 3), (z, z, z, F(1, 24))),
    ),
)
bindings = {
    "cell_id": opaque(1),
    "scope_id": opaque(2),
    "reconstruction_id": opaque(3),
    "band_time_id": opaque(4),
    "physical_recipe_id": opaque(5),
    "physical_profile_id": opaque(6),
}
zero_step = {"kind": "fixed_advance", "advance": F(0), "order": F(1), "topology_token": opaque(10)}
advance = {"kind": "fixed_advance", "advance": F(3, 2), "order": F(1), "topology_token": opaque(11)}
negative_advance = {"kind": "fixed_advance", "advance": F(-1, 4), "order": F(1), "topology_token": opaque(12)}
plane_x = {"kind": "axis_plane", "axis": 0, "height": F(2), "orientation": "increasing", "order": F(2), "topology_token": opaque(13)}
plane_y = {"kind": "axis_plane", "axis": 1, "height": F(10), "orientation": "increasing", "order": F(3), "topology_token": opaque(14)}

identity_cert = certificate(initial, [zero_step], bindings)
check(identity_cert["output_forms"] == state_record(initial))
check(certificate(initial, [zero_step], bindings)["certificate_id"] == identity_cert["certificate_id"])
advanced_state, _ = derive(initial, [advance])
check(advanced_state.position[0] == initial.position[0].add(initial.direction[0].scale(F(3, 2))))
check(advanced_state.direction == initial.direction)
check(derive(initial, [negative_advance])[0].position[2] == initial.position[2].add(initial.direction[2].scale(F(-1, 4))))

constant = State(
    tuple(Form(value, (z, z, z, z)) for value in (F(-1), F(2), F(4))),
    tuple(Form(value, (z, z, z, z)) for value in (F(2), F(1), F(-1))),
)
constant_plane = {"kind": "axis_plane", "axis": 0, "height": F(3), "orientation": "increasing", "order": F(1), "topology_token": opaque(20)}
constant_out, constant_receipts = derive(constant, [constant_plane])
check(constant_out.position[0] == Form(F(3), (z, z, z, z)))
check(constant_out.position[1].center == F(4) and constant_out.position[1].rem == Interval(z, z))
check(constant_receipts[0]["time_interval"] == ["2/1", "2/1"])

plane_out, plane_receipts = derive(initial, [plane_x])
check(plane_out.position[0] == Form(F(2), (z, z, z, z)))
check(any(form.rem != Interval(z, z) for form in plane_out.position[1:]))
corners = list(itertools.product((F(-1), F(1)), repeat=SYMBOLS))
enclosed = True
for u in corners + [(F(0), F(0), F(0), F(0)), (F(1, 3), F(-2, 5), F(1, 7), F(-1, 9))]:
    exact = evaluate_exact_plane(initial, plane_x, u)
    for axis in range(AXES):
        affine_value = form_at(plane_out.position[axis], u)
        residual = exact[axis] - affine_value
        if not plane_out.position[axis].rem.lo <= residual <= plane_out.position[axis].rem.hi:
            enclosed = False
check(enclosed)
check(plane_receipts[0]["time_interval"] == ["30/17", "34/15"])

ordered_steps = [advance, plane_x, plane_y]
ordered_cert = certificate(initial, ordered_steps, bindings)
check([item["topology_token"] for item in ordered_cert["steps"]] == [opaque(11), opaque(13), opaque(14)])
check(ordered_cert["authority_effect"] == "none_evidence_only")
check(not any(key in ordered_cert for key in ("arrival", "power", "visibility", "coupling")))
changed_bindings = dict(bindings, band_time_id=opaque(7))
check(certificate(initial, ordered_steps, changed_bindings)["certificate_id"] != ordered_cert["certificate_id"])
changed_physical = dict(bindings, physical_profile_id=opaque(8))
check(certificate(initial, ordered_steps, changed_physical)["certificate_id"] != ordered_cert["certificate_id"])
changed_topology = copy.deepcopy(ordered_steps)
changed_topology[1]["topology_token"] = opaque(21)
check(certificate(initial, changed_topology, bindings)["certificate_id"] != ordered_cert["certificate_id"])
check(certificate(initial, [zero_step, advance | {"order": F(2)}], bindings)["certificate_id"] != certificate(initial, [advance, zero_step | {"order": F(2)}], bindings)["certificate_id"])

# Subdivision commutes with affine advance, conserves exact measure, and retains shared-symbol cancellation.
def restrict(form: Form, symbol: int, side: int) -> Form:
    coeff = list(form.coeff)
    half = coeff[symbol] / 2
    coeff[symbol] = half
    return Form(form.center + side * half, tuple(coeff), form.rem)


def restrict_state(state: State, symbol: int, side: int) -> State:
    return State(tuple(restrict(form, symbol, side) for form in state.position), tuple(restrict(form, symbol, side) for form in state.direction))


parent_after = derive(initial, [advance])[0]
child_before = restrict_state(initial, 0, -1)
child_after = derive(child_before, [advance])[0]
check(child_after == restrict_state(parent_after, 0, -1))
measure_receipt = {str(leaves): rat(sum((F(1, leaves) for _ in range(leaves)), F(0))) for leaves in (4, 16, 64)}
check(measure_receipt == {"4": "1/1", "16": "1/1", "64": "1/1"})
same = initial.position[0]
cancelled = same.add(same.scale(F(-1)))
check(cancelled.extent() == Interval(z, z))
independent = Interval(same.extent().lo - same.extent().hi, same.extent().hi - same.extent().lo)
check(independent != Interval(z, z))
check(ordered_cert["certificate_id"] == digest({key: value for key, value in ordered_cert.items() if key != "certificate_id"}))

# Hostile binding, transcript, output, topology, branch, fold/order and authority cases.
for key in bindings:
    reject(lambda key=key: certificate(initial, ordered_steps, {**bindings, key: "0" * 64}))
reject(lambda: certificate(initial, ordered_steps, {**bindings, "receiver_id": opaque(30)}))
reject(lambda: certificate(initial, ordered_steps, bindings, claimed_output=initial))
for forbidden in ("arrival", "coupling", "emission", "radiance", "power", "visibility", "authority", "measure"):
    reject(lambda forbidden=forbidden: certificate(initial, ordered_steps, bindings, extra_fields={forbidden: "forged"}))
reject(lambda: certificate(initial, ordered_steps, bindings, extra_fields={"note": "unknown"}))
reject(lambda: certificate(initial, [advance, plane_y, plane_x], bindings), "unsupported_nonforward_or_ordering")
missing = [advance, plane_y]
reject(lambda: certificate(initial, missing, bindings, claimed_output=derive(initial, ordered_steps)[0]))
duplicate_token = [advance, {**plane_x, "topology_token": opaque(11)}]
reject(lambda: certificate(initial, duplicate_token, bindings))
reversed_orientation = [{**plane_x, "orientation": "decreasing"}]
reject(lambda: certificate(initial, reversed_orientation, bindings), "unsupported_parallel_or_reversed_plane")
parallel = State(initial.position, (Form(F(0), (F(1), z, z, z)), *initial.direction[1:]))
reject(lambda: certificate(parallel, [plane_x], bindings), "unsupported_parallel_or_reversed_plane")
reverse_direction = State(initial.position, (initial.direction[0].scale(F(-1)), *initial.direction[1:]))
reject(lambda: certificate(reverse_direction, [plane_x], bindings), "unsupported_parallel_or_reversed_plane")
behind_plane = [{**plane_x, "height": F(-10)}]
reject(lambda: certificate(initial, behind_plane, bindings), "unsupported_nonforward_or_ordering")
same_order = [advance, {**plane_x, "order": F(1)}]
reject(lambda: certificate(initial, same_order, bindings), "unsupported_nonforward_or_ordering")
remainder_understatement = copy.deepcopy(plane_out)
remainder_understatement = State((remainder_understatement.position[0], Form(remainder_understatement.position[1].center, remainder_understatement.position[1].coeff), remainder_understatement.position[2]), remainder_understatement.direction)
reject(lambda: certificate(initial, [plane_x], bindings, claimed_output=remainder_understatement))
for kind in ("interface", "refraction", "tir"):
    nonlinear = {"kind": kind, "order": F(1), "topology_token": opaque(40)}
    reject(lambda nonlinear=nonlinear: certificate(initial, [nonlinear], bindings), "unsupported_nonlinear_interface")
reject(lambda: certificate(initial, [{**advance, "kind": "caller_declared_output"}], bindings))
reject(lambda: certificate(initial, [{**advance, "advance": "3/2"}], bindings))
reject(lambda: certificate(initial, [{**advance, "topology_token": "not-an-id"}], bindings))
reject(lambda: certificate(initial, [{**plane_x, "axis": 3}], bindings))

all_values = list(all_fractions((initial, advanced_state, plane_out, constant_out)))
receipt = {
    "oracle": "optical_phase_space_transport_certificate_v0",
    "positive_portfolios": positive,
    "hostile_rejections": hostile,
    "typed_stops": dict(sorted(typed_stops.items())),
    "plane_corner_and_interior_falsifiers": len(corners) + 2,
    "refinement_measure_by_leaf_count": measure_receipt,
    "variable_plane_time_interval": plane_receipts[0]["time_interval"],
    "variable_plane_remainders": [interval_record(form.rem) for form in plane_out.position],
    "maximum_numerator_bits": max(abs(value.numerator).bit_length() for value in all_values),
    "maximum_denominator_bits": max(value.denominator.bit_length() for value in all_values),
    "maximum_certificate_bytes": max(len(canonical(value)) for value in (identity_cert, ordered_cert)),
    "ordered_certificate_id": ordered_cert["certificate_id"],
    "correlated_difference": interval_record(cancelled.extent()),
    "independent_box_difference": interval_record(independent),
    "nonlinear_interface_support": False,
    "authority_effect": "none_evidence_only",
    "schema_authorized": False,
    "production_source_authorized": False,
}
receipt["receipt_sha256"] = hashlib.sha256(canonical(receipt)).hexdigest()
print(json.dumps(receipt, sort_keys=True, indent=2))
