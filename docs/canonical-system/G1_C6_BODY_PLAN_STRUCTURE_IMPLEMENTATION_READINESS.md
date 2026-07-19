# G1 C6 body-plan structure V1 implementation readiness

Status: **owner-authorized for one bounded test-first implementation.**

## Exact scope

Implement `contracts/body-plan-structure-contract.md` as a standalone
`body-plan-structure` Rust crate using only `serde`, `serde_json` and `sha2`.
Bind its content-derived family identity to the existing opaque
`MacroLineageCandidate.body_plan_ref` through one additive validator. Preserve
the existing macro-lineage constructor and encoded record unchanged.
The sole consuming crate is `macro-lineage-binding`.

`body_plan_ref` means family identity, not expression identity. A family is the
stable topology/homology contract; an expression is one lawful realization.
This is the seam later applicability and variation work may consume, but V1
contains no sex, dimorphism, caste or biological applicability labels.

## Frozen API and identities

The public records are `BodyPlanFamily`, `PartTemplate`, `Cardinality`,
`PresenceRule`, `RelationRule`, `RelationKind`, `HomologyGroup`,
`SymmetryDeclaration`, `SymmetryPattern`, `StructuralExpression`,
`PartOccurrence`, `RelationInstance`, `SymmetryPosition`, `ValidationReport`,
`ValidationStatus` and `Violation`.

The public operations build, validate, canonically encode/decode and fingerprint
families and expressions; validate exact family references; expose the three
synthetic fixtures; and produce one authority-negative deterministic reference
receipt. Family, expression, fixture-suite and receipt identities use distinct
domains. Unknown schema versions and noncanonical bytes fail closed.

The deterministic receipt contains exactly: schema version, fixture-suite ID,
humanoid family ID, radial family ID, ordered radial expression IDs, withheld
family ID, hostile-registry digest, validation examination count, empty
capabilities, and false approval/promotion/runtime/geometry/biology-truth flags.

The exact synthetic controls are `HUMANOID_BILATERAL_V1`,
`RADIAL_POLYRAY_V1` and `WITHHELD_SERIAL_V1`. The withheld control is introduced
only for negative-transfer verification after the contract surface is frozen.

## Frozen focused matrix

Exactly 18 test groups are required:

1. strict codec round trip plus whitespace, unknown, duplicate and trailing rejection;
2. deterministic humanoid family/expression vectors;
3. lawful radial five/seven expressions share family ID and differ in expression ID;
4. family identity is expression-independent and expression identity is change-sensitive;
5. list permutations canonicalize while duplicates reject;
6. required/optional/conditional selection passes and fails exactly;
7. macro-lineage accepts only the exact validated family ID;
8. withheld serial fixture validates alone and rejects forced family transfer;
9. `C6-H200` opaque nonzero reference laundering rejects;
10. `C6-H201` dangling, orphaned and disconnected structure rejects atomically;
11. `C6-H202` topology, homology and symmetry contradiction rejects;
12. `C6-H203` capacity, sense, fitness and locomotion field injection rejects;
13. `C6-H204` coordinates, dimensions, proportions, pose and root field injection rejects;
14. `C6-H205` semantic reordering is stable while structural changes alter identity;
15. sex, gender, reproductive-role, dimorphism, caste, personality and rank injection rejects;
16. lineage/species/individual/population field injection and identity fabrication reject;
17. every exact resource maximum passes and maximum-plus-one rejects before expansion;
18. static capability and authority audit rejects filesystem, network, process, time, RNG, Forge Kernel, renderer and runtime reachability.

These tests implement only the body-plan slice of the frozen 82-case C6
registry. They do not claim that all C6 hostile cases or C6 closure pass.

## Verification and platform classification

The lower tiers are contract review, typed tests, in-memory fixtures and exact
codec vectors. Then require `cargo fmt`, native x64 tests, strict Clippy, native
i686 test execution where the installed target supports it, Android ARM64
compile-only classification, macro-lineage regression, all five retained C6
prototype suites, modularity, module-context freshness, record roles, exact C6
successor routes, retained C4/C5/GP3/GP4 shields, independent source review and
one registered full Forge gate.

Compile-only is never execution proof. Budget exhaustion is indeterminate.
Failure produces no partial receipt or authority.

## Rollback and stop

Rollback deletes the new crate and additive macro-lineage validator/dependency,
removes their workspace/module registrations and restores the pre-transition
checkpoint. Existing macro-lineage candidate bytes remain unchanged throughout.

Stop after the body-plan result is recorded. Do not automatically activate C6
identity, ecology, physiology, reproduction, heredity, dimorphism, population,
comparison, C7, runtime or Forge-port work.
