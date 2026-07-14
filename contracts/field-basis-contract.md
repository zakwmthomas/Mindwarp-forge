# Field Basis Contract v1

Field basis is an engine-neutral, capability-free numerical reference layered
on universe identity. Canonical coordinates are signed Q32.32 values; canonical
terms and outputs are signed Q16.48 values. Checked i128 intermediates and
round-to-nearest, ties-to-even are mandatory. Overflow and invalid structure
fail closed; wrapping, saturation, floats, implicit defaults, executable graphs,
cache state, renderer state, and runtime objects are never canonical inputs.

Recipes use strict deterministic CBOR and ordered, backward-only term
references. V1 admits constants, 2D value-lattice fields, addition,
multiplication, and ridged remapping. Philox4x32-10 supplies stateless lattice
words using a versioned little-endian key/counter mapping. Exact recipe bytes,
contract version, reconstruction identity, generator mapping, and sample domain
bind cache keys. Cache contents remain disposable.

The Rust CPU implementation is a correctness reference, not a production,
visual-quality, GPU, SIMD, or engine-performance claim. Accelerated previews
are non-canonical until exact or explicitly bounded semantic equivalence is
proved. Cross-platform receipts remain required before `reference_proven`.
