# G1 / C3 optical-lineage binding implementation readiness

Date: 2026-07-16

Status: **implementation-ready behind one exact owner action; new additive
cross-module evidence crate is not yet authorized.**

## Decision

The thin immutable per-band manifest plus explicit replayed object-bundle
candidate is ready for one bounded additive reference implementation. It can
bind existing local evidence into ordered lineage without changing or copying
the physical cell-step, interval-interface or interval-bulk numerical owners.

This is a materially new cross-module schema and crate. General continuation
does not authorize it. Implementation requires explicit owner approval of the
exact package below.

## Exact package

Create one capability-free workspace crate:

`crates/optical-lineage-binding`

Its only local dependencies are:

- `physical-path-substrate` for recipe/volume reconstruction and cell-step
  objects;
- `visible-radiance-bulk-transfer` for profile, one-band query and transfer
  replay;
- `visible-radiance-interface-event` for optional interface input/event replay;
- `serde` and `serde_json` for strict target-neutral codecs; and
- `sha2` for domain-separated identities and bundle receipts.

It must not depend on `forge-kernel`, Tauri, a database, filesystem, network,
process, runtime engine, renderer or platform adapter. It must not depend on
`fixed-interval-arithmetic` because v1 owns no numerical fold.

## Frozen public surface

The additive crate may expose only these semantic groups (exact Rust naming
may vary only mechanically inside the same schema):

1. `OpticalLineageBundleInputV1`: schema version, one nonzero lane source ID,
   one complete validated bulk profile, one band and an ordered vector of
   `OpticalLineageStepEvidenceV1`.
2. `OpticalLineageStepEvidenceV1`: one complete conditional interval bulk
   query/transfer pair and either both or neither of one interval-interface
   input/event pair.
3. `OpticalLineageBundleReceiptV1`: exact object count, exact canonical byte
   total and SHA-256 over the sorted `(object ID, canonical object SHA-256,
   canonical byte length)` entries.
4. `OpticalLaneStepV1`: lane ID, ordinal, predecessor step ID, four mandatory
   bulk/cell local IDs, two optional interface IDs, selected band disposition,
   typed terminal and step ID.
5. `OpticalLaneManifestV1`: reconstruction/profile/band/lane-source binding,
   ordered steps, bundle receipt, final terminal, transcript ID, limitations
   and authority-negative effect.
6. `OpticalLineageTerminalV1`: exactly ten distinct families—outer-domain
   exit, unavailable neighbour, unavailable current, ambiguous next face, no
   forward progress, all TIR, ambiguous interface branch, nonconvergent
   interface, unsupported interface model and work exhaustion.

The compiler accepts the complete bundle and derives every manifest field. A
caller cannot submit lane, step, predecessor, bundle-receipt, terminal or
transcript identity as canonical output. Validation recompiles and compares
the exact result.

## Frozen domains and derivation rules

Use distinct domains:

- `mindwarp.optical-lineage.lane.v1`;
- `mindwarp.optical-lineage.derived-source.v1`;
- `mindwarp.optical-lineage.step.v1`;
- `mindwarp.optical-lineage.bundle-receipt.v1`; and
- `mindwarp.optical-lineage.transcript.v1`.

Lane identity binds reconstruction ID, bulk profile ID, band, initial
cell-step input ID and declared lane source. Ordinals start at zero and are
contiguous. Predecessor is absent only at zero and otherwise equals the exact
previous step ID.

The initial cell-step input must use the declared lane source and revision one.
Every successor cell-step source and every interface-input incident source is
the domain-separated hash of lane ID, ordinal, predecessor and role. Successor
cell-state and interface-incident revisions equal `ordinal + 1`. These rules
close the caller-selected source-alias found by the oracle.

## Mandatory replay and adjacency

The implementation reconstructs the recipe and volume from the validated bulk
profile, then replays every supplied local object through its owner. It rejects
missing, duplicate, unused, foreign or hash-drifted evidence before emitting a
manifest.

For every step it must prove:

- reconstruction, scope, recipe, volume, profile and band are exact;
- the bulk query's nested cell input/event are the same objects whose IDs bind
  the transfer;
- next current cell equals the prior certified known neighbour;
- next Q160 point endpoint strings exactly equal the prior certified hit box;
- same-medium continuation has no interface and copies Q1.62 direction strings
  exactly;
- a changed known medium requires one replayed interface input/event whose
  source/target cells and face evidence reconstruct the certified transition;
- the next direction uses only the lane band's transmitted direction and maps
  interface `Q1_62` to physical `fractional_bits = 62` without numeric recoding;
