#!/usr/bin/env python3
"""Disposable exact oracle for the calibrated spectral/time basis design."""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction as F
import hashlib
import json
from typing import Callable


BANDS = ("blue", "green", "red")


def canonical_rational(text: str) -> F:
    if text.count("/") != 1:
        raise ValueError("rational shape")
    numerator_text, denominator_text = text.split("/")
    if not numerator_text.isdigit() or not denominator_text.isdigit():
        raise ValueError("unsigned decimal form")
    if (len(numerator_text) > 1 and numerator_text[0] == "0") or (
        len(denominator_text) > 1 and denominator_text[0] == "0"
    ):
        raise ValueError("leading zero")
    denominator = int(denominator_text)
    if denominator == 0:
        raise ValueError("zero denominator")
    value = F(int(numerator_text), denominator)
    if f"{value.numerator}/{value.denominator}" != text:
        raise ValueError("noncanonical rational")
    return value


@dataclass(frozen=True)
class Interval:
    lower: F
    upper: F

    def validate(self) -> None:
        if self.lower < 0 or self.lower >= self.upper:
            raise ValueError("invalid interval")


@dataclass(frozen=True)
class SpectralBasis:
    coordinate: str
    weighting: str
    intervals: tuple[tuple[str, Interval], ...]

    def validate(self) -> None:
        if self.coordinate != "vacuum_wavelength_metre":
            raise ValueError("spectral coordinate")
        if self.weighting != "unit_energy_integral":
            raise ValueError("spectral weighting")
        if tuple(name for name, _ in self.intervals) != BANDS:
            raise ValueError("band order")
        for _, interval in self.intervals:
            interval.validate()
        for index in range(len(self.intervals) - 1):
            if self.intervals[index][1].upper != self.intervals[index + 1][1].lower:
                raise ValueError("spectral overlap or gap")

    def interval(self, band: str) -> Interval:
        self.validate()
        for name, interval in self.intervals:
            if name == band:
                return interval
        raise ValueError("unknown band")


@dataclass(frozen=True)
class TimeCell:
    origin_id: str
    start_tick: int
    end_tick: int
    seconds_per_tick: F
    time_basis_id: str

    def validate(self) -> None:
        if not self.origin_id or not self.time_basis_id:
            raise ValueError("time identity")
        if self.start_tick < 0 or self.start_tick >= self.end_tick:
            raise ValueError("time interval")
        if self.seconds_per_tick <= 0:
            raise ValueError("tick scale")


@dataclass(frozen=True)
class Calibration:
    quantity_kind: str
    unit: str
    version: int
    provenance_id: str
    spectral: SpectralBasis
    time: TimeCell
    legacy_pairs: tuple[tuple[str, str], ...]

    def validate(self) -> None:
        if self.quantity_kind != "radiant_energy" or self.unit != "joule":
            raise ValueError("quantity")
        if self.version <= 0 or not self.provenance_id:
            raise ValueError("calibration identity")
        self.spectral.validate()
        self.time.validate()
        if tuple(band for band, _ in self.legacy_pairs) != BANDS:
            raise ValueError("legacy band mapping")
        if any(time_id != self.time.time_basis_id for _, time_id in self.legacy_pairs):
            raise ValueError("legacy time mapping")
        if len(set(self.legacy_pairs)) != len(self.legacy_pairs):
            raise ValueError("duplicate legacy pair")

    def identity(self) -> str:
        self.validate()
        body = {
            "legacy_pairs": self.legacy_pairs,
            "provenance_id": self.provenance_id,
            "quantity_kind": self.quantity_kind,
            "spectral": {
                "coordinate": self.spectral.coordinate,
                "intervals": [
                    [name, rational(interval.lower), rational(interval.upper)]
                    for name, interval in self.spectral.intervals
                ],
                "weighting": self.spectral.weighting,
            },
            "time": {
                "end_tick": self.time.end_tick,
                "origin_id": self.time.origin_id,
                "seconds_per_tick": rational(self.time.seconds_per_tick),
                "start_tick": self.time.start_tick,
                "time_basis_id": self.time.time_basis_id,
            },
            "unit": self.unit,
            "version": self.version,
        }
        encoded = json.dumps(body, sort_keys=True, separators=(",", ":")).encode()
        return hashlib.sha256(b"mindwarp.calibrated-band-time.oracle.v1\0" + encoded).hexdigest()


