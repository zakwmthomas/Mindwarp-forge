from __future__ import annotations

from dataclasses import dataclass, replace
from fractions import Fraction as F
import hashlib
import json


CALIBRATED_BASIS_ID = "a9913e0d498c2e686574b1a755675d32ce0be3bdc59bf3335cb8d40716684a22"
CALIBRATION_PROVENANCE_ID = "calibration-provenance-7"
ROOT_ID = "8724e0219d44bc40dbcb7315369dabe3153710617def82854d1ad490a802141f"
RECONSTRUCTION_ID = "phase-space-reconstruction-3"
BAND_TIME_IDS = {
    "blue": "6ba64d5c9bc8a2ac97dbb83725ae7de4023194187a2693e73204ad81209fe951",
    "green": "bbf46da98bd7492ba9e0766f30701e53642a0ae61821a6bdfbda579470de7c79",
    "red": "d80738a5589306c4f08162009ea705444cb1cfda16bd78d37d8dc1a79bbca52c",
}


def digest(domain: str, value: object) -> str:
    payload = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(domain.encode() + b"\0" + payload).hexdigest()


def canonical_rational(text: str) -> F:
    if text.count("/") != 1:
        raise ValueError("rational shape")
    numerator, denominator = text.split("/")
    if (
        not numerator.isdigit()
        or not denominator.isdigit()
        or (len(numerator) > 1 and numerator.startswith("0"))
        or (len(denominator) > 1 and denominator.startswith("0"))
    ):
        raise ValueError("rational decimal")
    value = F(int(numerator), int(denominator))
    if f"{value.numerator}/{value.denominator}" != text:
        raise ValueError("noncanonical rational")
    return value


@dataclass(frozen=True)
class Calibration:
    calibrated_basis_id: str
    band: str
    band_time_id: str
    calibration_provenance_id: str

    def validate(self) -> None:
        if self.calibrated_basis_id != CALIBRATED_BASIS_ID:
            raise ValueError("foreign calibrated basis")
        if self.band not in BAND_TIME_IDS or self.band_time_id != BAND_TIME_IDS[self.band]:
            raise ValueError("band/time derivation mismatch")
        if self.calibration_provenance_id != CALIBRATION_PROVENANCE_ID:
            raise ValueError("calibration provenance mismatch")


@dataclass(frozen=True)
class Subject:
    source_id: str
    scope_id: str
    source_provenance_id: str
    source_revision: int
    root_id: str
    reconstruction_id: str
    calibration: Calibration

    def validate(self) -> None:
        self.calibration.validate()
        if not self.source_id or not self.scope_id or not self.source_provenance_id:
            raise ValueError("missing source provenance")
        if self.source_revision <= 0:
            raise ValueError("source revision")
        if self.source_provenance_id == self.calibration.calibration_provenance_id:
            raise ValueError("source and calibration provenance conflated")
        if self.root_id != ROOT_ID or self.reconstruction_id != RECONSTRUCTION_ID:
            raise ValueError("foreign phase-space subject")


def cell_id(root_id: str, reconstruction_id: str, path: str) -> str:
    return digest("mindwarp.disposable.phase-space-cell-ref.v1", {
        "root_id": root_id,
        "reconstruction_id": reconstruction_id,
        "path": path,
    })


@dataclass(frozen=True)
class Allocation:
    root_id: str
    reconstruction_id: str
    path: str
    measure: F
    cell_id: str
    joules: F
    resolution: str

    def validate(self) -> None:
        if any(bit not in "01" for bit in self.path):
            raise ValueError("path")
        if self.root_id != ROOT_ID or self.reconstruction_id != RECONSTRUCTION_ID:
            raise ValueError("cell subject")
        expected_measure = F(1, 1 << len(self.path))
        if self.measure != expected_measure or self.measure <= 0:
            raise ValueError("cell measure")
        if self.cell_id != cell_id(self.root_id, self.reconstruction_id, self.path):
            raise ValueError("cell identity")
        if self.joules < 0:
            raise ValueError("negative energy")
        if self.resolution not in {"resolved_leaf", "unresolved_within_cell"}:
            raise ValueError("allocation resolution")


