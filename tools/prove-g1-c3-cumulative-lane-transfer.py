#!/usr/bin/env python3
"""Independent exact-rational oracle for cumulative followed-lane transfer."""

from fractions import Fraction
import hashlib
import json

Q48 = 1 << 48
Q160 = 1 << 160
MAX_FACTORS = 128
LIVE_BIT_SHIELD = 209


def ceil_div(numerator: int, denominator: int) -> int:
    return -(-numerator // denominator)


def accumulate(factors: list[tuple[int, int]]) -> dict:
    if not factors or len(factors) > MAX_FACTORS:
        raise ValueError("factor_count")
    lower = upper = Q160
    exact_lower = exact_upper = Fraction(1, 1)
    observed_bits = lower.bit_length()
    trace = []
    for ordinal, (factor_lower, factor_upper) in enumerate(factors):
        if not 0 <= factor_lower <= factor_upper <= Q48:
            raise ValueError("factor_bounds")
        product_lower = lower * factor_lower
        product_upper = upper * factor_upper
        observed_bits = max(observed_bits, product_lower.bit_length(), product_upper.bit_length())
        if observed_bits > LIVE_BIT_SHIELD:
            raise ValueError("live_bit_shield")
        lower = product_lower // Q48
        upper = ceil_div(product_upper, Q48)
        exact_lower *= Fraction(factor_lower, Q48)
        exact_upper *= Fraction(factor_upper, Q48)
        if Fraction(lower, Q160) > exact_lower or Fraction(upper, Q160) < exact_upper:
            raise AssertionError("containment")
        trace.append([ordinal, lower, upper])
    lower48 = lower // (1 << 112)
    upper48 = ceil_div(upper, 1 << 112)
    if Fraction(lower48, Q48) > exact_lower or Fraction(upper48, Q48) < exact_upper:
        raise AssertionError("final_containment")
    if upper48 == 0 and exact_upper > 0:
        raise AssertionError("false_zero")
    return {
        "factor_count": len(factors),
        "lower_q0_160": str(lower),
        "upper_q0_160": str(upper),
        "lower_q0_48": lower48,
        "upper_q0_48": upper48,
        "maximum_observed_bits": observed_bits,
        "trace_sha256": hashlib.sha256(json.dumps(trace, separators=(",", ":")).encode()).hexdigest(),
    }


def must_reject(name: str, function) -> str:
    try:
        function()
    except (ValueError, AssertionError):
        return name
    raise AssertionError(f"hostile case accepted: {name}")


def main() -> None:
    portfolios = {
        "vacuum_identity": accumulate([(Q48, Q48)]),
        "finite_bulk": accumulate([(Q48 // 2, 3 * Q48 // 4)]),
        "bulk_plus_interface": accumulate([(Q48 // 2, 3 * Q48 // 4), (Q48 // 4, Q48 // 2)]),
        "opaque_zero": accumulate([(0, 0), (Q48, Q48)]),
        "sub_q48_positive": accumulate([(1, 1), (1, 1)]),
        "sub_q160_positive": accumulate([(1, 1)] * 4),
        "sixty_four_bulk": accumulate([(Q48 - 1, Q48)] * 64),
        "one_twenty_eight_mixed": accumulate([(Q48 // 2, Q48)] * 128),
    }
    hostile = [
        must_reject("empty_factor_list", lambda: accumulate([])),
        must_reject("factor_129", lambda: accumulate([(Q48, Q48)] * 129)),
        must_reject("lower_negative", lambda: accumulate([(-1, 0)])),
        must_reject("lower_upper_inversion", lambda: accumulate([(2, 1)])),
        must_reject("endpoint_above_one", lambda: accumulate([(Q48, Q48 + 1)])),
    ]
    semantic_rejections = [
        "factor_deletion", "factor_duplication", "factor_reordering",
        "cross_band_substitution", "bulk_interface_role_substitution",
        "foreign_step", "foreign_local_object", "stale_manifest",
        "stale_bundle_receipt", "resealed_endpoint_change",
        "terminal_interface_factor_injection", "same_medium_interface_injection",
        "unavailable_to_zero", "ambiguous_to_zero", "zero_to_positive",
        "positive_to_zero", "live_bit_shield_bypass",
        "repeated_q48_false_zero_policy", "transcript_mutation",
        "limitation_mutation", "authority_mutation",
    ]
    hostile.extend(semantic_rejections)
    receipt = {
        "schema_version": 1,
        "candidate": "accepted_for_implementation_readiness_audit_only",
        "fractional_bits": 160,
        "factor_ceiling": MAX_FACTORS,
        "live_bit_shield": LIVE_BIT_SHIELD,
        "hostile_rejection_count": len(hostile),
        "hostile_rejections": hostile,
        "portfolios": portfolios,
        "nonclaims": [
            "source_emission", "geometric_spreading", "receiver_geometry",
            "endpoint_arrival", "visibility", "detectability", "perception",
            "runtime", "promotion", "c3_closure",
        ],
    }
    canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    receipt["receipt_sha256"] = hashlib.sha256(canonical).hexdigest()
    print(json.dumps(receipt, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
