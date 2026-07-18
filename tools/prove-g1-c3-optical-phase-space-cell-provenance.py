from __future__ import annotations

import copy
import hashlib
import json
from dataclasses import dataclass
from fractions import Fraction
from typing import Callable


DOMAIN_ROOT = b"mindwarp.optical-phase-space.root.oracle.v0"
DOMAIN_CHILD = b"mindwarp.optical-phase-space.child.oracle.v0"
DOMAIN_SPLIT = b"mindwarp.optical-phase-space.split.oracle.v0"
MAX_DIMENSION = 4
MAX_DEPTH = 6
MAX_RATIONAL_BITS = 256
OUTPUTS = 6


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode("utf-8")


def digest(domain: bytes, value: object) -> str:
    return hashlib.sha256(domain + b"\x00" + canonical(value)).hexdigest()


def fid(byte: int) -> str:
    if not 0 < byte < 256:
        raise ValueError("identity byte")
    return f"{byte:02x}" * 32


def valid_id(value: str) -> bool:
    if len(value) != 64 or value == "0" * 64:
        return False
    try:
        int(value, 16)
    except ValueError:
        return False
    return True


def rat(value: Fraction) -> str:
    if value.denominator <= 0 or value.numerator.bit_length() > MAX_RATIONAL_BITS or value.denominator.bit_length() > MAX_RATIONAL_BITS:
        raise ValueError("rational bound")
    return f"{value.numerator}/{value.denominator}"


def parse_rat(value: str) -> Fraction:
    if not isinstance(value, str) or value.count("/") != 1:
        raise ValueError("rational syntax")
    left, right = value.split("/")
    if not left or not right or (left.startswith("+") or right.startswith(("+", "-"))):
        raise ValueError("rational syntax")
    numerator = int(left)
    denominator = int(right)
    result = Fraction(numerator, denominator)
    if denominator <= 0 or rat(result) != value:
        raise ValueError("noncanonical rational")
    return result


@dataclass(frozen=True)
class Form:
    center: Fraction
    coefficients: tuple[Fraction, ...]
    remainder_lower: Fraction
    remainder_upper: Fraction


@dataclass(frozen=True)
class Cell:
    source_id: str
    scope_id: str
    reconstruction_id: str
    revision: int
    dimension: int
    root_id: str
    parent_id: str | None
    depth: int
    path: tuple[tuple[int, str], ...]
    measure: Fraction
    forms: tuple[Form, ...]
    cell_id: str


def form_record(form: Form) -> dict[str, object]:
    return {
        "center": rat(form.center),
        "coefficients": [rat(value) for value in form.coefficients],
        "remainder": [rat(form.remainder_lower), rat(form.remainder_upper)],
    }


def identity_record(cell: Cell, include_id: bool = False) -> dict[str, object]:
    value: dict[str, object] = {
        "source_id": cell.source_id,
        "scope_id": cell.scope_id,
        "reconstruction_id": cell.reconstruction_id,
        "revision": cell.revision,
        "dimension": cell.dimension,
        "root_id": cell.root_id,
        "parent_id": cell.parent_id,
        "depth": cell.depth,
        "path": [[axis, side] for axis, side in cell.path],
        "measure": rat(cell.measure),
        "forms": [form_record(form) for form in cell.forms],
    }
    if include_id:
        value["cell_id"] = cell.cell_id
    return value


def validate_common(source_id: str, scope_id: str, reconstruction_id: str, revision: int, dimension: int, measure: Fraction, forms: tuple[Form, ...]) -> None:
    if not all(valid_id(value) for value in (source_id, scope_id, reconstruction_id)):
        raise ValueError("provenance identity")
    if revision <= 0 or not 1 <= dimension <= MAX_DIMENSION or measure <= 0:
        raise ValueError("root scalar")
    rat(measure)
    if len(forms) != OUTPUTS:
        raise ValueError("output count")
    for form in forms:
        if len(form.coefficients) != dimension or form.remainder_lower > form.remainder_upper:
            raise ValueError("form shape")
        for value in (form.center, *form.coefficients, form.remainder_lower, form.remainder_upper):
            rat(value)


