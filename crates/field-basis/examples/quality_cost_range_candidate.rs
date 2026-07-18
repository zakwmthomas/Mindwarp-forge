//! Disposable-simulation-tier evidence for G1-C3: bounded quality, cost,
//! causal-range and regional-variation portfolio, plus one P16 scoped
//! candidate-versus-baseline comparison for the fractal-octave-sum
//! technique against a single-lattice baseline.
//!
//! This binary is evidence only. It establishes no visual quality,
//! aesthetic fitness, spectral correctness, production runtime, or
//! cross-platform claim. All measurements are CPU-reference, single-machine,
//! disposable diagnostics over the existing canonical `sample` function.

use field_basis::{FieldError, FieldRecipe, ONE, Term, sample};
use serde::Serialize;
use std::time::Instant;

const GRID: i64 = 64;
const BUCKETS: usize = 256;

fn octave_recipe(octaves: usize) -> FieldRecipe {
    assert!((1..=32).contains(&octaves));
    let mut terms = Vec::new();
    let mut idx = Vec::new();
    let mut amplitude = ONE / 2;
    for o in 0..octaves {
        let frequency = 1u32 << o;
        terms.push(Term::ValueLattice2 {
            frequency,
            amplitude,
            component: o as u32,
        });
        idx.push((terms.len() - 1) as u16);
        amplitude = (amplitude / 2).max(1);
    }
    let mut acc = idx[0];
    for &i in &idx[1..] {
        terms.push(Term::Add {
            left: acc,
            right: i,
        });
        acc = (terms.len() - 1) as u16;
    }
    FieldRecipe::new(terms, acc).expect("octave recipe must be valid")
}

#[derive(Serialize)]
struct RegionMetric {
    distinct_buckets: usize,
    mean: f64,
    stddev: f64,
}

fn region_metric(recipe: &FieldRecipe, key: [u8; 32]) -> RegionMetric {
    let mut buckets = [0u32; BUCKETS];
    let mut sum = 0f64;
    let mut sumsq = 0f64;
    let mut n = 0f64;
    for gy in 0..GRID {
        for gx in 0..GRID {
            let x = gx << 32;
            let y = gy << 32;
            let v = sample(recipe, key, x, y).expect("bounded grid must not overflow");
            let vf = v as f64 / ONE as f64;
            sum += vf;
            sumsq += vf * vf;
            n += 1.0;
            let clamped = vf.clamp(-1.0, 1.0);
            let bucket = (((clamped + 1.0) / 2.0) * (BUCKETS as f64 - 1.0)).round() as usize;
            buckets[bucket.min(BUCKETS - 1)] += 1;
        }
    }
    let distinct = buckets.iter().filter(|&&c| c > 0).count();
    let mean = sum / n;
    let variance = (sumsq / n - mean * mean).max(0.0);
    RegionMetric {
        distinct_buckets: distinct,
        mean,
        stddev: variance.sqrt(),
    }
}

#[derive(Serialize)]
struct CostMeasurement {
    samples: u64,
    elapsed_seconds: f64,
    samples_per_second: f64,
}

fn measure_cost(recipe: &FieldRecipe, key: [u8; 32], samples: u64) -> CostMeasurement {
    let start = Instant::now();
    let mut acc: i64 = 0;
    for i in 0..samples {
        let x = (i as i64) << 20;
        let y = ((i as i64) * 7) << 20;
        acc ^= sample(recipe, key, x, y).expect("cost loop must not overflow");
    }
    std::hint::black_box(acc);
    let elapsed = start.elapsed().as_secs_f64();
    CostMeasurement {
        samples,
        elapsed_seconds: elapsed,
        samples_per_second: if elapsed > 0.0 {
            samples as f64 / elapsed
        } else {
            f64::INFINITY
        },
    }
}

#[derive(Serialize)]
struct RangeCase {
    description: &'static str,
    outcome: String,
}

fn range_portfolio() -> Vec<RangeCase> {
    let mut cases = Vec::new();

    let max_term_recipe = octave_recipe(32);
    let outcome = match sample(&max_term_recipe, [9; 32], 1_i64 << 40, -(1_i64 << 40)) {
        Ok(v) => format!("ok value_permille={}", (v * 1000) / ONE),
        Err(e) => format!("controlled_err {e:?}"),
    };
    cases.push(RangeCase {
        description: "max-term (63-term, 32-octave) recipe at large positive/negative coordinates",
        outcome,
    });

    let single = octave_recipe(1);
    let extreme_coords: [(i64, i64); 4] = [
        (i64::MAX >> 8, i64::MAX >> 8),
        (i64::MIN >> 8, i64::MIN >> 8),
        (0, 0),
        (-(1_i64 << 40), 1_i64 << 40),
    ];
    for (x, y) in extreme_coords {
        let outcome = match sample(&single, [1; 32], x, y) {
            Ok(v) => format!("ok value_permille={}", (v * 1000) / ONE),
            Err(FieldError::Overflow) => "controlled_err Overflow".to_string(),
            Err(e) => format!("controlled_err {e:?}"),
        };
        cases.push(RangeCase {
            description: "single-lattice sample at extreme i64 coordinate (no panic required)",
            outcome,
        });
    }

    let mut prev: Option<i64> = None;
    let mut max_cell_jump_permille: i64 = 0;
    for step in -4..=4 {
        let x = (1_i64 << 32) + (step * (1_i64 << 24));
        let v = sample(&single, [1; 32], x, 0).unwrap();
        if let Some(p) = prev {
            max_cell_jump_permille = max_cell_jump_permille.max(((v - p).abs() * 1000) / ONE);
        }
        prev = Some(v);
    }
    cases.push(RangeCase {
        description: "max observed step-to-step permille change while sweeping across a lattice cell boundary (bounded continuity, not a smoothness proof)",
        outcome: format!("max_jump_permille={max_cell_jump_permille}"),
    });

    cases
}

