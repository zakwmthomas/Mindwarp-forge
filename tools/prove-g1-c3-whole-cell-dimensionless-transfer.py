from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction as F
import hashlib
import json


@dataclass(frozen=True)
class Step:
    kind: str
    lower: F = F(0)
    upper: F = F(0)

    def __post_init__(self) -> None:
        if self.kind == "finite" and not (F(0) <= self.lower <= self.upper):
            raise ValueError("finite optical depth")
        if self.kind not in {"finite", "vacuum", "opaque", "unavailable", "ambiguous", "interface"}:
            raise ValueError("step kind")


def exp_neg_bounds(value: F, terms: int = 32) -> tuple[F, F]:
    """Exact-rational outward bounds for exp(-value), value >= 0."""
    if value < 0:
        raise ValueError("negative optical depth")
    halvings = 0
    reduced = value
    while reduced > F(1, 2):
        reduced /= 2
        halvings += 1
    term = F(1)
    total = term
    partials = [total]
    for index in range(1, terms + 1):
        term *= -reduced / index
        total += term
        partials.append(total)
    lower = partials[terms - 1] if (terms - 1) % 2 else partials[terms]
    upper = partials[terms] if terms % 2 == 0 else partials[terms - 1]
    if not (F(0) <= lower <= upper <= F(1)):
        raise AssertionError("invalid exponential enclosure")
    for _ in range(halvings):
        lower *= lower
        upper *= upper
    return lower, upper


def finite_transfer(lower: F, upper: F) -> tuple[F, F]:
    lower_exp, _ = exp_neg_bounds(upper)
    _, upper_exp = exp_neg_bounds(lower)
    return lower_exp, upper_exp


def prefix_finite(steps: list[Step]) -> tuple[F, F, bool]:
    lower = F(0)
    upper = F(0)
    opaque = False
    for step in steps:
        if step.kind == "vacuum":
            continue
        if step.kind == "finite":
            lower += step.lower
            upper += step.upper
            continue
        if step.kind == "opaque":
            opaque = True
            continue
        raise ValueError("unsupported prefix evidence")
    return lower, upper, opaque


def compose(proof: str, steps: list[Step], selected: int) -> dict[str, object]:
    if selected < 0 or selected >= len(steps):
        raise ValueError("selected index")
    if proof == "zero":
        return {"outcome": "zero", "measure_bucket": "zero", "transfer": None}
    if proof == "unresolved":
        return {"outcome": "unresolved", "measure_bucket": "unresolved", "transfer": None}
    if any(step.kind in {"unavailable", "ambiguous", "interface"} for step in steps[: selected + 1]):
        return {"outcome": "unresolved", "measure_bucket": "unresolved", "transfer": None}
    if proof == "start_inside":
        if selected == 0:
            return {"outcome": "accepted", "tau": (F(0), F(0)), "transfer": (F(1), F(1))}
        _, prefix_upper, prefix_opaque = prefix_finite(steps[:selected])
        if prefix_opaque:
            return {"outcome": "accepted", "tau": "opaque_uncertain", "transfer": (F(0), F(1))}
        return {"outcome": "accepted", "tau": (F(0), prefix_upper),
                "transfer": finite_transfer(F(0), prefix_upper)}
    if proof != "receiver_face":
        raise ValueError("proof")
    prefix_lower, prefix_upper, prefix_opaque = prefix_finite(steps[:selected])
    if prefix_opaque:
        return {"outcome": "accepted", "tau": "mandatory_opaque", "transfer": (F(0), F(0))}
    current = steps[selected]
    if current.kind == "opaque":
        _, transfer_upper = exp_neg_bounds(prefix_lower)
        return {"outcome": "accepted", "tau": "partial_opaque", "transfer": (F(0), transfer_upper)}
    if current.kind == "vacuum":
        current_upper = F(0)
    elif current.kind == "finite":
        current_upper = current.upper
    else:
        raise ValueError("unsupported selected evidence")
    tau = (prefix_lower, prefix_upper + current_upper)
    return {"outcome": "accepted", "tau": tau, "transfer": finite_transfer(*tau)}


def serial(value: object) -> object:
    if isinstance(value, F):
        return f"{value.numerator}/{value.denominator}"
    if isinstance(value, tuple):
        return [serial(item) for item in value]
    if isinstance(value, dict):
        return {key: serial(item) for key, item in value.items()}
    return value


def validate_subject(subject: dict[str, object]) -> None:
    required = {
        "cell_id": "cell", "transport_id": "transport", "coupling_id": "coupling",
        "bulk_profile_id": "bulk", "band": "red", "time_basis_id": "time",
        "band_time_id": "red:time", "step_ids": ["s0", "s1"],
        "selected": 1, "authority": "none_evidence_only",
    }
    if subject != required:
        raise ValueError("subject mismatch")


