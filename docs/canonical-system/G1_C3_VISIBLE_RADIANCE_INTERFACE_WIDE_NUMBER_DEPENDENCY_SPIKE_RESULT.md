# G1 C3 Visible-Radiance Interface Wide-Number Dependency Spike Result

Date: 2026-07-16

Status: **pinned `crypto-bigint` 0.7.5 fixed-width candidate supported for the
bounded adaptive-kernel reference; permanent Forge integration remains at an
explicit owner gate.**

## Result

The owner authorized a free, local, disposable dependency evaluation only. The
spike ran entirely under
`%TEMP%\forge-crypto-bigint-spike-v1` and did not add a dependency, crate or
schema to Forge. Forge `Cargo.toml` and `Cargo.lock` retained their pre-spike
SHA-256 values:

- `Cargo.toml`: `bcc38f1d3e241fcd3ebf4da9d531d8d9262c88aa88fb5a6e7dcbc777ccbc71a7`
- `Cargo.lock`: `49f83cd5ebc0385e3106a1e520466c401745ab0690e6b3c1d748f94fd30582b5`

The disposable manifest pinned `crypto-bigint = =0.7.5` with
`default-features = false`. Required fixed-width signed/unsigned construction,
checked multiplication and overflow rejection, 452-bit live values, unsigned
division/remainder reconstruction, integer floor root, signed checked
arithmetic and target-independent deterministic vectors passed.

## Two failures found and engineered out

The first compile rejected unsigned primitive constructors for `I512`. The
fixture now uses signed constructors, and future code must keep signed and
unsigned conversion boundaries explicit rather than relying on coercion.

The first signed floor-division fixture then exposed that
`checked_div_rem_floor_vartime` returns a remainder convention unsuitable for
Forge's `q * divisor + remainder = dividend` reconstruction invariant on a
negative dividend. The candidate therefore forbids that helper. Directed
division must operate on `U512` magnitudes, retain quotient and remainder, and
apply sign plus floor/ceiling adjustment in Forge-owned checked logic. Ordinary
signed truncating division was retained only as an adversarial comparison and
reconstructed correctly.

The initial second-target compile also rejected a checksum that read native
`as_words()` limbs directly into `u64`. On 32-bit targets the word is `u32`.
The repaired fixture reduces values through a fixed `U512` modulus before
reading the small remainder. This is now a permanent design shield: no codec,
identity, fixture or result may expose native limb count, width, order or
endianness.

These were disposable-harness defects/API hazards, not silent arithmetic
corruption. Both failed closed before Forge integration.

## Operation receipt

The final vector checked:

- correct and overflowing `U512` multiplication;
- a 452-bit live magnitude and checked product;
- 448-bit dividend division with exact quotient/remainder reconstruction;
- `floor_sqrt_vartime` lower and upper inequalities;
- correct and overflowing `I512` multiplication;
- signed truncating quotient/remainder reconstruction; and
- 2,048 deterministic mixed shift, floor-root, division and remainder cases.

Five repeated 64-bit runs and three final repaired runs were stable. The final
target-neutral checksum was `39042` on both targets.

| Target | Rust target | Word width | Final runs | Elapsed per 2,048-case vector |
|---|---|---:|---:|---:|
| Primary | `x86_64-pc-windows-msvc` | 64 | 3 | 1,997-2,086 microseconds |
| Second | `i686-pc-windows-msvc` | 32 | 3 | 4,921-4,965 microseconds |

The second target was installed through Rust's official toolchain component and
executed locally on Windows. It did not replace the active 64-bit toolchain.

## Build and size receipt

- clean optimized 64-bit build after the corrected constructor fixture:
  `4,649.8 ms`;
- warm unchanged optimized build: `58.6 ms`;
- final incremental rebuild after target-neutral checksum repair: `1.14 s`
  for x64 and `1.36 s` for i686;
- x64 executable: `160,256` bytes;
- i686 executable: `158,208` bytes.

These are disposable dependency/API measurements, not full adaptive-kernel
performance. The final module still requires its own cost receipt against the
three-evaluation/384-work-unit contract.

## Dependency, feature and license receipt

The normal pinned graph contains:

| Package | Version | Role | License | MSRV |
|---|---:|---|---|---:|
| `crypto-bigint` | 0.7.5 | fixed-width arithmetic | Apache-2.0 OR MIT | 1.85 |
| `cpubits` | 0.1.1 | target word-size selection | MIT OR Apache-2.0 | 1.85 |
| `ctutils` | 0.4.2 | checked/constant-time option utilities | Apache-2.0 OR MIT | 1.85 |
| `cmov` | 0.5.4 | conditional-move backend | Apache-2.0 OR MIT | 1.85 |
| `num-traits` | 0.2.19 | numeric traits | MIT OR Apache-2.0 | 1.60 |
| `autocfg` | 1.5.1 | build-time compiler detection only | Apache-2.0 OR MIT | 1.0 |

Cargo resolved no `crypto-bigint` feature. In particular, `alloc`, `serde`,
`rand_core`, `getrandom`, `rlp`, `der`, `subtle`, `zeroize` and
`hybrid-array` were not enabled. `BoxedUint` and the crate's `alloc` import are
both feature-gated and absent from the candidate surface.

Every registry package supplied matching MIT and Apache-2.0 license files. The
workspace uses rustc 1.97.0, above the graph's maximum declared 1.85 MSRV.
Production must pin the direct version and retain the exact lockfile checksums;
the upstream policy permits MSRV changes in patch releases, so an unreviewed
version float is forbidden.

RustSec records an older ARM32 constant-time issue in `cmov` as patched from
0.4.4; the resolved 0.5.4 is beyond that patched boundary. The kernel processes
public physical evidence rather than secrets, but dependency-advisory checks
remain mandatory at integration and promotion.

## Disposable provenance

The final disposable inputs were hashed before cleanup:

- `Cargo.toml`:
  `6d5d087bb21e3be05687e33ab51a1c37deb3d30a82990f4e7e1173a77e63908e`
- `Cargo.lock`:
  `01b67547cb00583ebfd86a2a0d05fab3105bae8a10d05b5ac39e79bd8f4d1e72`
- `src/main.rs`:
  `ff4cb9060da8dc153358fcc5320662048e990eea8207942646d0679b0de16efd`

The lock pinned registry checksums for all six external packages. The temporary
source and build products are not a second canonical implementation and may be
deleted after this receipt.

## Readiness consequence

The preferred representation condition passes with two binding restrictions:

1. use checked fixed-width `U512`/`I512` only, with allocation features disabled;
2. implement directed division from unsigned magnitude quotient/remainder and
   never serialize or compare native limbs.

This supports the isolated adaptive-kernel implementation described by
`G1_C3_VISIBLE_RADIANCE_INTERFACE_ADAPTIVE_KERNEL_IMPLEMENTATION_READINESS.md`.
It does not itself authorize permanent manifest changes or module creation.

## Remaining implementation gate

The exact action prepared for the owner is:

> Authorize the permanent additive `visible-radiance-interface-event` reference
> module and the exact pinned `crypto-bigint = 0.7.5`, default-features-disabled
> dependency under the readiness contract and spike restrictions. Require the
> complete independent Python portfolio, signed-magnitude directed division,
> dependency-neutral codec, x64/i686 vector replay, warnings-denied tests,
> measured module cost and rollback to `interface_model_required`. Grant no
> downstream path, passage, perception, rendering, biome, planet, terrain,
> runtime, promotion or C3-closure authority.

Stop until the owner explicitly releases that permanent integration action.

