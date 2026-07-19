# Organism subject identity V1 contract

Status: **code-free implementation candidate; no source authority.**

## Purpose

Define strict, deterministic identity-only records between the verified
macro-lineage/body-plan seam and later organism biology. V1 distinguishes a
validated lineage subject, an organism form template, a species candidate, an
individual and a population identity. It binds one individual exactly to the
existing C4 lifecycle/history target without treating age state, an opaque ID,
a template, a label or a population reference as biological membership,
descent, viability or truth.

This package exists so later ecology, physiology, reproduction, heredity,
development, variation and comparison cannot reuse one untyped 32-byte value
for incompatible subjects. It does not decide those later concepts.

## Records and identity domains

All identity-layer semantic IDs are nonzero 32-byte values. Upstream references,
including the world-packet string, retain their exact owning type. Every record
has schema version 1 and a content-derived identity under its own SHA-256 domain.
Cross-kind equality never grants substitutability.

- `LineageSubjectRefV1`: exact `MacroLineageCandidate` record fingerprint,
  exact upstream `lineage_id`, world-packet reference and validated body-plan
  family ID. Validation replays the macro-lineage candidate against its exact
  world input, packet and opportunity graph, then validates its family
  reference. This record creates no second lineage identity.
- `OrganismFormTemplateIdentityV1`: form-template ID, exact lineage-subject ID,
  body-plan family ID and one exact lawful structural-expression ID. It
  identifies a reusable structural form only. It is not an organism instance,
  species, sex, caste, capability set or visual asset.
- `SpeciesCandidateIdentityV1`: species-candidate ID, exact lineage-subject ID,
  body-plan family ID, nonzero candidate seed and fixed
  `membership_status = unresolved`. Labels are absent and excluded from
  identity. V1 cannot assert that an individual belongs to a biological
  species.
- `IndividualIdentityV1`: individual ID, exact world-packet reference and
  nonzero individual seed. Its stable identity is form-, expression-, lineage-
  and species-candidate-neutral so developmental change or later classification
  cannot remint the subject or break its C4 history target.
- `IndividualSubjectBindingV1`: content-derived binding ID, exact individual,
  species-candidate and form-template IDs. A changed form or classification
  creates a new binding while preserving the individual ID. The binding is not
  species membership, developmental state or a lifecycle event.
- `PopulationIdentityV1`: population ID, exact world-packet reference, nonzero
  population seed and fixed
  `membership_status = unresolved`. V1 contains no members, counts, weights,
  distribution, cohort, species-candidate association or persistence policy.
- `LifecycleHistorySubjectBindingV1`: individual ID, exact C4
  ambient-cohort-binding fingerprint, baseline key, history target logical ID,
  the exact initial and final C4 lifecycle fields (`mode`, `cohort`,
  `maturity_permille`, `elder_permille`, `appearance_lock`), final history head
  and stored-delta count. The history target must equal the individual ID.
  The received cohort binding, baseline, encoded deltas and
  final state are replayed through the unchanged C4 validators and reducer
  before this binding is valid.

Exact identity domains are:

- `mindwarp/c6-lineage-subject-ref/v1`
- `mindwarp/c6-organism-form-template/v1`
- `mindwarp/c6-species-candidate/v1`
- `mindwarp/c6-individual-identity/v1`
- `mindwarp/c6-individual-subject-binding/v1`
- `mindwarp/c6-population-identity/v1`
- `mindwarp/c6-lifecycle-history-subject-binding/v1`
- `mindwarp/c6-organism-subject-reference-receipt/v1`

Identity fields contain no free-form biological label. Diagnostic text is
non-semantic and absent from identity codecs.

## Exact encoded schemas and identity preimages

Every encoded V1 record is one compact UTF-8 JSON object with keys in the exact
order below. `Id32` is exactly 64 lowercase hexadecimal characters; an optional
`Id32` is that string or JSON `null`. `WorldPacketId` is the exact nonempty
upstream UTF-8 string, at most 256 bytes, without normalization. Enums are the
listed lowercase strings. Counts are unsigned JSON integers in their stated Rust
width. `schema_version` is the JSON integer `1`.