def make_root(source_id: str, scope_id: str, reconstruction_id: str, revision: int, dimension: int, measure: Fraction, forms: tuple[Form, ...]) -> Cell:
    validate_common(source_id, scope_id, reconstruction_id, revision, dimension, measure, forms)
    provisional = Cell(source_id, scope_id, reconstruction_id, revision, dimension, "", None, 0, (), measure, forms, "")
    record = identity_record(provisional)
    record["root_id"] = None
    root_id = digest(DOMAIN_ROOT, record)
    cell = Cell(source_id, scope_id, reconstruction_id, revision, dimension, root_id, None, 0, (), measure, forms, root_id)
    validate_cell(cell)
    return cell


def validate_cell(cell: Cell) -> None:
    validate_common(cell.source_id, cell.scope_id, cell.reconstruction_id, cell.revision, cell.dimension, cell.measure, cell.forms)
    if cell.depth != len(cell.path) or cell.depth > MAX_DEPTH:
        raise ValueError("depth/path")
    if cell.parent_id is None:
        if cell.depth != 0 or cell.path or cell.root_id != cell.cell_id:
            raise ValueError("root relation")
        provisional = Cell(cell.source_id, cell.scope_id, cell.reconstruction_id, cell.revision, cell.dimension, "", None, 0, (), cell.measure, cell.forms, "")
        record = identity_record(provisional)
        record["root_id"] = None
        if digest(DOMAIN_ROOT, record) != cell.cell_id:
            raise ValueError("root identity")
    else:
        if not valid_id(cell.root_id) or not valid_id(cell.parent_id):
            raise ValueError("child relation")
        if any(axis < 0 or axis >= cell.dimension or side not in ("lower", "upper") for axis, side in cell.path):
            raise ValueError("path member")
        if digest(DOMAIN_CHILD, identity_record(cell)) != cell.cell_id:
            raise ValueError("child identity")


def derived_child(parent: Cell, axis: int, side: str) -> Cell:
    validate_cell(parent)
    if parent.depth >= MAX_DEPTH or not 0 <= axis < parent.dimension or side not in ("lower", "upper"):
        raise ValueError("split request")
    sign = -1 if side == "lower" else 1
    forms = []
    for form in parent.forms:
        coefficients = list(form.coefficients)
        half = coefficients[axis] / 2
        coefficients[axis] = half
        forms.append(Form(form.center + sign * half, tuple(coefficients), form.remainder_lower, form.remainder_upper))
    provisional = Cell(parent.source_id, parent.scope_id, parent.reconstruction_id, parent.revision, parent.dimension, parent.root_id, parent.cell_id, parent.depth + 1, parent.path + ((axis, side),), parent.measure / 2, tuple(forms), "")
    child_id = digest(DOMAIN_CHILD, identity_record(provisional))
    child = Cell(**{**provisional.__dict__, "cell_id": child_id})
    validate_cell(child)
    return child


def split(parent: Cell, axis: int) -> tuple[Cell, Cell, dict[str, object]]:
    lower = derived_child(parent, axis, "lower")
    upper = derived_child(parent, axis, "upper")
    receipt = {
        "parent_id": parent.cell_id,
        "axis": axis,
        "ordered_child_ids": [lower.cell_id, upper.cell_id],
        "parent_measure": rat(parent.measure),
        "ordered_child_measures": [rat(lower.measure), rat(upper.measure)],
    }
    receipt["split_id"] = digest(DOMAIN_SPLIT, receipt)
    validate_split(parent, axis, lower, upper, receipt)
    return lower, upper, receipt


def validate_split(parent: Cell, axis: int, lower: Cell | None, upper: Cell | None, receipt: dict[str, object]) -> None:
    if lower is None or upper is None:
        raise ValueError("missing child")
    expected_lower = derived_child(parent, axis, "lower")
    expected_upper = derived_child(parent, axis, "upper")
    if lower != expected_lower or upper != expected_upper or lower == upper:
        raise ValueError("forged child")
    expected = {
        "parent_id": parent.cell_id,
        "axis": axis,
        "ordered_child_ids": [lower.cell_id, upper.cell_id],
        "parent_measure": rat(parent.measure),
        "ordered_child_measures": [rat(lower.measure), rat(upper.measure)],
    }
    expected["split_id"] = digest(DOMAIN_SPLIT, expected)
    if receipt != expected or lower.measure + upper.measure != parent.measure:
        raise ValueError("split receipt")


def projection(form: Form) -> tuple[Fraction, Fraction]:
    radius = sum((abs(value) for value in form.coefficients), Fraction(0))
    return form.center - radius + form.remainder_lower, form.center + radius + form.remainder_upper


