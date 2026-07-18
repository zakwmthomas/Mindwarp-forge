# G1 / C3 optical phase-space transport certificate implementation readiness

Date: 2026-07-17

Status: **ready for one explicit additive implementation decision. No crate,
contract schema, dependency, production test or production source is
authorized by this document.**

## Decision scope and evidence

The proposed action is one capability-free sibling owner named
`optical-phase-space-transport-certificate`. It replays a complete exact
phase-space cell and physical volume, derives a bounded ordered run of
same-medium axis-face events through the existing physical owner, and derives
every correlated face form directly from the immutable origin.

It is supported by three independent receipts:

- transport subject oracle source
  `a678220e7aebd3ec9e71c4df3a2a791d323848e81b3255da7f66e077aac185b5`
  and receipt
  `17b2edb4757b852470bdd9fab8d813b3b184605b4c555edecb55c18ce8fb197f`;
- rejected repeated-relinearization source
  `112fbb78356b38c0b2fad53a49c07040ea4812e484a54725b823b3f8c011d71d`
  and receipt
  `f7d2db26715b9a015918e3f48e25da98e9faaab2abc30ada0f4ce3801820c0c9`;
  and
- surviving immutable-origin source
  `97b287ec78d2d8f5031a3c7fbddbcd435db77a649e63ee4697519e1d6f66c156`
  and receipt
  `bbedc5a632b112b6eb633af57830034dfb99f98881f4f8968fbd44a42be93e76`.

The package has no arrival, coupling, radiance, emission, attenuation, power,
visibility, perception, runtime, promotion or C3 closure effect. Every object
carries `authority_effect = none_evidence_only`.

## Additive dependency boundary

The new crate may depend only on:

- `optical-phase-space-cell-binding` for complete V1 cell replay and immutable
  four-symbol origin forms;
- `physical-path-substrate` for physical recipe/volume/cell replay and the
  unchanged conditional interval cell-step compiler;
- `fixed-interval-arithmetic` for opaque checked `Signed512`, canonical
  decimals, multiplication, shifts and directed division;
- `serde` with derive;
- `serde_json`; and
- `sha2`.

Both domain owners already depend only downward on fixed arithmetic. The new
sibling creates no cycle. No existing crate may import it in this package. No
third-party dependency, feature, arbitrary-precision integer, native limb,
floating point or copied private arithmetic is allowed.

## Frozen V1 input

`OriginAnchoredTransportInputV1` contains:

- `schema_version: u16`, exactly `1`;
- one complete `OpticalPhaseSpaceCellV1`;
- one complete `PhysicalVolumeRecipeV1` and matching `PhysicalVolumeV1`;
- `current_cell: CellIndex3V1`;
- `band_time_id: [u8; 32]`, nonzero equality binding only; and
- `maximum_steps: u8`, from `1` through `64`.

The caller supplies no plane, face, height, direction mutation, output form,
topology token, event, successor cell, branch or terminal. Unknown fields fail.

The cell must replay through its strict codec, match the physical scope and
reconstruction, use `TransverseAreaDirection4d`, retain the six fixed roles,
and have every common denominator and signed form numerator at most **64
bits**. This is an additional transport admission cap; it does not change the
broader cell V1 contract.

The initial point and direction projection must construct a valid unchanged
`ConditionalIntervalCellStepInputV1`. The physical recipe, volume and current
cell replay before transport work begins. Current cell evidence must be
available.

## Frozen direct-origin arithmetic

Physical face height is signed Q32.32: `H / S` with `S=2^32`. For immutable
origin denominator `D`, plane axis `j`, point numerators `p` and direction
numerators `v`, define:

`A = H D - p_j S` and `b = v_j`.

For transverse output `i`:

- centre is `(p_i S b + A v_i) / (D S b)`;
- coefficient `k` denominator is `D S b^2`; and
- coefficient numerator is
  `p_i,k S b^2 + (-p_j,k b S - A v_j,k) v_i + A v_i,k b`.

The hit-axis form is exactly `H/S`. Complete exact interval endpoints use the
original common-denominator extents. Residual subtraction uses the deliberate
shared denominator `D S V_endpoint b^2`. It never performs generic repeated
rational addition and never uses a reduced predecessor form as a new origin.

All numerator, denominator, product, sum and Q160 shift intermediates are
checked before canonical reduction. The conservative no-cancellation bound is
`4B+234`; at `B=64` the live shield is **490 bits**, leaving 22 bits below
storage. `B=70` is rejected because its guarantee is 514 bits even when a
favourable fixture reduces below storage.

## Run compilation and terminals

At each step the compiler:

1. projects the current direct-origin face enclosure outward to Q160 position
   and Q1.62 direction;
2. constructs and compiles the unchanged physical interval cell-step input;
3. accepts only the owner's unique certified face with complete positive
   progress;
4. obtains Q32.32 height and orientation from the replayed physical cell;
5. derives the complete correlated face forms from the immutable origin;
6. requires their directed projection to lie inside the physical event's face
   receipt;
7. records the event and direct-form identities; and
8. advances only to a successor with exactly equal current medium evidence.

`OriginAnchoredTransportTerminalV1` is exactly:

- `InterfaceRequired` for any medium/substance change;
- `OuterDomainExit`;
- `UnavailableNeighbor`;
- `AmbiguousNextFace`;
- `NoForwardProgress`;
- `ArithmeticShieldExceeded`;
- `ProjectionOutOfRange`; and
- `WorkExhausted` after exactly the requested bounded number of otherwise
  valid steps.

