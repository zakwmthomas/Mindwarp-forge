from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction as F
import hashlib
import json
from itertools import product

N = 4
ZERO = (0,) * N


@dataclass(frozen=True)
class Poly:
    terms: dict[tuple[int, ...], F]

    @staticmethod
    def constant(value: int | F) -> "Poly":
        value = F(value)
        return Poly({ZERO: value} if value else {})

    @staticmethod
    def affine(constant: int | F, *coefficients: int | F) -> "Poly":
        terms: dict[tuple[int, ...], F] = {}
        if F(constant):
            terms[ZERO] = F(constant)
        for index, coefficient in enumerate(coefficients):
            if F(coefficient):
                exponent = [0] * N
                exponent[index] = 1
                terms[tuple(exponent)] = F(coefficient)
        return Poly(terms)

    def __add__(self, other: "Poly") -> "Poly":
        terms = dict(self.terms)
        for exponent, coefficient in other.terms.items():
            terms[exponent] = terms.get(exponent, F(0)) + coefficient
            if not terms[exponent]:
                del terms[exponent]
        return Poly(terms)

    def __neg__(self) -> "Poly":
        return Poly({exponent: -coefficient for exponent, coefficient in self.terms.items()})

    def __sub__(self, other: "Poly") -> "Poly":
        return self + (-other)

    def __mul__(self, other: "Poly") -> "Poly":
        terms: dict[tuple[int, ...], F] = {}
        for left_exp, left_coefficient in self.terms.items():
            for right_exp, right_coefficient in other.terms.items():
                exponent = tuple(a + b for a, b in zip(left_exp, right_exp))
                terms[exponent] = terms.get(exponent, F(0)) + left_coefficient * right_coefficient
        return Poly({exponent: coefficient for exponent, coefficient in terms.items() if coefficient})

    def scale(self, value: int | F) -> "Poly":
        return self * Poly.constant(value)

    def substitute_box(self, centers: tuple[F, ...], radii: tuple[F, ...]) -> "Poly":
        result = Poly.constant(0)
        for exponent, coefficient in self.terms.items():
            term = Poly.constant(coefficient)
            for axis, power in enumerate(exponent):
                factor = Poly.affine(centers[axis], *(
                    radii[axis] if index == axis else F(0) for index in range(N)
                ))
                for _ in range(power):
                    term = term * factor
            result = result + term
        return result

    def bounds(self) -> tuple[F, F]:
        lower = F(0)
        upper = F(0)
        for exponent, coefficient in self.terms.items():
            if exponent == ZERO:
                term_lower = term_upper = coefficient
            else:
                odd = any(power % 2 for power in exponent)
                base_lower, base_upper = (F(-1), F(1)) if odd else (F(0), F(1))
                if coefficient >= 0:
                    term_lower, term_upper = coefficient * base_lower, coefficient * base_upper
                else:
                    term_lower, term_upper = coefficient * base_upper, coefficient * base_lower
            lower += term_lower
            upper += term_upper
        return lower, upper

    def canonical(self) -> list[list[object]]:
        return [[list(exponent), str(coefficient)] for exponent, coefficient in sorted(self.terms.items())]


@dataclass(frozen=True)
class Cell:
    centers: tuple[F, ...] = (F(0),) * N
    radii: tuple[F, ...] = (F(1),) * N
    measure: F = F(1)


@dataclass(frozen=True)
class Case:
    name: str
    start: tuple[Poly, Poly, Poly]
    face: tuple[Poly, Poly, Poly]
    receiver_min: tuple[F, F, F]
    receiver_max: tuple[F, F, F]


def bounds(poly: Poly, cell: Cell) -> tuple[F, F]:
    return poly.substitute_box(cell.centers, cell.radii).bounds()


def strictly_inside(poly: Poly, low: F, high: F, cell: Cell) -> bool:
    lower, upper = bounds(poly, cell)
    return low < lower and upper < high


def positive(poly: Poly, cell: Cell) -> bool:
    return bounds(poly, cell)[0] > 0


def nonnegative(poly: Poly, cell: Cell) -> bool:
    return bounds(poly, cell)[0] >= 0


def classify(case: Case, cell: Cell = Cell()) -> tuple[str, str]:
    if any(low >= high for low, high in zip(case.receiver_min, case.receiver_max)):
        raise ValueError("invalid receiver")
    if all(strictly_inside(case.start[axis], case.receiver_min[axis], case.receiver_max[axis], cell) for axis in range(3)):
        return "full", "uniform_start_inside"

    direction = tuple(face - start for start, face in zip(case.start, case.face))
    for axis in range(3):
        for side, plane in (("lower", case.receiver_min[axis]), ("upper", case.receiver_max[axis])):
            sign = F(1) if side == "lower" else F(-1)
            denominator = direction[axis].scale(sign)
            numerator = (Poly.constant(plane) - case.start[axis]).scale(sign)
            if not positive(denominator, cell) or not nonnegative(numerator, cell):
                continue
            if not positive(denominator - numerator, cell):
                continue
            admitted = True
            for cross_axis in range(3):
                if cross_axis == axis:
                    continue
                at_low = ((case.start[cross_axis] - Poly.constant(case.receiver_min[cross_axis])) * denominator
                          + direction[cross_axis] * numerator)
                below_high = ((Poly.constant(case.receiver_max[cross_axis]) - case.start[cross_axis]) * denominator
                              - direction[cross_axis] * numerator)
                if not positive(at_low, cell) or not positive(below_high, cell):
                    admitted = False
                    break
            if admitted:
                return "full", f"uniform_{side}_face_entry_axis_{axis}"

    for axis in range(3):
        start_bounds = bounds(case.start[axis], cell)
        face_bounds = bounds(case.face[axis], cell)
        swept_lower = min(start_bounds[0], face_bounds[0])
        swept_upper = max(start_bounds[1], face_bounds[1])
        if swept_upper < case.receiver_min[axis] or swept_lower > case.receiver_max[axis]:
            return "zero", f"separating_axis_{axis}"
    return "unresolved", "mixed_or_uncertified_ordering"


