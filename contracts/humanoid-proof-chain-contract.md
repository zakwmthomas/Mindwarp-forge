# H6 humanoid proof-chain contract

Status: `prototype_tested`. This contract rebuilds and recovers the exact H1-H5
engine-neutral humanoid evidence chain. It does not generate or import an asset,
run a graphics tool, choose a runtime, approve or promote a candidate, or mutate
protected authority.

The v1 manifest is strict canonical JSON with five ordered stage receipts. H1
has no dependency; H2 depends on H1; H3 on H2; H4 on H3; and H5 on H4. Every
stage retains its stable receipt identity, content fingerprint, outcome, and
limitations. A domain-separated content ID binds the full ordered manifest.

H5 has a separate strict decision receipt binding:

- approved preview SHA-256
  `f3242d18b962103ec0b78fe424baf01db60c84d2c2a6f468c85d27818c145051`;
- cute, sweet, approachable feminine direction;
- strong, powerful, commanding masculine direction;
- bald, featureless, modular neutral construction; and
- all no-import, no-topology, no-rig, no-runtime, no-capability-rule, and
  non-human-lineage limitations.

The H5 receipt ID is
`5c4eb3041ced04e1c1a5cd0e011babafe1826a4d8caf420bf267c8bff0617520`.
The aggregate H1-H5 manifest ID is
`a0eb0796a4a0edd800fcd937049eaa3ed7c65e695531daf0f754e835591ada2d`.

Unknown fields, noncanonical bytes, truncation, missing, duplicate or reordered
stages, stale dependency links, fingerprint drift, changed outcomes or
limitations, unknown schema versions, and widened authority fail closed. A
hostile candidate is never repaired in place: recovery discards it and rebuilds
the known-good manifest from the existing typed H1-H4 constructors and the
canonical H5 decision constructor.

The durable fixture stores all five stage receipts, the H5 decision, and the
aggregate manifest as content-addressed Kernel evidence objects. The existing
authority-negative ProofReceipt projection and SQLite online-backup mechanism
must reopen with exact object bytes, IDs, links, receipt, counts, byte length,
and fixity. Generated projections are never recovery authority.
