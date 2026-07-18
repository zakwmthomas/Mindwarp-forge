# G1 C3 Visible-Radiance Interface Adaptive-Kernel Implementation Readiness

Date: 2026-07-16

Status: **implementation-ready as one bounded capability-free reference; the
pinned dependency spike subsequently passed with signed-magnitude and native-
limb restrictions, while permanent code and dependency integration remain at
an explicit owner gate.**

## Readiness decision

The retained adaptive strategy is ready to be implemented only as a local
interface-event reference with a production ladder of
`96 -> 128 -> 160 fractional bits` and a hard 160-bit ceiling. Failure to
certify every required output by that ceiling is a normal typed
`nonconvergent_enclosure` result. It is never permission to raise precision,
change arithmetic, narrow the admitted domain or emit a best-effort event.

This deliberately does not carry the experimental 192/256/384 levels into the
first production reference. The retained portfolio certified all general cases
by 160, while a forced 128-bit cap proved the failure path. Keeping 384 would
increase the derived live ceiling to 900 bits without proving universal
convergence. A bounded 160-bit cap plus an honest unavailable result is the
smaller, safer contract.

## Whole-plan alignment

The package advances only the local smooth-dielectric event between two
already-proved positive-length path media. It consumes exact path and explicit
face-bound interaction evidence; it does not create occupancy, choose world
geometry or continue the refracted ray through the volume.

Nothing here forms a planet, sphere, terrain surface, biome, material seam or
rendered colour. Biome continuity remains protected: continuous physical causes
must produce deterministic ecotones, while sharp presentation boundaries need
sharp physical evidence.

## Representation comparison

No suitable wide-number facility is currently present in the Forge dependency
graph. The first implementation therefore needs an explicitly reviewed
dependency rather than an unreviewed local multi-limb subsystem.

| Candidate | Fit to bounded kernel | Principal risk | Disposition |
|---|---|---|---|
| RustCrypto `crypto-bigint` fixed-width `Uint`/`Int` | Pure Rust, stack fixed-width, checked operations, division and floor square root; 512 bits cover the measured 448-bit maximum and the 452-bit derived 160-level ceiling | Cryptographic constant-time defaults may cost more than this public-data calculation needs; MSRV may move in patch releases; prior audit no longer covers every current change | **Preferred bounded candidate, subject to a pinned-version build/API/performance spike and owner dependency approval** |
| `num-bigint` `BigUint`/`BigInt` | Mature pure-Rust dynamic width and portable serde support | Heap allocation and unbounded growth weaken the deterministic resource contract; width must be policed separately | Retain as implementation fallback and independent-test option, not first reference |
| `ibig` `UBig`/`IBig` | Pure-Rust dynamic width with conventional arithmetic | Same unbounded-allocation problem and less direct alignment with the fixed 512-bit ceiling | Do not select for v1 |
| `rug` | Mature GMP-backed arbitrary precision and rounding facilities | Native GMP/MPFR/MPC build and LGPL obligations add cross-platform, packaging and governance cost; dynamic allocation remains | Reject for this bounded capability-free reference |
| Local bespoke limbs | Could be tailored exactly to the schedule | Creates a new arithmetic subsystem, audit surface and long-term maintenance burden | Reject unless every reviewed dependency fails the bounded spike |

Primary-source comparison basis: the current `crypto-bigint` documentation
describes const-generic fixed-width stack integers, checked arithmetic, division
and square-root traits, and warns that its MSRV may change in patch releases;
`num-bigint` documents dynamically allocated pure-Rust integers and portable
serde; `ibig` documents dynamically sized pure-Rust integers; `rug` documents
its GMP/MPFR/MPC and LGPL boundary.

The audit does **not** select or install `crypto-bigint`. It freezes the
decision test: an authorized implementation must pin one reviewed version,
disable unused features, record license and transitive dependencies, compile on
both required targets, and prove every required checked operation and codec
before the dependency becomes accepted.

## Arithmetic and resource contract

