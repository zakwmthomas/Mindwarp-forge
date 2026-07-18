"""Disposable arbitrary-precision proof for the G1 C3 bulk-radiance design.

This is a proof harness, not a consumer implementation or coefficient source.
It compares directed fixed-point candidates with Fraction/Decimal oracles.
"""

from __future__ import annotations

from decimal import Decimal, localcontext
from fractions import Fraction
from math import gcd, isqrt
from random import Random
from time import perf_counter

Q32 = 1 << 32
Q48 = 1 << 48
Q64 = 1 << 64
U64_MAX = (1 << 64) - 1


def floor_fraction(value: Fraction) -> int:
    return value.numerator // value.denominator


def ceil_fraction(value: Fraction) -> int:
    return -((-value.numerator) // value.denominator)


def floor_mul_div(left: int, right: int, denominator: int) -> int:
    return (left * right) // denominator


def ceil_mul_div(left: int, right: int, denominator: int) -> int:
    return -((-(left * right)) // denominator)


def exp_base_q64(y_q64: int) -> tuple[int, int]:
    """Enclose exp(-y) at Q0.64 for exact y=y_q64/Q64 in [0,1]."""
    assert 0 <= y_q64 <= Q64
    term_lo = term_hi = Q64
    sum_lo = sum_hi = Q64
    for n in range(1, 192):
        denominator = Q64 * n
        term_lo = floor_mul_div(term_lo, y_q64, denominator)
        term_hi = ceil_mul_div(term_hi, y_q64, denominator)
        if n & 1:
            sum_lo -= term_hi
            sum_hi -= term_lo
        else:
            sum_lo += term_lo
            sum_hi += term_hi

        next_lo = floor_mul_div(term_lo, y_q64, Q64 * (n + 1))
        next_hi = ceil_mul_div(term_hi, y_q64, Q64 * (n + 1))
        if n & 1:  # next term is positive
            bound_lo, bound_hi = sum_lo, sum_hi + next_hi
        else:  # next term is negative
            bound_lo, bound_hi = sum_lo - next_hi, sum_hi
        if next_hi <= 1:
            return max(0, bound_lo), min(Q64, bound_hi)
    raise AssertionError("base exponential series did not converge")


def exp_neg_q64_bounds(value: Fraction) -> tuple[int, int]:
    """Directed Q0.64 enclosure of exp(-value) for nonnegative value."""
    assert value >= 0
    reductions = 0
    reduced = value
    while reduced > 1:
        reduced /= 2
        reductions += 1

    y_lo = floor_fraction(reduced * Q64)
    y_hi = ceil_fraction(reduced * Q64)
    # exp is decreasing: the larger directed input supplies the lower bound.
    lower, _ = exp_base_q64(y_hi)
    _, upper = exp_base_q64(y_lo)
    for _ in range(reductions):
        lower = floor_mul_div(lower, lower, Q64)
        upper = ceil_mul_div(upper, upper, Q64)
    return lower, upper


def decimal_exp_neg(value: Fraction) -> Decimal:
    with localcontext() as context:
        context.prec = 120
        decimal_value = Decimal(value.numerator) / Decimal(value.denominator)
        return (-decimal_value).exp()


def assert_exp_enclosure(value: Fraction) -> int:
    lower, upper = exp_neg_q64_bounds(value)
    truth = decimal_exp_neg(value)
    with localcontext() as context:
        context.prec = 120
        scaled = truth * Decimal(Q64)
        assert Decimal(lower) <= scaled <= Decimal(upper), (value, lower, scaled, upper)
    return ceil_mul_div(upper, Q48, Q64) - floor_mul_div(lower, Q48, Q64)


def length_raw_bounds(delta: tuple[int, int, int]) -> tuple[int, int, int]:
    squared = sum(component * component for component in delta)
    lower = isqrt(squared)
    upper = lower if lower * lower == squared else lower + 1
    return lower, upper, squared.bit_length()


def merge_spans(
    spans: list[tuple[Fraction, Fraction, tuple[int, int, int]]]
) -> list[tuple[Fraction, Fraction, tuple[int, int, int]]]:
    merged: list[tuple[Fraction, Fraction, tuple[int, int, int]]] = []
    for start, end, coefficients in spans:
        assert 0 <= start < end <= 1
        if merged and merged[-1][1] == start and merged[-1][2] == coefficients:
            merged[-1] = (merged[-1][0], end, coefficients)
        else:
            merged.append((start, end, coefficients))
    return merged


def optical_depth_bounds(
    delta: tuple[int, int, int],
    spans: list[tuple[Fraction, Fraction, tuple[int, int, int]]],
    band: int,
) -> tuple[Fraction, Fraction, int]:
    merged = merge_spans(spans)
    weighted_parameter = sum(
        (end - start) * Fraction(coefficients[band], Q48)
        for start, end, coefficients in merged
    )
    length_lo, length_hi, squared_bits = length_raw_bounds(delta)
    return (
        Fraction(length_lo, Q32) * weighted_parameter,
        Fraction(length_hi, Q32) * weighted_parameter,
        squared_bits,
    )


def transfer_q48_bounds(
    delta: tuple[int, int, int],
    spans: list[tuple[Fraction, Fraction, tuple[int, int, int]]],
    band: int,
) -> tuple[int, int]:
    tau_lo, tau_hi, _ = optical_depth_bounds(delta, spans, band)
    transmission_lo, _ = exp_neg_q64_bounds(tau_hi)
    _, transmission_hi = exp_neg_q64_bounds(tau_lo)
    return (
        floor_mul_div(transmission_lo, Q48, Q64),
        ceil_mul_div(transmission_hi, Q48, Q64),
    )


def assert_transfer_enclosure(
    delta: tuple[int, int, int],
    spans: list[tuple[Fraction, Fraction, tuple[int, int, int]]],
    band: int,
) -> int:
    tau_lo, tau_hi, _ = optical_depth_bounds(delta, spans, band)
    lower, upper = transfer_q48_bounds(delta, spans, band)
    weighted = sum(
        (end - start) * Fraction(coefficients[band], Q48)
        for start, end, coefficients in merge_spans(spans)
    )
    squared = sum(component * component for component in delta)
    with localcontext() as context:
        context.prec = 120
        true_length = Decimal(squared).sqrt() / Decimal(Q32)
        true_weighted = Decimal(weighted.numerator) / Decimal(weighted.denominator)
        true_tau = true_length * true_weighted
        true_transmission = (-true_tau).exp() * Decimal(Q48)
        assert Decimal(tau_lo.numerator) / Decimal(tau_lo.denominator) <= true_tau
        assert true_tau <= Decimal(tau_hi.numerator) / Decimal(tau_hi.denominator)
        assert Decimal(lower) <= true_transmission <= Decimal(upper)
    return upper - lower


def classify_open_path(
    intervals: list[tuple[Fraction, Fraction, str]],
    *,
    stationary_unavailable: bool = False,
) -> str:
    if stationary_unavailable:
        return "unavailable"
    if not intervals:
        return "identity"
    breakpoints = sorted({point for start, end, _ in intervals for point in (start, end)})
    sequence: list[str] = []
    for start, end in zip(breakpoints, breakpoints[1:]):
        if start == end:
            continue
        active = [substance for left, right, substance in intervals if left <= start and end <= right]
        if "unavailable" in active:
            return "unavailable"
        if len(active) != 1:
            return "ambiguous_boundary_lane"
        if not sequence or sequence[-1] != active[0]:
            sequence.append(active[0])
    if any(left != right for left, right in zip(sequence, sequence[1:])):
        return "interface_model_required"
    return "bulk_transfer"


def lcm(left: int, right: int) -> int:
    return left // gcd(left, right) * right


def main() -> None:
    started = perf_counter()
    checks = 0
    worst_q48_width = 0

    fixed_exp = [
        Fraction(0),
        Fraction(1, Q64),
        Fraction(1, 10),
        Fraction(1),
        Fraction(7, 3),
        Fraction(20),
        Fraction(1 << 16),
        Fraction((1 << 48) - 1, Q48),
    ]
    for value in fixed_exp:
        worst_q48_width = max(worst_q48_width, assert_exp_enclosure(value))
        checks += 1

    rng = Random(0xF0A63C3)
    for _ in range(512):
        numerator = rng.randrange(0, 1 << 70)
        denominator = rng.randrange(1, 1 << 40)
        value = Fraction(numerator, denominator)
        worst_q48_width = max(worst_q48_width, assert_exp_enclosure(value))
        checks += 1

    for _ in range(256):
        delta = tuple(rng.randrange(-(1 << 56), 1 << 56) for _ in range(3))
        if delta == (0, 0, 0):
            delta = (1, 0, 0)
        cuts = sorted({Fraction(0), Fraction(1), *(Fraction(rng.randrange(1, 1024), 1024) for _ in range(5))})
        coefficients = tuple(rng.randrange(0, Q48) for _ in range(3))
        random_spans = [(left, right, coefficients) for left, right in zip(cuts, cuts[1:])]
        for band in range(3):
            worst_q48_width = max(
                worst_q48_width,
                assert_transfer_enclosure(delta, random_spans, band),
            )
            checks += 1
        # Same-substance subdivision is erased before every rounding operation.
        assert merge_spans(random_spans) == [(Fraction(0), Fraction(1), coefficients)]
        checks += 1

    perfect = length_raw_bounds((3 * Q32, 4 * Q32, 0))
    assert perfect[:2] == (5 * Q32, 5 * Q32)
    adjacent = length_raw_bounds((Q32, Q32, Q32))
    assert adjacent[1] == adjacent[0] + 1
    maximum = length_raw_bounds((U64_MAX, U64_MAX, U64_MAX))
    assert maximum[2] == 130
    checks += 3

    coefficients_a = (Q48 // 8, Q48 // 4, Q48 // 2)
    coefficients_b = (Q48 // 3, Q48 // 5, Q48 // 7)
    whole = [(Fraction(0), Fraction(1), coefficients_a)]
    subdivided = [
        (Fraction(0), Fraction(1, 7), coefficients_a),
        (Fraction(1, 7), Fraction(4, 9), coefficients_a),
        (Fraction(4, 9), Fraction(1), coefficients_a),
    ]
    assert merge_spans(subdivided) == whole
    for band in range(3):
        assert optical_depth_bounds((5 * Q32, 7 * Q32, 11 * Q32), whole, band) == optical_depth_bounds(
            (5 * Q32, 7 * Q32, 11 * Q32), subdivided, band
        )
        assert transfer_q48_bounds((5 * Q32, 7 * Q32, 11 * Q32), whole, band) == transfer_q48_bounds(
            (5 * Q32, 7 * Q32, 11 * Q32), subdivided, band
        )
        checks += 2

    layered = [
        (Fraction(0), Fraction(1, 5), coefficients_a),
        (Fraction(1, 5), Fraction(3, 4), coefficients_b),
        (Fraction(3, 4), Fraction(1), coefficients_a),
    ]
    reversed_layered = [
        (Fraction(0), Fraction(1, 4), coefficients_a),
        (Fraction(1, 4), Fraction(4, 5), coefficients_b),
        (Fraction(4, 5), Fraction(1), coefficients_a),
    ]
    for band in range(3):
        assert optical_depth_bounds((9 * Q32, -4 * Q32, Q32), layered, band) == optical_depth_bounds(
            (-9 * Q32, 4 * Q32, -Q32), reversed_layered, band
        )
        checks += 1

    vacuum = (0, 0, 0)
    assert transfer_q48_bounds((Q32, 0, 0), [(Fraction(0), Fraction(1), vacuum)], 0) == (Q48, Q48)
    thin = [(Fraction(0), Fraction(1, 1 << 20), coefficients_a)]
    tangent: list[tuple[Fraction, Fraction, tuple[int, int, int]]] = []
    assert transfer_q48_bounds((Q32, 0, 0), tangent, 0) == (Q48, Q48)
    assert transfer_q48_bounds((Q32, 0, 0), thin, 0)[0] < Q48
    checks += 3

    assert classify_open_path([]) == "identity"
    assert classify_open_path([], stationary_unavailable=True) == "unavailable"
    assert classify_open_path([(Fraction(0), Fraction(1), "unavailable")]) == "unavailable"
    assert classify_open_path(
        [(Fraction(0), Fraction(1), "air"), (Fraction(0), Fraction(1), "glass")]
    ) == "ambiguous_boundary_lane"
    assert classify_open_path(
        [(Fraction(0), Fraction(1, 2), "air"), (Fraction(1, 2), Fraction(1), "glass")]
    ) == "interface_model_required"
    assert classify_open_path([(Fraction(0), Fraction(1), "air")]) == "bulk_transfer"
    checks += 6

    short = transfer_q48_bounds((Q32, 0, 0), whole, 0)
    long = transfer_q48_bounds((2 * Q32, 0, 0), whole, 0)
    stronger = transfer_q48_bounds((Q32, 0, 0), [(Fraction(0), Fraction(1), coefficients_b)], 0)
    assert long[1] <= short[1]
    assert stronger[1] <= short[1]
    checks += 2

    denominators = (U64_MAX, U64_MAX - 2, U64_MAX - 4)
    common_denominator_bits = lcm(lcm(*denominators[:2]), denominators[2]).bit_length()
    accumulator_numerator_bits = 64 + common_denominator_bits + 16
    length_product_bits = accumulator_numerator_bits + 65
    assert common_denominator_bits <= 192
    assert length_product_bits <= 337
    assert length_product_bits < 384
    checks += 3

    elapsed_ms = round((perf_counter() - started) * 1000)
    print("visible-radiance bulk-transfer oracle: PASS")
    print(f"checks={checks}")
    print(f"random_exp_enclosures=512")
    print(f"random_transfer_enclosures=768")
    print(f"random_subdivision_invariance=256")
    print(f"worst_transmission_bound_width_q0_48_units={worst_q48_width}")
    print(f"maximum_squared_length_bits={maximum[2]}")
    print(f"worst_common_denominator_bits={common_denominator_bits}")
    print(f"conservative_accumulator_product_bits={length_product_bits}")
    print(f"elapsed_ms={elapsed_ms}")


if __name__ == "__main__":
    main()
