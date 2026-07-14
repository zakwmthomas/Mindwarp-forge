# Universe Identity Contract v1

Universe identity is an engine-neutral, capability-free module. A logical
identity is the domain-separated SHA-256 fingerprint of one strict canonical
address envelope: identity schema, 256-bit universe seed, and ordered typed
path. Generator version is deliberately excluded from logical identity.

A reconstruction fingerprint additionally binds the exact generator-version
tuple and derivation-contract identifier. The same logical address therefore
survives a generator upgrade while old and new baselines remain distinguishable.
Migration is an append-only mapping receipt between reconstruction fingerprints;
it never rewrites the logical identity or world delta links.

The v1 wire profile is deterministic RFC 8949 CBOR using definite arrays,
unsigned integers, and byte strings only. Maps, floats, text, indefinite forms,
unknown tags, non-minimal values, trailing bytes, excessive depth/length, and
non-byte-identical re-encoding fail closed. Human-readable address text is a
non-canonical projection.

Stream keys use HKDF-SHA-256 over the seed with domain-separated information
containing the canonical address, generator version, derivation contract, and
bounded ASCII stream label. Reference counter blocks use HMAC-SHA-256 over a
domain separator and unsigned 64-bit counter. This is a correctness/reference
primitive, not a bulk field-generation performance claim. A future faster
generator must be separately named, versioned, benchmarked, and fixed-vector
compatible at its own contract boundary.

Admission is idempotent for equal canonical bytes. A fingerprint already bound
to different canonical bytes is a collision and must fail visibly; it never
merges identities. Cache, coordinates, renderer objects, mutable deltas,
filesystem state, network state, ambient library versions, and runtime-engine
IDs never participate in logical identity.

Required proof includes committed byte/hash/key/block vectors, fresh-process
replay, sibling and stream separation, generator-version migration, strict
malformed-CBOR rejection, injected collision detection, maximum counter/depth
boundaries, and a ProofReceipt-compatible evidence envelope. The module has no
filesystem, network, process, approval, promotion, execution, or protected-
Kernel capability.
