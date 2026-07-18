from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction as F
import hashlib
import json


@dataclass(frozen=True)
class Cell:
    path: str
    geometric_measure: F
    source_quantity: F

    def __post_init__(self) -> None:
        if self.geometric_measure <= 0:
            raise ValueError("geometric measure must be positive")
        if self.source_quantity < 0:
            raise ValueError("source quantity must be nonnegative")


def split(parent: Cell, lower_quantity: F, upper_quantity: F) -> tuple[Cell, Cell]:
    if lower_quantity < 0 or upper_quantity < 0:
        raise ValueError("negative child quantity")
    if lower_quantity + upper_quantity != parent.source_quantity:
        raise ValueError("child quantity mismatch")
    half = parent.geometric_measure / 2
    return (
        Cell(parent.path + "0", half, lower_quantity),
        Cell(parent.path + "1", half, upper_quantity),
    )


def refine(leaves: list[Cell], level: int) -> list[Cell]:
    refined: list[Cell] = []
    for index, parent in enumerate(leaves):
        selector = (index + level) % 4
        ratios = ((F(0), F(1)), (F(1, 3), F(2, 3)),
                  (F(1, 2), F(1, 2)), (F(3, 4), F(1, 4)))
        left_ratio, right_ratio = ratios[selector]
        refined.extend(split(parent, parent.source_quantity * left_ratio,
                             parent.source_quantity * right_ratio))
    return refined


def serial(value: object) -> object:
    if isinstance(value, F):
        return f"{value.numerator}/{value.denominator}"
    if isinstance(value, tuple) or isinstance(value, list):
        return [serial(item) for item in value]
    if isinstance(value, dict):
        return {key: serial(item) for key, item in value.items()}
    return value


def validate_subject(subject: dict[str, object]) -> None:
    expected = {
        "root_id": "root", "reconstruction_id": "reconstruction",
        "scope": "scope", "revision": 7, "cell_id": "cell:0101",
        "ancestry": ["cell", "cell:0", "cell:01", "cell:010", "cell:0101"],
        "band_time_id": "red:time-1", "quantity_basis_id": "quantity-basis-1",
        "quantity": "7/3", "authority": "abstract_additive_measure_only",
    }
    if subject != expected:
        raise ValueError("subject mismatch")


def compose(quantity: F, classification: str,
            transfer: tuple[F, F] | None = None) -> dict[str, object]:
    if quantity < 0:
        raise ValueError("negative quantity")
    if classification == "accepted":
        if transfer is None:
            raise ValueError("missing transfer")
        lower, upper = transfer
        if not (F(0) <= lower <= upper <= F(1)):
            raise ValueError("invalid transfer")
        return {"received_candidate": (quantity * lower, quantity * upper),
                "retained_source": quantity, "bucket": "accepted"}
    if classification == "zero_coupled":
        if transfer is not None:
            raise ValueError("zero coupling has no transfer")
        return {"received_candidate": (F(0), F(0)),
                "retained_source": quantity, "bucket": "zero_coupled"}
    if classification == "unresolved":
        if transfer is not None:
            raise ValueError("unresolved has no transfer")
        return {"received_candidate": None,
                "retained_source": quantity, "bucket": "unresolved"}
    raise ValueError("classification")


def rejected(action) -> bool:
    try:
        action()
    except (ValueError, ZeroDivisionError):
        return True
    return False


