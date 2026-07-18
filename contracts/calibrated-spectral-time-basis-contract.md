# Calibrated spectral/time basis contract

Status: owner-approved additive V1 implementation contract.

## Purpose and authority

`calibrated-spectral-time-basis` is a capability-free evidence owner for one
exact physical spectral partition and time cell. It derives a calibrated-basis
identity, an opaque legacy time commitment, and three byte-compatible legacy
RGB band/time identities. Every result carries
`authority_effect = none_evidence_only`.

It does not own source allocation, transport coefficients or applicability,
spatial calibration, received energy, detector response, visibility, runtime,
promotion or C3 closure. Fixture wavelength boundaries, clock identities and
tick scales are test data rather than normative calibration.

## Frozen V1

The input fixes radiant energy in joules, vacuum wavelength in metres,
unit-energy integration, exactly three ordered contiguous nonempty half-open
blue/green/red intervals, one nonempty half-open u64 tick cell, a nonzero clock
origin, nonzero provenance and positive basis version. Boundaries and seconds
per tick are reduced unsigned `u128` rationals with canonical decimal strings
of at most 39 digits. No float conversion is allowed.

The result embeds the validated input, the calibrated-basis ID, derived legacy
time ID, exactly three derived legacy band/time IDs, limitations and
authority-negative effect. Derived identities are reconstructed rather than
trusted.

## Identities and codecs

Basis, legacy-time and legacy-band/time identities use SHA-256 over their
frozen domain separator, a zero byte and canonical JSON. The legacy band/time
domain and tuple bytes are unchanged from V1. Strict codecs deny unknown and
duplicate fields, reject aliases, invalid UTF-8 and trailing bytes, revalidate
every identity, require exact byte replay and enforce 16 KiB input, 32 KiB
result and 64 KiB aggregate live-canonical ceilings.

## Dependency and rollback boundary

The crate depends only on `serde`, `serde_json` and `sha2`. Its first authorized
consumer is the separate additive `calibrated-source-energy-distribution`
evidence owner; that import does not change this owner's API, bytes, identities
or behavior. Rollback of the calibration owner remains deletion-only: remove
the crate, contract, tests, fixtures, verifier, result and governance entries.
No legacy data, identity or behavior migrates.
