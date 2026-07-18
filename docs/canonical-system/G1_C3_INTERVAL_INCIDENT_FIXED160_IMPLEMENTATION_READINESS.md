# G1 / C3 fixed-160 interval-incident implementation readiness

Date: 2026-07-16

Status: **implementation-ready behind one exact owner action; no implementation
has been performed.**

## Decision

The corrected interval-incident capability is ready for a narrow additive
implementation inside `visible-radiance-interface-event`. The numerical rule
is production-computable: classify each band over the whole declared component
box exactly, return a typed mixed-branch band when classification is not
uniform, and otherwise run one outward 160-bit evaluation whose result is a
`bounded_enclosure`. It never claims numerical tightness and never consults the
384-bit verification oracle.

The remaining seams close only with the input and API shape below. In
particular, the interval input does **not** embed a physical recipe and does not
accept an exact path query. The compiler receives already-canonical recipe and
volume objects separately, revalidates them, and binds their identities. This
keeps decode allocation bounded, avoids recursively nesting prior events, and
states the evidence honestly as a declared conditional direction box rather
than fabricated end-to-end path reconstruction.

## Additive public surface

Implementation is confined to a private `interval` submodule with additive
re-exports from the existing crate. It introduces a separately versioned
`VisibleRadianceIntervalInterfaceInputV1`,
`VisibleRadianceIntervalInterfaceEventV1`, per-band result enum, fixed-160
arithmetic receipt, strict codecs, compiler and validator. It uses new domain
separators:

- `forge-visible-radiance-interval-interface-input-v1`; and
- `forge-visible-radiance-interval-interface-event-v1`.

Existing `CONTRACT_VERSION`, public V1 point types, enum tags, functions,
domain separators, bytes and identities remain untouched. The interval
compiler accepts `&PhysicalVolumeRecipeV1`, `&PhysicalVolumeV1` and the new
input. Its validator recompiles the full interval event from those same three
objects.

## Input provenance and replay

The new input contains exactly:

- schema version `1`;
- nonzero incident source, scope and reconstruction identities plus a positive
  incident revision;
- evidence kind `declared_conditional_direction_box`;
- the canonical physical-volume recipe and volume identities;
- ordered source and target cell indices;
- the existing canonical `FaceInteractionEvidenceV1`; and
- three Q1.62 incident component intervals encoded with the existing strict
  signed-decimal representation.

The compiler validates the supplied recipe and volume through their public
replay APIs, requires scope/reconstruction agreement, rebuilds both cells,
requires exactly one shared face, and requires the face record's canonical
cell/media pair to match those rebuilt cells. The ordered source/target cells
determine orientation; swapping them changes the interval input identity.

This is conditional local evidence. Nonzero provenance labels do not prove a
prior optical event, endpoint arrival or a continuous path. Those nonclaims are
encoded in every event and its identity. A future composer must provide its own
bounded lineage contract; this API neither nests nor trusts a prior event.

## Exact incident-box validity

All component endpoints are canonical signed decimal Q1.62 integers in
`[-2^62, 2^62]`. Before arithmetic, validation rejects:

1. a reversed component interval;
2. a box containing the zero vector;
3. a box that does not intersect the unit sphere; and
4. a normal component that does not point strictly from source to target over
   the entire box.

The unit-sphere test is integer exact. For component interval `[l_i,u_i]`, let
`m_i = 0` when it contains zero and otherwise
`min(l_i^2,u_i^2)`, and let `M_i = max(l_i^2,u_i^2)`. The box intersects the
unit sphere exactly when

`sum(m_i) <= 2^124 <= sum(M_i)`.

The zero vector is contained exactly when every component contains zero. For
the unique shared-face axis, a positive source-to-target face requires
`lower > 0`; a negative face requires `upper < 0`. The implementation
canonicalizes that sign only inside the evaluator and maps output directions
back to world orientation.

## Per-band result shape

Dispersion is not collapsed into one event-wide branch. Each RGB band is one
of:

- `bounded_enclosure { branch: all_tir | all_transmit, event }`;
- `ambiguous_interface_branch`; or
- `nonconvergent_enclosure { reason_code }`.

`all_tir` contains reflected power/direction and no transmitted direction.
`all_transmit` contains reflected and transmitted power/direction. A mixed
branch contains no representative power or direction. Physical width remains
valid output width. `nonconvergent_enclosure` is reserved for inability of the
fixed evaluator to form a finite enclosure at the declared cap; overflow or an
invariant breach is an arithmetic error, not nonconvergence.

