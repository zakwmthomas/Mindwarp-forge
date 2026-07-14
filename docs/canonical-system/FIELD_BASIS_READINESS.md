# Field Basis Fixed-Vector Harness: Readiness Package

**Status:** discovery plus researched numerical-policy recommendation. The
exact implementation gate is retained in `FIELD_BASIS_DESIGN_GATE.md`. This
package does not create a renderer, texture atlas, engine object, or runtime
field implementation.

## Source evidence and limits

The recovered master specification defines a seeded field matrix at
hierarchical coordinates/scales. Its reusable basis alphabet includes harmonic
waves, fractal noise, ridged response, cellular response, domain warps, flow,
sparse events, temporal fields, and blue-noise distributions. It also requires
consumer recipes to record atlas/version references, selected channels,
transforms, remapping, seed, and quantisation mode; caches are disposable and
recipes plus versioned addresses are canonical.

That is architectural evidence, retained with fixity through
`evidence/handover-manifest.json`, not a reference-proven numerical library or
a claim about future engine performance.

## Boundary to establish

The first harness should use a data-only `FieldRecipe` and `FieldSampleVector`:

| Element | Required behavior | Must remain outside the first contract |
|---|---|---|
| `recipe_id`, `schema_version` | Versioned interpretation and traceability | Cache location or renderer resource |
| `basis_terms` | Named, ordered basis family and parameters | Arbitrary executable expression/code |
| `identity_context` | Universe address/version/stream reference from identity | Mutable deltas or residency state |
| `transform` | Explicit coordinate mapping and domain-warp inputs | Ambient global transform/state |
| `composition` | Declared blend, mask, remap, and quantisation rules | Implicit term ordering or hidden defaults |
| `sample_domain` | Typed coordinate and scale units | Engine-space object references |
| `output_descriptor` | Named channels, range, and semantic class | Direct gameplay/semantic claims |
| `fingerprint` | Stable recipe/sample comparison record | Unlabelled floating-point tolerance |

Field output is a bounded numerical signal or packet. It cannot by itself
declare culture, agency, conflict, ownership, or gameplay truth; those require
derived rules, entities, history, and explicit ledgers.

## Initial proof fixture matrix

| Fixture | Assertion |
|---|---|
| Same recipe/sample twice | Same versioned result/fingerprint under the declared numerical policy |
| Cross-seed pair | Meaningful variation without invalid range or missing channels |
| Order-sensitive composition pair | Reordered terms either reproduce the documented result or fail as incompatible |
| Transform/domain-warp pair | Explicit transform change affects only the documented coordinate relation |
| Quantisation pair | Quantised and unquantised outputs are not silently compared as exact equals |
| Cache-independence pair | Warm/cold/disposed cache paths yield equivalent canonical samples |
| Poison parameter set | Invalid frequency, range, NaN/infinite, unknown basis family, and malformed mask fail visibly |
| Boundary/frequency set | Sampling near domain limits and across declared bands reports aliasing/range diagnostics |

## Required measurements and labels

Each future receipt must record sample count, domain extent, precision policy,
elapsed time, allocated/transient memory where available, cache state, and
whether a value is measured, simulated, or estimated. No result may be used as
an engine or hardware performance claim before the runtime-adapter phase.

## Contract neighbours

| Neighbour | Receives from field basis | Must not leak back |
|---|---|---|
| Universe identity | Requests deterministic address/version/stream context | Field-specific cache or numeric state |
| Derived world rules | Versioned FieldPacket/channel ranges and provenance | Direct entity/history/semantic authority |
| Lazy hierarchy | Recipe references and reconstructable sample descriptors | Residency/eviction state into recipe meaning |
| ProofReceipt | Fixture, recipe, sample fingerprints, warnings, costs | Pass/fail authority into numerical output |
| Reference Studio | Read-only sample/recipe inspection data | Editing/execution/network controls |

## Readiness gaps deliberately left open

The current evidence does not select a numeric representation, deterministic
math policy, basis implementation library, coordinate units, canonical
parameter encoding, allowed composition operators, tolerance/equivalence rule,
or cache-key derivation. These are coupled technical choices: changing one can
invalidate fixed vectors and caches. They must be evaluated together in a
bounded design/readiness package, not improvised during implementation.

The bounded comparison is now recorded in `FIELD_BASIS_DESIGN_GATE.md`. It
recommends an exact fixed-point CPU reference, strict integer-only recipe
encoding, and a versioned `Philox4x32-10` random-access lane, while retaining
float/SIMD/GPU libraries only as non-canonical comparison paths. The owner
approved the bounded CPU reference; its tested result and remaining limits are
retained in that document.

## Entry criteria for a future implementation package

- ProofReceipt storage/authority boundary is resolved.
- Universe identity vector semantics are selected and reference-tested.
- A field numerical-policy decision covers precision, deterministic math,
  serialization, tolerance, and cross-platform comparison.
- Fixture matrix includes valid, cross-seed, cache, poison, and boundary cases
  before implementation is assessed as reference-proven.
- Field module has no engine, filesystem, network, process, or mutable-world
  capability.