#[derive(Serialize)]
struct CandidateBaselineResult {
    hypothesis: &'static str,
    mathematical_abstraction: &'static str,
    assumptions: &'static str,
    target_local_metric: &'static str,
    baseline: RegionMetric,
    baseline_cost: CostMeasurement,
    candidate: RegionMetric,
    candidate_cost: CostMeasurement,
    falsifier: &'static str,
    locally_supported: bool,
    non_applicable_scope: &'static str,
}

fn candidate_vs_baseline() -> CandidateBaselineResult {
    let key = [42; 32];
    let baseline_recipe = octave_recipe(1);
    let candidate_recipe = octave_recipe(6);
    let baseline = region_metric(&baseline_recipe, key);
    let candidate = region_metric(&candidate_recipe, key);
    let baseline_cost = measure_cost(&baseline_recipe, key, 200_000);
    let candidate_cost = measure_cost(&candidate_recipe, key, 200_000);

    let occupancy_gain =
        candidate.distinct_buckets as f64 / baseline.distinct_buckets.max(1) as f64;
    let cost_ratio = if baseline_cost.samples_per_second > 0.0 {
        baseline_cost.samples_per_second / candidate_cost.samples_per_second.max(1.0)
    } else {
        f64::INFINITY
    };
    let locally_supported = occupancy_gain >= 1.3 && cost_ratio <= 20.0;

    CandidateBaselineResult {
        hypothesis: "fractal octave-sum (6 lattice octaves, halving amplitude, added) increases bounded value-occupancy over a single lattice term at bounded extra cost",
        mathematical_abstraction: "sum_{o=0..5} amplitude(o) * lattice(frequency=2^o, component=o), amplitude(o) = ONE/2^(o+1) floor-clamped to >=1",
        assumptions: "fixed 64x64 grid, fixed stream key, canonical fixed-point reference math only; no claim about any other grid size, key, or accelerated implementation",
        target_local_metric: "distinct occupied buckets out of 256 over a fixed 64x64 grid (occupancy), plus mean/stddev of sampled permille value",
        baseline,
        baseline_cost,
        candidate,
        candidate_cost,
        falsifier: "hypothesis is locally supported only if candidate occupancy >= 1.3x baseline occupancy AND candidate is no more than 20x baseline cost on this fixture; otherwise retain the single-lattice baseline for this fixture",
        locally_supported,
        non_applicable_scope: "does not establish visual quality, aesthetic fitness, spectral correctness, biome/aesthetic-grammar applicability, GPU/runtime cost, or any cross-platform result; applies only to this fixed grid/key/recipe family on this reference implementation",
    }
}

#[derive(Serialize)]
struct RegionalVariationCase {
    stream_key_byte: u8,
    metric: RegionMetric,
}

#[derive(Serialize)]
struct Report {
    range_portfolio: Vec<RangeCase>,
    regional_variation: Vec<RegionalVariationCase>,
    regions_are_distinct: bool,
    candidate_vs_baseline: CandidateBaselineResult,
}

fn main() {
    let range_portfolio = range_portfolio();

    let recipe = octave_recipe(4);
    let mut regional_variation = Vec::new();
    for key_byte in 1u8..=8 {
        let metric = region_metric(&recipe, [key_byte; 32]);
        regional_variation.push(RegionalVariationCase {
            stream_key_byte: key_byte,
            metric,
        });
    }
    let mut seen = std::collections::HashSet::new();
    let mut regions_are_distinct = true;
    for case in &regional_variation {
        let key = (
            case.metric.distinct_buckets,
            (case.metric.mean * 1_000_000.0).round() as i64,
            (case.metric.stddev * 1_000_000.0).round() as i64,
        );
        if !seen.insert(key) {
            regions_are_distinct = false;
        }
    }

    let report = Report {
        range_portfolio,
        regional_variation,
        regions_are_distinct,
        candidate_vs_baseline: candidate_vs_baseline(),
    };

    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
