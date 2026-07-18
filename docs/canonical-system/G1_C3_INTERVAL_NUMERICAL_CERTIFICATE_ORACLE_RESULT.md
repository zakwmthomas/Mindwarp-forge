# G1 / C3 interval numerical-certificate oracle result

Date: 2026-07-16

Status: **fixed-160 sound-enclosure semantics supported for a revised
implementation-readiness audit; no schema or implementation is authorized.**

## Result

The disposable production-decision audit ran a rule that has no access to the
384-bit reference: classify the whole component box exactly, return
`ambiguous_interface_branch` for a mixed box, and otherwise perform one fixed
outward 160-bit evaluation. A successful result is named
`bounded_enclosure`, not `known` or `numerically tight`.

Across all 265 retained cases it returned 263 bounded enclosures and two exact
mixed-branch outcomes. Every bounded result contained the external 384-bit
test result, and projected endpoint excess at Q0.48/Q1.62 was zero in this
portfolio. Production work reached 388 observed live bits and 165 stored
endpoint bits.

The admitted-domain ceiling retains the existing expression
`max(F + 232, 2F + 132)`: Q1.62 component-square extrema and Q16.48 index
products require at most the 232-bit exact branch geometry, while the widest
fixed operations require at most `2F + 132`. At `F = 160`, the derived maximum
is 452 bits inside checked 512-bit storage. This derivation must become a
source-level implementation shield before later readiness can close.

## Correction to the earlier forced-cap interpretation

The critical 80-bit raw evaluator returns `all_transmit`; it does **not**
return `nonconvergent_enclosure`. Its 2,991-unit numerical excess is discovered
only by comparing it with the oracle-only 384-bit result. The earlier receipt's
forced-cap disposition was therefore an external test judgment, not a
production-computable evaluator outcome.

This distinction is now permanent: verification checks the raw outcome and
also checks that the production decision function contains no reference-
precision dependency. A future contract may not present external-oracle
judgment as an emitted runtime outcome.

## Strategy disposition

- An analytic operation-by-operation padding budget remains unselected because
  no proof yet separates interval dependency width from rounding padding.
- A widened shadow evaluation remains unselected because another finite-
  precision enclosure does not by itself bound the limiting enclosure.
- Fixed 160-bit sound enclosure is supported because outward arithmetic makes
  soundness production-computable without a hidden tightness test.

Under the selected semantics, `nonconvergent_enclosure` is reserved for the
fixed evaluator's inability to form a finite enclosure at the declared cap,
such as a required denominator interval containing zero after validated input.
Overflow or an invariant breach remains an arithmetic defect. A physically
wide but finite enclosure remains a valid bounded result.

## Repeated-event and resource receipt

All red, green and blue lanes again completed 64 alternating transmissions.
The worst direction width remained 150,283,463 Q1.62 units and repeated work
remained at 378 observed live bits. The 64-event ceiling belongs to a future
composer; here it is only a hostile widening/resource fixture for the local
output representation.

The deterministic receipt contains 2,340 direct checks and checksum
`6f4c5997d23bd6a463ccc7e7d0d3a843f52f453425b4eea5566721f4535dc082`.

## Next route and nonclaims

The next package is a revised interval-input implementation-readiness audit.
It may replace adaptive tightness language with fixed-160
`bounded_enclosure`, but must still close input provenance/replay, zero-vector
and unit-sphere validity, strict face orientation, per-band mixed outcomes,
canonical codec byte ceilings, allocation measurements, v1 byte identity,
single-module ownership and rollback.

No interval schema, Rust module change, dependency, composer, coefficient
catalogue, perception, rendering, collision, navigation, organism, biome,
sphere, planet, terrain, runtime, promotion or C3 closure is authorized.

