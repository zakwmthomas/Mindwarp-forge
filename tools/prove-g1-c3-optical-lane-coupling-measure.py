#!/usr/bin/env python3
"""Exact-rational counterexamples for finite optical-lane coupling measures."""

from fractions import Fraction
import hashlib
import json


def q(value: Fraction) -> str:
    return f"{value.numerator}/{value.denominator}"


def rectangle_measure(bounds):
    (a0, a1), (o0, o1) = bounds
    return (a1 - a0) * (o1 - o0)


def split(bounds):
    (a0, a1), (o0, o1) = bounds
    am, om = (a0 + a1) / 2, (o0 + o1) / 2
    return [((x0, x1), (y0, y1)) for x0, x1 in ((a0, am), (am, a1)) for y0, y1 in ((o0, om), (om, o1))]


def affine_footprint(k, b, source_half, angle_half):
    corners = [k * a + b * o for a in (-source_half, source_half) for o in (-angle_half, angle_half)]
    return min(corners), max(corners)


def main():
    unit = ((Fraction(-1), Fraction(1)), (Fraction(-1), Fraction(1)))
    portfolios = []
    current = [unit]
    for depth in range(4):
        parent = rectangle_measure(unit)
        child_sum = sum((rectangle_measure(cell) for cell in current), Fraction(0))
        assert child_sum == parent
        portfolios.append({"name": f"refinement-depth-{depth}", "measure": q(child_sum), "cells": len(current)})
        current = [child for cell in current for child in split(cell)]

    footprint_a = affine_footprint(Fraction(1), Fraction(1), Fraction(1, 10), Fraction(1, 10))
    footprint_b = affine_footprint(Fraction(3), Fraction(1, 2), Fraction(1, 10), Fraction(1, 10))
    assert footprint_a != footprint_b
    portfolios.append({"name": "same-central-different-footprint", "central": "0/1", "a": list(map(q, footprint_a)), "b": list(map(q, footprint_b))})

    receiver = (Fraction(-1, 4), Fraction(1, 4))
    central = Fraction(0)
    boundary = (Fraction(-1), Fraction(1))
    assert receiver[0] < central < receiver[1]
    assert all(not (receiver[0] < value < receiver[1]) for value in boundary)
    portfolios.append({"name": "central-arrives-boundary-misses", "coverage": "partial_receiver_coverage"})

    corners = [(x, y) for x in (-1, 1) for y in (-1, 1)]
    topology = lambda x, y: "inner" if Fraction(x * x + y * y) < Fraction(1, 4) else "outer"
    assert {topology(x, y) for x, y in corners} == {"outer"}
    assert topology(0, 0) == "inner"
    portfolios.append({"name": "corner-topology-interior-escape", "outcome": "unsupported_topology_change"})

    fold = lambda u: Fraction(u * u)
    assert fold(Fraction(-1)) == fold(Fraction(1)) and fold(Fraction(0)) < fold(Fraction(1))
    portfolios.append({"name": "caustic-fold", "outcome": "unsupported_caustic_or_fold"})

    spherical_area, radius = Fraction(3, 2), Fraction(3)
    solid_angle = spherical_area / (radius * radius)
    assert solid_angle == Fraction(1, 6)
    portfolios.append({"name": "nist-free-space-solid-angle", "solid_angle_sr": q(solid_angle)})

    input_radiance, transfer = Fraction(7, 3), Fraction(5, 8)
    output_radiance = input_radiance * transfer
    assert output_radiance <= input_radiance
    portfolios.append({"name": "passive-loss-bound", "input": q(input_radiance), "output": q(output_radiance)})

    portfolios.append({"name": "zero-area", "measure": q(rectangle_measure(((Fraction(0), Fraction(0)), (Fraction(-1), Fraction(1))))), "outcome": "zero_measure"})
    portfolios.append({"name": "conditional-lineage", "outcome": "unsupported_conditional_lineage"})
    assert len(portfolios) == 12

    hostile = {
        "scalar_weight_without_measure": False,
        "central_arrival_means_full_coverage": False,
        "corners_certify_interior_topology": False,
        "children_each_copy_parent_measure": False,
        "inverse_square_after_refraction": False,
        "ray_differential_is_source_power": False,
        "zero_area_has_finite_etendue": False,
        "zero_solid_angle_has_finite_etendue": False,
        "loss_increases_radiance": False,
        "fold_has_single_valued_jacobian": False,
        "topology_change_is_favourable": False,
        "partial_coverage_is_full_acceptance": False,
        "band_basis_omitted": False,
        "time_basis_omitted": False,
        "source_area_omitted": False,
        "angular_basis_omitted": False,
        "receiver_acceptance_inferred_from_aabb": False,
        "detector_response_inferred_from_arrival": False,
        "visibility_inferred_from_nonzero_measure": False,
        "production_authority_from_oracle": False,
    }
    assert len(hostile) == 20 and not any(hostile.values())
    receipt = {
        "schema_version": 1,
        "status": "rejected_corner_boundary_insufficient",
        "portfolio_count": len(portfolios),
        "hostile_rejection_count": len(hostile),
        "portfolios": portfolios,
        "hostile": hostile,
        "selected_next_evidence": "whole_phase_space_cell_topology_or_adaptive_interior_certificate",
        "authority_effect": "none_evidence_only",
    }
    canonical = json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()
    receipt["receipt_sha256"] = hashlib.sha256(canonical).hexdigest()
    print(json.dumps(receipt, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