@dataclass(frozen=True)
class Applicability:
    calibration_id: str
    mode: str
    lower: F
    upper: F
    transport_profile_id: str
    spatial_calibration_id: str

    def validate(self, calibration: Calibration) -> None:
        if self.calibration_id != calibration.identity():
            raise ValueError("foreign calibration")
        if self.mode != "whole_cell_pointwise_enclosure":
            raise ValueError("insufficient applicability")
        if not self.transport_profile_id or not self.spatial_calibration_id:
            raise ValueError("transport calibration")
        if not (F(0) <= self.lower <= self.upper <= F(1)):
            raise ValueError("transfer interval")


def register(calibrations: tuple[Calibration, ...]) -> None:
    legacy: dict[tuple[str, str], str] = {}
    for calibration in calibrations:
        calibration_id = calibration.identity()
        for pair in calibration.legacy_pairs:
            previous = legacy.get(pair)
            if previous is not None and previous != calibration_id:
                raise ValueError("legacy alias ambiguity")
            legacy[pair] = calibration_id


def received_bound(
    calibration: Calibration,
    applicability: Applicability | None,
    allocations: tuple[F, ...],
) -> tuple[F, F]:
    calibration.validate()
    if applicability is None:
        raise ValueError("missing applicability")
    applicability.validate(calibration)
    if not allocations or any(value < 0 for value in allocations):
        raise ValueError("source allocation")
    total = sum(allocations, F(0))
    return total * applicability.lower, total * applicability.upper


def add_adjacent_time_energy(
    first: TimeCell, first_energy: F, second: TimeCell, second_energy: F
) -> F:
    first.validate()
    second.validate()
    if first.origin_id != second.origin_id or first.seconds_per_tick != second.seconds_per_tick:
        raise ValueError("time coordinate mismatch")
    if first.end_tick != second.start_tick:
        raise ValueError("time overlap or gap")
    if first_energy < 0 or second_energy < 0:
        raise ValueError("negative time-cell energy")
    return first_energy + second_energy


def rational(value: F) -> str:
    return f"{value.numerator}/{value.denominator}"


def serial(value: object) -> object:
    if isinstance(value, F):
        return rational(value)
    if isinstance(value, Interval):
        return [serial(value.lower), serial(value.upper)]
    if isinstance(value, tuple) or isinstance(value, list):
        return [serial(item) for item in value]
    if isinstance(value, dict):
        return {key: serial(item) for key, item in value.items()}
    return value


def rejected(action: Callable[[], object]) -> bool:
    try:
        action()
    except ValueError:
        return True
    return False


def make_calibration(
    *,
    spectral: SpectralBasis | None = None,
    time: TimeCell | None = None,
    version: int = 1,
    provenance_id: str = "calibration-provenance-a",
) -> Calibration:
    spectral = spectral or SpectralBasis(
        "vacuum_wavelength_metre",
        "unit_energy_integral",
        (
            ("blue", Interval(F(2, 5_000_000), F(1, 2_000_000))),
            ("green", Interval(F(1, 2_000_000), F(3, 5_000_000))),
            ("red", Interval(F(3, 5_000_000), F(7, 10_000_000))),
        ),
    )
    time = time or TimeCell("clock-origin-a", 100, 116, F(1, 1000), "legacy-time-a")
    return Calibration(
        "radiant_energy",
        "joule",
        version,
        provenance_id,
        spectral,
        time,
        tuple((band, time.time_basis_id) for band in BANDS),
    )


