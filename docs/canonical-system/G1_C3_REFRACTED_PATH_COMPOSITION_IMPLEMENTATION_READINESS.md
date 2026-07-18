# G1 / C3 refracted-path composition implementation readiness

Date: 2026-07-16

Status: **not implementation-ready; no owner implementation action prepared.**

## Decision

The transmitted-only three-band composition candidate is bounded at the graph
level, but its second step cannot be expressed through the existing verified
interfaces. Implementation now would require one of four unaudited behaviors:

1. snap an interval event point and direction to Q32.32;
2. treat an arbitrary representative ray as the physical path;
3. duplicate the bulk and interface numerical kernels inside a composer; or
4. modify the verified bulk/interface contracts before proving their broader
   interval-input semantics.

All four are rejected. The package stops before schema, crate, dependency or
owner implementation approval.

## Exact API mismatch

### Bulk transfer

`VisibleRadianceBulkQueryV1` contains a `PhysicalPathQueryV1`, whose two
endpoints are exact signed Q32.32 triples. The compiler reconstructs exact
closed-cell witnesses from those points. After refraction, however, both the
origin and direction are outward enclosures. A next-face intersection is also
an enclosure. No canonical exact Q32.32 segment is implied by that evidence.

Calling the existing bulk compiler would therefore require shrinking an
enclosure to a point. Outward rounding afterward cannot repair a cell or face
chosen from the wrong representative. A one-bit near-edge fixture can change
topology before attenuation arithmetic begins.

### Interface event

`VisibleRadianceInterfaceInputV1` likewise contains one exact
`PhysicalPathQueryV1`. Its local kernel derives the incident direction from
that exact integer delta. A transmitted direction from the first event is
instead three Q1.62 component enclosures per spectral band. The current API
has no interval-incident input and makes no closure claim under repeated
composition.

Submitting a new exact path inside that enclosure would turn a caller-selected
approximation into canonical incidence. Replaying the local event would prove
only that approximation, not the full prior enclosure.

## Failure points engineered out at readiness

| Tempting repair | Failure | Permanent rule |
|---|---|---|
| round the event point to the nearest Q32.32 coordinate | a face/edge neighborhood can enter a different cell | topology is certified over the entire enclosure or the result is ambiguous |
| use the interval midpoint direction | the midpoint may select one face while another admitted direction selects another | no representative ray becomes canonical truth |
| call the old bulk kernel on enclosing endpoint corners | corner pairing does not enclose every correlated ray and can invent impossible lengths | retain point-direction correlation explicitly |
| copy exponential and Snell arithmetic into the composer | fixes diverge and two implementations can claim different physics | one owning numerical kernel per semantic operation |
| silently broaden the old module inputs | invalidates their existing oracle and rollback claims | broader interval inputs require a new versioned candidate and oracle |
| recurse reflected and transmitted lanes while solving the mismatch | multiplies an unresolved enclosure problem exponentially | v1 remains transmitted-only and at most three bands |

## What remains valid from the design

- transmitted-only continuation is the narrowest direct-path scope;
- red, green and blue require separate lanes because dispersion is real in the
  existing output contract;
- total internal reflection is terminal for one lane;
- edge/corner overlap, repeated state, no forward progress and hard event/work
  ceilings remain typed terminations;
- no epsilon, native float, runtime ray cast or platform-specific semantic fork
  is admissible; and
- the existing point path, bulk transfer, local interface event and swept-AABB
  references remain unchanged and removable independently.

## Required prerequisite

The next package is a capability-free **interval optical continuation
counterexample and representation audit**, not composer implementation. It must
answer two questions before any readiness retry:

1. Can a correlated point-and-direction enclosure certify a unique next cell
   face, positive progress and endpoint relation with fixed 512-bit bounded
   arithmetic, returning typed ambiguity otherwise?
2. Can the existing smooth-dielectric equations accept an incident direction
   enclosure and produce containing transmitted direction/power enclosures
   within a declared hard precision/work ceiling, or must local composition
   remain unavailable?

The cheapest proof is an independent arbitrary-precision Python portfolio. It
must compare at least:

- affine/correlated ray enclosures;
- independent axis boxes;
- finite candidate-chain validation; and
- exact symbolic or rational witnesses used only as oracle truth.

Required hostile cases are one-bit next-face reversal, exact edge/corner,
near-parallel travel, prior-face re-entry, point-direction correlation loss,
normal incidence, critical-angle neighborhoods, interval-induced TIR/transfer
ambiguity, alternating-interface loops, widening, 64-event work exhaustion and
three-band dispersion.

## Resource question, not commitment

The earlier proposed ceilings—three lanes, 64 events, fixed 512-bit storage,
8 MiB canonical result and 64 MiB peak reference memory—remain candidate
budgets only. The prerequisite oracle must measure whether correlated interval
state and interval-in/interval-out interface evaluation fit them. It may lower
the domain or return typed nonconvergence, but may not silently raise precision
or discard admitted states.

## Rollback and authority

No code was added, so rollback is this readiness correction and checkpoint
route only. `physical-path-substrate`, `visible-radiance-bulk-transfer`,
`visible-radiance-interface-event` and `swept-aabb-passage` remain untouched.

There is deliberately no implementation phrase for the owner to approve. A
later readiness audit may prepare one only after the interval-continuation
oracle proves a bounded representation and exact failure semantics. General
instructions to continue do not authorize composer code, numerical-kernel
duplication or modification of the verified optical modules.
