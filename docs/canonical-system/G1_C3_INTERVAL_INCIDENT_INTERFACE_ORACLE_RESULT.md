# G1 / C3 interval-incident interface oracle result

Date: 2026-07-16

Status: **bounded candidate supported for readiness design; implementation and
verified-module modification remain unauthorized.**

## Receipt

`tools/prove-g1-c3-interval-incident-interface.py` ran twice with byte-identical
output. The receipt checksum is
`ff0da6f60432a42c10e45371459e1b2a44ea98dc0bba8d664879dc8c20eaa488`.

The disposable oracle combines an exact integer whole-box branch classifier,
outward fixed dyadic interval operations, a 384-bit sensitivity reference,
5,483 independently generated point-reference containment regressions, a
forced-cap failure and three 64-event spectral continuation lanes. It changes
no Rust crate, contract, dependency or verified v1 behavior.

## Exact whole-box branch rule

For a face-normal component square interval `N2=[N2min,N2max]`, summed
tangential component square interval `T2=[T2min,T2max]`, incident index `ni`
and target index `nt`, the existing point TIR predicate is equivalent to:

`D = (ni^2 - nt^2) * T2 - nt^2 * N2 >= 0`.

Because the component box is independent and squaring extrema are exact, the
minimum and maximum of `D` are selected mechanically from the coefficient sign:

- `Dmin >= 0` means every admitted vector is `all_tir`;
- `Dmax < 0` means every admitted vector is `all_transmit`; and
- otherwise the only sound outcome is `ambiguous_interface_branch`.

Branch selection therefore occurs before square roots or rounded Fresnel work.
Numerical refinement cannot turn a mixed box into one physical branch.

## Portfolio result

The deterministic portfolio covered 9 named and 256 generated component
boxes. It produced:

| Outcome | Cases |
|---|---:|
| `all_tir` | 101 |
| `all_transmit` | 162 |
| `ambiguous_interface_branch` | 2 |

All 263 non-mixed boxes certified at 96 fractional bits against the 384-bit
interval sensitivity reference with no projected endpoint excess at Q0.48
power or Q1.62 direction. This is a portfolio result, not proof that every
future admitted box certifies at 96 bits.

Named controls include exact normal incidence, a one-unit normal box, exact
critical TIR, the adjacent transmitting point, a critical-straddling box,
near-critical narrow ambiguity, grazing transmission/TIR, and a tangent box
crossing zero. The normal point had zero Q1.62 direction width; named physical
box widths ranged through 1, 8, 18 and 98 direction units as expected from the
admitted input variation.

The candidate 96/128/160-bit ladder used at most **324 live integer bits** and
101 stored endpoint bits in the main portfolio. An intentionally insufficient
80-bit cap on the adjacent critical transmitting case left 2,991 target units
of numerical excess and returned `nonconvergent_enclosure`; it did not emit a
known event.

## Repeated-event result

Red (`4/3`), green (`7/5`) and blue (`8/5`) material lanes each completed 64
alternating material/air transmissions at 160 bits. Every event remained
`all_transmit`; no branch was selected from a representative direction.

| Lane | Direction width after 16 | after 32 | after 64 (Q1.62 units) |
|---|---:|---:|---:|
| Red | 435 | 30,732 | 150,283,463 |
| Green | 383 | 24,285 | 95,331,985 |
| Blue | 325 | 15,965 | 37,449,924 |

The largest repeated-event live integer width was 378 bits, below the 512-bit
candidate ceiling. The worst 64-event direction width is approximately
`3.26e-11` in real units. Widening is clearly cumulative, so the hard event
ceiling remains semantically necessary even though this portfolio stayed
useful.

## Adversarial interpretation

The result supports only a new interval-incident candidate:

- every nonzero vector in the component box is normalized before evaluation;
  this safely includes the physically correlated unit-vector subset but may
  add impossible vectors and false ambiguity;
- exact square extrema prove the branch classification, while fixed outward
  operations provide conservative algebraic enclosures;
- the 384-bit comparison uses the same interval expression at higher
  precision, so it measures numerical convergence rather than serving as an
  independent continuous-domain proof;
- independent point equations are retained as 5,483 containment regressions,
  not a proof over all real points;
- the generated portfolio, three material ratios and one repeated initial box
  do not prove universal utility, coefficients, platform speed or all admitted
  input ranges; and
- point-position correlation, next-face certification, bulk attenuation and
  end-to-end path composition remain separate operations.

The independent-axis representation therefore remains deliberately
conservative. A future result may return ambiguity even when a more expensive
correlation representation could decide the event. That is acceptable only if
the measured ambiguity rate remains useful.

## Readiness route

The next package is a design/readiness audit, not implementation. It must:

1. define a separate versioned interval-incident input/output candidate while
   preserving exact-path interface v1 byte-for-byte;
2. retain the exact whole-box branch classifier and typed
   `ambiguous_interface_branch`/`nonconvergent_enclosure` outcomes;
3. identify one owning numerical kernel or shared internal arithmetic seam so
   the composer cannot duplicate Snell/Fresnel semantics;
4. freeze input validity, normalization, codec, 512-bit, 96/128/160 ladder,
   three-band, 64-event, memory and work ceilings;
5. add hostile malformed, zero-vector, sign/orientation, critical, grazing,
   widening, cap, replay, authority and v1-regression fixtures;
6. define rollback and prove that removing the candidate leaves every verified
   module unchanged; and
7. prepare an explicit owner action only if those seams close without
   representative rays, native floats, runtime casts or semantic duplication.

## Authority and nonclaims

No schema, Rust implementation, dependency, coefficient catalogue, bulk
composer, perception, rendering, collision, navigation, organism, biome,
planet, terrain, runtime, promotion or C3 closure is authorized or proved.

