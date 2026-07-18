# G1 / C3 optical phase-space cell provenance implementation readiness

Date: 2026-07-16

Status: **ready for one explicit additive implementation decision; no crate,
dependency, contract schema, production test or production source is authorized
by this document.**

## Decision scope

The proposed action is one capability-free prerequisite owner named
`optical-phase-space-cell-binding`. It binds a declared four-dimensional source
parameter cell, exact positive measure, deterministic binary ancestry and six
correlated affine output forms. It can produce an authority-negative directed
fixed-point projection receipt for a later consumer.

It does not implement optical coupling. It does not classify receiver coverage
or compose a lineage. It changes no current physical, interface, lineage,
cumulative-transfer or receiver V1 type, byte, identity, fixture or behavior.

The package is supported by pinned oracle source
`7740595a08656d616f714bc3e1f249acd0a9b0fe95b486736f0227158981d5f6`
and receipt
`f9b354164a13bdaa312af6c8711915f661fea4a9abd7fe5ba097f872afb297e6`:
20 positive portfolios and 33 hostile rejections over 127 cells and 63 exact
splits.

## Additive dependency boundary

The new crate may depend only on:

- `fixed-interval-arithmetic` for opaque checked `Signed512`, canonical decimal
  integers, magnitude-bit accounting and directed division;
- `serde` with derive;
- `serde_json`; and
- `sha2`.

It must not depend on physical-path, interface-event, bulk-transfer,
optical-lineage, cumulative-transfer or receiver-arrival crates. No current
crate may import it in this package. This prevents a reverse dependency and
keeps later adapters/consumers responsible for joining independent evidence.

No new third-party dependency or feature is required. The pinned transitive
`crypto-bigint` remains encapsulated by `fixed-interval-arithmetic`; native
limbs and endianness remain inaccessible.

## Frozen V1 public types

All structs use `serde(deny_unknown_fields)` and all enums use explicit
snake-case names.

### Root input

`OpticalPhaseSpaceRootInputV1`

- `schema_version: u16`, exactly `1`;
- `source_id: [u8; 32]`, nonzero;
- `scope_id: [u8; 32]`, nonzero;
- `reconstruction_id: [u8; 32]`, nonzero;
- `source_revision: u32`, positive;
- `parameterization: PhaseSpaceParameterizationV1`, exactly
  `TransverseAreaDirection4d`;
- `measure: PositiveRationalV1`;
- `form_denominator: String`; and
- `forms: [CorrelatedAffineOutputV1; 6]` in the fixed role order below.

`PositiveRationalV1` contains canonical positive decimal `numerator` and
`denominator` strings. Both are nonzero, denominator is positive, their gcd is
one, and each magnitude is at most 192 bits at root input.

`CorrelatedAffineOutputV1` contains:

- `role: PhaseSpaceOutputRoleV1`;
- `center_numerator: String`;
- `coefficient_numerators: [String; 4]`;
- `remainder_lower_numerator: String`; and
- `remainder_upper_numerator: String`.

The fixed role order is `PointX`, `PointY`, `PointZ`, `DirectionX`,
`DirectionY`, `DirectionZ`. Every scalar shares `form_denominator`, which is a
positive canonical decimal at most 192 bits. Every signed numerator is a
canonical decimal with magnitude at most 192 bits. The remainder is ordered.
The gcd across the denominator and all 42 form numerators must be one. This
single collective normalization rule prevents identity aliases.

The root input makes no claim that its declared measure is physically correct;
`source_id` and revision bind the declaration. Measure is phase-space partition
evidence, not radiance, emitted power or probability.

### Cell and path

`OpticalPhaseSpaceCellV1`

- repeats schema, source, scope, reconstruction, revision and parameterization;
- `root_id: [u8; 32]`;
- `parent_id: Option<[u8; 32]>`;
- `depth: u8`, at most `12`;
- `path: Vec<PhaseSpaceSplitStepV1>`, length exactly depth and at most `12`;
- exact `measure`, common `form_denominator` and six forms;
- `cell_id: [u8; 32]`;
- fixed limitations; and
- `authority_effect: String`, exactly `none_evidence_only`.

`PhaseSpaceSplitStepV1` is `{ axis: PhaseSpaceParameterAxisV1, side:
PhaseSpaceSplitSideV1 }`. Axes are `U0..U3`; sides are `Lower` and `Upper`.
Root parent is absent, depth/path are zero/empty, and `cell_id == root_id`.

### Split query and receipt

