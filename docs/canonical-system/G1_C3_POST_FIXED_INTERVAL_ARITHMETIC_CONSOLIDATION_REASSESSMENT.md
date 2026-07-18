# G1 / C3 post-fixed-interval-arithmetic consolidation reassessment

Date: 2026-07-16

Status: **one-band interval bulk implementation readiness selected; optical
arithmetic migration remains optional maintenance and is not a prerequisite;
no source action authorized.**

## Decision

Proceed next with a code-facing implementation-readiness audit for the
additive one-band conditional interval bulk-transfer surface. Do not migrate
`visible-radiance-interface-event` first.

The selected route advances the missing local attenuation operation required
before any ordered optical composition can be designed. The shared
`fixed-interval-arithmetic` API already supplies the interval-bulk oracle's
wide operations: canonical signed-decimal reconstruction, checked signed-512
arithmetic, explicit-scale interval addition/subtraction/multiplication,
directed square root, sound intersection, outward projection and live-bit
accounting. Interval bulk can therefore consume the existing physical
cell-step evidence and reuse the existing bulk owner's Q64.64 optical-depth
and Q0.48 exponential kernel without depending on the optical interface
owner.

Optical migration removes a duplicated private arithmetic implementation but
does not supply a missing interval-bulk prerequisite. It is retained as a
separate maintenance candidate after the closure-bearing local operation is
proved.

## Evidence comparison

| Criterion | One-band interval bulk readiness | Optical arithmetic migration |
|---|---|---|
| C3 closure dependency | Supplies the missing local attenuation receipt before lineage or composition can replay one spectral lane | Supplies no new optical operation or composition evidence |
| Prerequisite state | Independent oracle passed; shared 512-bit interval core and physical cell-step consumer passed | Point and interval interface implementations already pass with their private arithmetic |
| Shared-core fit | Required parse, shift, multiply, add, subtract, root, intersection, projection and bit accounting are present | Base operations overlap, but optical code also requires interval division, unsigned-ratio construction, unit intersection, overlap/containment helpers, projected-interval certification, unsigned U512 products and owner-local 96/128/160 adaptive policy |
| Compatibility surface | Additive bulk-owned schema and dependency; existing exact three-band bulk V1 can remain byte- and identity-locked | Replaces a 429-line private arithmetic module used across the 1,344-line point interface and 852-line conditional interval surface |
| Rollback | Delete the additive interval surface and direct shared-core dependency; retain existing bulk V1 and oracle | Restore the private optical arithmetic module and dependency after proving all point-V1 and interval identities unchanged |
| Measured maintenance value | Prevents a third signed-512 implementation immediately and reuses the existing bulk exponential kernel | Removes one remaining duplicate later, but the exact source reduction is unproved because several optical-only helpers and policies must remain local or require a separately reviewed shared API extension |
| Information gain per bounded package | High: resolves whether the proved local oracle can become an exact owner-gated implementation package | Lower now: proves refactor compatibility but leaves the same closure operation missing |

## Why optical migration is not a blocker

The interval-bulk candidate consumes public decimal interval evidence from
`physical-path-substrate`, not private optical arithmetic values. Direction
Q1.62 endpoints can be lifted to Q160 with checked shifts; point, certified
time and hit-point boxes already use Q160. The selected dual length
certificate then uses only shared-core operations before projection into the
bulk owner's existing narrow optical-depth kernel.

No interface-event type, adaptive stopping rule, Fresnel branch, point-V1
identity or conditional interval-interface identity is needed to compute that
local bulk receipt. Adding an optical dependency would invert ownership and
is forbidden. Migrating optics first would therefore add compatibility work,
not close a prerequisite.

## Code-facing readiness obligations

The next audit must freeze, without implementing:

1. one additive bulk-owned input/event schema binding the validated bulk
   profile, one band, current cell, physical cell-step input and event;
2. exact treatment of certified, outer-exit, unavailable-neighbour,
   unavailable-current-cell, ambiguous and no-progress outcomes;
3. Q62-to-Q160 lifting, the 414-magnitude-bit wide intermediate shield, final
   below-192-bit length intersection and projection into the existing Q64.64
   optical-depth path;
4. error mapping from shared arithmetic into bulk-owned typed failures without
   leaking shared error text into canonical bytes;
5. permanent exact byte/identity fixtures for all existing bulk V1 families
   before any additive source change;
6. codec, event, work, allocation and 64-step lane ceilings;
7. direct dependency and module-boundary updates that do not make the optical
   interface an upstream dependency;
8. native x64, executable i686 Windows, Android ARM64, independent Python
   oracle, downstream, module and complete-Forge gates; and
9. deletion-only rollback on any arithmetic, identity, feature, platform,
   oracle or full-gate drift.

## Deferred optical maintenance gate

Optical migration should be reconsidered only after the interval-bulk package
is resolved, or earlier if a concrete arithmetic defect requires multi-owner
repair. Its future audit must measure actual removable source and call-site
changes, decide whether shared division/ratio/projected helpers belong in the
semantic-neutral core or stay in an optical adapter, preserve the exact
`crypto-bigint` feature set, and protect every point-V1 and conditional
interval byte/identity family across x64, i686 and Android gates.

Repository uniformity alone is not sufficient value. No shared API expansion
is selected by this reassessment.

## Authority and nonclaims

This reassessment authorizes only the next implementation-readiness audit. It
does not authorize interval bulk source, optical migration, a shared API
extension, schema or manifest changes, lineage, composition, coefficient
discovery, perception, rendering, gameplay visibility, collision, navigation,
organism, biome, sphere, planet, terrain, runtime, promotion or C3 closure.

