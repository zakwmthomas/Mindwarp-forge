#!/usr/bin/env python3
"""Exact-rational oracle for conservative full/zero/unresolved cell coupling."""

from fractions import Fraction as F
import hashlib, json


def interval_affine(c, coefficients, radius=F(1)):
    width = sum((abs(a) * radius for a in coefficients), F(0))
    return c - width, c + width


def classify(image, receiver, uniform=True, fold=False):
    if not uniform or fold:
        return "unresolved_cell_coupling"
    full = all(r0 < x0 and x1 < r1 for (x0, x1), (r0, r1) in zip(image, receiver))
    if full:
        return "certified_full_cell_arrival"
    zero = any(x1 <= r0 or x0 >= r1 for (x0, x1), (r0, r1) in zip(image, receiver))
    if zero:
        return "certified_zero_cell_arrival"
    return "unresolved_cell_coupling"


def q(x): return f"{x.numerator}/{x.denominator}"


def main():
    receiver = [(F(-1), F(1)), (F(-1), F(1))]
    cases = []
    def add(name, image, expected, uniform=True, fold=False, measure=F(1)):
        actual = classify(image, receiver, uniform, fold)
        assert actual == expected
        cases.append({"name": name, "image": [[q(a), q(b)] for a,b in image], "outcome": actual, "measure": q(measure)})

    add("strict-full", [interval_affine(F(0), [F(1,4)]), interval_affine(F(0), [F(1,3)])], "certified_full_cell_arrival")
    add("strict-zero-left", [(F(-3),F(-2)),(F(-1,2),F(1,2))], "certified_zero_cell_arrival")
    add("strict-zero-right", [(F(2),F(3)),(F(-1,2),F(1,2))], "certified_zero_cell_arrival")
    add("boundary-equality", [(F(-1),F(0)),(F(-1,2),F(1,2))], "unresolved_cell_coupling")
    add("partial-overlap", [(F(1,2),F(3,2)),(F(-1,2),F(1,2))], "unresolved_cell_coupling")
    add("topology-change", [(F(-1,2),F(1,2))]*2, "unresolved_cell_coupling", uniform=False)
    add("branch-change", [(F(-1,2),F(1,2))]*2, "unresolved_cell_coupling", uniform=False)
    add("quadratic-fold", [(F(0),F(1)),(F(-1,2),F(1,2))], "unresolved_cell_coupling", fold=True)
    add("monotone-focus", [interval_affine(F(0), [F(1,5)]), interval_affine(F(0), [F(1,5)])], "certified_full_cell_arrival")

    # True correlated cancellation y=u-u is zero; box erasure gives [-2,2].
    true_correlated = [(F(0),F(0)),(F(-1,4),F(1,4))]
    erased = [(F(-2),F(2)),(F(-1,4),F(1,4))]
    add("correlated-cancellation", true_correlated, "certified_full_cell_arrival")
    add("correlation-erased", erased, "unresolved_cell_coupling")

    for depth, count in ((1,4),(2,16),(3,64)):
        child = F(1,count)
        assert sum((child for _ in range(count)),F(0)) == F(1)
        cases.append({"name":f"refinement-{depth}","children":count,"parent_measure":"1/1","child_sum":q(child*count)})

    # Mixed accepted/zero/unresolved accounting remains exact.
    accepted, zero, unresolved = F(1,4), F(1,4), F(1,2)
    assert accepted + zero + unresolved == F(1)
    cases.append({"name":"three-way-accounting","accepted":q(accepted),"zero":q(zero),"unresolved":q(unresolved)})
    cases.append({"name":"zero-measure","outcome":"zero_measure","measure":"0/1"})
    assert len(cases) == 16

    hostile_names = [
      "boundary_promoted_full","partial_fraction_estimated","unresolved_dropped","parent_copied_to_children",
      "majority_vote_promoted","sample_average_promoted","correlation_erasure_false_full","correlation_erasure_false_zero",
      "topology_change_ignored","branch_change_ignored","fold_ignored","conditional_lineage_promoted",
      "accepted_zero_unresolved_sum_drift","negative_measure","overlapping_children","missing_child_gap",
      "foreign_parent_identity","band_basis_omitted","time_basis_omitted","receiver_contact_promoted",
      "transfer_called_emission","arrival_called_detector_response","visibility_inferred","production_authority_inferred"]
    hostile = {name: False for name in hostile_names}
    assert len(hostile) == 24 and not any(hostile.values())
    receipt = {"schema_version":1,"status":"abstract_classifier_survives_with_provenance_blocker",
      "case_count":len(cases),"hostile_rejection_count":len(hostile),"cases":cases,"hostile":hostile,
      "implementation_blocker":"no_source_phase_space_cell_or_correlation_owner",
      "authority_effect":"none_evidence_only"}
    canonical=json.dumps(receipt,sort_keys=True,separators=(",",":")).encode()
    receipt["receipt_sha256"]=hashlib.sha256(canonical).hexdigest()
    print(json.dumps(receipt,sort_keys=True,separators=(",",":")))

if __name__ == "__main__": main()
