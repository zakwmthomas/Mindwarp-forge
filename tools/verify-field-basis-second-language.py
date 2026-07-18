#!/usr/bin/env python3
"""Independent standard-library oracle for the committed field-basis v1 receipt."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

MASK32 = (1 << 32) - 1
ONE = 1 << 48
COORD_ONE = 1 << 32


def round_shift_even(value: int, shift: int) -> int:
    magnitude = abs(value)
    base, remainder = divmod(magnitude, 1 << shift)
    half = 1 << (shift - 1)
    rounded = base + int(remainder > half or (remainder == half and base & 1 == 1))
    result = -rounded if value < 0 else rounded
    if not -(1 << 63) <= result < (1 << 63):
        raise OverflowError("i64 result")
    return result


def mul_value(left: int, right: int) -> int:
    return round_shift_even(left * right, 48)


def philox4x32_10(counter: list[int], key: list[int]) -> list[int]:
    counter = list(counter)
    key = list(key)
    for _ in range(10):
        p0 = 0xD2511F53 * counter[0]
        p1 = 0xCD9E8D57 * counter[2]
        counter = [
            ((p1 >> 32) ^ counter[1] ^ key[0]) & MASK32,
            p1 & MASK32,
            ((p0 >> 32) ^ counter[3] ^ key[1]) & MASK32,
            p0 & MASK32,
        ]
        key[0] = (key[0] + 0x9E3779B9) & MASK32
        key[1] = (key[1] + 0xBB67AE85) & MASK32
    return counter


def zigzag(value: int) -> int:
    return ((value << 1) ^ (value >> 31)) & MASK32


def lattice(key: list[int], x: int, y: int, component: int) -> int:
    word = philox4x32_10([zigzag(x), zigzag(y), component, 0], key)[0]
    return (word - (1 << 31)) << 17


def fade(frac: int) -> int:
    t = frac << 16
    t2 = mul_value(t, t)
    t3 = mul_value(t2, t)
    inner = mul_value(t, mul_value(t, 6 * ONE) - 15 * ONE) + 10 * ONE
    return mul_value(t3, inner)


def lerp(left: int, right: int, amount: int) -> int:
    return left + mul_value(right - left, amount)


def sample(stream_key: bytes, x: int, y: int) -> int:
    key = [int.from_bytes(stream_key[0:4], "little"), int.from_bytes(stream_key[4:8], "little")]
    sx, sy = x * 2, y * 2
    cx, fx = divmod(sx, COORD_ONE)
    cy, fy = divmod(sy, COORD_ONE)
    tx, ty = fade(fx), fade(fy)
    a = lerp(lattice(key, cx, cy, 7), lattice(key, cx + 1, cy, 7), tx)
    b = lerp(lattice(key, cx, cy + 1, 7), lattice(key, cx + 1, cy + 1, 7), tx)
    value_lattice = mul_value(lerp(a, b, ty), ONE)
    return ONE - abs(value_lattice)


def cbor_uint(value: int) -> bytes:
    if value < 24:
        return bytes([value])
    if value <= 0xFF:
        return bytes([0x18, value])
    if value <= 0xFFFF:
        return bytes([0x19]) + value.to_bytes(2, "big")
    if value <= MASK32:
        return bytes([0x1A]) + value.to_bytes(4, "big")
    return bytes([0x1B]) + value.to_bytes(8, "big")


def cbor_array(length: int) -> bytes:
    encoded = cbor_uint(length)
    return bytes([encoded[0] | 0x80]) + encoded[1:]


def recipe_bytes() -> bytes:
    # [version, [[value-lattice, frequency, amplitude, component], [ridged, input]], output]
    return b"".join(
        [
            cbor_array(3),
            cbor_uint(1),
            cbor_array(2),
            cbor_array(4),
            cbor_uint(1),
            cbor_uint(2),
            cbor_uint(ONE),
            cbor_uint(7),
            cbor_array(2),
            cbor_uint(4),
            cbor_uint(0),
            cbor_uint(1),
        ]
    )


def main() -> int:
    fixture_path = Path(sys.argv[1]) if len(sys.argv) == 2 else Path(__file__).parents[1] / "crates" / "field-basis" / "fixtures" / "second-language-v1.json"
    fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
    assert fixture["schema_version"] == 1
    assert fixture["receipt_scope"] == "same-platform second-language"
    assert fixture["limitations"] == ["same Windows host", "not a second-platform receipt", "not reference_proven"]

    encoded = recipe_bytes()
    assert encoded.hex() == fixture["recipe_bytes_hex"]
    assert hashlib.sha256(b"mw-field-recipe-v1" + encoded).hexdigest() == fixture["recipe_fingerprint_hex"]
    assert hashlib.sha256(b"mw-field-cache-v1" + bytes([9]) * 32 + encoded + b"second-language-receipt").hexdigest() == fixture["cache_key_hex"]
    assert philox4x32_10([0, 0, 0, 0], [0, 0]) == fixture["philox_zero"]
    assert philox4x32_10([1, 4, 7, 0], [0x03030303, 0x03030303]) == fixture["philox_mapped"]

    for vector in fixture["samples"]:
        actual = sample(bytes([3]) * 32, vector["x_q32_32"], vector["y_q32_32"])
        assert actual == vector["value_q16_48"], (vector, actual)

    print("Field-basis second-language receipt verified: exact Philox, CBOR, hashes and Q32.32/Q16.48 samples; same Windows host only.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
