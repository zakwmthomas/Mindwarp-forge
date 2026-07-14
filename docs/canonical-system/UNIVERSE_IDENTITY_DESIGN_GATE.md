# Universe Identity Design Gate

**State:** approved for bounded reference implementation after independent
revalidation. **Date:** 2026-07-13.

## Independent revalidation and approval

The owner requested a final comparison with established practice, internal
debate, and repair before approval. The review found:

- JAX uses a counter-based, splittable PRNG design specifically to remove
  sequential call-order dependence and support reproducible parallel work.
- NumPy's `SeedSequence` mixes a root seed with a spawn-tree path to create
  reproducible independent child streams.
- Random123 documents stateless counter/key generators for CPU and GPU use.
- Rust Rand explicitly warns that seeding alone does not guarantee stable
  output and recommends a named, fixed algorithm for reproducibility.
- RFC 8949 deterministic CBOR still requires an application profile; therefore
  this package restricts the identity preimage to fixed arrays, unsigned
  integers, and byte strings, then rejects any input that does not byte-match
  its strict re-encoding.

The strongest alternative was HKDF-derived keys feeding Threefry, Philox, or
ChaCha for every random value. That is faster for bulk generation, but it adds
another permanent algorithm/layout decision to the identity foundation. The
final design keeps HMAC-SHA-256 counter blocks as a small, portable reference
primitive and explicitly does **not** claim it as the future high-throughput
field generator. Field-basis work must benchmark and version its named bulk
generator while preserving these stream keys and fixed vectors.

Additional primary/maintainer sources:

- JAX PRNG design: <https://docs.jax.dev/en/latest/jep/263-prng.html>
- NumPy parallel random generation: <https://numpy.org/doc/stable/reference/random/parallel.html>
- Rust Rand reproducibility: <https://rust-random.github.io/book/crate-reprod.html>

After that review, the owner approved the recommendation. The authorization is
limited to the engine-neutral contract, strict codec, derivation functions,
fixed vectors, migration/collision fixtures, and ProofReceipt-compatible test
evidence. Runtime, engine, production promotion, and protected-Kernel authority
remain outside scope.

## Decision that cannot be hidden in code

The long-lived invariant is whether a logical address keeps its identity when
the generator implementation changes. The recovered evidence requires stable,
versioned hierarchical addresses, a baseline key plus generator version, and
explicit deltas, but it does not resolve whether the version participates in
logical identity.

**Recommended invariant:** logical identity is
`universe + address-schema + typed path`, independent of generator version.
The reconstruction fingerprint is separately
`logical identity + generator version + derivation contract`. This lets saved
deltas remain attached to the same logical place while every baseline remains
reconstructable and version drift stays visible. A generator change never
silently rewrites identity or baseline evidence.

## Recommended standards lane

| Concern | Recommendation | Reason and boundary |
|---|---|---|
| Canonical wire form | RFC 8949 deterministic CBOR, definite-length arrays only; unsigned integers and byte strings in fingerprint preimages; no maps, floats, or text | Standard cross-platform encoding while eliminating map order, float width/NaN, and Unicode-normalization ambiguity |
| Logical fingerprint | Domain-separated SHA-256 over the exact canonical identity envelope | Forge already uses SHA-256; FIPS 180-4 provides a stable public definition; collision is diagnosed and never merged |
| Stream partition key | HKDF-SHA-256 with explicit `info = identity-schema, generator-version, typed-address, stream-label` | RFC 5869 defines extract/expand and includes fixed vectors; changing a label creates an independent derived key without mutable shared state |
| Random-access stream block | HMAC-SHA-256 of a domain separator and unsigned 64-bit counter under the derived stream key | RFC 2104-defined primitive; stateless counter access avoids traversal/order dependence and is simple to reproduce across platforms |
| Version policy | Exact unsigned `{major, minor, patch}` tuple in every reconstruction input; any output-affecting change gets a new tuple | No ambient/latest version and no silent upgrade; old vectors either replay under retained semantics or receive an explicit migration receipt |
| Human display | Non-canonical text projection only | Display spelling cannot change identity bytes |

Primary sources:

- RFC 8949 deterministic encoding: <https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2>
- NIST FIPS 180-4 SHA-256: <https://csrc.nist.gov/pubs/fips/180-4/upd1/final>
- RFC 5869 HKDF: <https://www.rfc-editor.org/rfc/rfc5869.html>
- RFC 2104 HMAC: <https://www.rfc-editor.org/rfc/rfc2104.html>
- Random123 counter-based design comparison: <https://www.random123.com/releases/docs/index.html>

## Rejected or deferred alternatives

- Generator-version-dependent logical identity makes an upgraded baseline look
  like a different place and risks orphaning persistent deltas. Rejected.
- Ambient library serialization is not a contract and can drift with map,
  float, text, or version behavior. Rejected.
- One mutable PRNG stream makes results depend on traversal and scheduling.
  Rejected.
- BLAKE3 plus Philox is a credible faster future lane, but it adds two new
  standards and should be admitted only by a versioned compatibility package
  with cross-language vectors and measured need. Deferred.
- Coordinates as identity fail hierarchy, migration, and mutable-history
  boundaries. Rejected.

## Fixed-vector manifest required before implementation closes

1. Root identity with a 256-bit zero seed and empty typed path.
2. Two sibling typed paths differing in one numeric payload.
3. Same address and generator version with `terrain` and `ecology` stream
   labels; identity equal, stream keys and counter blocks unequal.
4. Counter values 0, 1, and maximum unsigned 64-bit value.
5. Same logical address under generator versions 1.0.0 and 2.0.0; logical
   fingerprint equal, reconstruction fingerprints unequal.
6. Old-version replay and explicit migration-receipt cases.
7. Rejections for unknown schema, unknown node tag, duplicate forbidden
   segment, excess depth/length, indefinite CBOR, maps, floats, text in a
   canonical preimage, trailing bytes, and non-minimal integer encoding.
8. An injected fingerprint-collision fixture that fails visibly rather than
   merging records.

Every successful vector produces a ProofReceipt naming exact input/output
objects, contract and generator versions, byte-equivalence method, simulated or
measured cost, and limitations.

## Failure and recovery matrix

| Failure | Required result |
|---|---|
| Unsupported identity or generator version | Fail before derivation; retain version-mismatch receipt |
| Invalid or non-deterministic encoding | Reject bytes; never normalize silently |
| Stream-label or counter reuse | Domain/counter trace remains inspectable; identical request is idempotent |
| Fingerprint collision | Quarantine both inputs and emit diagnostic evidence; never merge |
| Old implementation unavailable | Block reconstruction and request migration evidence; never use newest semantics |
| Cache/materialized corruption | Recompute from canonical identity/version; cache never participates in identity |
| Partial migration | Original identity, version, and delta links remain recoverable; migration receipt is append-only |

## Approval result

Approved after independent review for one bounded, engine-neutral reference
implementation package. This does not select a runtime engine, promote a
generator to production, authorize arbitrary code, or alter protected Kernel
authority.