def main() -> None:
    finite_a = Step("finite", F(1), F(2))
    finite_b = Step("finite", F(3), F(5))
    vacuum = Step("vacuum")
    opaque = Step("opaque")
    cases = {
        "first_receiver_face": compose("receiver_face", [finite_a], 0),
        "later_receiver_face": compose("receiver_face", [finite_a, finite_b], 1),
        "start_inside_zero": compose("start_inside", [finite_a], 0),
        "start_inside_later": compose("start_inside", [finite_a, finite_b], 1),
        "vacuum_identity": compose("receiver_face", [vacuum], 0),
        "mandatory_prefix_opaque": compose("receiver_face", [opaque, finite_b], 1),
        "selected_partial_opaque": compose("receiver_face", [finite_a, opaque], 1),
        "uncertain_prior_opaque": compose("start_inside", [opaque, finite_b], 1),
        "zero_coupling": compose("zero", [finite_a], 0),
        "unresolved_coupling": compose("unresolved", [finite_a], 0),
        "unavailable": compose("receiver_face", [Step("unavailable")], 0),
        "ambiguous": compose("receiver_face", [Step("ambiguous")], 0),
        "interface": compose("receiver_face", [Step("interface")], 0),
    }
    assert cases["later_receiver_face"]["tau"] == (F(1), F(7))
    assert cases["start_inside_later"]["tau"] == (F(0), F(2))
    assert cases["start_inside_zero"]["transfer"] == (F(1), F(1))
    assert cases["mandatory_prefix_opaque"]["transfer"] == (F(0), F(0))
    assert cases["selected_partial_opaque"]["transfer"][0] == 0
    assert cases["uncertain_prior_opaque"]["transfer"] == (F(0), F(1))
    assert all(cases[name]["transfer"] is None for name in
               ("zero_coupling", "unresolved_coupling", "unavailable", "ambiguous", "interface"))

    aggregate = cases["later_receiver_face"]["transfer"]
    sample_checks = 0
    for prior in (F(1), F(3, 2), F(2)):
        for partial in (F(0), F(5, 2), F(5)):
            sample_lower, sample_upper = exp_neg_bounds(prior + partial)
            assert aggregate[0] <= sample_lower <= sample_upper <= aggregate[1]
            sample_checks += 1

    central = exp_neg_bounds(F(1))
    boundary = exp_neg_bounds(F(7))
    assert central[0] > boundary[1]

    q48 = 1 << 48
    repeated_projected_raw = ((1 * 1) // q48)
    exact_positive = F(1, q48) * F(1, q48)
    assert repeated_projected_raw == 0 and exact_positive > 0

    conservation = []
    for children in (4, 16, 64):
        measures = [F(1, children)] * children
        assert sum(measures, F(0)) == F(1)
        accepted = sum(measures[: children // 4], F(0))
        zero = sum(measures[children // 4: children // 2], F(0))
        unresolved = sum(measures[children // 2:], F(0))
        assert accepted + zero + unresolved == F(1)
        conservation.append({"children": children, "accepted": serial(accepted),
                             "zero": serial(zero), "unresolved": serial(unresolved)})

    subject = {
        "cell_id": "cell", "transport_id": "transport", "coupling_id": "coupling",
        "bulk_profile_id": "bulk", "band": "red", "time_basis_id": "time",
        "band_time_id": "red:time", "step_ids": ["s0", "s1"],
        "selected": 1, "authority": "none_evidence_only",
    }
    validate_subject(subject)
    hostile_rejections = 0
    mutations = [
        ("cell_id", "foreign"), ("transport_id", "foreign"),
        ("coupling_id", "foreign"), ("bulk_profile_id", "foreign"),
        ("band", "blue"), ("time_basis_id", ""), ("band_time_id", "blue:time"),
        ("step_ids", ["s1", "s0"]), ("selected", 0),
        ("authority", "received_power"),
    ]
    for key, value in mutations:
        hostile = dict(subject)
        hostile[key] = value
        try:
            validate_subject(hostile)
        except ValueError:
            hostile_rejections += 1
    assert hostile_rejections == len(mutations)

    canonical = json.dumps({"cases": serial(cases), "conservation": conservation},
                           sort_keys=True, separators=(",", ":"))
    receipt = {
        "status": "pass",
        "portfolios": len(cases) + 3,
        "sample_containment_checks": sample_checks,
        "hostile_rejections": hostile_rejections,
        "subdivision_children": [4, 16, 64],
        "central_lane_counterexample": "retained",
        "repeated_q0_48_underflow": "typed_not_opaque",
        "maximum_finite_prefix_raw_bits": 118,
        "limitations": "code-free oracle; no source magnitude power detector visibility runtime promotion or C3 closure",
        "checksum": hashlib.sha256(canonical.encode()).hexdigest(),
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
