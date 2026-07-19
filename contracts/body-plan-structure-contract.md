# Body-plan structure V1 contract

Status: **owner-authorized capability-free implementation contract.**

## Purpose

Represent a coordinate-free body-plan family and its lawful structural
expressions without treating the bounded humanoid representation proof as a
universal organism model. `body_plan_ref` is the content-derived family
identity. Expression identity is separate so later lawful variation does not
fork macro-lineage identity.

## Records

- `BodyPlanFamily`: schema version, content-derived family ID, part templates,
  relation rules, homology groups, symmetry declarations and non-semantic
  limitations.
- `PartTemplate`: opaque template and role IDs, compressed cardinality, and an
  unconditional or opaque-predicate conditional presence rule.
- `RelationRule`: opaque rule ID, containment or structural-connection kind,
  endpoint templates, and bounded inbound/outbound degrees.
- `HomologyGroup`: explicit structural correspondence between at least two
  templates; it is not ancestry evidence.
- `SymmetryDeclaration`: explicit none, bilateral, radial, serial or bounded
  other-declared pattern plus exact member templates.
- `StructuralExpression`: schema version, content-derived expression ID, exact
  family ID, active predicate IDs, occurrences, relation instances, symmetry
  positions and non-semantic limitations.

Semantic identity fields are fixed IDs, never free-form biological labels.
Collections canonicalize by stable identity for encoding and hashing;
duplicates reject rather than silently collapse. Strict JSON decoding rejects
unknown fields, duplicate keys, trailing content and noncanonical ordering.

## Invariants

- all IDs are nonzero and every reference resolves;
- family and expression identities use separate SHA-256 domains;
- required, optional and conditional cardinalities are enforced without
  unbounded expansion;
- inactive conditional templates have zero occurrences;
- relation instances match their declared endpoint templates and both degree
  bounds;
- containment is acyclic with at most one incoming container per occurrence,
  while multiple containment roots and structural-connection cycles are legal;
- the combined realized structural graph is connected;
- homology members resolve and each group contains at least two templates;
- symmetry positions are unique, complete, contiguous and match the declared
  none, bilateral, radial, serial or other pattern;
- equivalent ordering yields identical canonical bytes and identity;
- exhausted validation work is `indeterminate_budget`, never biological
  impossibility;
- validation is atomic and emits no partial durable result.

No universal pelvis, root, tree, bilateral pattern, unique semantic role or
fixed limb vocabulary is permitted.

## Resource envelope

| Resource | V1 maximum |
|---|---:|
| part templates | 64 |
| relation rules | 128 |
| homology groups | 32 |
| symmetry declarations | 32 |
| active predicates | 64 |
| occurrences | 256 |
| relation instances | 512 |
| symmetry positions | 256 |
| limitations per record | 16 |
| bytes per limitation | 160 |
| canonical family or expression bytes | 262,144 |
| validation examinations | 4,096 |

Limits are engineering bounds, not biological maxima. Oversize input rejects
before occurrence expansion or partial output.

## Exact fixtures

- `HUMANOID_BILATERAL_V1`: coordinate-free structural control only; no sex,
  species, capability, geometry, pelvis-root or pose claim.
- `RADIAL_POLYRAY_V1`: one radial family with lawful five-ray and seven-ray
  expressions sharing one family ID and holding distinct expression IDs.
- `WITHHELD_SERIAL_V1`: schema-valid serial morphology used only after the
  contract is frozen to reject forced transfer into either earlier family. It
  is not product content or a promoted species.

## One consumer

`macro-lineage-binding` may validate that a fully valid family fingerprint is
exactly equal to the existing `MacroLineageCandidate.body_plan_ref`. It may not
copy anatomy into the lineage record, accept an expression ID, infer similarity
or change existing candidate bytes.

## Explicit exclusions

V1 owns no sex or dimorphism applicability, reproductive role, caste,
heredity, development, organism/species/individual/population identity,
physiology, viability, senses, locomotion, capacity, ecology, occupancy,
evolutionary homology, proportions, coordinates, units, pose, geometry,
surface, material, rig, animation, visual quality, generation solver,
randomness, clock, persistence, runtime capability, approval or promotion.
