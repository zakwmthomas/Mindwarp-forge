# Optical phase-space receiver-coupling contract

Version: 1

## Purpose

This contract defines capability-free evidence that one complete correlated
optical phase-space cell is uniformly inside a receiver before a selected
same-medium physical face, uniformly separated from that receiver, or remains
unresolved. It does not estimate a partial fraction.

## Bound inputs

`WholeCellReceiverCouplingInputV1` binds one complete unchanged
`OriginAnchoredTransportInputV1`, its replayed
`OriginAnchoredTransportCertificateV1`, one existing selected step index and
one unchanged `ReceiverAabbV1`. Scope, reconstruction, coordinate frame,
physical volume, immutable cell, certificate and selected step must agree.

## Classification

Full is certified only by strict uniform start-inside or one uniformly inward
receiver face with receiver entry after-or-at segment start, strictly before
the selected physical face, and strict cross-axis interior for the complete
correlated cell. Zero is certified only by strict swept separating-axis
evidence. Equality, tangency, face coincidence, mixed direction/order,
partial overlap, unsupported evidence and exhausted work remain unresolved.

Exactly one of accepted, zero and unresolved measure equals the complete input
cell measure; the other two are canonical zero. No measure may be sampled,
averaged, copied across children or discarded.

## Arithmetic

The implementation replays the immutable common-denominator origin and
physical Q32 face. It combines like correlated monomials before termwise exact
bounds. It must never multiply independently reduced public transport forms.
The live shield is 391 bits in opaque checked signed-512 storage, with at most
16,384 checked integer operations and 4,096 bound comparisons.

## Codecs and resources

All public structs deny unknown fields. Canonical JSON inputs are capped at 40
MiB, results at 256 KiB and aggregate live canonical data at 64 MiB. Validation
replays nested owners, recompiles the result and compares complete bytes and
identities. Identity domains are
`mindwarp.optical-phase-space.receiver-coupling.input.v1` and
`mindwarp.optical-phase-space.receiver-coupling.result.v1`.

## Authority

Every result carries `none_evidence_only`. This contract grants no partial
arrival fraction, source magnitude, radiance, attenuation, power, detector,
visibility, perception, runtime, promotion, persistence or C3 closure. It
changes no existing owner source or V1 behavior.