The event-level arithmetic receipt records three exact classifications, the
number of evaluated bands, fixed precision `160`, work units equal to
`160 * evaluated_band_count`, maximum stored endpoint bits, 512-bit storage and
the derived live ceiling `452`. A 64-event continuation is a hostile test
fixture only and never appears as one local-call work receipt.

## Codec and allocation ceilings

The interval input does not contain the physical recipe or volume payload.
Canonical input bytes are hard-capped at 16 KiB before decode; canonical event
bytes are hard-capped at 64 KiB before decode. Encoding also fails if either
cap is exceeded. Endpoint strings are bounded by Q0.48/Q1.62 target ranges,
and unknown fields, leading zeros, plus signs, negative zero, whitespace,
trailing bytes, scale drift and noncanonical object order fail replay.

Implementation tests must measure the maximum admitted input and the widest
three-band event, record decoded structure sizes, and prove both remain below
their caps. No allocation occurs from unbounded caller bytes. The separately
supplied physical recipe retains its existing 65,536-cell/run validation and
is not duplicated into interval identity bytes.

## Arithmetic and source shield

The internal evaluator reuses the crate's private checked signed-512 and
fixed-interval primitives. It adds no dependency and no native float. Each
non-ambiguous band executes once at 160 fractional bits. Source verification
must reject any production decision reference to `384`,
`REFERENCE_PRECISION`, point-oracle output, adjacent-precision agreement or
external endpoint excess.

The admitted-domain shield is `max(F + 232, 2F + 132)`. At `F = 160` this is
452 live bits. Checked overflow, zero division, an invalid square root, empty
intersection or violation of that shield is an arithmetic defect. The 384-bit
and point evaluators remain external test truth only.

## V1 identity freeze and rollback

Before adding the interval module, the implementation package must capture and
commit canonical input bytes, event bytes, input IDs and event IDs from the
unchanged point-v1 source for normal incidence, index match, reverse direction,
TIR/critical, unsupported-model and retained wide-coprime fixtures. The capture
is bound to the current generated module fingerprint and must pass before any
interval source is compiled. The same vectors must pass byte-for-byte after the
addition, alongside the existing independent Python portfolio checksum.

Rollback deletes the interval submodule, its additive re-exports, new fixtures
and the additive contract section. There is no new dependency, migration,
stored-data reinterpretation or change to point-v1 identities. Any point-v1
byte, ID, test, dependency or behavior change triggers immediate rollback.

## Adversarial closure table

| Seam | Closed rule | Stop/rollback trigger |
|---|---|---|
| provenance | declared conditional box with explicit identity and nonclaims | any claim of reconstructed prior path or endpoint arrival |
| recipe replay | recipe/volume passed separately and fully validated | embedded recipe, ambient latest object or trusted caller cell/media |
| validity | exact range, ordering, zero, sphere and strict-face tests | representative direction, clamping or inclusive normal orientation |
| mixed bands | independent tagged result per RGB band | event-wide majority branch or fabricated representative result |
| numerical rule | one outward fixed-160 evaluation after exact classification | hidden 384-bit, adaptive tightness or adjacent-precision stopping |
| resources | 16 KiB input, 64 KiB event, at most three 160-bit evaluations | pre-cap decode, unmeasured maximum fixture or 64 events charged locally |
| portability | checked integer Rust and target-neutral decimal codecs | native limb bytes, native float or platform-specific canonical output |
| ownership | private additive submodule in the existing interface crate | second optical crate or public duplicated numerical kernel |
| compatibility | committed pre-change point-v1 byte/ID vectors | any point-v1 byte, identity, dependency or behavior drift |
| rollback | delete additive surface with no migration | shared refactor that cannot be removed independently |

## Exact owner action

Approve one test-first implementation package inside
`crates/visible-radiance-interface-event` that:

1. captures and locks the unchanged point-v1 byte/ID vectors before interval
   code;
2. adds only the private additive interval submodule and re-exports described
   above;
3. implements exact box validity, per-band classification and one fixed outward
   160-bit evaluation with the 452-bit source shield;
4. enforces 16 KiB input and 64 KiB event pre-decode caps and records maximum
   allocation/byte receipts;
5. tests hostile critical, zero, sphere, orientation, sign, codec, forced-cap,
   mixed-band, 64-event widening, x64, i686 and Android ARM64 compile lanes; and
6. stops and rolls back on any point-v1 drift, hidden reference precision,
   unbounded allocation, dependency addition or failed platform lane.

This action does not authorize a composer, representative ray, coefficient
catalogue, persistence, runtime integration, perception, rendering, collision,
navigation, organism behavior, biome/ecotone meaning, sphere, planet, terrain,
promotion or C3 closure.
