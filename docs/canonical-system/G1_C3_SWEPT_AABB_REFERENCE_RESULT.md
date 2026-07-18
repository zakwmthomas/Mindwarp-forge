# G1 / C3 swept AABB reference result

Status: implemented reference; capability-free; not promoted runtime behavior.

The approved fixed-orientation translated-AABB passage layer now exists as an
additive consumer of the validated physical-volume contract. It distinguishes
closed contact from strict interior overlap, including the dangerous
positive-duration face-slide case, and requires explicit subject-specific
mechanical rules instead of deriving mechanics from gas, liquid, or solid
phase names.

The reference returns typed initial overlap, contact, first interior entry,
outer-domain, unavailable-evidence, missing-interaction-model, and unsupported
motion outcomes. It provides no response vector and makes no claim about
walkability, organisms, planets, terrain, or biomes.

Independent evidence is produced by
`tools/prove-g1-c3-swept-aabb.py`, which enumerates exact critical plane times
and midpoints using Python `Fraction`. Its hostile vector checksum is
`7d34a367c5cbfb418bf0caca084f1757e9c2539bc110d252c9e7ebf20ab4d43e`.
The nine Rust tests cover face/edge contact, boundary-away/inward behavior, initial
overlap, unavailable and missing profiles, phase non-authority, strict codecs,
forged results, unsupported rotation, oversized probes, true expansion overflow, and the 65,536-cell
ceiling. The maximum-cell debug test completed inside the 30-second gate and
the canonical output remained below 32 MiB.

Rollback remains additive: remove this crate and its registrations, contract,
oracle, verifier, and result record. `physical-path-substrate` was not edited.