The first reference uses signed 512-bit fixed-width values for interval
endpoints and unsigned 512-bit values for nonnegative products, divisors and
root inputs. Every add, subtract, multiply, shift, conversion and sign change is
checked. Division uses explicit nonzero divisors and separate floor/ceiling
rounding derived from quotient and remainder. Square root uses integer floor
root plus an exact-square test for the upper endpoint.

The admitted deterministic ceiling is:

- at most three staged evaluations per general event;
- exactly the declared 96, 128 and 160 levels, in that order;
- at most `384` fractional-bit work units per general event;
- a derived maximum live magnitude of
  `max(F + 232, 2F + 132) = 452` bits at `F = 160`;
- a 512-bit storage type, leaving 60 magnitude bits above that derivation;
- no recursion, background work, filesystem access, network access or silent
  heap-backed arithmetic fallback; and
- checked overflow or an internal invariant breach returns a typed arithmetic
  defect and fails verification; it is not ordinary nonconvergence.

Exact fast paths require zero staged evaluations. The retained portfolio's
observed maximum remains 448 live bits and 288 stored endpoint bits. Those
measurements support the ceiling but do not replace the algebraic 452-bit guard.

## Result semantics

Exact fast paths are validated internal optimizations, not public physical
event classes. Callers receive the same result shape regardless of whether an
exact path or staged evaluation produced it.

The future result union has three distinct meanings:

1. `known`: every required Q0.48 power and Q1.62 direction interval is ordered,
   reference-containing under tests, and no wider than one target unit;
2. `nonconvergent_enclosure`: validation and arithmetic remained sound, but the
   declared 160-bit ceiling did not certify the full event; and
3. an existing typed unavailable/unsupported input outcome, or an internal
   arithmetic/invariant defect that rejects the computation.

`nonconvergent_enclosure` contains the attempted level list, final retained
intervals, maximum live/stored bit receipts and a stable reason code. Consumers
may propagate or report it, but may not treat it as darkness, opacity, total
reflection, zero transmission, a categorical boundary or permission to retry
with hidden resources.

## Codec requirements

The future schema remains separately versioned from the bulk-transfer schema.
Before implementation is accepted, its strict canonical codec must prove:

- `schema_version = 1` and explicit profile, scope, reconstruction and query
  provenance bindings;
- fixed lowercase enum spellings and rejection of unknown or duplicate fields;
- decimal-string encoding for signed wide endpoints with no leading plus sign,
  no leading zero except `0`, no negative zero and an explicit 512-bit range
  check;
- numeric precision levels restricted to `[96, 128, 160]`, strictly ordered and
  without duplicates;
- canonical component order and exact Q0.48/Q1.62 scale tags;
- byte-identical encode/decode/encode replay on both required targets; and
- poison fixtures for oversized integers, noncanonical strings, missing
  provenance, unknown status, fabricated fast-path labels and a `known` result
  wider than one unit.

The public codec does not expose dependency-specific limb order or native-word
layout. This engineers out 32/64-bit target serialization drift and permits a
future dependency replacement without changing physical meaning.

## Mandatory implementation fixtures

An authorized reference must port the complete retained oracle portfolio and
keep the Python exact oracle independent. At minimum it retains:

- normal-incidence, index-matched and TIR perfect-square fast paths plus every
  negative neighbor;
- the coprime-wide transmit case and exact 232-bit post-cancellation critical
  comparison;
- below/equal/above-critical neighborhoods;
- target-aligned and near-aligned projected outputs;
- structural-zero components, grazing directions and sign permutations;
- empty-intersection, lost-containment, retained-widening and false-fast-path
  defect injections;
- forced 128-bit-cap nonconvergence after exactly two evaluations;
- checked overflow, divide-by-zero, invalid root and codec poison cases; and
- energy, unit-vector, Snell, no-clamp, no-silent-fallback and authority-negative
  shields.

The Rust result must match the retained exact branch and contain the independent
reference for every fixture. The stop distribution is a regression signal, not
a semantic promise: changes require explanation, while any earlier false
certification or later hidden work fails the gate.

## Integration and rollback

