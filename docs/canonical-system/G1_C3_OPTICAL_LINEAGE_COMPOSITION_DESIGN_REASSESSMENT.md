# G1 / C3 optical-lineage composition design reassessment

Date: 2026-07-16

Status: **thin immutable lane manifest selected for counterexample/oracle
proof; no schema, crate or composer implementation authorized.**

## Decision

The current local APIs are sufficient to describe a candidate transmitted
optical lane without changing their numerical semantics, but they are not
sufficient for a caller-trusted list of IDs to become canonical lineage. A
future validator must receive and replay every referenced local object and
prove every cross-owner derivation in order.

The selected next proof candidate is therefore a **thin immutable lane
manifest plus an explicitly supplied bounded object bundle**. The manifest
owns only order, band, predecessor and terminal bindings. It does not own
cell traversal, interface equations, extinction, exponentials, cumulative
power, endpoint classification or ambient object lookup.

This selection authorizes an independent counterexample/oracle audit only. It
does not authorize Rust types or the resource targets below.

## Actual public seam inventory

### Physical cell step

`ConditionalIntervalCellStepInputV1` binds recipe/volume identity, current
cell, Q160 point box and Q1.62 direction box. Its event binds the input ID,
current cell, certified face/time/hit-point evidence and typed terminal. The
event does not bind a spectral band, prior interface event or path history.

### One-band bulk transfer

`ConditionalIntervalBulkQueryV1` nests the complete cell-step input/event and
binds one bulk profile and one band. Its transfer binds the derived query ID,
nested input/event IDs, current cell, local transfer and terminal. It therefore
provides the strongest existing local step anchor, but intentionally has no
predecessor or successor field.

### Interval interface event

`VisibleRadianceIntervalInterfaceInputV1` binds volume, source/target cells,
one complete face-interaction record and one Q1.62 incident-direction box. Its
event binds the input ID and three band outcomes. It does not bind the
cell-step event, certified hit-point box or one selected continuation lane.

The interface `DecimalIntervalV1` and physical
`SignedDecimalIntervalV1` encode the same Q1.62 integer endpoints differently:
the former carries `FixedScaleV1::Q1_62`; the latter carries
`fractional_bits = 62`. Exact endpoint-string equality plus those fixed scale
tags is a lossless adapter rule. Numeric reparsing and rerounding are
unnecessary and forbidden for lineage binding.

## Candidate comparison

| Candidate | Replay and corruption behavior | Size/cost behavior | Disposition |
|---|---|---|---|
| IDs-only thin manifest with ambient lookup | order is compact, but missing or substituted objects depend on unspecified external state | smallest bytes; unbounded lookup and persistence semantics | reject |
| complete nested local objects per step | self-contained and simple, but bulk queries already nest cell input/event and repeated volume/profile data grows quickly | worst-case local codec caps permit tens of MiB before three dispersed lanes are complete | reject as canonical v1 candidate |
| streaming fold with only a final accumulator | cannot replay an omitted intermediate ambiguity, face or identity substitution | low retained bytes but destroys proof history | reject |
| no composer | preserves every local boundary but cannot prove that adjacent receipts describe one lane | zero new surface; leaves the named lineage prerequisite open | retain as fallback |
| thin manifest plus explicit bounded object bundle | manifest order is immutable; validator replays each supplied object and rejects missing, duplicate, foreign or unused objects | canonical manifest remains small while proof bundle cost is measured separately and never hidden | **select for oracle** |

The object bundle is an input to validation, not an ambient store and not part
of manifest identity. A valid receipt must include deterministic hashes and
counts for the exact supplied bundle so two validations cannot silently use
different evidence.

## Candidate identity model for the oracle

The oracle should model, but not yet freeze as Rust schema:

1. `lane_id`: domain-separated hash of reconstruction ID, bulk profile ID,
   band, initial cell-step input ID and a caller-declared nonzero lane source.
2. `step_ordinal`: zero-based contiguous integer with no gaps or duplicates.
3. `predecessor_step_id`: absent only at ordinal zero; otherwise exact prior
   step ID.
4. `cell_step_input_id`, `cell_step_event_id`, conditional bulk query ID and
   conditional bulk transfer ID for every step.
5. Optional interval-interface input/event IDs only when the certified known
   neighbour changes medium and valid face evidence exists.
6. `step_id`: domain-separated hash of lane ID, ordinal, predecessor, all local
   IDs, selected band outcome disposition and typed terminal.
7. `transcript_id`: domain-separated hash of lane ID, ordered step IDs, exact
   object-bundle receipt and final typed terminal.