Interface, reflection, refraction, TIR, scattering and direction mutation are
unsupported. A terminal never becomes zero power, no arrival, darkness or
visibility.

## Frozen outputs and identities

`ExactRationalV1` is canonical signed numerator and positive denominator
decimal strings with gcd one. `TransportAffineFormV1` contains role, centre,
four coefficients and ordered remainder endpoints as exact rationals.

`OriginAnchoredFaceStepV1` contains:

- zero-based step index;
- current cell and certified face;
- complete internally constructed physical interval input and event;
- successor cell when present;
- six complete direct-origin forms;
- their Q160/Q1.62 directed projection;
- `direct_form_id` and `step_id`;
- exact arithmetic receipt; and
- authority/limitations.

`OriginAnchoredTransportCertificateV1` contains input ID, immutable origin
cell ID, physical recipe/volume IDs, band/time binding, ordered face steps,
typed terminal, aggregate receipt, `certificate_id`, fixed limitations and
authority effect. The caller never supplies any derived object.

Identity domains are:

- input: `mindwarp.optical-phase-space.transport.input.v1`;
- direct form: `mindwarp.optical-phase-space.transport.form.v1`;
- step: `mindwarp.optical-phase-space.transport.step.v1`; and
- certificate: `mindwarp.optical-phase-space.transport.certificate.v1`.

Every identity is `SHA-256(domain || 0x00 || canonical_json)`. It binds complete
nested objects, order, terminal, receipts, limitations and authority. Validation
recompiles and compares the complete certificate.

## Resource and codec ceilings

V1 freezes:

- maximum steps: 64;
- immutable-origin scalar cap: 64 bits;
- live arithmetic shield: 490 bits in checked 512-bit storage;
- input bytes: 16 MiB;
- one face step: 256 KiB;
- certificate output: 20 MiB;
- aggregate live canonical bytes: 40 MiB;
- at most 64 existing physical compiler calls;
- at most 1,024 direct-form scalar derivations;
- at most 24,576 checked integer operations; and
- at most 768 directed projections.

Byte ceilings are enforced before decode or allocation. Step vectors enforce
length before collection. Canonical decimal length is rejected before parsing
when it cannot fit its frozen bit class.

All public types deny unknown fields. `to_bytes` validates/recompiles before
canonical JSON encoding; `from_bytes` checks the cap before decode, reconstructs
the complete object, re-encodes and requires byte equality. Alternate decimals,
negative zero, leading zeroes, unreduced rationals, reordered steps, trailing
content, stale nested events and stale identities fail typed.

## Required implementation tests

An approved implementation must add:

1. parity with all 24 positive and 33 hostile base transport portfolios;
2. the repeated-relinearization 16-bit/513-bit and 24-bit/778-bit rejection
   shields;
3. parity with 18 origin-algebra equivalence falsifiers and 15 hostiles;
4. pinned 64-bit `x+, y+, x+` three-face result and identity;
5. 64-bit acceptance, 65-69-bit conservative analysis fixtures and mandatory
   70-bit typed rejection independent of favourable reduction;
6. exact Q32.32 height, Q160/Q1.62 outward projection and 490-bit live shield;
7. current-owner event replay, face containment and successor ownership;
8. medium/substance change, outer, unavailable, ambiguity, non-forward and
   exact work-exhaustion terminals;
9. forged cell, band/time, face, event, output, order, receipt and authority
   rejection;
10. codec size, unknown/trailing/noncanonical and allocation ceilings;
11. pinned input, form, step and certificate byte/identity families;
12. warnings-denied native all-target tests;
13. executable `i686-pc-windows-msvc` tests;
14. `aarch64-linux-android` compilation;
15. unchanged cell, physical, interface, lineage, cumulative and receiver V1
    fixture hashes; and
16. focused module, record-role and complete Forge verification.

## Files and rollback if approved

The action may add only:

- `crates/optical-phase-space-transport-certificate/Cargo.toml`;
- `crates/optical-phase-space-transport-certificate/src/lib.rs`;
- focused tests under that crate;
- generated `MODULE.md` through canonical registries;
- `contracts/optical-phase-space-transport-certificate-contract.md`;
- one implementation-result record;
- one permanent implementation verifier; and
- workspace, lockfile, boundary/context, record, README, master-program and
  active-checkpoint integration required by those files.

No existing domain crate source or test may change. If an existing public API
cannot be consumed unchanged, implementation stops and returns to readiness.

Rollback is deletion-only. Remove the new crate, contract, tests, verifier,
result and registry/workspace references. No current data, codec or V1 identity
is migrated.

## Exact serious owner action

Approval authorizes exactly this:

> Add the capability-free
> `optical-phase-space-transport-certificate` sibling crate and contract with
> the frozen 64-bit immutable-origin input cap, 490-bit checked live shield,
> exact Q32.32 direct-face algebra, Q160/Q1.62 projections, unchanged physical
> cell-step replay, at most 64 same-medium ordered faces, typed interface and
> resource terminals, strict canonical codecs, hostile/platform/full-gate
> tests and deletion-only rollback described in this readiness record. Add no
> coupling consumer and modify no existing domain owner source or V1 behavior.

Any change to the 64-bit cap, immutable-origin rule, 490-bit shield, dependency
graph, face semantics, interface stop, step/resource ceiling, output fields,
authority boundary or current owner source requires a new serious owner
decision.

## Stop

This is the serious change gate. Until the owner explicitly approves or
rejects the exact action above, do not add a crate, contract schema, dependency,
production test or production source. Pause the Forge heartbeat and wait.