`OpticalPhaseSpaceSplitQueryV1` contains schema, one complete validated cell and
one axis. The caller supplies no child fields.

`OpticalPhaseSpaceSplitReceiptV1` contains:

- schema and parent ID;
- selected axis;
- ordered `[lower, upper]` complete child cells;
- exact parent and ordered child measures;
- `arithmetic_receipt: PhaseSpaceArithmeticReceiptV1`;
- `split_id: [u8; 32]`;
- fixed limitations; and
- `authority_effect: none_evidence_only`.

A query at depth 12 returns typed `DepthLimit`; it creates no child and the
caller retains the complete parent measure. The owner does not schedule a cell
set or drop unresolved measure.

### Directed projection receipt

`OpticalPhaseSpaceProjectionQueryV1` contains one complete validated cell and
the fixed target `ExistingOpticalIntervalSeamV1`.

`OpticalPhaseSpaceProjectionReceiptV1` contains:

- source `cell_id`;
- three position intervals with `fractional_bits = 160`;
- three direction intervals with `fractional_bits = 62`;
- each interval as canonical signed decimal lower/upper integers;
- the exact common-form denominator hash;
- `arithmetic_receipt`;
- `projection_id`;
- limitations; and
- `authority_effect: none_evidence_only`.

Projection uses mathematical floor for lower and ceiling for upper after
multiplication by `2^160` or `2^62`. Direction endpoints must remain within
`[-2^62, 2^62]`; otherwise compilation returns typed `ProjectionOutOfRange`.
The receipt deliberately does not import or construct a current owner input.
A later consumer must bind this receipt and independently construct/replay the
unchanged physical/interface input.

## Domain-separated identity rules

The exact separators are:

- root: `mindwarp.optical-phase-space.root.v1`;
- child cell: `mindwarp.optical-phase-space.cell.v1`;
- split: `mindwarp.optical-phase-space.split.v1`; and
- projection: `mindwarp.optical-phase-space.projection.v1`.

Every hash is `SHA-256(domain || 0x00 || canonical_json_bytes)`.

Root identity commits to every normalized root input field plus fixed
limitations and authority effect. Child identity commits to root ID, parent ID,
depth, complete ordered path, derived measure, normalized form, limitations and
authority. Split identity commits to query parent/axis, ordered complete child
IDs, exact measure tuple, arithmetic receipt, limitations and authority.
Projection identity commits to cell ID, target, all six directed intervals,
denominator hash, arithmetic receipt, limitations and authority.

Receiver, lineage, cumulative transfer, source magnitude, topology and branch
fields are absent and rejected as unknown codec fields.

## Exact common-denominator split arithmetic

Let the parent form denominator be `D`, centre numerator `c`, selected-axis
coefficient `a`, every other coefficient `b`, and remainder numerator `r`.
Before collective normalization:

- child denominator is `2D`;
- lower/upper centre is `2c - a` / `2c + a`;
- selected-axis coefficient remains `a`;
- every other coefficient becomes `2b`; and
- every remainder endpoint becomes `2r`.

The implementation then divides the denominator and all 42 numerators by their
single positive collective gcd. Root and child measure are normalized
separately; each child is exactly `parent_numerator / (2 *
parent_denominator)` reduced by its pairwise gcd.

This is algebraically identical to `u=(v-1)/2` and `u=(v+1)/2`. It uses shifts,
signed addition/subtraction and gcd on magnitudes. It performs no general
rational cross-product and no floating-point operation.

## Derived bit ceiling

Root numerators and denominators are at most 192 bits. At depth `k <= 12`:

- denominator, ordinary coefficient and remainder magnitudes are at most
  `192 + k <= 204` bits before normalization;
- a centre numerator is bounded by the original centre plus four offset
  coefficients, at most `192 + k + ceil(log2(5)) <= 207` bits;
- an independent projection endpoint adds the centre, four absolute
  coefficients and one remainder, at most 208 bits; and
- Q160 outward projection shifts that endpoint numerator by 160 bits before
  directed division, so the maximum live magnitude is **368 bits**.

All arithmetic uses checked 512-bit storage. Any value above the derived
368-bit shield returns typed `ArithmeticShieldExceeded`, even if storage has
spare capacity. Q1.62 projection is lower at 270 bits. No multiplication of two
arbitrary rational magnitudes is admitted.

## Resource ceilings

The V1 implementation freezes:

- root input: 16 KiB;
- cell: 32 KiB;
- split query: 32 KiB;
- split receipt: 64 KiB;
- projection query: 32 KiB;
- projection receipt: 16 KiB;
- aggregate live canonical bytes per operation: 160 KiB;
- depth/path: 12;
- parameter symbols: exactly 4;
- output forms: exactly 6;
- canonical numeric strings per cell: at most 50;
- decimal parses per validated object: at most 50;
- split pair: at most 128 shifts, 32 signed additions/subtractions and 96 gcd
  reductions/checks; and
- projection: at most 64 absolute/add operations, 12 shifts and 12 directed
  divisions.

All byte ceilings are checked before decode/allocation. Path allocation is
bounded before collection. Strings are rejected before arithmetic when their
decimal length cannot fit 192 root bits or 368 derived bits.

## Strict codecs and errors

`to_bytes` first validates/recompiles the complete object, encodes canonical
JSON and enforces the relevant cap. `from_bytes` enforces the cap before decode,
denies unknown fields, reconstructs the value, re-encodes and requires exact
byte equality. Trailing content, alternate number spellings, `+`, whitespace,
negative zero, leading zeroes, unreduced rationals, reordered roles and stale
IDs fail.

Public errors remain typed into: invalid schema/provenance, noncanonical
decimal/rational/form, reversed remainder, identity mismatch, depth limit,
projection out of range, byte/live/resource ceiling, arithmetic shield,
arithmetic defect and codec defect. No error maps to darkness, zero power,
arrival, visibility or authority.

## Required implementation tests

The owner-authorized package must add tests for:

1. exact parity with all 20 positive and 33 hostile oracle families;
2. pinned root ID and 4/16/64-leaf conservation receipts;
3. exact `u-u=0` correlated difference versus `[-2,2]` independent boxes;
4. fixed role order, collective gcd normalization and identity sensitivity;
5. lower/upper split derivation and caller inability to forge children;
6. depth-12 success and depth-13 typed stop with parent measure retained;
7. Q160/Q1.62 outward rounding, sign edges and direction range rejection;
8. 192-bit root acceptance, 193-bit rejection, 368-bit derived shield and
   overflow-before-storage defenses;
9. unknown/trailing/oversize/noncanonical codecs and canonical round trips;
10. receiver/radiance/topology/branch/authority field injection rejection;
11. exact input/result/identity fixture hashes for V1 drift shielding;
12. warnings-denied native all-target tests;
13. executable `i686-pc-windows-msvc` tests; and
14. `aarch64-linux-android` compilation.

The permanent verifier must rerun the pinned Python oracle, focused Rust tests,
platform gates, module context, record roles and the complete `tools/verify.ps1`
gate. Existing physical, interface, lineage, cumulative and receiver fixture
hashes must remain unchanged.

## Module declaration and files, if approved

The implementation action may add only:

- `crates/optical-phase-space-cell-binding/Cargo.toml`;
- `crates/optical-phase-space-cell-binding/src/lib.rs`;
- focused tests under that crate;
- generated `MODULE.md` through the canonical registry;
- `contracts/optical-phase-space-cell-binding-contract.md`;
- one implementation-result record;
- one permanent implementation verifier;
- workspace membership, lockfile, module-boundary/context, record-role,
  canonical README, master-program and active-checkpoint integration required
  by those files.

It may not edit source or tests in existing domain crates. If an existing API
cannot be consumed unchanged, implementation stops and returns to readiness.

## Rollback

Rollback is deletion-only: remove the new crate, contract, tests, verifier,
result and their registry/workspace references. No data migration exists. No
current V1 identity or fixture is rewritten. A rollback drill must prove the
pre-action workspace and relevant owner suites still pass.

## Exact owner action

Approval authorizes exactly this:

> Add the capability-free `optical-phase-space-cell-binding` crate and contract
> with the frozen four-symbol common-denominator V1 types, 12-level binary
> refinement, 192-bit root inputs, 368-bit live shield, strict canonical
> codecs, Q160/Q1.62 projection receipts, hostile/platform/full-gate tests and
> deletion-only rollback described in this readiness document.
> Add no coupling consumer and modify no existing owner source or V1 behavior.

Any change to dimension, refinement rule, arithmetic representation, root bit
cap, live shield, dependency graph, output roles, authority boundary or current
owner source requires a new serious owner decision.

## Stop

This is the serious change gate. Until the owner explicitly approves or
rejects the exact action above, **do not add a crate, dependency, contract
schema, production test or production source.** Pause the Forge heartbeat and
wait; repeated `Continue` messages count only if they are user-authored after
this exact gate is presented.
