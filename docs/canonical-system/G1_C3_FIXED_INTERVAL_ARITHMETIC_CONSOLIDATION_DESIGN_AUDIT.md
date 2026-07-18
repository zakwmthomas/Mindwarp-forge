# G1 / C3 fixed-interval arithmetic consolidation design audit

Date: 2026-07-16

Status: **staged consolidation selected; first package may add one
semantic-neutral arithmetic crate and migrate only the physical cell-step;
no source authorized by this audit.**

## Problem

Two verified owners now contain near-duplicate private checked signed-512
machinery:

- `physical-path-substrate` uses canonical decimal values, signed-magnitude
  add/subtract/multiply/shift and mathematical floor/ceiling division for its
  fixed-160 conditional cell step.
- `visible-radiance-interface-event` uses the same base plus fixed intervals,
  intersection, projection and directed square root across 96, 128 and 160
  fractional bits.

The selected one-band interval bulk candidate needs the shared base,
fixed-160 interval multiplication, intersection and square root. Copying the
machinery a third time would make every overflow, directed-rounding, decimal
or platform repair a three-site consistency problem.

## Candidate comparison

| Candidate | Benefit | Risk/cost | Disposition |
|---|---|---|---|
| third private copy in interval bulk | smallest immediate diff | permanent three-way drift and duplicated platform/security review | reject |
| expose interface-event private arithmetic | no copy | reverses semantic ownership and would make physical/bulk geometry depend on optics | reject |
| migrate both existing owners and add bulk in one action | maximum immediate deduplication | large multi-owner blast radius obscures which migration caused drift | reject |
| add a semantic-neutral arithmetic crate, migrate physical cell-step first, then reassess | proves the shared seam with one protected consumer and no optical mutation | one additional staged gate | **select** |
| retain both private implementations indefinitely | no migration risk | blocks the compounding repair benefit the Forge requires | fallback only if compatibility proof fails |

## Selected foundational boundary

A new capability-free workspace crate named `fixed-interval-arithmetic` owns
only target-neutral checked arithmetic representation and operations. It has
one exact external dependency:

`crypto-bigint = { version = "=0.7.5", default-features = false }`

The package is already resolved in the workspace. The new local crate adds no
registry package, version or feature.

The public cross-crate surface is intentionally semantic-neutral:

- opaque `Signed512` with zero/one, checked signed construction and canonical
  decimal parse/format;
- checked add, subtract, negate, multiply and variable shift;
- mathematical floor/ceiling division using unsigned magnitudes;
- ordered `FixedInterval` with explicit fractional-bit metadata;
- checked interval add, subtract, multiply and intersection;
- directed square root for nonnegative intervals;
- outward precision projection that never increases precision; and
- `maximum_magnitude_bits` inspection for owner-specific receipts and shields.

The crate owns one closed arithmetic error type for invalid canonical decimal,
storage overflow, zero division, reversed interval, incompatible scale,
negative root and invalid projection. Semantic owners map those failures to
their existing public error categories.

## Explicit non-ownership

The shared crate owns no:

- physical coordinates, cells, faces, paths or topology;
- spectral bands, coefficients, media, Fresnel or exponential policy;
- domain separators, object identities, semantic JSON schema or limitations;
- source lineage, endpoint, visibility, collision, biome or runtime meaning;
- filesystem, network, process, persistence, UI or external capability.

It exposes no native limbs, word count, limb order or endianness. Semantic
codecs remain signed decimal strings owned and capped by each consumer.

## Staged migration

### Stage A: compatibility capture

Before consumer source changes, permanently capture physical interval
cell-step canonical bytes and IDs for:

1. normal certified face;
2. reverse/outer face;
3. exact ambiguity;
4. no-forward-progress;
5. unavailable neighbour;
6. finite transfer-ready near-parallel state; and
7. negative/near-maximum coordinates.

The existing five exact-path V1 families remain mandatory. This engineers a
regression shield for both the legacy and newly implemented surfaces.

### Stage B: independent arithmetic crate

Implement the shared crate with differential vectors matching both retained
private implementations. Hostile tests cover signed division in all sign and
remainder quadrants, decimal poison, `-0`, storage edges, all checked
operations, scale mismatch, interval products across zero, exact/nonsquare
roots and every owner-specific live-bit ceiling.

### Stage C: physical cell-step migration only

Replace the physical module's private `Signed512` implementation with the
shared primitive while retaining its local geometry intervals and all public
types, functions, bytes, identities, errors, constants and resource receipts.
No optical owner changes in this package.

### Stage D: reassessment

After native, i686, Android, oracle, identity and full-Forge gates pass,
reassess whether to migrate the optical interface arithmetic before or after
the interval bulk extension. This decision uses measured diff size and reuse,
not aesthetic uniformity.

## Why physical migrates first

The physical interval module is the smaller consumer and already has a
five-family legacy freeze plus a narrow fixed-160 surface. Migrating it proves
the cross-crate error mapping, decimal behavior and platform dependency with
less blast radius than moving the adaptive 96/128/160 optical kernel first.

The interface module remains a differential oracle and fallback during this
stage. Its point-V1 and interval behavior are not touched merely to make the
repository look uniform.

## Failure points engineered out

| Failure | Permanent response |
|---|---|
| shared arithmetic starts owning physics or optics | semantic-neutral types and forbidden dependency/import gates |
| native limb representation leaks into identities | opaque magnitude and decimal-only boundary |
| one migration changes canonical bytes | pre-migration interval and exact-path fixtures, full replay after migration |
| error text/category drifts | explicit owner-side error mapping fixtures |
| precision policy moves into the foundation | arithmetic accepts explicit scale; owners retain allowed precisions and shields |
| feature unification enables allocation/default features | exact no-default-features pin and Cargo metadata verification |
| broad refactor hides the failing owner | migrate physical only, then reassess |
| dormant abstraction grows without use | first package includes one real protected consumer migration |
| later repair still needs multiple fixes | interval bulk must use the shared core; optical migration is separately ranked |

## Platform and performance gates

The shared crate and migrated physical consumer require warnings-denied Windows
x64 tests, executable i686 Windows tests and Android ARM64 compilation. A
fixed corpus must compare arithmetic outputs and physical event bytes across
x64 and i686. The 64-step cell-step portfolio may not regress its fixed work
ceiling or introduce unbounded allocation. Actual mobile-device performance
remains later promotion evidence.

## Authority boundary

This design audit authorizes no crate, manifest, dependency, migration,
interval bulk source or composer. The next action is a code-facing
implementation-readiness audit for exactly Stages A-C. Any prepared source
action must require explicit owner approval and delete the complete staged
package on identity, feature, arithmetic, platform or full-gate drift.
