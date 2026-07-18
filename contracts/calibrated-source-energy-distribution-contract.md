# Calibrated source-energy distribution contract

Status: owner-approved additive V1 implementation contract.

## Purpose and authority

`calibrated-source-energy-distribution` is a capability-free evidence owner. It
binds one validated calibrated spectral/time basis and one replayed depth-zero
optical phase-space cell to an immutable closed frontier of exact radiant-energy
allocations. Every result carries `authority_effect = none_evidence_only`.

It does not discover a normative emission, infer radiance, authorize transport,
couple a receiver, model a detector, decide visibility, mutate runtime state,
select a superseding record, promote evidence or close C3. V1 has zero downstream
consumers.

## Frozen V1 query and replay

The query embeds the complete calibrated basis, selected RGB band, nonzero source
provenance distinct from calibration provenance, complete replayable optical root,
one reduced exact root energy in joules and at most 63 ordered refinement
directives. A directive names one current unresolved allocation and contains only
an upstream phase-space axis, ordered child energies and ordered
`resolved_leaf | unresolved_within_cell` dispositions.

The owner calls the unchanged upstream `split_optical_phase_space_cell` function.
Callers cannot provide child geometry, ancestry, measures, cell identities or
phase-split identities. A resolved leaf cannot be refined again. Both children
remain explicit, including zero-energy children.

## Exact quantities and conservation

Energy components are reduced canonical unsigned `u128` decimals with at most 39
digits. Denominators are positive and zero has only `0/1`. Every atomic replacement
proves `parent = lower + upper` using checked `Signed512` arithmetic under a
385-bit live shield. The upstream owner independently proves the two child
measures exactly halve their parent. Energy density is at most a coordinate-local
derived view; it is neither stored in identity nor evidence of within-cell
uniformity.

## Identities, codecs and correction

Subject, allocation, split and distribution identities use SHA-256 over a frozen
domain separator, a zero byte and canonical JSON. The identity graph is
non-circular. Strict codecs deny unknown fields and reject noncanonical or trailing
bytes. Full replay reconstructs all upstream cells, quantities and identities
before accepting a claimed result.

Evidence is immutable. A correction uses a higher positive upstream source
revision and creates a new subject. V1 contains no active, supersedes, tombstone,
promotion or selection field.

## Resource, dependency and rollback boundary

V1 admits at most 64 frontier allocations, 63 directives/receipts, upstream depth
12, 128 KiB canonical query, 256 KiB canonical result and 4 MiB conservative live
canonical bytes. Production dependencies are exactly the calibrated basis,
optical phase-space cell and fixed-interval arithmetic owners plus serde,
serde_json and sha2. No downstream owner imports this module.

Rollback is deletion-only: remove this crate, contract, tests, fixture, verifier,
result and governance entries. Existing owner APIs, bytes, identities and behavior
remain unchanged.
