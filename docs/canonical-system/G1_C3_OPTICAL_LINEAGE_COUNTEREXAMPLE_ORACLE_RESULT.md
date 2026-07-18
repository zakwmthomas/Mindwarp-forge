# G1 / C3 optical-lineage counterexample oracle result

Date: 2026-07-16

Status: **thin per-band manifest plus explicit replayed bundle survives the
bounded oracle; code-facing readiness audit justified; no source authorized.**

## Result

`tools/prove-g1-c3-optical-lineage.py` ran twice with byte-identical canonical
receipt hash
`85b308953d2112a4bd2723c3e01ded96abdf4a33d16e9c1ff32cc4e3f0627937`.
The retained script source SHA-256 is
`baeb6d315e422af8932d4aa5706d44d290d5b8028f548baf558fac94fc778385`.

The selected representation survives as a candidate: one immutable band per
lane, contiguous step ordinals, predecessor-linked step IDs, exact local object
IDs, a domain-separated transcript ID and a receipt for the complete explicitly
supplied object bundle. Validation has no ambient lookup and rejects every
missing, duplicate, foreign or unused object.

This is an identity and sequencing oracle. Its local-object models stand in
for already verified Rust codecs and numerical results; it does not recalculate
cell faces, interface equations, attenuation, cumulative power or endpoints.

## Hostile receipt

All 26 hostile cases were rejected:

- gaps, duplicate steps, cyclic predecessors and transcript-ID mutation;
- foreign band, profile, reconstruction, step and local object identities;
- bundle-receipt mutation, duplicate, unused and changed objects;
- hit-point narrowing, direction recoding, caller-selected successor source
  aliases, partial interfaces, wrong interface cells, cross-lane event
  substitution, early terminal and wrong final terminal; and
- six **resealed attacker** cases for hit narrowing, successor-source alias,
  wrong interface cells, incident-direction change, cross-band transmitted
  direction and early terminal.

The resealed cases recompute every affected local object ID, manifest local
reference, step ID, predecessor chain, bundle receipt and transcript ID. They
therefore fail the semantic adjacency rules rather than merely presenting a
stale checksum.

Ten terminal families remain distinct and replayable: outer-domain exit,
unavailable neighbour, unavailable current cell, ambiguous next face, no
forward progress, all total internal reflection, ambiguous interface branch,
nonconvergent interface, unsupported interface model and work exhaustion.

## Resource receipt

| Portfolio | Manifest bytes | Modeled bundle bytes | Conservative modeled validation bytes |
|---|---:|---:|---:|
| one lane, one step | 1,249 | 1,549 | 3,304 |
| three lanes, one step each | 3,748 | 4,653 | 8,907 |
| one lane, 64 steps | 47,611 | 128,934 | 177,059 |
| three lanes, 64 steps each | 142,834 | 387,186 | 530,534 |

The modeled 192-step manifest is below the provisional 1 MiB target. Because
the oracle uses compact abstract local objects, its bundle measurement is not a
real-codec worst case. Applying all current public codec caps to every one of
192 steps—16 KiB cell input, 32 KiB cell event, 64 KiB bulk query, 16 KiB bulk
transfer, 16 KiB interface input and 64 KiB interface event—gives 40,894,464
object bytes. Adding the measured manifest and one maximum 64 KiB validation
object gives 41,102,834 bytes. That is below the provisional 48 MiB supplied-
object and 64 MiB validation targets even without credit for final steps that
need no interface.

These are readiness ceilings, not runtime performance proof. Actual Rust
allocation, Android-device memory, persistence delivery and repeated decode
cost remain unmeasured.

## Identity disposition

The oracle confirms that caller-selected successor source IDs create an alias
surface even when every local object remains individually valid. The candidate
therefore derives successor cell-input and interface-input source IDs from the
lane ID, ordinal, predecessor step ID and role. Ordinal zero retains one
declared nonzero lane source and binds it into lane identity.

Point propagation compares exact Q160 endpoint strings. Direction propagation
selects only the lane's band and compares exact Q1.62 endpoint strings under
the fixed physical/interface scale-tag adapter. No numeric recoding,
normalization, midpoint, epsilon or representative ray is admitted.

## Remaining boundaries

The candidate does not yet own cumulative power. A later audit must separately
define directed ordered multiplication of bulk transmission and interface
transmitted-power intervals. The candidate also cannot claim receiver arrival:
no receiver aperture/time/band contract exists, and a terminal point box is not
an exact endpoint.

The code-facing readiness audit must keep the new module capability-free and
additive, depend only on the three local evidence owners plus hash/codec
support, replay every supplied object, own no local numerical kernel and retain
deletion-only rollback. It must freeze exact schema fields, domain separators,
byte/object/work caps, derived-source rules, terminal taxonomy, hostile
fixtures and x64/i686/Android/full-gate requirements.

## Authority and next action

Run a separate optical-lineage binding implementation-readiness audit. Stop at
an exact owner action. Do not create the crate, schema, dependency or source;
do not add cumulative-power arithmetic, receiver semantics, endpoint arrival,
visibility, runtime integration, promotion or C3 closure without that later
authority.