def split(cell: Cell, axis: int) -> tuple[Cell, Cell]:
    child_radius = cell.radii[axis] / 2
    children = []
    for direction in (F(-1), F(1)):
        centers = list(cell.centers)
        radii = list(cell.radii)
        centers[axis] += direction * child_radius
        radii[axis] = child_radius
        children.append(Cell(tuple(centers), tuple(radii), cell.measure / 2))
    return tuple(children)


def refine(depth: int) -> list[Cell]:
    cells = [Cell()]
    for level in range(depth):
        cells = [child for cell in cells for child in split(cell, level % N)]
    return cells


def accounting(case: Case, depth: int) -> dict[str, F]:
    totals = {"full": F(0), "zero": F(0), "unresolved": F(0)}
    for cell in refine(depth):
        outcome, _ = classify(case, cell)
        totals[outcome] += cell.measure
    assert sum(totals.values(), F(0)) == F(1)
    return totals


u = Poly.affine(0, 1, 0, 0, 0)
v = Poly.affine(0, 0, 1, 0, 0)
c = Poly.constant

CASES = [
    Case("start-inside", (u.scale(F(1, 10)), v.scale(F(1, 10)), c(0)), (c(2), c(0), c(0)), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("uniform-lower-entry", (c(-2), u.scale(F(1, 4)), v.scale(F(1, 4))), (c(2), u.scale(F(1, 4)), v.scale(F(1, 4))), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("face-coincident", (c(-2), c(0), c(0)), (c(-1), c(0), c(0)), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("separated", (c(-3), u, v), (c(-2), u, v), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("partial-cross-axis", (c(-2), u.scale(2), c(0)), (c(2), u.scale(2), c(0)), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("mixed-direction", (u, c(0), c(0)), (u.scale(-1), c(0), c(0)), (F(-1, 4), F(-1), F(-1)), (F(1, 4), F(1), F(1))),
    Case("correlated-cancellation", (c(-2), u - u, v.scale(F(1, 4))), (c(2), u - u, v.scale(F(1, 4))), (F(-1), F(-1, 2), F(-1)), (F(1), F(1, 2), F(1))),
    Case("tangent-cross-axis", (c(-2), c(1), c(0)), (c(2), c(1), c(0)), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("uniform-upper-entry", (c(2), u.scale(F(1, 4)), v.scale(F(1, 4))), (c(-2), u.scale(F(1, 4)), v.scale(F(1, 4))), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("start-boundary-inward", (c(-1), c(0), c(0)), (c(2), c(0), c(0)), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("receiver-after-face", (c(-3), c(0), c(0)), (c(-2), c(0), c(0)), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
    Case("stationary-mixed-cell", (c(0), u.scale(2), c(0)), (c(0), u.scale(2), c(0)), (F(-1), F(-1), F(-1)), (F(1), F(1), F(1))),
]


def main() -> None:
    expected = {
        "start-inside": "full",
        "uniform-lower-entry": "full",
        "face-coincident": "unresolved",
        "separated": "zero",
        "partial-cross-axis": "unresolved",
        "mixed-direction": "unresolved",
        "correlated-cancellation": "full",
        "tangent-cross-axis": "unresolved",
        "uniform-upper-entry": "full",
        "start-boundary-inward": "full",
        "receiver-after-face": "zero",
        "stationary-mixed-cell": "unresolved",
    }
    outcomes = []
    checks = 0
    for case in CASES:
        outcome, reason = classify(case)
        assert outcome == expected[case.name]
        outcomes.append({"case": case.name, "outcome": outcome, "reason": reason})
        checks += 1

    conservation = []
    for case in CASES:
        for depth in (2, 4, 6):
            totals = accounting(case, depth)
            conservation.append({"case": case.name, "children": 2 ** depth,
                                 "full": str(totals["full"]), "zero": str(totals["zero"]),
                                 "unresolved": str(totals["unresolved"])})
            checks += 2 ** depth

    hostile_cases = sum(1 for item in outcomes if item["outcome"] != "full")
    hostile_rejections = 0
    for axis in range(3):
        low = [F(-1), F(-1), F(-1)]
        high = [F(1), F(1), F(1)]
        low[axis] = high[axis]
        try:
            classify(Case(f"bad-receiver-{axis}", (c(0), c(0), c(0)), (c(1), c(1), c(1)),
                          tuple(low), tuple(high)))
        except ValueError:
            hostile_rejections += 1
    assert hostile_rejections == 3

    canonical = json.dumps({"outcomes": outcomes, "conservation": conservation}, sort_keys=True, separators=(",", ":"))
    receipt = {
        "status": "pass",
        "checks": checks,
        "positive_cases": len(CASES),
        "hostile_cases": hostile_cases,
        "hostile_rejections": hostile_rejections,
        "subdivision_children": [4, 16, 64],
        "classifier": "uniform start-inside or one uniform inward receiver-face proof; separating-axis zero; otherwise unresolved",
        "limitations": "code-free oracle; no source magnitude power detector visibility runtime promotion or C3 closure",
        "checksum": hashlib.sha256(canonical.encode()).hexdigest(),
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
