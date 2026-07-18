# G1 C2 universe identity result

Status: **verified and promoted as the G1 identity foundation**.

The G1 reassessment found no new owner choice. The approved v1 invariant remains
fit for the unified route:

- logical identity is the strict universe seed plus typed hierarchical address
  and does not change when a generator implementation changes;
- reconstruction identity separately binds generator version and derivation
  contract, so old and new baselines remain explicit;
- deterministic CBOR, domain-separated SHA-256, HKDF stream keys and HMAC
  counter blocks form the portable reference boundary;
- stream labels partition consumers without traversal-order state; and
- migration is append-only and collisions fail visibly instead of merging.

This identity is sufficient for C3-C7 recipes, cache keys, history deltas,
organism identities, aesthetic recipes and representation lineage. Consumers
must not bind logical identity to generator version, use ambient `latest`
semantics, share mutable traversal-order random state, or treat the HMAC
reference block as the future bulk field generator.

The focused `universe-identity` suite passes seven cases covering exact vectors,
strict/noncanonical input rejection, sibling/version/stream separation,
migration, collision handling, bounded labels and authority-negative evidence.
The complete Forge gate passes after the unified registry alignment repair.

Limits remain explicit: second-platform receipts and production performance are
not proved. C3 owns the separately named, versioned and benchmarked bulk field
generator and derived-world contract. Those limits do not reopen the already
approved logical identity policy.
