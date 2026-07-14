# Field Basis Numerical Policy Design Gate

**State:** approved and implemented as a bounded reference prototype after
owner confirmation. **Date:** 2026-07-13.

## Question and evidence

The permanent decision is not which noise picture looks best. It is which
numerical result may participate in reconstruction fingerprints and cache keys
without changing across CPU, GPU, compiler, optimization, or traversal order.

Primary and maintainer evidence reviewed:

- Random123 defines counter-based generators as stateless functions of a key
  and counter. Its `Philox4x32` family returns four 32-bit words, names the
  round count, includes tests, and recommends testing the implementation before
  use: <https://www.thesalmons.org/john/random123/releases/latest/docs/CBRNG.html>.
- Rust documents that NaN-producing floating-point operations can choose
  different bit patterns across executions, compiler versions, and flags:
  <https://doc.rust-lang.org/stable/core/primitive.f32.html>.
- WGSL permits implementation-dependent rounding, flushing some subnormal
  values to zero, finite-math assumptions, and multiply/add fusion differences:
  <https://www.w3.org/TR/WGSL/#floating-point-evaluation>.
- Ken Perlin's improved-noise reference sought consistent implementations and
  corrected interpolation/gradient artifacts, but its reference code uses
  floating-point math and a fixed permutation table:
  <https://mrl.cs.nyu.edu/~perlin/noise/>.
- FastNoise Lite is a strong MIT-licensed reuse and comparison candidate with
  many basis families and ports, but it supports float/double implementations
  rather than promising byte-identical cross-platform canonical output:
  <https://github.com/Auburn/FastNoiseLite>.
- PCG's minimal API advances mutable state. It is compact and fast, but mapping
  random access and parallel traversal onto state/sequence positions adds an
  avoidable scheduling contract:
  <https://www.pcg-random.org/using-pcg-c-basic.html>.

## Recommended bounded lane

1. **Canonical reference math:** signed fixed-point only. Coordinates use
   `Q32.32` in `i64`; scalar terms and outputs use `Q16.48` in `i64`; every
   multiply/divide uses checked `i128` intermediates and round-to-nearest,
   ties-to-even. Overflow, zero division, invalid shift, or out-of-domain input
   fails before producing a packet. There is no saturation or wrapping.
2. **Canonical recipe encoding:** the strict deterministic-CBOR profile already
   used by universe identity: definite arrays, unsigned/signed integers and
   byte strings only. No maps, floats, text identifiers, implicit defaults, or
   executable expressions participate in a fingerprint.
3. **Bulk random access:** propose `Philox4x32-10`, with the exact family,
   rounds, word order, byte order, and key/counter mapping versioned in the
   contract. The universe-identity stream key supplies key material. A field
   counter contains bounded local lattice coordinates, octave, component, and
   lane; no mutable global PRNG state exists.
4. **First reference alphabet:** constant, affine transform, 2D/3D value-lattice
   basis with quintic interpolation, add, multiply, min, max, absolute/ridged
   remap, bounded fractal sum, and bounded domain warp. Trigonometric waves,
   square roots, normalized gradients, blue noise, temporal fields, and
   arbitrary graphs remain out until each has an exact numerical contract.
5. **Canonical comparison:** exact canonical bytes and SHA-256 fingerprints.
   Tolerance applies only to explicitly non-canonical preview/accelerated
   outputs and can never admit a cache entry or reconstruction proof.
6. **Cache key:** domain-separated SHA-256 over identity/reconstruction
   fingerprint, field contract version, exact recipe bytes, sample-domain
   bytes, output descriptor, and generator family/version. Cache contents are
   disposable and never authority.

The first implementation is an engine-neutral, capability-free CPU reference.
It is not a production generator, visual-quality approval, runtime choice, or
performance claim.

## Internal challenge and alternatives