No identity may hash native limbs, memory layout, platform paths, unordered
maps or a mutable database key.

## Mandatory adjacency proofs

For each admitted successor, a validator or oracle must prove all of the
following from replayed objects:

- reconstruction, recipe, volume, scope, profile and spectral band remain
  exact;
- the bulk query's nested cell-step input/event exactly equal the separately
  supplied objects and all four IDs recompute;
- the current cell of step `n + 1` equals the certified known neighbour of
  step `n`;
- the next Q160 point box equals the prior certified hit-point box byte for
  byte; no narrowing, epsilon or coordinate conversion occurs;
- without an interface, the next Q1.62 direction endpoints equal the prior
  direction endpoints byte for byte and the medium evidence is unchanged;
- with an interface, source/target cells equal the certified transition, face
  evidence reconstructs both media, and the next direction equals only the
  matching band's transmitted direction under the lossless codec adapter;
- all-TIR has no successor; ambiguous branch, nonconvergence, ambiguous face,
  no progress, unavailable current/next evidence and outer exit terminate with
  distinct typed reasons; and
- no event from another band or lane can satisfy a predecessor binding.

The local objects currently permit arbitrary nonzero source IDs and revisions.
A future readiness audit must choose deterministic derived source/revision
rules or explicitly include those declarations in lane identity. The oracle
must demonstrate that leaving them caller-selected enables identity aliases.

## Cumulative power and endpoint disposition

Cumulative power is deliberately excluded from the first lineage oracle.
Multiplying bulk transmission by interface transmitted power is a new ordered
semantic operation. Adding it now would blur whether a failure comes from
lineage or directed accumulation. After lineage survives hostile proof, a
separate composition arithmetic audit may compare Q0.48 fold rules using the
shared arithmetic core without copying either local optical kernel.

Endpoint arrival is also excluded. A Q160 point box intersecting or enclosing
a receiver declaration is not automatically exact arrival, and no current
receiver contract defines containment, aperture, time or band acceptance. The
candidate can prove only an ordered local optical-opportunity transcript and a
typed physical terminal.

## Counterexample and oracle requirements

The next independent audit must include:

- swapped, duplicated, skipped and cyclic ordinals;
- one-bit predecessor, band, profile, reconstruction and local-ID substitution;
- a valid event paired with a foreign but structurally equal input;
- hit-point narrowing, widening and endpoint-string recoding;
- same-medium continuation with a fabricated interface and changed-medium
  continuation without one;
- wrong face orientation, reversed source/target cells and stale interaction
  revision;
- red/green/blue cross-lane event substitution after dispersion;
- all-TIR, ambiguous-interface, nonconvergent, ambiguous-face, no-progress,
  unavailable-current, unavailable-neighbour and outer-exit terminals;
- repeated physical state with distinct ordinal IDs and an explicit loop/work
  exhaustion policy; and
- one, three, 64 and 192-step generated portfolios with measured manifest,
  object-bundle and peak-validation bytes.

The audit must compare at least these provisional measurement targets rather
than assume them: at most three lanes, 64 steps per lane, a 1 MiB manifest,
48 MiB supplied canonical objects and 64 MiB peak validation memory. A target
is reduced or the candidate rejected if exact replay cannot fit; it is not
silently raised.

## Failure points engineered out

| Failure | Permanent response |
|---|---|
| trust a list of valid-looking IDs | replay every explicitly supplied object and bind the exact bundle receipt |
| hide persistence behind content lookup | no ambient lookup in canonical validation |
| nest duplicate full objects until a cap happens to pass | separate compact identity from measured proof-bundle delivery |
| choose a representative hit point or direction | require byte-exact box propagation and lossless scale-tag conversion |
| recombine RGB after dispersion | one immutable band per lane and matching-band interface output only |
| treat lineage as cumulative transfer | prove order first; audit directed accumulation separately |
| claim receiver arrival from a terminal point box | retain physical terminal only until a receiver contract exists |
| let the composer repair local ambiguity | every local typed failure terminates unchanged |
| migrate interface arithmetic for convenience | the manifest owns no numerical kernel |

## Authority and next action

Run the capability-free optical-lineage counterexample/oracle audit above.
Stop after accepting or rejecting the thin-manifest-plus-explicit-bundle
candidate, measuring hostile portfolio costs and freezing alias, loop and
terminal semantics. Do not create a crate, schema, dependency, manifest codec,
composer, cumulative-power fold, receiver contract or endpoint claim. Any
code-facing readiness requires a later exact package and owner action.

