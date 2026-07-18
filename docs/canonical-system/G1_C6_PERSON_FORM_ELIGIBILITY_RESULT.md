# G1-C6 Corrected Result: Person-Form Comparative Prerequisite Contract

**Status:** prerequisite contract only. The earlier person-form eligibility
claim was invalidated by logical audit on 2026-07-15.

## Audit finding

The first implementation treated a `Claim.evidence_ref` derived from a
`CausalWorldPacket` as if it were a biological lineage identity. That is
false: the reference identifies world evidence, not an organism, species,
macro-lineage, or body plan. Two claims can cite the same world without
belonging to the same lineage, and two claims for one lineage can legitimately
cite different evidence.

It also made two unsupported policy leaps:

- environmental electric or magnetic field availability was described as a
  real grounded communication capacity, although the world packet contains no
  organism emitter, receiver, physiology, behaviour, or lineage evidence;
- the master plan's phrase "the most ... compatible native lineage" was
  converted into a binary rule that all five dimensions must be present for
  eligibility. The source requires comparative compatibility; it does not
  define a universal all-five threshold or weighting.

Finally, person-form evaluation was attempted before macro-lineages and body
plans, contrary to the recorded causal generation order.

The old claims that every real Forge lineage was correctly "ineligible" and
that mismatched world evidence proved a "grotesque retrofit" are therefore
withdrawn. The Forge currently has no real lineage population to classify
either eligible or ineligible.

## Second adversarial correction

The first correction still called caller-supplied capacity labels "grounded"
without inspecting their claims. It accepted zero lineage/body-plan IDs, an
unrelated claim concept, and one duplicated arbitrary claim across all five
dimensions. That could produce `ReadyForComparativeEvaluation` without any
validated evidence. That readiness claim is withdrawn.

## Corrected bounded contract

`crates/person-form-eligibility` now exposes only
`evaluate_person_form_prerequisites`:

- the assessed lineage has an explicit `lineage_id`; identity is never inferred
  from `evidence_ref`;
- zero lineage and body-plan identities fail closed;
- each capacity binding requires a nonzero, nonduplicated claim ID, nonzero
  evidence reference, and the exact canonical capacity concept ID;
- foreign-lineage groundings are reported and cannot fill the assessed
  lineage's comparison dimension;
- all five valid bindings and a body-plan reference reach only
  `StructurallyCompleteBindings`;
- there is deliberately no `eligible: bool`.

Structural completeness does not validate claim truth or evidence provenance
and does not mean ready for comparison. Future owning modules must validate
those claims before structural compatibility, candidate comparison,
thresholds, weights, or incoherent-retrofit rejection can be evaluated.

## Evidence

`cargo test -p person-form-eligibility`: six focused tests cover:

- no body plan never reaches structural completeness;
- partial dimensions fail closed without claiming ineligibility;
- foreign-lineage evidence is reported and excluded;
- all five valid bindings plus a body plan yield structural completeness only;
- input ordering does not change the report.
- zero identities, unrelated concepts, and duplicated arbitrary claims cannot
  produce completeness.

The Forge Desktop read-only receipt is renamed
`person-form-comparative-prerequisite-contract`. Its fixture is explicitly
synthetic and records no real lineage or eligibility result.

## Retained limits and next dependency

This does not close person-form eligibility. Macro-lineage identity and body
plans must be defined first, followed by grounded sensory/species/ecomorph
records and a separate structural-compatibility proof. Only then can a
comparative person-form candidate portfolio be evaluated without inventing
biology or taste.