| Record | Exact field order and scalar/reference types |
|---|---|
| `LineageSubjectRefV1` | `schema_version: u16`, `subject_ref_id: Id32`, `macro_lineage_candidate_fingerprint: Id32`, `lineage_id: Id32`, `world_packet_id: WorldPacketId`, `body_plan_family_id: Id32` |
| `OrganismFormTemplateIdentityV1` | `schema_version: u16`, `form_template_id: Id32`, `lineage_subject_ref_id: Id32`, `body_plan_family_id: Id32`, `structural_expression_id: Id32` |
| `SpeciesCandidateIdentityV1` | `schema_version: u16`, `species_candidate_id: Id32`, `lineage_subject_ref_id: Id32`, `body_plan_family_id: Id32`, `candidate_seed: Id32`, `membership_status: "unresolved"` |
| `IndividualIdentityV1` | `schema_version: u16`, `individual_id: Id32`, `world_packet_id: WorldPacketId`, `individual_seed: Id32` |
| `IndividualSubjectBindingV1` | `schema_version: u16`, `subject_binding_id: Id32`, `individual_id: Id32`, `species_candidate_id: Id32`, `form_template_id: Id32` |
| `PopulationIdentityV1` | `schema_version: u16`, `population_id: Id32`, `world_packet_id: WorldPacketId`, `population_seed: Id32`, `membership_status: "unresolved"` |
| `LifecycleHistorySubjectBindingV1` | `schema_version: u16`, `lifecycle_binding_id: Id32`, `individual_id: Id32`, `ambient_cohort_binding_fingerprint: Id32`, `baseline_key: Id32`, `history_target_logical_id: Id32`, `initial_mode: "ambient" or "tracked"`, `initial_cohort: "young" or "juvenile" or "adult" or "elderly"`, `initial_maturity_permille: u16`, `initial_elder_permille: u16`, `initial_appearance_lock: bool`, `final_mode`, `final_cohort`, `final_maturity_permille`, `final_elder_permille`, `final_appearance_lock` with the same exact types, `final_history_head: optional Id32`, `stored_delta_count: u32` |
| `OrganismSubjectReferenceReceiptV1` | `schema_version: u16`, `receipt_id: Id32`, `fixture_suite_id: Id32`, `lineage_subject_ref_ids: [Id32; 2]` ordered humanoid then radial, `form_template_ids: [Id32; 3]` ordered humanoid then radial-five then radial-seven, `species_candidate_ids: [Id32; 2]` ordered humanoid then radial, `individual_ids: [Id32; 2]`, `individual_subject_binding_ids: [Id32; 2]`, `population_ids: [Id32; 2]`, `lifecycle_binding_id: Id32`, `final_history_head: Id32`, `hostile_registry_digest: Id32`, `identity_validation_examinations: u32`, `body_plan_validation_examinations: u32`, `capabilities: []`, `biological_membership: false`, `ancestry: false`, `viability: false`, `runtime_authority: false`, `approval_authority: false`, `promotion_authority: false` |

For each encoded record, its semantic preimage is compact JSON for one array of
the listed field values in that same order, omitting only its own derived ID
field (`subject_ref_id`, `form_template_id`, `species_candidate_id`,
`individual_id`, `subject_binding_id`, `population_id`, `lifecycle_binding_id`
or `receipt_id`). The derived ID is exactly
`SHA-256(UTF8(domain) || 0x00 || semantic_preimage_bytes)` using the matching
domain above. There is no Unicode, numeric or collection normalization beyond
the frozen representation; decoders reject any byte sequence whose decode and
exact canonical re-encode differ.

`OrganismSubjectBundleV1` is not encoded and has no identity or hash domain. It
contains exactly one validated value of each lineage-subject, form-template,
species-candidate, individual, individual-subject-binding and lifecycle-binding
record. Population identities remain separate identity-only evidence. The
binding validator dereferences the bundle records and requires one exact world,
lineage subject and body-plan family; it does not derive individual identity
from mutable associations.

## Canonical validation

Strict canonical JSON decoding rejects unknown fields, duplicate keys, trailing
content, unsupported versions, wrong field order, noncanonical ID spelling and
input above the record-byte ceiling. Re-encoding a decoded record must reproduce
the exact received bytes. Collections, where present only in proof receipts,
are ordered by typed identity; duplicates reject rather than collapse.

Validation is atomic and deterministic:

1. replay the exact macro-lineage world/input/packet/opportunity evidence;
2. validate the exact body-plan family and exact expression-to-family relationship;
3. reconstruct each identity from its semantic inputs and compare every received field;
4. require one consistent lineage subject, world and family through form and species candidate, then validate their versioned binding to the stable individual;
5. require population identity to remain membership-free;
6. validate the C4 ambient cohort binding, baseline, ordered encoded delta stream, head chain and reconstructed lifecycle state using the unchanged C4 schemas and reducers;
7. require the C4 logical target to equal the exact individual ID; and
8. emit a result only after all checks and budgets pass.

Identity-layer or body-plan examination-budget exhaustion is
`indeterminate_budget`, never invalid biology. Inherited C4 bounds retain their
existing typed C4 errors and are never translated into biological impossibility.
Failure emits no partial identity, binding or receipt.

## C4 lifecycle/history boundary