def main() -> None:
    calibration = make_calibration()
    calibration_id = calibration.identity()
    applicability = Applicability(
        calibration_id,
        "whole_cell_pointwise_enclosure",
        F(1, 4),
        F(3, 4),
        "transport-profile-a",
        "spatial-calibration-a",
    )
    applicability.validate(calibration)
    register((calibration, calibration))

    spectral_energies = (F(2, 5), F(7, 10), F(9, 10))
    covered_energy = sum(spectral_energies, F(0))
    assert covered_energy == F(2)

    time_a = calibration.time
    time_b = TimeCell(time_a.origin_id, 116, 132, time_a.seconds_per_tick, "legacy-time-b")
    time_a.validate()
    time_b.validate()
    adjacent_time_energy = add_adjacent_time_energy(time_a, F(5, 4), time_b, F(3, 4))
    assert adjacent_time_energy == F(2)

    allocations = (
        (F(2), F(0), F(0), F(0)),
        (F(0), F(2), F(0), F(0)),
        (F(0), F(0), F(2), F(0)),
        (F(0), F(0), F(0), F(2)),
        (F(1, 5), F(2, 5), F(3, 5), F(4, 5)),
    )
    bounds = tuple(received_bound(calibration, applicability, values) for values in allocations)
    assert all(bound == (F(1, 2), F(3, 2)) for bound in bounds)

    # Equal source energy with opposite spectral placement has different exact
    # received energy; an average or midpoint cannot identify either result.
    spectral_transfer = (F(1), F(0))
    source_short = (F(2), F(0))
    source_long = (F(0), F(2))
    received_short = sum((source_short[i] * spectral_transfer[i] for i in range(2)), F(0))
    received_long = sum((source_long[i] * spectral_transfer[i] for i in range(2)), F(0))
    asserted_average = F(1, 2) * F(2)
    assert sum(source_short, F(0)) == sum(source_long, F(0)) == 2
    assert received_short == 2 and received_long == 0 and asserted_average == 1

    temporal_transfer = (F(0), F(1))
    early = (F(2), F(0))
    late = (F(0), F(2))
    received_early = sum((early[i] * temporal_transfer[i] for i in range(2)), F(0))
    received_late = sum((late[i] * temporal_transfer[i] for i in range(2)), F(0))
    assert received_early == 0 and received_late == 2

    base_intervals = calibration.spectral.intervals
    overlap = SpectralBasis(
        calibration.spectral.coordinate,
        calibration.spectral.weighting,
        (base_intervals[0], base_intervals[1], ("red", Interval(F(11, 20_000_000), F(7, 10_000_000)))),
    )
    gap = SpectralBasis(
        calibration.spectral.coordinate,
        calibration.spectral.weighting,
        (base_intervals[0], base_intervals[1], ("red", Interval(F(13, 20_000_000), F(7, 10_000_000)))),
    )
    swapped = SpectralBasis(
        calibration.spectral.coordinate,
        calibration.spectral.weighting,
        (("red", base_intervals[2][1]), ("green", base_intervals[1][1]), ("blue", base_intervals[0][1])),
    )
    frequency = SpectralBasis("frequency_hertz", "unit_energy_integral", base_intervals)
    weighted = SpectralBasis("vacuum_wavelength_metre", "detector_response", base_intervals)
    different_time = TimeCell("clock-origin-a", 100, 117, F(1, 1000), "legacy-time-a")
    different_scale = TimeCell("clock-origin-a", 100, 116, F(1, 2000), "legacy-time-a")
    different_origin = TimeCell("clock-origin-b", 100, 116, F(1, 1000), "legacy-time-a")
    alias_interval = make_calibration(time=different_time)
    alias_scale = make_calibration(time=different_scale)
    alias_origin = make_calibration(time=different_origin)
    version_two = make_calibration(version=2)
    provenance_two = make_calibration(provenance_id="calibration-provenance-b")
    alternate_valid_spectral = SpectralBasis(
        "vacuum_wavelength_metre",
        "unit_energy_integral",
        (
            ("blue", Interval(F(2, 5_000_000), F(11, 20_000_000))),
            ("green", Interval(F(11, 20_000_000), F(13, 20_000_000))),
            ("red", Interval(F(13, 20_000_000), F(7, 10_000_000))),
        ),
    )
    alias_spectral = make_calibration(spectral=alternate_valid_spectral)

    hostile = (
        lambda: overlap.validate(),
        lambda: gap.validate(),
        lambda: swapped.validate(),
        lambda: frequency.validate(),
        lambda: weighted.validate(),
        lambda: SpectralBasis("vacuum_wavelength_metre", "unit_energy_integral", (("blue", Interval(F(1), F(1))),) + base_intervals[1:]).validate(),
        lambda: TimeCell("", 100, 116, F(1, 1000), "legacy-time-a").validate(),
        lambda: TimeCell("clock-origin-a", 116, 100, F(1, 1000), "legacy-time-a").validate(),
        lambda: TimeCell("clock-origin-a", 100, 116, F(0), "legacy-time-a").validate(),
        lambda: TimeCell("clock-origin-a", 100, 116, F(1, 1000), "").validate(),
        lambda: register((calibration, alias_interval)),
        lambda: register((calibration, alias_scale)),
        lambda: register((calibration, alias_origin)),
        lambda: register((calibration, alias_spectral)),
        lambda: register((calibration, version_two)),
        lambda: register((calibration, provenance_two)),
        lambda: add_adjacent_time_energy(time_a, F(1), TimeCell("clock-origin-a", 117, 132, F(1, 1000), "legacy-time-c"), F(1)),
        lambda: add_adjacent_time_energy(time_a, F(1), TimeCell("clock-origin-a", 115, 132, F(1, 1000), "legacy-time-c"), F(1)),
        lambda: add_adjacent_time_energy(time_a, F(1), TimeCell("clock-origin-b", 116, 132, F(1, 1000), "legacy-time-c"), F(1)),
        lambda: add_adjacent_time_energy(time_a, F(1), TimeCell("clock-origin-a", 116, 132, F(1, 2000), "legacy-time-c"), F(1)),
        lambda: received_bound(calibration, None, (F(1),)),
        lambda: Applicability(calibration_id, "midpoint_sample", F(1, 2), F(1, 2), "transport-profile-a", "spatial-calibration-a").validate(calibration),
        lambda: Applicability("foreign", "whole_cell_pointwise_enclosure", F(0), F(1), "transport-profile-a", "spatial-calibration-a").validate(calibration),
        lambda: Applicability(calibration_id, "whole_cell_pointwise_enclosure", F(3, 4), F(1, 4), "transport-profile-a", "spatial-calibration-a").validate(calibration),
        lambda: Applicability(calibration_id, "whole_cell_pointwise_enclosure", F(0), F(1), "", "spatial-calibration-a").validate(calibration),
        lambda: Applicability(calibration_id, "whole_cell_pointwise_enclosure", F(0), F(1), "transport-profile-a", "").validate(calibration),
        lambda: received_bound(calibration, applicability, (F(1), F(-1))),
        lambda: canonical_rational("-1/2"),
        lambda: canonical_rational("+1/2"),
        lambda: canonical_rational("01/2"),
        lambda: canonical_rational("1/02"),
        lambda: canonical_rational("2/4"),
        lambda: canonical_rational("0/2"),
        lambda: canonical_rational("1/0"),
    )
    hostile_rejections = sum(rejected(case) for case in hostile)
    assert hostile_rejections == len(hostile)

    # Two overlapping unit-response channels count the same joule twice.
    physical_energy = F(1)
    overlapping_channel_sum = physical_energy + physical_energy
    assert overlapping_channel_sum == 2 * physical_energy

    evidence = {
        "adjacent_time_energy": adjacent_time_energy,
        "allocation_bounds": bounds,
        "calibration_id": calibration_id,
        "covered_energy": covered_energy,
        "overlapping_channel_sum": overlapping_channel_sum,
        "spectral_counterexample": {
            "asserted_average": asserted_average,
            "received_long": received_long,
            "received_short": received_short,
        },
        "temporal_counterexample": {
            "received_early": received_early,
            "received_late": received_late,
        },
    }
    canonical = json.dumps(serial(evidence), sort_keys=True, separators=(",", ":"))
    receipt = {
        "additive_spectral_bands": 3,
        "allocation_portfolios": len(allocations),
        "calibration_witness": "survives_code_free_oracle",
        "checksum": hashlib.sha256(canonical.encode()).hexdigest(),
        "hostile_rejections": hostile_rejections,
        "legacy_v1": "unchanged_identity_only",
        "limitations": "no normative boundaries tick duration schema source transport calibration detector visibility runtime promotion or C3 closure",
        "pointwise_transport_applicability": "required",
        "scalar_average": "rejected_spectral_temporal_correlation",
        "status": "pass",
        "transport_physical_composition": "blocked_on_applicability_and_spatial_calibration",
        "weighting": "unit_energy_integral",
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
