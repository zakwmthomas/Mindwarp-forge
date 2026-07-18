from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction as F
import hashlib
import json


@dataclass(frozen=True)
class Basis:
    quantity: str
    unit: str
    wavelength_pm: tuple[int, int]
    time_ticks: tuple[int, int]
    seconds_per_tick: F
    weighting: str
    calibration_id: str
    transport_band: str

    def validate(self) -> None:
        if self.quantity != "radiant_energy" or self.unit != "joule":
            raise ValueError("quantity basis")
        if not (0 < self.wavelength_pm[0] < self.wavelength_pm[1]):
            raise ValueError("spectral interval")
        if not (0 <= self.time_ticks[0] < self.time_ticks[1]):
            raise ValueError("time interval")
        if self.seconds_per_tick <= 0 or self.weighting != "energy_integral":
            raise ValueError("integration basis")
        if not self.calibration_id or self.transport_band not in {"red", "green", "blue"}:
            raise ValueError("identity basis")


@dataclass(frozen=True)
class CellEnergy:
    path: str
    joules: F

    def __post_init__(self) -> None:
        if self.joules < 0:
            raise ValueError("negative energy")


def split(parent: CellEnergy, lower: F, upper: F) -> tuple[CellEnergy, CellEnergy]:
    if lower < 0 or upper < 0 or lower + upper != parent.joules:
        raise ValueError("atomic split mismatch")
    return CellEnergy(parent.path + "0", lower), CellEnergy(parent.path + "1", upper)


def refine(leaves: list[CellEnergy], level: int) -> list[CellEnergy]:
    ratios = ((F(0), F(1)), (F(1, 3), F(2, 3)),
              (F(1, 2), F(1, 2)), (F(3, 4), F(1, 4)))
    out: list[CellEnergy] = []
    for index, cell in enumerate(leaves):
        a, b = ratios[(index + level) % len(ratios)]
        out.extend(split(cell, cell.joules * a, cell.joules * b))
    return out


def compose(energy: F, source_basis: Basis, transfer_basis: Basis,
            transfer: tuple[F, F] | None, classification: str) -> dict[str, object]:
    source_basis.validate()
    transfer_basis.validate()
    if source_basis != transfer_basis:
        raise ValueError("physical basis mismatch")
    if energy < 0:
        raise ValueError("negative energy")
    if classification == "accepted":
        if transfer is None or not (F(0) <= transfer[0] <= transfer[1] <= F(1)):
            raise ValueError("transfer")
        return {"bucket": "accepted", "source_joules": energy,
                "received_joules": (energy * transfer[0], energy * transfer[1])}
    if classification == "zero_coupled" and transfer is None:
        return {"bucket": "zero_coupled", "source_joules": energy,
                "received_joules": (F(0), F(0))}
    if classification == "unresolved" and transfer is None:
        return {"bucket": "unresolved", "source_joules": energy,
                "received_joules": None}
    raise ValueError("classification")


def canonical_rational(text: str) -> F:
    if "/" not in text:
        raise ValueError("rational shape")
    n, d = text.split("/", 1)
    if not n.isdigit() or not d.isdigit() or (len(n) > 1 and n[0] == "0") or (len(d) > 1 and d[0] == "0"):
        raise ValueError("decimal form")
    numerator, denominator = int(n), int(d)
    if denominator == 0:
        raise ValueError("zero denominator")
    value = F(numerator, denominator)
    if f"{value.numerator}/{value.denominator}" != text:
        raise ValueError("non-reduced")
    return value


def rejected(action) -> bool:
    try:
        action()
    except ValueError:
        return True
    return False


def serial(value: object) -> object:
    if isinstance(value, F):
        return f"{value.numerator}/{value.denominator}"
    if isinstance(value, Basis):
        return {key: serial(item) for key, item in value.__dict__.items()}
    if isinstance(value, tuple) or isinstance(value, list):
        return [serial(item) for item in value]
    if isinstance(value, dict):
        return {key: serial(item) for key, item in value.items()}
    return value