The package consumes, but does not modify, `entity-lifecycle`,
`entity-lifecycle-history-binding` and `hierarchy-history`. It adds no operation
key, reducer, lifecycle event, cohort, clock, persistence engine or mortality
path. It inherits the current C4 ceilings unchanged: 32 baseline dependencies,
65,536 bytes per operation, 1,024 recovery records and 16 MiB total recovery
bytes.

A valid lifecycle/history binding proves only that one identity is the exact
subject of deterministic C4 cohort and lifecycle replay. Age cohort is not
species distribution, development, reproductive maturity, heredity,
morphology, eligibility or presentation authority.

## Resource envelope

| Resource | V1 maximum |
|---|---:|
| canonical bytes per identity/binding record | 4,096 |
| canonical reference-receipt bytes | 32,768 |
| person-form capacity groundings at the sole consumer | 5 |
| identity-layer validation examinations | 2,048 |
| body-plan validation examinations | 4,096 inherited |
| C4 baseline dependencies | 32 inherited |
| C4 operation bytes | 65,536 inherited |
| C4 recovery records | 1,024 inherited |
| C4 recovery bytes | 16 MiB inherited |

Maximum-plus-one rejects before parsing, allocation expansion, replay or
receipt construction. These are engineering ceilings, not biological maxima or
product-scale proof.

## Exact controls

- one `HUMANOID_BILATERAL_V1` family/expression form-template subject, with no
  sex or species claim;
- the lawful five-ray and seven-ray `RADIAL_POLYRAY_V1` expressions as distinct
  form templates sharing one lineage subject, family and species-candidate
  identity;
- `WITHHELD_SERIAL_V1` only as a cross-family and cross-subject
  negative-transfer control;
- two distinct individuals associated through exact subject bindings to one
  form template, proving template or binding identity is not individual identity;
- two population identities alongside one species candidate, with no association
  claim, proving population identity supplies no member or count claim; and
- one ambient-to-tracked C4 lifecycle stream whose canonical replay target is
  the exact individual ID.

No control is promoted content or evidence of biological species, variation
applicability or population membership.

## One consumer

`person-form-eligibility` may add one validator/evaluator that accepts a fully
validated subject bundle and exact body-plan family/expression before invoking
its existing prerequisite evaluation. It must require the assessed lineage ID
and `body_plan_ref` to equal the bundle's exact lineage and family identities.
It may return only identity-bound structural completeness or incomplete
bindings.

The existing evaluator, public records and encoded report remain unchanged. The
consumer may not infer grounded capacity, comparative readiness, eligibility,
ranking, personhood, grotesqueness, species membership, sex, caste, capability
or appearance.

## Frozen rejection ownership

This package directly owns `C6-H400..405`: cross-kind identity collapse,
label-derived species identity, missing or laundered membership authority,
template-to-individual substitution, population member/count laundering,
identity-reminting through mutable form/classification, and unvalidated
cross-world reuse.

It owns authority-negative tests for `C6-H500..505`: the upstream optional
parent remains a hypothesis; no ancestry, descent, mutation, inherited delta,
similarity-derived relatedness, occupancy-derived evolution or provenance-free
biological event may appear in or be inferred from V1 records.

It also owns the applicable integration/authority slice: `C6-H1100`, `H1101`,
`H1102`, `H1103`, `H1105`, `H1106` and `H1108`. Disconnected green component
results, substituted receipts, partial output, ambiguous resource exhaustion,
codec/platform drift, runtime laundering and host-authority access all fail
closed. `H1104` scale generalization and `H1107` real C5 consumption remain
later C6 closure obligations, not claims of this package.

## Capability boundary

The candidate implementation may depend only on the capability-free upstream
`derived-world-rules`, `niche-graph-binding`, body-plan, macro-lineage and C4
reference crates plus deterministic serialization and SHA-256. It may not reach
Forge Kernel, filesystem, network, process, clock, randomness, database,
persistence, renderer, UI, engine, runtime executor or host authority.

V1 owns no ecological niche semantics, viability, physiology, sense,
locomotion, behavior, reproduction, parentage, heredity, development, realized
evolution, species membership policy, ecomorph, sex, dimorphism applicability,
caste, polyphenism, population membership/distribution, capacity truth,
comparative eligibility, aging presentation profile, culture, aesthetic intent,
C7 representation, gameplay permission, Companion or Greenfield integration.

## Rollback and stop

Rollback is deletion of the additive identity crate and the additive
person-form consumer/dependency/module registrations. Existing macro-lineage,
body-plan, C4 and person-form records, constructors, codecs, reducers and
identities remain byte-identical.

Stop after a separately authorized implementation result is verified and
recorded. Do not automatically activate ecology, physiology, reproduction,
heredity, development, realized evolution, applicable variation/dimorphism,
population semantics, comparison, C7, runtime or broad G1 closure.
