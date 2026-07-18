#!/usr/bin/env python3
"""Disposable symbolic width proof for origin-replayed receiver coupling."""

from __future__ import annotations

import hashlib
import json
import pathlib
import subprocess
import sys


STORAGE_BITS = 512
ORIGIN_BITS = 64
RECEIVER_Q160_NUMERATOR_BITS = 224
PHYSICAL_FACE_Q32_NUMERATOR_BITS = 96


def add_bits(*values: int) -> int:
    assert values
    return max(values) + (len(values) - 1).bit_length()


def mul_bits(*values: int) -> int:
    return sum(values)


def affine_product_bits(
    left_center: int, left_coefficient: int, right_center: int, right_coefficient: int
) -> tuple[int, int, int]:
    constant = mul_bits(left_center, right_center)
    linear = add_bits(
        mul_bits(left_center, right_coefficient),
        mul_bits(left_coefficient, right_center),
    )
    quadratic = add_bits(
        mul_bits(left_coefficient, right_coefficient),
        mul_bits(left_coefficient, right_coefficient),
    )
    return constant, linear, quadratic


def shift(poly: tuple[int, int, int], bits: int) -> tuple[int, int, int]:
    return tuple(value + bits for value in poly)


def combine(*polys: tuple[int, int, int]) -> tuple[int, int, int]:
    return tuple(add_bits(*(poly[index] for poly in polys)) for index in range(3))


def polynomial_bound_bits(poly: tuple[int, int, int]) -> int:
    # One constant, four linear monomials and sixteen ordered quadratic terms.
    return add_bits(poly[0], poly[1] + 2, poly[2] + 4)


def main() -> None:
    root = pathlib.Path(__file__).resolve().parent
    classifier = root / "prove-g1-c3-whole-cell-receiver-coupling.py"
    prior = json.loads(subprocess.check_output([sys.executable, str(classifier)], text=True))
    assert prior["status"] == "pass"
    assert prior["checks"] == 1020
    assert prior["positive_cases"] == 12
    assert prior["hostile_cases"] == 7
    assert prior["hostile_rejections"] == 3
    assert prior["subdivision_children"] == [4, 16, 64]
    assert prior["checksum"] == "8c9c2c6d5f5d6ab38483d1e7c769b833d5d0373378cacbf86ff59ccfba4a91aa"

    # Receiver plane numerator: receiver_q160 * D - point * 2^160.
    receiver_plane = (
        add_bits(
            mul_bits(RECEIVER_Q160_NUMERATOR_BITS, ORIGIN_BITS),
            ORIGIN_BITS + 160,
        ),
        ORIGIN_BITS + 160,
    )
    # Physical face numerator: face_q32 * D - point * 2^32.
    physical_plane = (
        add_bits(
            mul_bits(PHYSICAL_FACE_Q32_NUMERATOR_BITS, ORIGIN_BITS),
            ORIGIN_BITS + 32,
        ),
        ORIGIN_BITS + 32,
    )

    receiver_times_direction = affine_product_bits(
        receiver_plane[0], receiver_plane[1], ORIGIN_BITS, ORIGIN_BITS
    )
    face_times_direction = affine_product_bits(
        physical_plane[0], physical_plane[1], ORIGIN_BITS, ORIGIN_BITS
    )

    # q_receiver < q_face after preserving the Q160 and Q32 scale factors.
    ordered = combine(
        shift(receiver_times_direction, 32),
        shift(face_times_direction, 160),
    )
    ordered_bound = polynomial_bound_bits(ordered)

    # Cross-axis interior at receiver hit: two receiver-plane/direction products.
    cross_axis = combine(receiver_times_direction, receiver_times_direction)
    cross_axis_bound = polynomial_bound_bits(cross_axis)

    # Public reduced transport terms may independently consume the upstream
    # 490-bit shield. Even one product is therefore outside Signed512.
    public_form_product_bits = mul_bits(490, 490)
    assert public_form_product_bits == 980
    assert public_form_product_bits > STORAGE_BITS

    maximum_live_bits = max(
        ordered_bound,
        cross_axis_bound,
        receiver_plane[0],
        physical_plane[0],
    )
    assert receiver_plane == (289, 224)
    assert physical_plane == (161, 96)
    assert ordered_bound == 391
    assert cross_axis_bound == 359
    assert maximum_live_bits == 391
    assert maximum_live_bits < STORAGE_BITS

    evidence = {
        "classifier_checksum": prior["checksum"],
        "classifier_checks": prior["checks"],
        "hostile_non_full": prior["hostile_cases"],
        "invalid_receiver_rejections": prior["hostile_rejections"],
        "subdivision_children": prior["subdivision_children"],
        "origin_scalar_bits": ORIGIN_BITS,
        "receiver_q160_numerator_bits": RECEIVER_Q160_NUMERATOR_BITS,
        "physical_face_q32_numerator_bits": PHYSICAL_FACE_Q32_NUMERATOR_BITS,
        "receiver_plane_center_bits": receiver_plane[0],
        "receiver_plane_coefficient_bits": receiver_plane[1],
        "physical_plane_center_bits": physical_plane[0],
        "physical_plane_coefficient_bits": physical_plane[1],
        "receiver_face_order_polynomial_bound_bits": ordered_bound,
        "receiver_cross_axis_polynomial_bound_bits": cross_axis_bound,
        "maximum_live_bits": maximum_live_bits,
        "storage_bits": STORAGE_BITS,
        "storage_margin_bits": STORAGE_BITS - maximum_live_bits,
        "public_reduced_form_product_bits": public_form_product_bits,
        "selected_representation": "immutable_origin_shared_symbol_receiver_and_face_plane_numerators",
        "limitations": "symbolic no-favourable-cancellation width proof; no production authority",
    }
    canonical = json.dumps(evidence, sort_keys=True, separators=(",", ":"))
    receipt = {
        "status": "pass",
        **evidence,
        "checksum": hashlib.sha256(canonical.encode()).hexdigest(),
    }
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
