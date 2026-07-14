# Universe Identity Fixed-Vector Harness: Readiness Package

**Status:** the bounded engine-neutral reference prototype is verified. No bulk
field generator, runtime project, or engine adapter is implemented.

## Source evidence and limits

The recovered master specification says that Mind Warp has one seeded,
addressable, lazily materialised universe; canonical reconstruction uses a
baseline key plus generator version and explicit deltas. It also states that
cached maps are disposable while recipes and versioned hierarchical addresses
are canonical. The source is retained with fixity in
`evidence/handover-manifest.json` as
`MINDWARP_FORGE_CONTINUATION_SURVIVAL_PACK_2026-07-12.zip`, and is source
evidence rather than a passing production implementation.

## Boundary to establish

The first harness needs a small data-only `UniverseIdentityVector` contract:

| Element | Required behavior | Not decided here |
|---|---|---|
| `universe_id` | Identifies the one canonical universe | Exact encoding/length |
| `generator_version` | Makes interpretation and migration explicit | Version numbering policy |
| `address` | Ordered hierarchical path from universe through a typed node | Final hierarchy vocabulary and canonical serialization |
| `stream_label` | Separates deterministic random streams by purpose | PRNG/hash algorithm |
| `derivation_context` | Records parent/revision inputs needed for repeatability | Domain-specific world-rule contents |
| `identity_fingerprint` | Detects encoding or derivation drift | Hash function and collision policy |

The contract must remain independent of coordinates, renderer objects, cache
locations, mutable world deltas, and runtime-engine identifiers. Coordinates
can be an address attribute later, but cannot alone identify mutable state or
survive generator-version changes.

## Initial fixed-vector set

The future harness should contain versioned, hand-readable vectors for:

1. one root universe identity;
2. sibling paths at every currently accepted hierarchy boundary;
3. two stream labels at the same address, demonstrating partition rather than
   accidental shared random state;
4. a parent/child derivation context with inherited versus independent labels;
5. canonical-equivalent address text that must produce the same result after
   normalisation, if text input is supported;
6. malformed, missing-version, unknown-node-type, duplicate-segment, and
   unsupported-version inputs that must fail visibly; and
7. an old-version vector that is either reconstructed under an explicit
   compatibility rule or rejected with a migration receipt.

No finite vector set claims to enumerate the universe. It demonstrates stable
sampled observations only.

## Required proof assertions

- Same vector, fresh process, and retained version produce identical identity
  and stream results.
- Changing only `stream_label` changes the derived stream without changing the
  address identity.
- Changing a path segment changes the identity fingerprint without mutating its
  siblings.
- Cached/materialised state cannot affect the reconstructed identity.
- A version mismatch is visible and cannot silently use the newest generator.
- Invalid structure fails before downstream field, hierarchy, or history work.
- A collision is treated as a failure/diagnostic event, not silently merged.

## Contract neighbours

| Neighbour | Receives from identity | Must not leak back |
|---|---|---|
| Field basis | Stable address, version, partitioned stream context | Cache, texture, renderer, or floating-point implementation details |
| Lazy universe hierarchy | Typed address and descriptor key | Residency and eviction state |
| World history ledger | Baseline reconstruction key and version | Mutable deltas as identity inputs |
| ProofReceipt harness | Fixture ID, inputs, contract version, fingerprints | Pass status or authority into identity derivation |

## Implemented proof result

The approved v1 contract is implemented in `crates/universe-identity` with
seven tests covering committed byte/hash/key/counter vectors, sibling and
version partitioning, explicit migration, strict malformed-CBOR rejection,
hierarchy/counter bounds, injected collision handling, and authority-negative
ProofReceipt-compatible evidence. A desktop integration test persists the
evidence through the existing read-only ProofReceipt projection without adding
Kernel events or candidates.

This is `prototype_tested`, not `reference_proven`: the current receipt is from
the Windows/Rust lane and cross-platform replay evidence remains outstanding.

## Readiness gaps deliberately left open

The identity package now selects strict deterministic CBOR, SHA-256,
HKDF-SHA-256, and reference HMAC-SHA-256 counter blocks. It deliberately leaves
the high-throughput bulk field generator open. Field-basis work must select and
version that algorithm with benchmarks and independent vectors rather than
silently treating the identity reference primitive as a performance decision.

## Entry criteria for a future implementation package

- The ProofReceipt storage/authority boundary is resolved first.
- A concise technical decision record selects serialization, derivation, and
  version-migration policy with a reuse/security/determinism rationale.
- At least one platform-independent test environment is named.
- Fixed vectors, invalid vectors, and version-mismatch vectors are committed
  before the implementation is considered reference-proven.
- The implementation stays in an engine-neutral module with no file, network,
  process, or game-runtime capability.
