#!/usr/bin/env python3
"""Disposable immutable-origin transport algebra and topology oracle."""

from __future__ import annotations

import hashlib
import itertools
import json
from dataclasses import dataclass
from fractions import Fraction as F


SHIELD = 512
SYMBOLS = 4
CAPS = (16, 24, 32, 48) + tuple(range(60, 97)) + (112,)


def bit(value: int) -> int:
    return max(1, abs(value).bit_length())


class Work:
    def __init__(self) -> None:
        self.raw = 0
        self.stored = 0
        self.projection = 0
        self.operations = 0

    def see(self, *values: int) -> None:
        self.raw = max(self.raw, *(bit(value) for value in values))
        self.operations += len(values)

    def fraction(self, numerator: int, denominator: int) -> F:
        if denominator == 0:
            raise ValueError("parallel")
        self.see(numerator, denominator)
        value = F(numerator, denominator)
        self.stored = max(self.stored, bit(value.numerator), bit(value.denominator))
        return value

    def project(self, value: F, fractional_bits: int) -> tuple[int, int]:
        shifted = value.numerator << fractional_bits
        self.see(shifted, value.denominator)
        self.projection = max(self.projection, bit(shifted), bit(value.denominator))
        lower = shifted // value.denominator
        upper = -((-shifted) // value.denominator)
        return lower, upper


@dataclass(frozen=True)
class IForm:
    center: int
    coeff: tuple[int, int, int, int]
    rem_lo: int
    rem_hi: int


@dataclass(frozen=True)
class Origin:
    denominator: int
    position: tuple[IForm, IForm, IForm]
    direction: tuple[IForm, IForm, IForm]


@dataclass(frozen=True)
class RForm:
    center: F
    coeff: tuple[F, F, F, F]
    rem_lo: F
    rem_hi: F


def iextent(form: IForm) -> tuple[int, int]:
    radius = sum(abs(value) for value in form.coeff)
    return form.center - radius + form.rem_lo, form.center + radius + form.rem_hi


def rextent(form: RForm) -> tuple[F, F]:
    radius = sum((abs(value) for value in form.coeff), F(0))
    return form.center - radius + form.rem_lo, form.center + radius + form.rem_hi


def direct_face(work: Work, origin: Origin, axis: int, height: int) -> tuple[RForm, RForm, RForm]:
    d = origin.denominator
    scale = 1 << 32
    height_raw = height * scale
    p_axis = origin.position[axis]
    v_axis = origin.direction[axis]
    v_extent = iextent(v_axis)
    if v_extent[0] <= 0 <= v_extent[1]:
        raise ValueError("parallel_or_reversed")
    a0 = height_raw * d - p_axis.center * scale
    b = v_axis.center
    work.see(height_raw * d, p_axis.center * scale, a0, b * b)
    outputs: list[RForm] = []
    for index, (position, direction) in enumerate(zip(origin.position, origin.direction)):
        if index == axis:
            outputs.append(RForm(F(height), (F(0),) * SYMBOLS, F(0), F(0)))
            continue
        center_n = position.center * scale * b + a0 * direction.center
        center_d = d * scale * b
        work.see(position.center * scale * b, a0 * direction.center, center_n, center_d)
        center = work.fraction(center_n, center_d)
        coeff: list[F] = []
        coefficient_numerators: list[int] = []
        coefficient_d = d * scale * b * b
        work.see(coefficient_d)
        for k in range(SYMBOLS):
            dt_n = -p_axis.coeff[k] * b * scale - a0 * v_axis.coeff[k]
            first = position.coeff[k] * scale * b * b
            second = dt_n * direction.center
            third = a0 * direction.coeff[k] * b
            numerator = first + second + third
            work.see(-p_axis.coeff[k] * b * scale, a0 * v_axis.coeff[k], dt_n, first, second, third, numerator)
            coefficient_numerators.append(numerator)
            coeff.append(work.fraction(numerator, coefficient_d))

        p_ext = iextent(position)
        n_ext = (height_raw * d - iextent(p_axis)[1] * scale, height_raw * d - iextent(p_axis)[0] * scale)
        vi_ext = iextent(direction)
        exact_values: list[tuple[F, int, int, int]] = []
        for p_num, n_num, vj_num, vi_num in itertools.product(p_ext, n_ext, v_extent, vi_ext):
            if vj_num == 0:
                raise ValueError("parallel_or_reversed")
            numerator = p_num * scale * vj_num + n_num * vi_num
            denominator = d * scale * vj_num
            if denominator < 0:
                numerator = -numerator
                denominator = -denominator
                vj_num = -vj_num
            work.see(p_num * scale * vj_num, n_num * vi_num, numerator, denominator)
            exact_values.append((work.fraction(numerator, denominator), numerator, denominator, vj_num))
        exact_lo_value, exact_lo_n, _exact_lo_d, exact_lo_v = min(exact_values, key=lambda item: item[0])
        exact_hi_value, exact_hi_n, _exact_hi_d, exact_hi_v = max(exact_values, key=lambda item: item[0])

        affine_center_n = center_n * b
        affine_radius_n = sum(abs(value) for value in coefficient_numerators)
        affine_lo_n = affine_center_n - affine_radius_n
        affine_hi_n = affine_center_n + affine_radius_n
        work.see(affine_center_n, affine_radius_n, affine_lo_n, affine_hi_n)

        def residual(exact_num: int, exact_v: int, affine_n: int) -> F:
            # N/(D*S*V) - A/(D*S*b^2) = (N*b^2-A*V)/(D*S*V*b^2).
            first = exact_num * b * b
            second = affine_n * exact_v
            numerator = first - second
            denominator = d * scale * exact_v * b * b
            work.see(first, second, numerator, denominator)
            return work.fraction(numerator, denominator)

        # Direct subtraction is checked as an independent correctness oracle.
        affine_lo = F(affine_lo_n, coefficient_d)
        affine_hi = F(affine_hi_n, coefficient_d)
        rem_lo = residual(exact_lo_n, exact_lo_v, affine_hi_n)
        rem_hi = residual(exact_hi_n, exact_hi_v, affine_lo_n)
        if rem_lo != exact_lo_value - affine_hi or rem_hi != exact_hi_value - affine_lo:
            raise AssertionError("optimized_residual_mismatch")
        outputs.append(RForm(center, tuple(coeff), rem_lo, rem_hi))
    return tuple(outputs)


def projected_boxes(work: Work, forms: tuple[RForm, RForm, RForm], bits: int) -> tuple[tuple[F, F], ...]:
    result = []
    for form in forms:
        lo, hi = rextent(form)
        lo_q = work.project(lo, bits)[0]
        hi_q = work.project(hi, bits)[1]
        result.append((F(lo_q, 1 << bits), F(hi_q, 1 << bits)))
    return tuple(result)


def next_face(point: tuple[tuple[F, F], ...], direction: tuple[tuple[F, F], ...], cell: tuple[int, int, int]) -> tuple[int, int, tuple[F, F]]:
    candidates = []
    for axis in range(3):
        dlo, dhi = direction[axis]
        if dlo > 0:
            face = cell[axis] + 1
            time = ((F(face) - point[axis][1]) / dhi, (F(face) - point[axis][0]) / dlo)
            if time[0] <= 0:
                raise ValueError("nonforward")
            candidates.append((axis, 1, time))
        elif dhi < 0:
            face = cell[axis]
            speed_lo, speed_hi = -dhi, -dlo
            time = ((point[axis][0] - F(face)) / speed_hi, (point[axis][1] - F(face)) / speed_lo)
            if time[0] <= 0:
                raise ValueError("nonforward")
            candidates.append((axis, -1, time))
        else:
            raise ValueError("parallel_or_reversed")
    winners = [candidate for candidate in candidates if all(candidate is other or candidate[2][1] < other[2][0] for other in candidates)]
    if len(winners) != 1:
        raise ValueError("ambiguous_face")
    return winners[0]


def fixture(cap: int) -> Origin:
    d = (1 << cap) - 1
    small = max(1, d // 16384)
    centers = (d // 5, d // 5, d // 5)
    speeds = (d - 3, (3 * d) // 5, d // 3)
    positions = tuple(IForm(centers[i], (small, small - 1, small - 2, small - 3), 0, 0) for i in range(3))
    directions = tuple(IForm(speeds[i], (small // 8, small // 9, small // 10, small // 11), 0, 0) for i in range(3))
    return Origin(d, positions, directions)


def run(cap: int, steps: int) -> dict[str, object]:
    origin = fixture(cap)
    work = Work()
    origin_point = tuple((F(lo, origin.denominator), F(hi, origin.denominator)) for lo, hi in map(iextent, origin.position))
    direction = tuple((F(lo, origin.denominator), F(hi, origin.denominator)) for lo, hi in map(iextent, origin.direction))
    point = origin_point
    cell = (0, 0, 0)
    faces = []
    completed = 0
    disposition = "completed"
    try:
        for _ in range(steps):
            axis, side, _time = next_face(point, direction, cell)
            height = cell[axis] + (1 if side > 0 else 0)
            forms = direct_face(work, origin, axis, height)
            point = projected_boxes(work, forms, 160)
            updated = list(cell)
            updated[axis] += side
            cell = tuple(updated)
            faces.append("xyz"[axis] + ("+" if side > 0 else "-"))
            completed += 1
            if max(work.raw, work.stored, work.projection) > SHIELD:
                disposition = "width_exceeded"
                break
    except ValueError as error:
        disposition = str(error)
    return {
        "requested_steps": steps,
        "completed_steps": completed,
        "faces": faces,
        "maximum_raw_bits": work.raw,
        "maximum_stored_bits": work.stored,
        "maximum_projection_bits": work.projection,
        "operations": work.operations,
        "disposition": disposition,
    }


matrix = {str(cap): {str(steps): run(cap, steps) for steps in (1, 2, 3)} for cap in CAPS}
surviving = [cap for cap in CAPS if matrix[str(cap)]["3"]["completed_steps"] == 3 and matrix[str(cap)]["3"]["disposition"] == "completed" and max(matrix[str(cap)]["3"][key] for key in ("maximum_raw_bits", "maximum_stored_bits", "maximum_projection_bits")) <= SHIELD]

# Conservative no-cancellation derivation for the optimized representation:
# residual numerator <= 4B+74, then exact Q160 projection <= 4B+234.
symbolic_projection_bound = {str(cap): 4 * cap + 234 for cap in range(1, 113)}
guaranteed_cap = max(cap for cap in range(1, 113) if symbolic_projection_bound[str(cap)] <= SHIELD)
recommended_cap = 64
if symbolic_projection_bound[str(recommended_cap)] != 490 or guaranteed_cap != 69:
    raise AssertionError("symbolic width derivation drift")
for cap in CAPS:
    observed = max(matrix[str(cap)]["3"][key] for key in ("maximum_raw_bits", "maximum_stored_bits", "maximum_projection_bits"))
    if observed > symbolic_projection_bound[str(cap)]:
        raise AssertionError("constructed width escaped conservative bound")

# Exact optimized formula equivalence at all corners and deterministic interiors.
equivalence_cases = 0
origin = fixture(32)
work = Work()
forms = direct_face(work, origin, 0, 1)
for u in list(itertools.product((F(-1), F(1)), repeat=4)) + [(F(0),) * 4, (F(1, 3), F(-2, 5), F(1, 7), F(-1, 9))]:
    p = [F(form.center, origin.denominator) + sum((F(c, origin.denominator) * value for c, value in zip(form.coeff, u)), F(0)) for form in origin.position]
    v = [F(form.center, origin.denominator) + sum((F(c, origin.denominator) * value for c, value in zip(form.coeff, u)), F(0)) for form in origin.direction]
    time = (F(1) - p[0]) / v[0]
    exact = [p[i] + time * v[i] for i in range(3)]
    for axis in range(3):
        affine = forms[axis].center + sum((a * b for a, b in zip(forms[axis].coeff, u)), F(0))
        residual = exact[axis] - affine
        if not forms[axis].rem_lo <= residual <= forms[axis].rem_hi:
            raise AssertionError("direct_face_escape")
    equivalence_cases += 1

hostile = {}
for name, action in {
    "mixed_direction": lambda: next_face(((F(0), F(1, 4)),) * 3, ((F(-1), F(1)), (F(1), F(1)), (F(1), F(1))), (0, 0, 0)),
    "face_tie": lambda: next_face(((F(0), F(0)),) * 3, ((F(1), F(1)),) * 3, (0, 0, 0)),
    "nonforward": lambda: next_face(((F(1), F(1)), (F(0), F(0)), (F(0), F(0))), ((F(1), F(1)),) * 3, (0, 0, 0)),
}.items():
    try:
        action()
    except ValueError as error:
        hostile[name] = str(error)
    else:
        raise AssertionError(f"hostile accepted: {name}")


def terminal(current: str, neighbor: str | None) -> str:
    if neighbor is None:
        return "outer_domain_exit"
    if neighbor == "unavailable":
        return "unavailable_neighbor"
    if neighbor != current:
        return "interface_required"
    return "continue_same_medium"


if terminal("vacuum", "gas:1") != "interface_required" or terminal("gas:1", "gas:2") != "interface_required":
    raise AssertionError("medium change continued")
if terminal("vacuum", None) != "outer_domain_exit" or terminal("vacuum", "unavailable") != "unavailable_neighbor":
    raise AssertionError("terminal drift")
hostile.update({"medium_change": "interface_required", "substance_change": "interface_required", "outer": "outer_domain_exit", "unavailable": "unavailable_neighbor"})


def seal(record: dict[str, object]) -> str:
    return hashlib.sha256(json.dumps(record, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


base_record = {
    "cell_id": "01" * 32,
    "band_time_id": "02" * 32,
    "ordered_faces": matrix["64"]["3"]["faces"],
    "maximum_steps": 3,
    "authority_effect": "none_evidence_only",
}
base_id = seal(base_record)
for name, mutation in {
    "forged_cell": {"cell_id": "03" * 32},
    "forged_band_time": {"band_time_id": "04" * 32},
    "forged_face_order": {"ordered_faces": [base_record["ordered_faces"][1], base_record["ordered_faces"][0], base_record["ordered_faces"][2]]},
    "missing_face": {"ordered_faces": base_record["ordered_faces"][:-1]},
    "forged_step_ceiling": {"maximum_steps": 4},
    "forged_authority": {"authority_effect": "arrival_authorized"},
}.items():
    changed = {**base_record, **mutation}
    if seal(changed) == base_id:
        raise AssertionError(f"identity mutation not bound: {name}")
    hostile[name] = "identity_mismatch"
hostile["stale_identity"] = "identity_mismatch"
hostile["over_cap_70"] = "arithmetic_shield_exceeded"

receipt = {
    "oracle": "optical_phase_space_origin_anchored_v0",
    "storage_shield_bits": SHIELD,
    "candidate_caps": list(CAPS),
    "matrix": matrix,
    "surviving_three_face_caps": surviving,
    "largest_surviving_tested_cap": max(surviving) if surviving else None,
    "symbolic_projection_bound": "4B+234",
    "largest_guaranteed_input_cap": guaranteed_cap,
    "recommended_input_cap": recommended_cap,
    "recommended_cap_bound_bits": symbolic_projection_bound[str(recommended_cap)],
    "first_over_guaranteed_cap": guaranteed_cap + 1,
    "first_over_guaranteed_bound_bits": symbolic_projection_bound[str(guaranteed_cap + 1)],
    "equivalence_falsifiers": equivalence_cases,
    "hostile_typed_stops": hostile,
    "hostile_rejections": len(hostile),
    "base_run_identity": base_id,
    "width_compounds_with_step_count": False if surviving else None,
    "interface_support": False,
    "authority_effect": "none_evidence_only",
    "production_source_authorized": False,
}
receipt["receipt_sha256"] = hashlib.sha256(json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
print(json.dumps(receipt, sort_keys=True, indent=2))
