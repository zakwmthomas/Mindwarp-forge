# G1 C6 organism subject identity V1 implementation readiness

Status: **implementation-ready behind one exact owner action; source absent.**

## Exact scope

Implement `contracts/organism-subject-identity-contract.md` as one standalone
capability-free `organism-subject-identity` Rust crate. Consume the exact
validated `MacroLineageCandidate`, `BodyPlanFamily`, lawful
`StructuralExpression`, `AmbientCohortBindingV1`, lifecycle state and
hierarchy-history stream. Add one identity-bound prerequisite evaluator to
`person-form-eligibility`. Preserve every existing macro-lineage, body-plan, C4
and person-form constructor, codec, reducer, identity and report unchanged.

This package distinguishes identity kinds and proves exact subject binding
only. It does not complete biological species membership, ecology, physiology,
reproduction, heredity, development, evolution, dimorphism, population
semantics or comparative eligibility.

## Frozen public surface

The public records are:

- `LineageSubjectRefV1`;
- `OrganismFormTemplateIdentityV1`;
- `SpeciesCandidateIdentityV1`;
- `IndividualIdentityV1`;
- `IndividualSubjectBindingV1`;
- `PopulationIdentityV1`;
- `LifecycleHistorySubjectBindingV1`;
- `OrganismSubjectBundleV1` as a validated in-memory composition only; and
- `OrganismSubjectReferenceReceiptV1`.

The public operations build, validate, strictly encode/decode and fingerprint
each identity or binding record and the reference receipt; assemble and validate
one exact subject bundle; bind and replay the exact C4 lifecycle/history subject;
validate the one person-form consumer;
expose only the frozen synthetic controls; and produce one deterministic
authority-negative reference receipt.

Each record and the receipt use the distinct identity domains frozen in the
contract. `LineageSubjectRefV1` replays the existing lineage candidate and does
not mint a replacement lineage ID. `OrganismFormTemplateIdentityV1` binds an
exact expression to its family. Species and population records carry fixed
`membership_status = unresolved`. Individual, template, binding and population
IDs can never substitute for one another even when their semantic inputs
overlap. Individual identity is derived only from world and individual seed;
form and species-candidate association live in `IndividualSubjectBindingV1`, so
a changed binding never remints the individual or its C4 history target.
Population identity is likewise species-candidate-neutral.

Strict canonical JSON rejects unsupported versions, unknown or duplicate
fields, trailing bytes, noncanonical field order, noncanonical ID spelling and
oversized input. Unknown semantic variants reject. Re-encoding must reproduce
the received bytes exactly.

The reference receipt contains exactly: schema version; fixture-suite ID;
ordered humanoid and radial lineage-subject IDs; ordered humanoid, radial-five
and radial-seven form-template IDs; ordered humanoid and radial species-candidate
IDs; ordered individual IDs; ordered
individual-subject-binding IDs; ordered population IDs;
lifecycle-history-binding ID; final C4 history head;
hostile-registry digest;
identity and body-plan validation
examination counts; empty capabilities; and false biological-membership,
ancestry, viability, runtime, approval and promotion flags.

`OrganismSubjectBundleV1` is a fixed in-memory composition of the validated
lineage, form, species-candidate, individual, individual-subject-binding and
lifecycle-binding records. Population identities are separate identity-only
evidence and are never inputs to the person-form consumer. The bundle has no
independent identity, codec or fingerprint and contains no unbounded collection.

## Frozen focused matrix

Exactly 33 test groups are required:

1. strict codec round trip and whitespace, unknown-field, duplicate-key, wrong-order and trailing-content rejection;
2. all identity kinds and the receipt use distinct domains and cross-kind substitutions reject;
3. lineage-subject construction replays the exact macro-lineage world, packet, graph, candidate and family binding;
4. form-template construction requires an exactly validated expression of the bound family;
5. radial five/seven forms share lineage, family and species-candidate identity while retaining distinct form-template identities;
6. the withheld serial control validates alone and rejects cross-family or cross-subject transfer;
7. species-candidate identity is label-free, deterministic and always membership-unresolved;
8. two individuals associated to one form template retain stable distinct individual identities; a changed form/species binding does not remint either individual or its C4 target;
9. population identities are species-candidate-neutral, deterministic and contain no members, counts, weights, cohorts or distribution;
10. the exact ambient cohort binding must name the individual identity and preserve cohort/assignment evidence;
11. encoded C4 deltas replay to the exact received head and final state with the individual as baseline/history target;
12. the additive person-form consumer accepts only exact lineage/family bindings and reaches at most structural completeness while the legacy evaluator/report remain unchanged;
13. `C6-H400` lineage, template, species-candidate, individual and population cross-type collapse rejects;
14. `C6-H401` labels, aliases and presentation text cannot derive or change species-candidate identity;
15. `C6-H402` absent, opaque-nonzero or asserted membership policy cannot turn the unresolved candidate into biological membership;
16. `C6-H403` a form template or species candidate cannot be used as an individual;
17. `C6-H404` population member/count/distribution injection and invalid aggregate references reject;
18. `C6-H405` foreign-world candidate, form, individual, population, cohort or history reuse rejects;
19. `C6-H500` optional macro-lineage parent cannot become descent or ancestry proof;
20. `C6-H501` ancestry edge, cycle or time-order claims are outside the schema and reject;
21. `C6-H502` inherited or changed biological delta claims are outside the schema and reject;
22. `C6-H503` similarity of family, expression or codec cannot derive ancestry;
23. `C6-H504` hypothetical opportunity occupancy cannot become evolution or realized occupancy;
24. `C6-H505` biological event identity without separately versioned provenance is unavailable and rejects;
25. `C6-H1100` disconnected green component receipts cannot compose a subject bundle;
26. `C6-H1101` substituted lineage, family, expression, cohort, baseline, delta, head or person-form inputs/report rejects;
27. `C6-H1102` any failure emits no partial identity, lifecycle binding or durable receipt;
28. `C6-H1103` exhausted identity/body examination budget is typed indeterminate; inherited C4 overbounds retain their unchanged typed C4 errors, and neither classification implies biological invalidity;
29. `C6-H1105` native/i686/Android canonical vectors and source inputs must match exactly, with compile-only never called execution;
30. `C6-H1106` a desktop or reference receipt cannot claim runtime, persistence or production authority;
31. `C6-H1108` filesystem, network, process, clock, RNG, database, Forge Kernel, UI, renderer and runtime reachability reject;
32. every exact record, receipt, grounding, examination and inherited C4 maximum passes while maximum-plus-one rejects before expansion or partial replay; and
33. a static dependency, capability and vocabulary audit confirms no ecology, physiology, sex, dimorphism, caste, reproduction, heredity, development, asserted-membership/member/count/distribution, capacity-truth, representation or gameplay field leaked into V1.