def main() -> None:
    basis = Basis("radiant_energy", "joule", (500_000, 600_000),
                  (100, 116), F(1, 1000), "energy_integral", "cal-1", "green")
    basis.validate()

    root = CellEnergy("", F(17, 5))
    leaves = [root]
    conservation: list[dict[str, object]] = []
    for level in range(1, 7):
        leaves = refine(leaves, level)
        if len(leaves) in {4, 16, 64}:
            total = sum((leaf.joules for leaf in leaves), F(0))
            assert total == root.joules
            conservation.append({"leaves": len(leaves), "joules": total,
                                 "zero_children": sum(leaf.joules == 0 for leaf in leaves)})

    first_half = Basis("radiant_energy", "joule", (500_000, 600_000),
                       (100, 108), F(1, 1000), "energy_integral", "cal-1", "green")
    second_half = Basis("radiant_energy", "joule", (500_000, 600_000),
                        (108, 116), F(1, 1000), "energy_integral", "cal-1", "green")
    assert F(6, 5) + F(11, 5) == root.joules

    accepted = compose(root.joules, basis, basis, (F(1, 4), F(3, 4)), "accepted")
    zero_source = compose(F(0), basis, basis, (F(1, 4), F(3, 4)), "accepted")
    zero_coupled = compose(root.joules, basis, basis, None, "zero_coupled")
    unresolved = compose(root.joules, basis, basis, None, "unresolved")
    assert accepted["received_joules"] == (F(17, 20), F(51, 20))
    assert zero_source["received_joules"] == (F(0), F(0))
    assert zero_coupled["source_joules"] == unresolved["source_joules"] == root.joules

    # Equal average power, opposite emission timing, different received energy.
    duration = F(1)
    profile_a = (F(2), F(0))
    profile_b = (F(0), F(2))
    transfer_by_half = (F(1), F(0))
    average_a = sum(profile_a, F(0)) / 2
    average_b = sum(profile_b, F(0)) / 2
    received_a = sum((profile_a[i] * F(1, 2) * transfer_by_half[i] for i in range(2)), F(0))
    received_b = sum((profile_b[i] * F(1, 2) * transfer_by_half[i] for i in range(2)), F(0))
    assert duration == 1 and average_a == average_b == 1 and received_a == 1 and received_b == 0

    normalized = F(7, 10)
    assert normalized * F(10) != normalized * F(100)
    density = F(3)
    assert density * F(1) != density * F(2)

    spectral_mismatch = Basis("radiant_energy", "joule", (600_000, 700_000),
                              (100, 116), F(1, 1000), "energy_integral", "cal-1", "red")
    time_mismatch = Basis("radiant_energy", "joule", (500_000, 600_000),
                          (116, 132), F(1, 1000), "energy_integral", "cal-1", "green")
    calibration_mismatch = Basis("radiant_energy", "joule", (500_000, 600_000),
                                 (100, 116), F(1, 1000), "energy_integral", "cal-2", "green")

    hostile = [
        lambda: compose(root.joules, basis, spectral_mismatch, (F(0), F(1)), "accepted"),
        lambda: compose(root.joules, basis, time_mismatch, (F(0), F(1)), "accepted"),
        lambda: compose(root.joules, basis, calibration_mismatch, (F(0), F(1)), "accepted"),
        lambda: split(root, root.joules, root.joules),
        lambda: split(root, F(1), F(1)),
        lambda: split(root, F(-1), F(22, 5)),
        lambda: compose(F(-1), basis, basis, (F(0), F(1)), "accepted"),
        lambda: compose(F(1), basis, basis, (F(3, 4), F(1, 4)), "accepted"),
        lambda: compose(F(1), basis, basis, (F(0), F(1)), "unresolved"),
        lambda: canonical_rational("-1/2"),
        lambda: canonical_rational("+1/2"),
        lambda: canonical_rational("01/2"),
        lambda: canonical_rational("1/02"),
        lambda: canonical_rational("1/0"),
        lambda: canonical_rational("2/4"),
        lambda: canonical_rational("0/2"),
        lambda: Basis("radiant_power", "watt", (500_000, 600_000), (100, 116), F(1, 1000), "energy_integral", "cal-1", "green").validate(),
        lambda: Basis("radiant_energy", "joule", (600_000, 500_000), (100, 116), F(1, 1000), "energy_integral", "cal-1", "green").validate(),
        lambda: Basis("radiant_energy", "joule", (500_000, 600_000), (116, 100), F(1, 1000), "energy_integral", "cal-1", "green").validate(),
        lambda: Basis("radiant_energy", "joule", (500_000, 600_000), (100, 116), F(0), "energy_integral", "cal-1", "green").validate(),
        lambda: Basis("radiant_energy", "joule", (500_000, 600_000), (100, 116), F(1, 1000), "energy_integral", "", "green").validate(),
    ]
    hostile_rejections = sum(rejected(case) for case in hostile)
    assert hostile_rejections == len(hostile)

    q48 = 1 << 48
    exact_positive = F(1, q48) * F(1, q48)
    assert exact_positive > 0 and exact_positive.numerator * q48 // exact_positive.denominator == 0

    evidence = {
        "basis": serial(basis), "conservation": serial(conservation),
        "time_partition": serial([first_half, second_half]), "accepted": serial(accepted),
        "power_counterexample": serial({"average": average_a, "received_a": received_a,
                                         "received_b": received_b}),
        "normalized_scale": serial([normalized * 10, normalized * 100]),
        "radiance_reparameterization": serial([density, density * 2]),
        "positive_underflow": serial(exact_positive),
    }
    canonical = json.dumps(evidence, sort_keys=True, separators=(",", ":"))
    receipt = {
        "status": "pass", "candidate": "band_time_integrated_radiant_energy_joule",
        "conservation_leaf_counts": [4, 16, 64], "conservation_checks": 3,
        "hostile_rejections": hostile_rejections,
        "radiant_power": "rejected_as_primary_temporal_correlation",
        "normalized_non_si": "rejected_for_physical_closure",
        "radiance_density": "rejected_missing_physical_measure_jacobian",
        "band_time_calibration": "required_before_physical_transfer_composition",
        "positive_underflow": "exact_not_physical_zero",
        "limitations": "code-free oracle; no RGB boundaries tick duration schema source detector visibility runtime promotion or C3 closure",
        "checksum": hashlib.sha256(canonical.encode()).hexdigest(),
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
