# G1 C3 Visible-Radiance Interface-Event Oracle Result

Date: 2026-07-16

Status: **mathematical candidate supported inside the exploratory domain;
fixed-width implementation readiness not established.**

## Result

The capability-free Python oracle implements the selected local
smooth-dielectric event with standard-library arbitrary-precision integers,
exact `Fraction` values and directed integer-square-root intervals. It uses no
binary float, epsilon, clamp, external package, runtime engine or production
schema.

The default 384-root-bit run passed **10,608 checks** across fixed analytic,
geometry, authority and **1,024 deterministic generated cases** using seed
`252341783`. The generated portfolio contained 627 transmitted events and 397
total-internal-reflection events.

All admitted events preserved:

- exact face versus edge/vertex/tangent/lane classifications;
- analytic normal-incidence reflectance;
- index-matched identity behavior;
- below-critical transmission and exact/above-critical TIR;
- normal-incidence reciprocity;
- outward energy containment;
- reflected and transmitted unit-vector containment;
- Snell tangential invariance;
- power bounds in `[0,1]`; and
- authority-negative interface/path/perception/runtime limitations.

## Exploratory domain

This proof used:

- exact nonzero integer path deltas admitted by the existing `u128`
  sum-of-three-squares ceiling;
- one reconstructed positive axis normal; and
- exact Q16.48 refractive-index fixtures between **1/4 and 16** on each side.

The refractive-index range is a computational stress envelope only. It is not a
scientific validity range, material catalogue, recommended public codec or
permission to fabricate coefficients.

## Enclosure receipt

At 384 internal root bits:

- worst outward Q0.48 reflected/transmitted power width: **1 unit**;
- worst outward Q1.62 direction-component width: **1 unit**;
- raw reflectance interval overshoot above one: **zero** in the retained
  portfolio; and
- maximum intermediate exact-rational numerator and denominator sizes:
  **8,330 bits** each.

Precision sensitivity over the identical 10,608-check portfolio was:

| Root interval bits | Worst Q0.48 power width | Worst Q1.62 direction width | Maximum intermediate bits |
|---:|---:|---:|---:|
| 62 | 1 | 3 | 1,895 |
| 64 | 1 | 2 | 1,926 |
| 72 | 1 | 1 | 2,086 |
| 80 | 1 | 1 | 2,245 |
| 96 | 1 | 1 | 2,571 |
| 128 | 1 | 1 | 3,213 |
| 192 | 1 | 1 | 4,492 |
| 256 | 1 | 1 | 5,772 |
| 384 | 1 | 1 | 8,330 |
| 512 | 1 | 1 | 10,894 |

The stable projected widths support the mathematical model, but the linear
growth of naive exact-rational intermediates rejects direct translation of the
oracle expression into a fixed-width reference. The 72-bit observation is an
empirical result for this portfolio, not proof that a 72-bit production kernel
is sufficient for every admitted input.

## Failure points engineered out

- The harness classifies unique faces before evaluating optics, so diagonal
  edge/vertex transitions never receive a fabricated normal.
- Exact critical equality returns TIR; there is no tolerance band.
- The event produces only local outgoing direction and power evidence. It has
  no downstream path or endpoint-arrival calculation.
- The theorem-backed `[0,1]` intersection is separately visible from raw
  interval arithmetic; the retained portfolio recorded zero raw overshoot.
- The stress domain is printed in every receipt, preventing it from silently
  becoming a scientific material range.
- Root precision is externally selectable through
  `FORGE_INTERFACE_ROOT_BITS`, allowing deterministic sensitivity reruns.
- The retained C3 verifier checks proof-shield tokens and executes the default
  oracle on every C3 foundation verification.

## Readiness consequence

The oracle supports the selected geometry and smooth-dielectric equations, but
it does **not** support immediate implementation readiness. A numerical-kernel
design audit must first compare:

1. staged directed fixed-point rounding with bounded intermediates;
2. retained arbitrary-precision integer/rational arithmetic;
3. a narrower admitted input and output domain; and
4. algebraic reformulation that avoids denominator explosion.

That audit must preserve one-unit output targets, exact TIR classification,
monotonic interval order, generated counterexamples and the full local-event
authority boundary. It must state dependency and cost consequences rather than
introduce a bespoke multi-limb subsystem silently.

## Nonclaims

This result defines no public interface schema, Rust module, real refractive
data, scientific three-band fidelity, rough/conductor/scattering behavior,
downstream refractive path, received radiance, perception, rendering, passage,
navigation, biome presentation, sphere, planet, terrain, persistence, runtime,
approval, promotion or C3 closure.

Biome continuity remains unchanged: categorical cells, regions and interface
records cannot paint visible biome seams. Continuous physical causes require
deterministic ecotones; sharp transitions require sharp physical evidence.

## Exact next action

Run a visible-radiance interface-event numerical-kernel design audit. Select
the smallest deterministic arithmetic strategy that can preserve the oracle's
directed one-unit output targets without naive multi-thousand-bit rational
growth. Stop before implementation readiness, dependency installation, Rust
code, coefficient selection or downstream path composition.