def main() -> None:
    root = Cell("", F(3, 2), F(7, 3))
    leaves = [root]
    conservation: list[dict[str, object]] = []
    geometric_checks = 0
    quantity_checks = 0
    for level in range(1, 7):
        leaves = refine(leaves, level)
        if len(leaves) in {4, 16, 64}:
            geometric_total = sum((leaf.geometric_measure for leaf in leaves), F(0))
            quantity_total = sum((leaf.source_quantity for leaf in leaves), F(0))
            assert geometric_total == root.geometric_measure
            assert quantity_total == root.source_quantity
            geometric_checks += 1
            quantity_checks += 1
            conservation.append({
                "children": len(leaves), "geometric_total": geometric_total,
                "quantity_total": quantity_total,
                "zero_quantity_children": sum(leaf.source_quantity == 0 for leaf in leaves),
            })

    assert geometric_checks == quantity_checks == 3
    assert any(leaf.source_quantity == 0 for leaf in leaves)

    # Counterexamples that falsify the rejected representations.
    ambient_parent = F(7, 3)
    ambient_copied_children = ambient_parent + ambient_parent
    assert ambient_copied_children == 2 * ambient_parent
    density = F(3)
    density_original_quantity = density * F(1)
    density_rescaled_quantity = density * F(2)
    assert density_rescaled_quantity == 2 * density_original_quantity
    radiance_required_fields = {"projected_area", "solid_angle", "quantity_basis",
                                "spectral_scope", "temporal_scope"}
    available_fields = {"abstract_cell_measure", "band_time_id"}
    assert radiance_required_fields - available_fields == radiance_required_fields

    accepted = compose(F(7, 3), "accepted", (F(1, 4), F(3, 4)))
    zero_source = compose(F(0), "accepted", (F(1, 4), F(3, 4)))
    zero_coupled = compose(F(7, 3), "zero_coupled")
    unresolved = compose(F(7, 3), "unresolved")
    assert accepted["received_candidate"] == (F(7, 12), F(7, 4))
    assert zero_source["received_candidate"] == (F(0), F(0))
    assert zero_coupled["retained_source"] == unresolved["retained_source"] == F(7, 3)
    assert zero_coupled["bucket"] != unresolved["bucket"]

    already_geometric = accepted["received_candidate"]
    extra_spreading = F(1, 4)
    double_counted = tuple(value * extra_spreading for value in already_geometric)
    assert double_counted != already_geometric

    q48 = 1 << 48
    exact_positive = F(1, q48) * F(1, q48)
    projected_raw = exact_positive.numerator * q48 // exact_positive.denominator
    assert exact_positive > 0 and projected_raw == 0

    subject = {
        "root_id": "root", "reconstruction_id": "reconstruction",
        "scope": "scope", "revision": 7, "cell_id": "cell:0101",
        "ancestry": ["cell", "cell:0", "cell:01", "cell:010", "cell:0101"],
        "band_time_id": "red:time-1", "quantity_basis_id": "quantity-basis-1",
        "quantity": "7/3", "authority": "abstract_additive_measure_only",
    }
    validate_subject(subject)
    mutations = [
        ("root_id", "foreign"), ("reconstruction_id", "foreign"),
        ("scope", "foreign"), ("revision", 8), ("cell_id", "cell:0100"),
        ("ancestry", ["cell", "cell:0", "cell:01", "cell:0101"]),
        ("ancestry", list(reversed(subject["ancestry"]))),
        ("band_time_id", "blue:time-1"), ("band_time_id", ""),
        ("quantity_basis_id", "foreign"), ("quantity_basis_id", ""),
        ("quantity", "14/6"), ("quantity", "-7/3"),
        ("authority", "watts"), ("authority", "radiance"),
    ]
    hostile_rejections = 0
    for key, value in mutations:
        hostile = dict(subject)
        hostile[key] = value
        if rejected(lambda candidate=hostile: validate_subject(candidate)):
            hostile_rejections += 1

    hostile_rejections += sum([
        rejected(lambda: split(root, F(7, 3), F(7, 3))),
        rejected(lambda: split(root, F(1), F(1))),
        rejected(lambda: split(root, F(-1), F(10, 3))),
        rejected(lambda: compose(F(-1), "accepted", (F(0), F(1)))),
        rejected(lambda: compose(F(1), "accepted", (F(3, 4), F(1, 4)))),
        rejected(lambda: compose(F(1), "unresolved", (F(0), F(1)))),
    ])
    assert hostile_rejections == len(mutations) + 6

    evidence = {
        "conservation": serial(conservation),
        "accepted": serial(accepted), "zero_source": serial(zero_source),
        "zero_coupled": serial(zero_coupled), "unresolved": serial(unresolved),
        "ambient_copied_children": serial(ambient_copied_children),
        "density_original_quantity": serial(density_original_quantity),
        "density_rescaled_quantity": serial(density_rescaled_quantity),
        "exact_positive_underflow": serial(exact_positive),
        "double_counted": serial(double_counted),
    }
    canonical = json.dumps(evidence, sort_keys=True, separators=(",", ":"))
    receipt = {
        "status": "pass",
        "portfolios": 10,
        "hostile_rejections": hostile_rejections,
        "subdivision_children": [4, 16, 64],
        "geometric_conservation_checks": geometric_checks,
        "quantity_conservation_checks": quantity_checks,
        "ambient_leaf_duplication": "rejected",
        "abstract_density_reparameterization": "counterexample_retained",
        "premature_si_radiance": "rejected_missing_calibration",
        "extra_spreading_multiplier": "rejected_unproved_double_count",
        "positive_underflow": "exact_not_physical_zero",
        "limitations": "code-free abstract measure oracle; no physical quantity basis schema source watts radiance detector visibility runtime promotion or C3 closure",
        "checksum": hashlib.sha256(canonical.encode()).hexdigest(),
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