The authorized module would be additive and capability-free, adjacent to
`visible-radiance-bulk-transfer` and dependent only on the exact physical path
substrate, existing local codec/provenance patterns and the separately approved
wide-number dependency. It may not modify the bulk-transfer result or continue a
refracted path.

Integration is read-only until the local module passes focused tests, the
independent Python oracles, warnings-denied workspace tests, canonical record
verification and a second-target build/vector replay. The prior
`interface_model_required` outcome remains the rollback target. Removing the
new crate and dependency must restore that exact behavior without data
migration, schema reinterpretation or consumer changes.

Promotion needs measured release-build time and allocation receipts on the
primary PC plus byte-identical vectors on a second architecture or operating
system. The current same-host Python proof is not that second-platform receipt.

## Failure points engineered out

- A hard 160-bit cap prevents an empirical success from becoming unbounded
  production work.
- Nonconvergence is data, while overflow and invariant failure remain defects;
  callers cannot conflate them.
- Exact shortcuts do not leak into public semantics and cannot become a caller-
  asserted authority flag.
- Fixed-width checked arithmetic blocks wraparound and hidden heap growth.
- Decimal canonical endpoints block native limb-width and endianness drift.
- Independent per-level recomputation and monotone intersection remain
  mandatory; higher precision cannot inherit lower-level error.
- Rollback preserves the existing explicit interface-required state instead of
  substituting a fabricated optical answer.
- Dependency approval, production performance and second-platform evidence are
  separate gates, preventing a passing numerical portfolio from silently
  approving operational risk.

## Remaining risks and falsifiers

Implementation readiness is revoked if the pinned candidate cannot express all
required checked operations without wrapping or hidden dynamic fallback, if its
MSRV conflicts with the workspace, if release cost exceeds the recorded budget,
or if second-target vectors differ. The route then returns to representation
comparison; it does not authorize bespoke limbs or a higher cap automatically.

The 160-bit ceiling intentionally permits admitted cases to return
`nonconvergent_enclosure`. If real coefficient evidence later shows that rate is
operationally unacceptable, the output contract, exactness rules and arithmetic
strategy must be reassessed together. Domain narrowing and silent cap increases
remain prohibited.

Real optical coefficients, metre mapping, end-to-end refractive path
composition and production workload frequency remain unknown. They do not block
this isolated reference, but they do block scientific fidelity, consumer
integration and C3 closure.

## Authority and nonclaims

This audit creates no schema, Rust module or dependency entry. It installs
nothing and selects no coefficient catalogue. It grants no received radiance,
perception, rendering, navigation, generic passage, ecology, biome presentation,
sphere, planet, terrain, persistence, runtime, promotion or C3 closure.

## Exact owner implementation gate

The bounded action prepared for the owner is:

> Authorize one isolated capability-free visible-radiance interface adaptive
> kernel reference with the `96 -> 128 -> 160` hard-capped ladder, typed
> `nonconvergent_enclosure`, internal-only exact fast paths, strict canonical
> codec, complete retained hostile portfolio and reversible integration exactly
> as bounded here. Permit a pinned `crypto-bigint` dependency only after the
> implementation spike proves the required checked U512/I512 operations,
> license/transitive review, primary release cost and second-target vector
> replay. Do not authorize downstream path composition, perception, rendering,
> passage, biome/planet/runtime semantics, promotion or C3 closure.

Stop here until the owner explicitly releases that action. General instructions
to continue analysis do not silently approve dependency installation or code.

## Subsequent dependency-spike disposition

The owner-authorized disposable spike recorded in
`G1_C3_VISIBLE_RADIANCE_INTERFACE_WIDE_NUMBER_DEPENDENCY_SPIKE_RESULT.md`
supports pinned `crypto-bigint` 0.7.5 on executable x64 and i686 vectors. It
adds two binding implementation restrictions: directed signed rounding must be
built from unsigned magnitude quotient/remainder, and no codec or fixture may
expose native limbs. The spike did not add the dependency to Forge and does not
release the permanent integration gate above.
