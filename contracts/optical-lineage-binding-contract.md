# Optical-Lineage Binding Contract v1

This additive capability-free contract binds ordered local optical evidence. It
replays, but does not replace, `physical-path-substrate`,
`visible-radiance-bulk-transfer`, and `visible-radiance-interface-event`.

An input binds one validated bulk profile, one red, green, or blue band, one
nonzero lane source, and between one and 64 complete step-evidence pairs. Every
physical cell-step input/event, conditional bulk query/transfer, and optional
interface input/event is reconstructed through its owning compiler. The binder
does not accept caller-authored classifications, output identities, or terminal
dispositions.

Lane, derived-source, step, bundle-receipt, and transcript identities use
separate frozen SHA-256 domains. The first cell input must use the lane source
at revision one. Each later cell input uses the derived cell-input source, the
next exact revision, the predecessor's certified neighbour and hit-point box,
and either the unchanged direction box or the selected band's owner-produced
transmitted direction box. Public pure identity helpers allow callers to build
that chain without granting traversal or numerical authority.

Same-medium adjacency forbids interface evidence. A changed known-medium face
requires a face input/event whose source, revision, cells, media, direction and
owner replay match the bulk predecessor. `all_transmit` may continue;
`all_tir`, ambiguous branch, fixed-160 nonconvergence, and unsupported model
remain distinct typed terminals.

The complete terminal taxonomy is outer-domain exit, unavailable neighbour,
unavailable current cell, ambiguous next face, no forward progress, all-TIR,
ambiguous interface branch, nonconvergent interface enclosure, unsupported
interface model, and work exhaustion. A terminal cannot have later evidence.
Work exhaustion is synthesized only when the 64th valid step would otherwise
continue.

Bundle receipts cover canonical bytes and identities for every nested local
object. A bundle admits at most 384 objects and 16 MiB; a manifest admits at
most 1 MiB. Strict JSON codecs reject unknown fields, trailing bytes,
noncanonical bytes, foreign identities, independently resealed adjacency
forgeries, limitation drift, and authority mutation. Validation recompiles the
entire manifest.

The binder has no dependency on fixed-interval arithmetic and performs no
attenuation, Fresnel, direction, coefficient, or cumulative-power fold. It
makes no claim of source emission, endpoint or receiver arrival, visibility,
perception, rendering, gameplay line of sight, passage, navigation, biome,
planet, terrain, persistence, runtime, approval, or promotion. Its authority
effect is exactly `none_evidence_only`.
