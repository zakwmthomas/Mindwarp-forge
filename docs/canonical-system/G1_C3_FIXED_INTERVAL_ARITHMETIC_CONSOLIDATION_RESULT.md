# G1 / C3 fixed-interval arithmetic consolidation result

Date: 2026-07-16

Status: **owner-authorized reversible experiment passed; semantic-neutral core
implemented and only the physical cell-step migrated; optical arithmetic and
interval bulk remain unchanged.**

## Result

The workspace now contains `fixed-interval-arithmetic`, a capability-free core
for opaque signed-512 values, canonical decimals, checked directed arithmetic,
explicit-scale intervals, intersection, projection, square root and live-bit
inspection. Its only dependency is the exact already-resolved
`crypto-bigint = 0.7.5` with default features disabled.

`physical-path-substrate` now imports that primitive and retains a narrow local
adapter mapping shared arithmetic defects into its existing physical error
categories and 414-bit shield. Its geometry intervals, schemas, domain
separators, codecs, work receipts and outcomes remain locally owned.

`visible-radiance-interface-event` was deliberately not migrated. Interval
bulk was not implemented. The staged result therefore proves one real
consumer without hiding multiple owner changes inside one refactor.

## Compatibility proof

Before migration, seven physical interval cell-step families captured
canonical input/event byte lengths and SHA-256 values plus recipe, volume,
input and event identities for normal, reverse, ambiguous, no-progress,
unavailable-neighbour, near-parallel and negative/near-maximum states.

After migration all seven pass exactly. The five older exact-path V1 families
also remain byte- and identity-identical. No public physical type, function,
constant, codec cap, domain separator, limitation or outcome changed.

## Arithmetic and platform receipt

- Four shared-core suites cover canonical decimal poison, all signed division
  quadrants, storage boundaries, interval order/scale, signed products,
  exact/nonsquare roots, intersection, projection, bit accounting and
  native-limb/capability shields.
- Native Windows warnings-denied tests pass for the shared and physical crates.
- Executable i686 Windows tests pass for both crates and all physical identity
  fixtures.
- Android ARM64 compilation passes for both crates.
- The unchanged physical suite passes 13 legacy, one exact-path lock and six
  interval tests.
- Swept AABB, radiance bulk and radiance interface downstream suites pass
  unchanged.
- The retained cell-step and one-band interval-bulk Python oracles pass.
- Cargo retains one resolved external `crypto-bigint` package; the change adds
  one local workspace package and no new external package, version or feature.

The physical debug suite remained approximately 1.3 seconds on the current
PC, which detects no obvious regression but is not a production benchmark.
Actual mobile-device execution and profiling remain later promotion evidence.

## Failure containment

- Native limbs cannot enter public values or identities.
- Semantic owners retain precision lists, bit shields, codecs and errors.
- The physical adapter is the only migration-specific layer.
- A permanent verifier rejects reintroduced private physical signed-512 code,
  dependency drift, missing identity families and capability imports.
- The complete package remains deletion-only reversible because no stored data
  or public identity migrated.

## Next route and nonclaims

The experiment supports, but does not itself authorize, the one-band interval
bulk extension. The next safe action is a post-consolidation consumer
reassessment comparing interval bulk implementation readiness against optional
optical arithmetic migration. Based on current evidence, interval bulk has the
direct closure dependency while optical migration is maintenance work and
should not be performed merely for visual uniformity.

This result does not authorize interval bulk source, optical migration, path
lineage, composition, coefficients, perception, rendering, collision,
navigation, organism, biome, sphere, planet, terrain, runtime, promotion or C3
closure.