def allocation(path: str, joules: F, resolution: str = "resolved_leaf") -> Allocation:
    return Allocation(
        root_id=ROOT_ID,
        reconstruction_id=RECONSTRUCTION_ID,
        path=path,
        measure=F(1, 1 << len(path)),
        cell_id=cell_id(ROOT_ID, RECONSTRUCTION_ID, path),
        joules=joules,
        resolution=resolution,
    )


def frontier_covers_root(paths: set[str]) -> bool:
    maximum_depth = max(map(len, paths), default=0)

    def covers(prefix: str) -> bool:
        if prefix in paths:
            return True
        if len(prefix) >= maximum_depth:
            return False
        return covers(prefix + "0") and covers(prefix + "1")

    return covers("")


def distribution_id(subject: Subject, root_joules: F, nodes: list[Allocation]) -> str:
    return digest("mindwarp.disposable.calibrated-source-energy-distribution.v1", {
        "subject": serial(subject),
        "root_joules": serial(root_joules),
        "frontier": serial(sorted(nodes, key=lambda node: node.path)),
        "quantity": "radiant_energy",
        "unit": "joule",
        "authority_effect": "none_evidence_only",
    })


def validate_distribution(subject: Subject, root_joules: F,
                          nodes: list[Allocation]) -> str:
    subject.validate()
    if root_joules < 0 or not nodes:
        raise ValueError("root energy or empty frontier")
    for node in nodes:
        node.validate()
        if node.root_id != subject.root_id or node.reconstruction_id != subject.reconstruction_id:
            raise ValueError("foreign node")
    paths = [node.path for node in nodes]
    if len(paths) != len(set(paths)):
        raise ValueError("duplicate frontier cell")
    for left in paths:
        for right in paths:
            if left != right and right.startswith(left):
                raise ValueError("ancestor/descendant overlap")
    if not frontier_covers_root(set(paths)):
        raise ValueError("incomplete frontier")
    if sum((node.measure for node in nodes), F(0)) != F(1):
        raise ValueError("geometric measure mismatch")
    if sum((node.joules for node in nodes), F(0)) != root_joules:
        raise ValueError("energy conservation mismatch")
    return distribution_id(subject, root_joules, nodes)


def split(parent: Allocation, lower_joules: F, upper_joules: F,
          lower_resolution: str = "resolved_leaf",
          upper_resolution: str = "resolved_leaf") -> tuple[Allocation, Allocation, str]:
    parent.validate()
    if lower_joules < 0 or upper_joules < 0 or lower_joules + upper_joules != parent.joules:
        raise ValueError("atomic split mismatch")
    lower = allocation(parent.path + "0", lower_joules, lower_resolution)
    upper = allocation(parent.path + "1", upper_joules, upper_resolution)
    receipt = digest("mindwarp.disposable.source-energy-split.v1", {
        "parent": serial(parent), "children": serial([lower, upper])
    })
    return lower, upper, receipt


def refine(nodes: list[Allocation], level: int) -> tuple[list[Allocation], list[str]]:
    ratios = ((F(0), F(1)), (F(1, 3), F(2, 3)),
              (F(1, 2), F(1, 2)), (F(3, 4), F(1, 4)))[::-1]
    out: list[Allocation] = []
    receipts: list[str] = []
    for index, parent in enumerate(sorted(nodes, key=lambda node: node.path)):
        lower_ratio, upper_ratio = ratios[(index + level) % len(ratios)]
        lower, upper, receipt = split(
            parent,
            parent.joules * lower_ratio,
            parent.joules * upper_ratio,
        )
        out.extend([lower, upper])
        receipts.append(receipt)
    return out, receipts


def compose_control(source_joules: F, transfer: F) -> dict[str, object]:
    if source_joules < 0 or not F(0) <= transfer <= F(1):
        raise ValueError("composition control")
    return {
        "source_joules": source_joules,
        "dimensionless_transfer": transfer,
        "product": source_joules * transfer,
        "case": "zero_source" if source_joules == 0 else (
            "zero_coupling" if transfer == 0 else "positive_product"
        ),
    }


def rejected(action) -> bool:
    try:
        action()
    except (ValueError, ZeroDivisionError):
        return True
    return False