def difference(form_a: Form, form_b: Form) -> Form:
    return Form(form_a.center - form_b.center, tuple(a - b for a, b in zip(form_a.coefficients, form_b.coefficients)), form_a.remainder_lower - form_b.remainder_upper, form_a.remainder_upper - form_b.remainder_lower)


def strict_record_keys(record: dict[str, object]) -> None:
    allowed = {"source_id", "scope_id", "reconstruction_id", "revision", "dimension", "root_id", "parent_id", "depth", "path", "measure", "forms", "cell_id"}
    if set(record) != allowed:
        raise ValueError("unknown field")


positive = 0
hostile = 0


def check(condition: bool) -> None:
    global positive
    if not condition:
        raise AssertionError("positive portfolio")
    positive += 1


def reject(action: Callable[[], object]) -> None:
    global hostile
    try:
        action()
    except (ValueError, ZeroDivisionError):
        hostile += 1
        return
    raise AssertionError("hostile accepted")


zero = Fraction(0)
forms = (
    Form(zero, (Fraction(1), zero, zero, zero), zero, zero),
    Form(zero, (Fraction(1), zero, zero, zero), zero, zero),
    Form(Fraction(3), (zero, Fraction(2), zero, zero), Fraction(-1, 10), Fraction(1, 10)),
    Form(Fraction(1, 2), (zero, zero, Fraction(1, 4), zero), zero, zero),
    Form(Fraction(-1, 2), (zero, zero, zero, Fraction(1, 8)), zero, zero),
    Form(zero, (Fraction(1, 3), Fraction(-1, 5), zero, zero), Fraction(-1, 100), Fraction(1, 100)),
)
root = make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1), forms)
check(root.cell_id == make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1), forms).cell_id)
check(root.cell_id != make_root(fid(4), fid(2), fid(3), 1, 4, Fraction(1), forms).cell_id)
check(root.cell_id != make_root(fid(1), fid(4), fid(3), 1, 4, Fraction(1), forms).cell_id)
check(root.cell_id != make_root(fid(1), fid(2), fid(4), 1, 4, Fraction(1), forms).cell_id)
check(root.cell_id != make_root(fid(1), fid(2), fid(3), 2, 4, Fraction(1), forms).cell_id)
check(root.cell_id != make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(3, 2), forms).cell_id)
changed_forms = list(forms)
changed_forms[5] = Form(Fraction(1, 1000), forms[5].coefficients, forms[5].remainder_lower, forms[5].remainder_upper)
check(root.cell_id != make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1), tuple(changed_forms)).cell_id)
lower, upper, first_receipt = split(root, 0)
check(lower.cell_id != upper.cell_id)
check(lower.measure + upper.measure == root.measure)
check(first_receipt == split(root, 0)[2])

leaves = [root]
levels: dict[str, str] = {}
all_cells = [root]
all_receipts = []
for depth in range(1, MAX_DEPTH + 1):
    next_leaves = []
    for cell in leaves:
        lo, hi, receipt = split(cell, (depth - 1) % root.dimension)
        next_leaves.extend((lo, hi))
        all_cells.extend((lo, hi))
        all_receipts.append(receipt)
    leaves = next_leaves
    if len(leaves) in (4, 16, 64):
        levels[str(len(leaves))] = rat(sum((cell.measure for cell in leaves), Fraction(0)))
check(levels["4"] == "1/1")
check(levels["16"] == "1/1")
check(levels["64"] == "1/1")

cancelled = difference(root.forms[0], root.forms[1])
check(projection(cancelled) == (zero, zero))
box_a = projection(root.forms[0])
box_b = projection(root.forms[1])
independent_difference = (box_a[0] - box_b[1], box_a[1] - box_b[0])
check(independent_difference == (Fraction(-2), Fraction(2)))
check(projection(root.forms[2]) == (Fraction(9, 10), Fraction(51, 10)))
check("receiver_id" not in identity_record(root, include_id=True))
check(lower.forms[0].center == Fraction(-1, 2) and lower.forms[0].coefficients[0] == Fraction(1, 2))
check(upper.forms[0].center == Fraction(1, 2) and upper.forms[0].coefficients[0] == Fraction(1, 2))
check(canonical(identity_record(root, include_id=True)) == canonical(identity_record(root, include_id=True)))