| Alternative | Advantage | Why it is not the canonical v1 recommendation |
|---|---|---|
| FastNoise Lite directly | Mature breadth, many language ports, useful preview tool | Float/double and port/compiler behavior do not establish exact canonical bytes; use for visual and speed comparisons |
| Improved Perlin directly | Small public reference and understood smoothness | Floating reference, fixed table periodicity, and insufficient recipe/cache contract |
| Native `f32` plus epsilon | Fast and maps naturally to GPUs | Epsilon equality is not transitive, does not define stable hashes, and WGSL/Rust explicitly retain divergent edge behavior |
| Software IEEE float | Could make operations exact under one implementation | Adds a large permanent math runtime and still needs fixed transcendentals; cost is unjustified for the smallest proof |
| HMAC counter blocks for all samples | Already fixed-vector tested and simple | Correct but likely wasteful for dense fields; retain as an oracle/key boundary, not an unmeasured bulk choice |
| PCG/xoshiro stateful stream | Compact and fast | Traversal, cancellation, parallelism, and random access must reproduce mutable stream positions |
| Threefry or ChaCha counter blocks | Credible stateless alternatives | Threefry needs the same mapping/vector decision; ChaCha adds cryptographic work not required by field sampling. Benchmark against Philox before confirmation |

Fixed-point is not automatically high quality. It can introduce quantisation,
overflow pressure, interpolation bias, and CPU/GPU cost. Those risks are why
the reference alphabet is small and every accelerated path must be compared to
the exact reference rather than silently replacing it.

## Required proof before `prototype_tested` may advance

- Committed Random123-compatible Philox known-answer vectors plus Forge
  key/counter-mapping vectors for zero, negative, boundary, octave, component,
  and adjacent-lattice inputs.
- Exact recipe bytes and fingerprints for constant, transform, interpolation,
  reordered composition, fractal, ridged, and domain-warp fixtures.
- Poison cases for overflow, division by zero, excess terms/octaves/warp depth,
  unknown tags/versions, malformed CBOR, coordinate escape, and invalid ranges.
- Warm, cold, disposed, corrupt, conflicting, and version-stale cache cases;
  all canonical samples remain cache-independent.
- Cross-seed separation, tile-boundary continuity, range, histogram, spectral,
  aliasing, interpolation monotonicity, and quantisation-bias reports.
- Measured scalar throughput and memory on the reference machine; comparative
  Philox/Threefry/HMAC and FastNoise Lite results labelled non-canonical.
- Fresh-process replay and at least one second-platform/language receipt before
  `reference_proven`.
- A real ProofReceipt projection that cannot mutate Kernel authority.

## Failure and recovery

| Failure | Required behavior |
|---|---|
| Numeric overflow or invalid operation | Fail with term/sample location; never wrap or saturate |
| Unknown recipe, math, or generator version | Reject before sampling; retain version-mismatch evidence |
| Cache miss/corruption/conflict | Discard and recompute; quarantine conflicting bytes |
| Accelerated output differs from reference | Mark accelerator incompatible; retain reference result and diagnostic |
| Bias, discontinuity, or spectral regression | Fail the named quality fixture; do not hide it behind exact determinism |
| Old implementation unavailable | Block reconstruction or use an explicit migration receipt; never use newest semantics silently |
| Cost exceeds the bounded proof budget | Reduce alphabet/sample scope or compare another generator; do not weaken determinism |

## Exact confirmation gate

Confirm the bounded fixed-point reference lane and the proposed
`Philox4x32-10` mapping package, or request a float/software-float/alternative
counter-generator lane. Confirmation authorizes only the capability-free
reference contract, vectors, fixtures, measurements, and ProofReceipt
integration. It does not authorize a runtime, GPU path, production promotion,
protected-Kernel mutation, arbitrary procedural content, spending, or
publishing.

## Approval and bounded result

The owner approved the recommended lane on 2026-07-13. The capability-free
`field-basis` crate now implements the strict recipe codec, checked fixed-point
reference math, Philox4x32-10 mapping, exact fingerprints/cache keys, and eight
fixtures for known answers, canonical bytes, repeatability/partitioning,
continuity, composition, poison/overflow, cache binding, and authority-negative
evidence. A desktop integration fixture persists that evidence through the
read-only ProofReceipt projection without adding Kernel events or candidates.

This result is `prototype_tested`, not `reference_proven`: the v1 alphabet is
intentionally small, comparative quality/cost expansion and a second-platform
receipt remain open, and no runtime/GPU path is authorized.