These groups own only the named identity and integration slices of the frozen
82-ID C6 registry. They do not claim all C6 hostiles or C6 closure.

## Exact resources and failure classification

The implementation freezes 4,096 canonical bytes per identity/binding record,
32,768 canonical receipt bytes, exactly 5 person-form capacity groundings and 2,048
identity-layer validation examinations. It inherits 4,096 body-plan
examinations and the unchanged C4 ceilings of 32 baseline dependencies, 65,536
bytes per operation, 1,024 recovery records and 16 MiB recovery bytes.

Maximum-plus-one rejects before semantic expansion or receipt creation.
Identity-layer or body-plan examination exhaustion returns indeterminate. C4
keeps its existing typed errors and known-good-prefix behavior, but a failed or
partial C4 replay cannot produce a C6 subject binding.

## One-consumer boundary

The sole downstream consumer is `person-form-eligibility`. One additive
evaluator validates the exact subject bundle, requires its existing assessed
lineage and body-plan references to equal the bundle's lineage and family, then
delegates to the existing prerequisite evaluator. Its exact result shape is
`Result<PersonFormPrerequisiteReport, BoundSubjectError>`: subject-binding
failures are typed errors, while incomplete or structurally complete status
remains the unchanged existing report. It cannot add `eligible`, rank, score,
threshold, truth, membership, sex, capability or presentation fields.

No existing person-form bytes or test vectors may change. No other crate may
consume the new identity package in V1.

## Dependencies and capabilities

The new crate may depend only on `derived-world-rules`, `niche-graph-binding`,
`macro-lineage-binding`, `body-plan-structure`, `entity-lifecycle`,
`entity-lifecycle-history-binding`, `hierarchy-history`, `serde`, `serde_json`
and `sha2`. Development fixtures may reuse the existing capability-free
world/field builders already required to reconstruct an exact macro-lineage
candidate.

The package may not depend on Forge Kernel, desktop/UI crates, a database,
filesystem, network, process, time, randomness, renderer, engine, runtime
executor, Companion or Greenfield. Capability and biological-vocabulary scans
are mandatory verification, not comments.

## Verification and platform classification

Before source authorization, require the code-free contract/readiness verifier,
its disposable document/route hostile harness, independent
design/test/governance review and one registered full gate. After a separate
exact owner authorization, proceed test-first and require all 33 focused
implementation groups and their named hostile cases against the source candidate.

Implementation verification requires `cargo fmt`, native x64 tests, strict
Clippy, body-plan and macro-lineage regressions, C4 lifecycle/history
regressions, person-form regression and the additive consumer test. Require
native i686 execution where the installed target supports it and Android ARM64
compile-only classification. Compile-only is never execution or
device-performance evidence.

Then require modularity, module-context freshness, record-role checks, exact C6
successor-route checks, retained C4/C5/GP3/GP4 shields, independent source review
and one registered full Forge gate. Failed attempts remain recorded and may not
be silently replaced by a later pass.

## Explicit owner action

Implementation requires one new exact owner authorization naming
`G1-C6-ORGANISM-SUBJECT-IDENTITY-IMPLEMENTATION-V1` and limiting authority to
the contract, standalone crate, one additive person-form consumer, exact tests,
governance projections and verification above.

The action grants no ecology, physiology, reproduction, heredity, development,
ancestry/evolution, species membership, applicable sex/dimorphism/caste/
polyphenism, population membership/distribution, capacity truth, comparison,
C7, runtime, Companion, Greenfield, broad G1 closure, promotion authority or
Kernel mutation.

## Rollback and stop

Rollback deletes the new crate and additive person-form consumer/dependency,
removes workspace/module registrations and restores the pre-transition
checkpoint. Existing upstream and person-form source, records, identities,
codecs, reducers and receipts remain unchanged.

Stop after the bounded implementation result is verified and recorded.
Reassess the dependency order before selecting ecological semantics. Do not
automatically activate any later C6 package, C7, runtime or broad G1 closure.