- all TIR and every ambiguous, unavailable, nonconvergent, unsupported or outer
  outcome terminates under its exact typed family; and
- a valid continuation at the 64-step ceiling returns `work_exhaustion`; a
  shorter truncated continuation is invalid incomplete evidence.

The compiler may orchestrate owner validators. It may not implement a cell
face test, Snell/Fresnel equation, extinction product, exponential,
cumulative-power multiplication or receiver intersection.

## Hard bounds

- one manifest contains exactly one band and at most 64 steps;
- a manifest codec is capped at 1 MiB before decode;
- a bundle input codec is capped at 16 MiB before decode;
- at most 384 individually receipted local objects are admitted;
- exactly one profile/recipe/volume reconstruction is performed per compile;
- every local object is replayed once, with no ambient lookup or recursion;
- a three-lane portfolio is three independent manifests, never one recombined
  RGB geometry object; and
- validation peak memory must remain below 24 MiB per lane in the retained
  maximum-cost test.

The 16 MiB bundle cap exceeds the cap-derived per-lane maximum of 13,631,488
bytes. The independent oracle's 64-step modeled lane used 47,611 manifest
bytes and 128,934 object bytes. These are proof ceilings, not production
throughput or mobile-device performance claims.

## Strict codec and identity rules

All bundle and manifest codecs are capped before decode, deny unknown fields
and require decode/revalidate/re-encode byte equality. Vectors are ordered;
bundle receipt entries are sorted by exact object ID; duplicate IDs and
different objects under one ID fail. Native words, pointer layout, maps with
ambient ordering, platform paths and database keys never enter identity.

Public limitations and authority effect are fixed output, not caller policy.
The exact authority effect is `none_evidence_only`.

## Mandatory pre-source compatibility capture

Before adding the workspace member, manifest or source, rerun and preserve:

- physical exact-path fixture SHA-256
  `32a9de48cde37174604785b8e1f967106babd46765498921f03b8fa4c56e1869`;
- physical interval-cell fixture SHA-256
  `1d04495829ebf997417a3638cbf82607e697a14c3b0bed3218ef03bebd92e453d`;
- bulk V1 fixture SHA-256
  `67783f4eae5f737979580fbddd6725d4faaa556fb031b90730cf7359ba27fce2`;
  and
- interface point-V1 fixture SHA-256
  `cd055393aef810152a164e4a000bcd6307a9d2bd45ea7ba3a8e63aee342b1b49`.

The physical, bulk and interface source modules and manifests remain
unchanged. Any fixture, ID, byte, dependency or local test drift rolls back the
complete lineage package.

## Test-first implementation order

1. Capture the four fixture hashes and run all three owner suites before any
   workspace or source change.
2. Add failing tests for lane/step/domain identity, derived sources, exact
   point/direction propagation and all ten terminals.
3. Port all 26 hostile oracle cases, including the six fully resealed attacker
   families, before the happy-path compiler passes.
4. Add strict unknown-field, noncanonical-byte, cap, duplicate/unused object,
   output forgery and limitation/authority drift tests.
5. Add one-step, three independent lanes, 64-step one-lane and 192-step
   three-lane cost receipts. Enforce the per-lane caps and record allocations.
6. Re-run every local owner suite and fixture after the additive crate exists.
7. Run native Windows with warnings denied, executable i686 Windows, Android
   ARM64 check, module-boundary/context verification and the complete Forge
   gate.

The permanent verifier must pin the oracle source and receipt hashes, domain
strings, dependency allowlist, hard caps, terminal list, resealed hostile
tests, fixture hashes, absence of arithmetic/capability imports and no changes
to the three local owner manifests.

## Rollback

Rollback is deletion-only: remove the new crate, workspace membership,
module-boundary/context entry, contract/result records and permanent verifier.
No existing schema, object, identity or data requires migration.

## Nonclaims

This package proves ordered local optical-opportunity lineage only. It does not
compute cumulative transmission, reflected lineage, receiver arrival,
irradiance, detectability, perception, rendering, gameplay visibility,
passage, navigation, organism meaning, biome presentation, sphere, planet,
terrain, persistence, runtime behavior, performance promotion or C3 closure.

## Exact owner action

Approval of this exact package authorizes only the additive
`optical-lineage-binding` reference above, test-first and behind all stated
rollback gates. It does not authorize changes to the three local numerical
owners or any excluded operation.

Absent that explicit approval, remain at this owner gate with Forge heartbeat
paused. Do not create the crate, schema, dependency, tests or source.

