# G1 / C3 cumulative lane-transfer implementation result

Date: 2026-07-16

Status: **implemented and verified.**

The explicitly approved additive `optical-lane-transfer-binding` crate now
exists. It replays a complete optical-lineage bundle and manifest, derives the
ordered same-band bulk and followed-interface factors without accepting a
caller factor list, accumulates them with directed Q0.160 checked arithmetic,
and outward-projects once to Q0.48.

The implementation retains the frozen 18 MiB input, 256 KiB output, 32 MiB
live-canonical, 128-factor and 209-live-bit ceilings. Separate factor, result,
and transcript domains bind the exact owner objects, ordering, arithmetic,
terminal, limitations, and `none_evidence_only` authority effect.

Five focused Rust tests pass. They cover vacuum identity, finite attenuation,
opaque zero, an unavailable-current terminal with no fabricated factor, a real
two-step all-transmit route, real All-TIR/ambiguous/unsupported terminal
interfaces, strict input and output replay, unknown and trailing bytes,
deletion/duplication/band/role/owner/endpoint/identity/terminal/limitation and
authority forgery, stale nested evidence, sub-Q0.48 retention, factor 129,
invalid endpoints, the 209-bit shield, and the exact 128-factor work ceiling.

Native warnings-denied tests passed. The same five tests executed on
`i686-pc-windows-msvc`; `aarch64-linux-android` ARM64 compilation passed. The
pinned independent oracle retained receipt
`ee5f237fe1c8b7581372646e01ab12c7ddedfa1707d1b0e5dbf199e81b2ba09d`,
eight portfolios, all ten terminal families and 26 hostile rejections.

The four upstream fixture hashes remained frozen:

- physical exact: `32a9de48cde37174604785b8e1f967106babd46765498921f03b8fa4c56e1869`;
- physical interval: `1d04495829ebf997417a3638cbf82607e697a14c3b0bed3218ef03bebd92e453d`;
- bulk: `67783f4eae5f737979580fbddd6725d4faaa556fb031b90730cf7359ba27fce2`;
- interface point: `cd055393aef810152a164e4a000bcd6307a9d2bd45ea7ba3a8e63aee342b1b49`.

Sixty-one upstream physical/bulk/interface/lineage tests and the five new
cumulative tests passed. Module boundaries and all 45 generated module front
doors passed. Complete `tools/verify.ps1` passed after the strengthened source
and tests in **234.0 seconds**, including record roles, the entire warnings-
denied Rust workspace, the isolated Forge desktop build, UI build, formatting,
and whitespace checks.

This result does not establish source emission, receiver geometry or arrival,
inverse-square spreading, detector response, detectability, visibility,
perception, rendering, gameplay line of sight, runtime integration, promotion,
or C3 closure. Actual mobile-device performance remains unmeasured; Android
ARM64 evidence is compilation-only.
