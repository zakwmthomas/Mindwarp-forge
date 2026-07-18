# G1 / C3 fixed-interval arithmetic consolidation implementation readiness

Date: 2026-07-16

Status: **implementation-ready behind one exact owner action; no arithmetic
crate, manifest change or consumer migration has been performed.**

## Bounded action

The prepared package adds one semantic-neutral
`crates/fixed-interval-arithmetic` crate and migrates only the private
signed-512 primitive used by `physical-path-substrate`'s conditional interval
cell step. It does not change the optical interface owner, add interval bulk,
or alter any semantic public contract.

Before compiling the new dependency into the physical crate, the package must
capture seven permanent conditional cell-step byte/ID families defined by the
design audit. The existing five exact-path V1 families remain unchanged.

## Frozen dependency and API

The new crate has exactly one dependency:

`crypto-bigint = { version = "=0.7.5", default-features = false }`

It adds one local workspace package but no new resolved external package,
version or feature. Its API is limited to opaque `Signed512`, ordered
`FixedInterval`, checked arithmetic, directed division/root/projection,
canonical decimal conversion and magnitude-bit inspection. It exposes no
native limbs or semantic serialization.

The crate must compile without `serde`, `serde_json`, `sha2`, Forge Kernel,
Tauri/UI, filesystem, network, process or persistence dependencies. Arithmetic
operations allocate no dynamic collections. Canonical decimal parsing and
formatting may allocate only their bounded string representation.

## Compatibility and error mapping

The physical migration may change private implementation paths only. It must
retain exactly:

- every existing public type, function and constant;
- both interval domain separators;
- 16/32 KiB codec caps;
- the 414/512-bit cell-step shield and work receipt;
- canonical signed decimal spelling;
- all typed outcomes and semantic limitations;
- all seven new interval event byte/ID fixtures; and
- all five legacy exact-path V1 byte/ID fixtures.

Shared `FixedArithmeticError` values map to the existing
`PhysicalPathError::Invalid` arithmetic categories. No shared error string may
enter canonical bytes. Tests must prove canonical event and input objects are
unchanged before and after migration.

## Arithmetic proof surface

Permanent shared-crate tests cover:

1. zero normalization and signed ordering;
2. decimal round trip plus empty, plus, whitespace, leading-zero, negative-zero
   and over-storage poison;
3. add/subtract cancellation and 512-bit overflow;
4. multiply and shift boundaries;
5. floor/ceiling division for every sign/remainder quadrant and zero divisor;
6. interval order and scale mismatch;
7. four-corner outward multiplication across sign changes;
8. exact and nonsquare directed square roots;
9. precision projection direction and rejection of precision increase;
10. magnitude-bit accounting at 414 and storage edges; and
11. absence of native-limb or capability imports.

Differential fixtures must match the retained physical implementation before
its deletion and the retained optical arithmetic vectors without migrating
the optical crate.

## Integration and platform gates

The exact package must pass:

- warnings-denied native tests for the shared and physical crates;
- executable `i686-pc-windows-msvc` tests for both crates;
- `aarch64-linux-android` compilation for both crates;
- all physical cell-step and exact-path identity fixtures;
- the independent cell-step and one-band interval bulk Python oracles;
- unchanged bulk, interface and swept-AABB downstream suites;
- Cargo metadata confirmation of no new external package/version/feature;
- module-boundary and module-context verification; and
- the complete Forge gate.

Actual Android-device execution and performance remain later promotion
evidence. No platform semantic fork is authorized.

## Rollback

On any identity, arithmetic, error-mapping, feature, platform, dependency or
full-gate failure, delete the new crate and its governance entries, remove the
physical dependency, and restore the retained private physical arithmetic
source. No data migration exists because public bytes and identities are
required to remain unchanged.

## Exclusions

This package does not authorize optical arithmetic migration, interval bulk
implementation, path lineage, composition, coefficients, perception,
rendering, collision, navigation, organism, biome, sphere, planet, terrain,
runtime, promotion or C3 closure.

## Exact owner action

Approve one test-first staged consolidation package that:

1. captures seven physical interval cell-step byte/ID families before source;
2. adds only the semantic-neutral `fixed-interval-arithmetic` crate with the
   exact already-resolved `crypto-bigint 0.7.5` no-default-features dependency;
3. implements and differentially verifies the frozen opaque arithmetic API;
4. migrates only `physical-path-substrate`'s private signed-512 primitive;
5. preserves every physical public byte, identity, error category, work limit
   and existing downstream result;
6. runs x64, i686, Android, oracle, module and full-Forge gates; and
7. rolls back the complete package on any drift.

General continuation does not authorize this source action. Explicit owner
approval of this exact package is required.