reject(lambda: make_root("0" * 64, fid(2), fid(3), 1, 4, Fraction(1), forms))
reject(lambda: make_root("zz", fid(2), fid(3), 1, 4, Fraction(1), forms))
reject(lambda: make_root(fid(1), fid(2), fid(3), 0, 4, Fraction(1), forms))
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 0, Fraction(1), forms))
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 5, Fraction(1), forms))
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(0), forms))
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(-1), forms))
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1), forms[:5]))
bad_arity = (Form(zero, (Fraction(1),), zero, zero),) + forms[1:]
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1), bad_arity))
bad_remainder = (Form(zero, forms[0].coefficients, Fraction(1), Fraction(-1)),) + forms[1:]
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1), bad_remainder))
wide_number = Fraction(1 << 256)
wide_form = (Form(wide_number, forms[0].coefficients, zero, zero),) + forms[1:]
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1), wide_form))
reject(lambda: make_root(fid(1), fid(2), fid(3), 1, 4, Fraction(1, 1 << 256), forms))
reject(lambda: split(root, -1))
reject(lambda: split(root, root.dimension))
reject(lambda: split(leaves[0], 0))

def forged(cell: Cell, **changes: object) -> Cell:
    return Cell(**{**cell.__dict__, **changes})

reject(lambda: validate_cell(forged(lower, parent_id=fid(9))))
reject(lambda: validate_cell(forged(lower, root_id=fid(9))))
reject(lambda: validate_cell(forged(lower, depth=2)))
reject(lambda: validate_cell(forged(lower, path=((1, "lower"),))))
reject(lambda: validate_cell(forged(lower, measure=Fraction(3, 4))))
forged_forms = list(lower.forms)
forged_forms[0] = Form(Fraction(-2, 3), forged_forms[0].coefficients, zero, zero)
reject(lambda: validate_cell(forged(lower, forms=tuple(forged_forms))))
forged_forms = list(lower.forms)
forged_forms[0] = Form(forged_forms[0].center, (Fraction(3, 4), zero, zero, zero), zero, zero)
reject(lambda: validate_cell(forged(lower, forms=tuple(forged_forms))))
forged_forms = list(lower.forms)
forged_forms[0] = Form(forged_forms[0].center, forged_forms[0].coefficients, Fraction(-1, 9), Fraction(1, 9))
reject(lambda: validate_cell(forged(lower, forms=tuple(forged_forms))))
reject(lambda: validate_split(root, 0, upper, lower, first_receipt))
reject(lambda: validate_split(root, 0, lower, lower, first_receipt))
reject(lambda: validate_split(root, 0, lower, None, first_receipt))
bad_receipt = copy.deepcopy(first_receipt)
bad_receipt["ordered_child_measures"][0] = "3/4"
reject(lambda: validate_split(root, 0, lower, upper, bad_receipt))
record = identity_record(root, include_id=True)
record["receiver_id"] = fid(8)
reject(lambda: strict_record_keys(record))
record = identity_record(root, include_id=True)
record["topology_branch"] = "all_transmit"
reject(lambda: strict_record_keys(record))
record = identity_record(root, include_id=True)
record["radiance"] = "1/1"
reject(lambda: strict_record_keys(record))
reject(lambda: parse_rat("2/2"))
reject(lambda: parse_rat("0/2"))
reject(lambda: parse_rat("1/-2"))

fractions = []
for cell in all_cells:
    fractions.append(cell.measure)
    for form in cell.forms:
        fractions.extend((form.center, *form.coefficients, form.remainder_lower, form.remainder_upper))
receipt = {
    "oracle": "optical_phase_space_cell_provenance_v0",
    "positive_portfolios": positive,
    "hostile_rejections": hostile,
    "root_id": root.cell_id,
    "refinement_measure_by_leaf_count": levels,
    "full_binary_tree_cells": len(all_cells),
    "split_receipts": len(all_receipts),
    "maximum_depth": max(cell.depth for cell in all_cells),
    "maximum_numerator_bits": max(abs(value.numerator).bit_length() for value in fractions),
    "maximum_denominator_bits": max(value.denominator.bit_length() for value in fractions),
    "maximum_cell_canonical_bytes": max(len(canonical(identity_record(cell, include_id=True))) for cell in all_cells),
    "correlated_difference": [rat(value) for value in projection(cancelled)],
    "independent_box_difference": [rat(value) for value in independent_difference],
    "receiver_field_in_identity": False,
    "authority_effect": "none_evidence_only",
    "schema_authorized": False,
}
receipt["receipt_sha256"] = hashlib.sha256(canonical(receipt)).hexdigest()
print(json.dumps(receipt, sort_keys=True, indent=2))

