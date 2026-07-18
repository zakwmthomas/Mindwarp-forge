# G1 / C3 interval bulk-transfer implementation result

Date: 2026-07-16

Status: **implemented and verified as one additive one-band local proof;
composition, endpoint arrival, visibility and C3 closure remain unapproved.**

## Result

`visible-radiance-bulk-transfer` now exposes a separately versioned
`ConditionalIntervalBulkQueryV1` and `ConditionalIntervalBulkTransferV1` from
its private `interval` module. Each call binds one validated bulk profile, one
red, green or blue band, and one complete conditional physical cell-step
input/event pair. The compiler reconstructs the profile volume and revalidates
the nested event before deriving any transfer evidence.

For a certified span, the implementation intersects two outward Q160 length
certificates: direction norm times face time, and start-box-to-hit-box
displacement norm. It then reuses the existing bulk-owned Q64.64 optical-depth
projection and Q0.48 exponential kernel. It never selects a midpoint,
normalizes an input box, or treats face time alone as Euclidean length.

The route returns typed unavailable-current, upstream ambiguity and
no-forward-progress outcomes. Known current-cell vacuum, finite or opaque
transfer remains attached before a known-neighbour, unavailable-neighbour or
outer-domain terminal disposition. It processes exactly one spectral band
because dispersed lanes may enter different cells.

## Frozen arithmetic and codec bounds

The implementation depends directly on the capability-free
`fixed-interval-arithmetic` crate. Wide intermediates use signed 512-bit
storage with a 414-magnitude-bit shield; the intersected final length is capped
at 192 magnitude bits. Its replayed receipt freezes seven shifts, eight
interval multiplications, four additions, three subtractions, two directed
square roots, one intersection, one Q64 projection and 192 exponential terms.

Query bytes are capped at 64 KiB and transfer bytes at 16 KiB before decode.
Strict reconstruct-and-compare JSON rejects unknown fields, foreign profile or
volume identities, forged nested cell-step events, result mutation,
noncanonical bytes and oversized input.

## Compatibility and hostile evidence

Before adding the dependency or conditional source, eight existing bulk V1
families were captured through the public API. Their committed fixture SHA-256
is `67783f4eae5f737979580fbddd6725d4faaa556fb031b90730cf7359ba27fce2`.
Vacuum identity, finite zero, finite positive, opaque, unavailable, ambiguous
boundary, interface-required and stationary families retain their exact byte
lengths, byte hashes and public profile, recipe, volume, query, witness and
transfer identities after the additive implementation.

Focused Rust tests cover exact dual-certificate agreement, all three band
interactions, outer and unavailable terminal retention, unavailable current
evidence, ambiguous and stationary geometry, widened directions, canonical
query/transfer replay, caps, unknown fields, forged nested evidence and forged
arithmetic receipts.

## Verification receipt

- Native Windows warnings-denied: 12 legacy, one eight-family identity lock and
  four additive interval tests passed.
- i686 Windows execution: the same 17 tests passed.
- Android ARM64: `cargo check -p visible-radiance-bulk-transfer --target
  aarch64-linux-android` passed.
- Independent oracle: canonical receipt
  `94b2fe43260c9a604ec6c22035f28f7026319531c22951a4e8747f8d242713c3`
  reproduced with 321 maximum observed live bits, 512 named and 15,808
  generated exact corner witnesses, plus four 64-step lanes.
- Permanent source and governance shields passed, including module dependency,
  contract, byte fixture, operation ceiling and no-optical/no-composition
  checks.
- Complete `tools/verify.ps1` passed in 232.8 seconds after the additive source,
  tests, contract, module projections and permanent verifier were present.

Actual mobile-device performance remains unmeasured. This result does not
authorize optical arithmetic migration, an ordered lineage adapter, path
composition, endpoint arrival, visibility, perception, rendering, gameplay,
organism meaning, biome presentation, planet, terrain, runtime, promotion or
C3 closure.

## Rollback

The change is additive. Rollback is deletion of the interval module, its
focused test and direct shared-arithmetic dependency plus removal of the
additive contract/module/verifier entries. Existing bulk V1 data and identities
require no migration.