def serial(value: object) -> object:
    if isinstance(value, F):
        return f"{value.numerator}/{value.denominator}"
    if hasattr(value, "__dataclass_fields__"):
        return {key: serial(getattr(value, key)) for key in value.__dataclass_fields__}
    if isinstance(value, (tuple, list)):
        return [serial(item) for item in value]
    if isinstance(value, dict):
        return {key: serial(item) for key, item in value.items()}
    return value


def main() -> None:
    calibration = Calibration(
        CALIBRATED_BASIS_ID,
        "green",
        BAND_TIME_IDS["green"],
        CALIBRATION_PROVENANCE_ID,
    )
    subject = Subject(
        "physical-source-11",
        "emission-scope-5",
        "source-provenance-19",
        4,
        ROOT_ID,
        RECONSTRUCTION_ID,
        calibration,
    )
    root_joules = F(17, 5)
    nodes = [allocation("", root_joules, "unresolved_within_cell")]
    root_only_id = validate_distribution(subject, root_joules, nodes)
    conservation: list[dict[str, object]] = []
    split_receipts: list[str] = []
    for level in range(1, 7):
        nodes, receipts = refine(nodes, level)
        split_receipts.extend(receipts)
        if len(nodes) in {4, 16, 64}:
            identity = validate_distribution(subject, root_joules, nodes)
            conservation.append({
                "leaves": len(nodes),
                "joules": root_joules,
                "measure": sum((node.measure for node in nodes), F(0)),
                "zero_energy_cells": sum(node.joules == 0 for node in nodes),
                "distribution_id": identity,
            })
    assert [item["leaves"] for item in conservation] == [4, 16, 64]
    assert any(node.joules == 0 for node in nodes)

    # A mixed-depth closed frontier retains unresolved detail without losing energy.
    mixed = [
        allocation("0", F(7, 5), "unresolved_within_cell"),
        allocation("10", F(0)),
        allocation("11", F(2)),
    ]
    mixed_id = validate_distribution(subject, root_joules, mixed)
    assert sum((node.joules for node in mixed), F(0)) == root_joules

    # Equal calibration does not imply equal source magnitude or distribution.
    lower_energy_id = validate_distribution(subject, F(1), [allocation("", F(1))])
    alternate = [allocation("0", F(2)), allocation("1", F(7, 5))]
    alternate_id = validate_distribution(subject, root_joules, alternate)
    assert lower_energy_id != root_only_id and alternate_id != root_only_id

    # Density is a coordinate-local derived average, never the canonical quantity.
    density = root_joules / F(1)
    reparameterized_density = root_joules / F(2)
    assert density != reparameterized_density
    concentrated_left = [allocation("0", root_joules), allocation("1", F(0))]
    concentrated_right = [allocation("0", F(0)), allocation("1", root_joules)]
    assert validate_distribution(subject, root_joules, concentrated_left) != validate_distribution(
        subject, root_joules, concentrated_right
    )
    assert root_joules / F(1) == root_joules / F(1)  # Equal parent density hides both.

    zero_source_positive_transfer = compose_control(F(0), F(3, 4))
    positive_source_zero_coupling = compose_control(F(3, 4), F(0))
    assert zero_source_positive_transfer["product"] == positive_source_zero_coupling["product"] == 0
    assert zero_source_positive_transfer["case"] != positive_source_zero_coupling["case"]

    hostile: list[object] = []
    hostile.extend([
        lambda: validate_distribution(replace(subject, source_id=""), root_joules, mixed),
        lambda: validate_distribution(replace(subject, scope_id=""), root_joules, mixed),
        lambda: validate_distribution(replace(subject, source_revision=0), root_joules, mixed),
        lambda: validate_distribution(replace(subject, source_provenance_id=CALIBRATION_PROVENANCE_ID), root_joules, mixed),
        lambda: validate_distribution(replace(subject, root_id="foreign"), root_joules, mixed),
        lambda: validate_distribution(replace(subject, reconstruction_id="foreign"), root_joules, mixed),
        lambda: validate_distribution(replace(subject, calibration=replace(calibration, calibrated_basis_id="foreign")), root_joules, mixed),
        lambda: validate_distribution(replace(subject, calibration=replace(calibration, band="red")), root_joules, mixed),
        lambda: validate_distribution(replace(subject, calibration=replace(calibration, band_time_id=BAND_TIME_IDS["red"])), root_joules, mixed),
        lambda: validate_distribution(replace(subject, calibration=replace(calibration, calibration_provenance_id="foreign")), root_joules, mixed),
    ])
    hostile.extend([
        lambda: validate_distribution(subject, root_joules, []),
        lambda: validate_distribution(subject, root_joules, [mixed[0], mixed[0], mixed[1], mixed[2]]),
        lambda: validate_distribution(subject, root_joules, [allocation("0", root_joules), allocation("00", F(0)), allocation("01", F(0)), allocation("1", F(0))]),
        lambda: validate_distribution(subject, root_joules, [allocation("0", F(7, 5)), allocation("10", F(0))]),
        lambda: validate_distribution(subject, root_joules, [allocation("0", F(7, 5)), allocation("1", F(1))]),
        lambda: validate_distribution(subject, root_joules, [replace(mixed[0], cell_id="foreign"), mixed[1], mixed[2]]),
        lambda: validate_distribution(subject, root_joules, [replace(mixed[0], measure=F(2)), mixed[1], mixed[2]]),
        lambda: validate_distribution(subject, root_joules, [replace(mixed[0], joules=F(-1)), mixed[1], mixed[2]]),
        lambda: validate_distribution(subject, root_joules, [replace(mixed[0], resolution="sampled") , mixed[1], mixed[2]]),
        lambda: validate_distribution(subject, F(-1), [allocation("", F(0))]),
        lambda: split(allocation("", root_joules), root_joules, root_joules),
        lambda: split(allocation("", root_joules), F(1), F(1)),
        lambda: split(allocation("", root_joules), F(-1), F(22, 5)),
    ])
    hostile.extend([
        lambda value=value: canonical_rational(value)
        for value in ["-1/2", "+1/2", "01/2", "1/02", "1/0", "2/4", "0/2", "1", "1/2/3"]
    ])
    hostile_rejections = sum(rejected(case) for case in hostile)
    assert hostile_rejections == len(hostile)

    # Independent leaf records cannot prove root coverage or single counting.
    leaf_only_missing = sum((node.joules for node in nodes[:-1]), F(0))
    leaf_only_duplicate = sum((node.joules for node in nodes + [nodes[0]]), F(0))
    assert leaf_only_missing != root_joules
    assert leaf_only_duplicate != root_joules or nodes[0].joules == 0
    assert rejected(lambda: validate_distribution(subject, root_joules, nodes[:-1]))
    assert rejected(lambda: validate_distribution(subject, root_joules, nodes + [nodes[0]]))

    evidence = {
        "subject": serial(subject),
        "root_only_id": root_only_id,
        "mixed_frontier_id": mixed_id,
        "conservation": serial(conservation),
        "split_receipt_count": len(split_receipts),
        "equal_basis_different_energy_id": lower_energy_id,
        "equal_energy_different_distribution_id": alternate_id,
        "density_reparameterization": serial([density, reparameterized_density]),
        "zero_cases": serial([zero_source_positive_transfer, positive_source_zero_coupling]),
    }
    canonical = json.dumps(evidence, sort_keys=True, separators=(",", ":"))
    receipt = {
        "status": "pass",
        "candidate": "closed_frontier_additive_calibrated_radiant_energy_measure",
        "portfolios": 8,
        "conservation_leaf_counts": [4, 16, 64],
        "conservation_checks": 3,
        "split_receipts": len(split_receipts),
        "hostile_rejections": hostile_rejections,
        "leaf_only_records": "rejected_missing_root_coverage_and_single_counting",
        "root_distribution": "selected_prefix_free_closed_frontier",
        "cell_measure_density": "derived_coordinate_local_average_only",
        "unresolved_allocation": "retained_at_coarser_frontier_cell",
        "source_calibration_provenance": "distinct_and_identity_bound",
        "zero_source_vs_zero_coupling": "typed_distinct",
        "limitations": "code-free exact-rational oracle; no schema crate consumer production source transport detector visibility runtime promotion or C3 closure",
        "checksum": hashlib.sha256(canonical.encode()).hexdigest(),
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
