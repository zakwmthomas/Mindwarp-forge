# G1 C3 quality/cost/range/candidate portfolio result

Status: **disposable-simulation-tier evidence recorded; C3 promotion remains an
owner decision.**

## Exact next action addressed

`context/active/WORKER_BATCH_STATE.json` next action: "Design and run a
bounded quality/cost, causal-range and regional-variation portfolio; evaluate
only locally applicable natural-method candidates against simple baselines
before deciding whether C3 is promotion-ready." This result records that
portfolio and its outcome. It does not itself promote C3.

## Evidence source

- `crates/field-basis/examples/quality_cost_range_candidate.rs`
- `crates/derived-world-rules/examples/range_regional_cost_portfolio.rs`

Both are disposable, capability-free CPU-reference binaries run once per this
record (`cargo run --release -p <crate> --example <name>`). They are evidence
only: no visual, aesthetic, GPU, production-performance, or cross-platform
claim is made.

## field-basis: quality, range, regional variation and one P16 candidate case

- **Range:** the maximum-term (63-term, 32-octave) recipe overflows in a
  controlled, non-panicking way at extreme coordinates (`Overflow` error,
  fixture-local). A single-lattice recipe returns finite, non-panicking
  values at all four tested extreme `i64` coordinate pairs. A fine-grained
  sweep across one lattice-cell boundary (step size `2^24` against a `2^32`
  cell) showed zero observed change at permille rounding resolution; this is
  a resolution/rounding limitation of the diagnostic, not a claim of true
  discontinuity-free smoothness.
- **Regional variation:** across 8 distinct stream keys with an identical
  4-octave recipe and grid, distinct-bucket occupancy ranged 203-210 of 256
  and no two regions produced an identical `(occupancy, mean, stddev)` triple.
  Regions are statistically distinguishable on this bounded fixture; this is
  not a claim about perceptual or aesthetic distinctness.
- **P16 candidate case — fractal octave-sum vs. single-lattice baseline:**
  - Hypothesis: a 6-octave halving-amplitude lattice sum increases bounded
    value-occupancy over one lattice term at bounded extra cost.
  - Target-local metric: distinct occupied buckets (of 256) over a fixed
    64x64 grid, plus mean/stddev.
  - Baseline: 128/256 buckets occupied, throughput ~10.5M samples/s.
  - Candidate: 210/256 buckets occupied, throughput ~2.4M samples/s.
  - Occupancy gain 1.64x; cost ratio 4.39x.
  - Falsifier: locally supported only if occupancy gain >= 1.3x and cost
    ratio <= 20x. **Result: locally supported** on this fixture.
  - Non-applicable scope: this says nothing about visual quality, spectral
    correctness, aesthetic-grammar fitness, GPU cost, or any other grid, key,
    or recipe family.

## derived-world-rules: causal range, regional variation and cost

- **Range:** a 5x5x5 = 125-case sweep of stellar irradiance, atmosphere
  transmission and dominant reflectance across `{0, 1, 500, 999, 1000}`
  permille produced 125/125 successful compiles with every resulting palette
  channel inside the required `[0, 1000]` permille bound and zero unexpected
  errors.
- **Regional variation / provenance separation:** 32 distinct
  `reconstruction_id` values with identical physical drivers produced 32
  distinct `input_id` values and exactly 1 distinct physical palette,
  extending the single-case unit test to a 32-point portfolio. Provenance
  changes identity without fabricating a physical difference, robustly
  across this range.
- **Causal coordinate variation:** 32 coordinates under one reconstruction
  and one canonical lattice recipe produced 31 distinct exposure values and
  30 distinct physical palettes. Coordinate variation therefore changes
  existing output rather than only changing provenance.
- **Cost:** the latest post-signal-potential run observed ~3,530
  `compile_world` calls/second over 50,000 varied inputs. This includes strict
  nested recipe and contract replay and is an observation, not a stable
  benchmark or production/cross-platform performance claim.
- **P16 applicability:** none. The v1 contract selects no diffusion, SDF,
  Voronoi or branching mechanism, so no candidate-versus-baseline comparison
  applies to this module; the contract text already states this.

## Retained limits

This is still a synthetic, single-machine, capability-free reference
portfolio, not scientific validation, a second-platform receipt, a
production-performance measurement, or a visual/aesthetic judgment. Both
example binaries are disposable and may be re-run but are not part of the
gating test suite. Focused gates separately cover field basis, seven regional
environment cases and ten derived-world cases.

## Readiness recommendation (evidence-only, not a promotion)

The requested bounded quality/cost/range/candidate portfolio is complete for
both modules. The field-basis P16 case was locally supported; derived-world-
rules range and regional-variation evidence passed at n=125 and n=32 without
a single unexpected failure. This is evidence for the owner's future C3
decision; it does not itself promote any module beyond `prototype_tested`.
