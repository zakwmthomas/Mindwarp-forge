# G1 / C3 receiver-arrival geometry oracle result

Date: 2026-07-16

Status: **the exact-ray bounded-AABB strict-interior candidate survives the
code-free counterexample oracle and may advance only to a code-facing
implementation-readiness audit.**

The independent Python oracle uses `fractions.Fraction` exact rational
arithmetic. It was run twice with byte-identical output.

- source SHA-256:
  `d1ea2e46e9e41e85b5523b629244b958b396903914fcf2f5dd70b7ad85f0a545`;
- receipt SHA-256:
  `25c31003ff4ee8d1be3b01a5a2203958238205e4adc80e6cc50623c27af69aea`;
- portfolio count: 18;
- hostile rejection count: 26.

The portfolios distinguish strict arrival before a face, after-face miss,
arrival at start, tangent and point contact, exact face tie, parallel-inside,
parallel-outside, reverse direction, fractional entry, a box spanning cells,
corner contact, three nondegenerate conditional-evidence failures, and three
upstream terminal-without-face outcomes. Exact parameter evidence includes the
fractional entry interval `(4/3, 8/3)`.

The decisive negative result is that nondegenerate point, direction or
face-time intervals are `unsupported_conditional_evidence`. The oracle does
not use a midpoint, corner sample or favourable witness. A point receiver is
contact-only, never strict-interior arrival. Contact whose first parameter is
the face parameter remains contact-only in the current step.

All 26 hostile families reject identity, scope, reconstruction, coordinate
frame, receiver bounds/volume, lineage transcript, lane, ordinal, owner-object,
rational endpoint, terminal, limitation, authority, deletion, duplication,
reordering, resealing, codec, cap, conditional-midpoint and face-tie mutations.

This result proves only the mathematical candidate. It does not derive a
fixed-width shield or authorize a crate, dependency, schema, test or source.
It makes no source emission, spreading, received-power, aperture, orientation,
detector, detectability, visibility, perception, rendering, gameplay, runtime,
promotion or C3-closure claim.

## Next bounded action

Prepare a code-facing implementation-readiness audit that derives fixed-width
arithmetic bounds from the existing Q160/Q1.62 owners, freezes strict schemas,
identity domains, byte/cost caps and hostile/platform tests, and then pauses at
one exact owner action. Do not implement before explicit approval.
