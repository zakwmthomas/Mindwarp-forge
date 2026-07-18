# G1 C3 Visible-Radiance Interface Event Implementation Result

Date: 2026-07-16

Status: **passed as one additive capability-free local reference; C3 and all
downstream refractive-path, perception, rendering, passage, biome, planet and
runtime work remain open or excluded.**

## Result

The owner-authorized `visible-radiance-interface-event` crate now fills the
local typed boundary deliberately left by `visible-radiance-bulk-transfer`.
It reconstructs the exact physical volume and path witness, admits at most one
distinct positive-length shared-face crossing, requires an explicit face-bound
smooth-lossless-unpolarized-dielectric record, and returns only local red,
green and blue reflected/transmitted power and direction enclosures.

It does not modify the bulk-transfer crate and does not continue a refracted
path. Removing the new crate and exact dependency pin restores the prior
`interface_model_required` behavior without migration.

## Permanent arithmetic boundary

- exact TIR classification precedes roots and rounded branch decisions;
- fixed 512-bit checked storage uses pinned `crypto-bigint = 0.7.5` with
  default features disabled;
- exact normal-incidence, index-matched perfect-square and TIR perfect-square
  fast paths are internal and produce the same public result shape;
- the general ladder is exactly 96, 128 and 160 fractional bits;
- no event exceeds three evaluations or 384 fractional-bit work units;
- geometry whose squared delta exceeds 64 bits conservatively reaches 160
  before it can certify, preserving the independent hostile schedule;
- failure at the declared production cap is typed
  `nonconvergent_enclosure`; and
- overflow, zero division, invalid roots, empty intersections, branch changes,
  energy loss, unit-vector loss or Snell-containment loss are defects rather
  than physical outcomes.

## Failure points found and engineered out

Implementation testing found four defects or drift risks before recording:

1. Target-scale intersection could certify the extreme coprime geometry at
   128 bits while the retained independent schedule requires 160. A structural
   wide-geometry guard now prevents that premature certification.
2. The local integer-square-root initial estimate could begin below the root
   for values such as 25. It skipped valid exact fast paths. The estimate now
   begins above the root, with index-matched and TIR perfect-square regression
   fixtures.
3. Normal-incidence exact directions initially divided by one instead of the
   exact path length. Power was correct but direction magnitude was wrong. The
   fast path now uses the exact square root, with signed unit-direction checks.
4. A negative quotient smaller than one normalized to unsigned zero before the
   floor correction, changing `-1/2` from floor `-1` to `0`. The directed
   divider now retains the pre-normalization sign. The coprime-wide reflected
   direction changed from the wrong `[0,1]` to the independently required
   `[-1,0]`.

The fourth repair directly enforces the disposable spike's warning: signed
floor/ceiling division is defined from unsigned magnitudes plus explicit sign
and remainder adjustment. A source shield also prevents native-limb access
from entering the target-neutral decimal codec.

## Evidence

- Twelve warnings-denied Rust tests pass on executable x64 Windows.
- The same twelve tests pass as executable i686 Windows code, including the
  independent vector checksum and target-neutral codecs.
- Android ARM64 `cargo check` passes; actual device execution is retained as a
  later mobile promotion gate rather than fabricated from cross-compilation.
- A 25-case fixed/hostile portfolio matches the independent Python adaptive
  output checksum
  `3e595f04af1d9cb560dfe0dc684ca7ac0eec6597b15aa449cf0bc984b3cf2593`.
- A separate 1,024-case deterministic Rust portfolio exercises all embedded
  energy, unit-vector, Snell, width and typed-outcome postconditions.
- The retained independent Python exact, staged and adaptive oracles remain in
  the C3 integration gate.
- Clean optimized test compilation plus execution took 9.54 seconds on the
  primary PC; the optimized twelve-test execution itself took 0.14 seconds.
  The optimized test executable was 1,273,856 bytes. These are reference-test
  receipts, not a production workload claim.

## Data and codec shields

The input codec validates nested physical reconstruction before admission.
The declared face and media must match reconstructed cells; forged coefficients
or media are validation failures, not typed optical outcomes. Unknown fields,
noncanonical bytes, invalid provenance, unsupported coefficient ranges,
reversed intervals, oversized values and fabricated known widths fail closed.

Public wide endpoints are canonical signed decimal strings with explicit
Q0.48 or Q1.62 scale. There is no native limb, word-width or endianness in the
schema, identity or cross-target fixture.

## Platform disposition

The one semantic core is retained for PC and mobile. Current evidence is
executable x64 and i686 Windows plus Android ARM64 compilation. Real Android
and iOS device execution, performance, packaging and thermal evidence remain
future promotion gates. PlayStation remains a strongly desired later backend;
Xbox and Mac are lower-priority; Linux is opportunistic. None of those
lower-ROI targets blocks this engine-neutral reference, and no platform fork is
introduced without a demonstrated priority-target requirement.

## Whole-plan alignment and next route

This result advances only C3 observer-independent local optical opportunity.
It creates no sphere or planet and does not change the biome rule: continuous
causal fields require deterministic ecotones, while a sharp presentation seam
requires a sharp physical cause.

The next safe action is a post-interface consumer reassessment against the
master plan. It must decide whether another bounded C3 substrate is genuinely
the highest-information blocker. This result does not authorize downstream
path composition, generic passage, perception, rendering, navigation, runtime,
promotion or C3 closure.
