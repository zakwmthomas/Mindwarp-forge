# G1 GP4 Signal Anchor fixed registry

Status: frozen pre-source registry; it grants no runtime authority.

## Exact authored selection

| Field | Exact value |
|---|---|
| bundle | `gp4.signal-anchor.bundle-v1` |
| run | `gp4.signal-anchor.vertical-1` |
| session | `gp0.s4.signal-anchor` |
| situation | `gp3.s4.signal-anchor` |
| approach | `s4.approach.temporary` |
| outcome | `s4.temporary-rescue` |
| preparation intent | `rescue-before-anchor-collapse` |
| preparation tool | `temporary-brace-kit` |
| optional threat | selected, `wire-scavengers`, nonterminal |
| C3A input fixture seed | 32 bytes of `0x4a` |
| universe-address seed | 32 bytes of `0x0e` |
| hub/place/player payloads | `signal-hub` / `signal-anchor` / `signal-player` |
| descriptor payload | `gp4-signal-anchor-place-v1` |
| vertical identity fingerprint | `d7f9437bd750f5a85660e2a604fa894be22c7689f9983f565be64bb653ac6867` |
| GP3 grammar digest | `e8865e011d8b7ada0787303d49e4c769ff19164dc7a51f52d396e80b2c408b44` |
| GP3 S4 situation digest | `c258b1b83e86cc52f30502c8e8d29d7bbda161ce7abc1031d5612a65c84d5328` |
| GP3 S4 session digest | `b6428f7febdfab0560b975f02b47bb3dcbd7e940a4a88fd65028aa6c685a4033` |
| canonical C3A input SHA-256 | `5f54137fa9de4b06514dbfde509ef5faf65a23b885a24288ed5cb51bbcee07ca` |
| C3A packet ID | `947a0564c7a08115d4ee63ff89bfbdafdc9303ecd7f86c846b4945c7e305492b` |
| canonical C3A packet SHA-256 | `e3479b36a3e7085ae892a358ba7e5e6415688ef0d82e0338b9226ae71c46576f` |
| C4V baseline bytes SHA-256 | `5ef8f69963d20b11bb57b99250ed0e934dd998b081b60085fdf2b18200722884` |
| C4V GP1 reducer fingerprint | `7034df7aa827fd3c6f24a2d9a5113c7b9b0d6415bf0ca44e300ead40cfe1282f` |
| C4V codec-v1 fingerprint | `daaac2a5e65ad645f6ce7319f1429d577947a6bc760c6a5748ba267fa1603684` |

GP3 exports a strict situation and threat digest but no approach digest. GP4
therefore derives two reference digests from validated GP3 values. For domain
`D`, situation ID bytes `S` and canonical JSON entity bytes `E`, hash exactly
`D || u32be(len(S)) || S || u64be(len(E)) || E`.

| GP4 reference | Domain including NUL | Exact digest |
|---|---|---|
| temporary approach | `mindwarp.gp4.gp3-approach-ref.v1\\0` | `21b33ac9883afa4df0667bd720af3c7f12b956726e8f2841f1eed1fd9fd0a24f` |
| wire-scavenger threat | `mindwarp.gp4.gp3-threat-ref.v1\\0` | `5e60a42c78bc085662fe8264fb362ecd9af141b5aa874d0ef8013254fcf3a735` |

The upstream GP3 threat digest, validated before the GP4 reference digest, is
`9d9e3507f19953aef3c7a2013fac50c370d15e94013318c45d5b31fad33aa248`.

Both are recomputed only after GP3 fixed-registry validation. This file does
not invent a GP0 contract digest or duplicate an upstream receipt.

## Exact C4V batches

Command IDs are 32-byte SHA-256 values, never labels. For identity fingerprint
`I`, bundle bytes `B`, run bytes `R`, sequence `n`, expected revision `v`,
expected parent `P` and canonical JSON action-vector bytes `A`, hash exactly:

`"mindwarp.gp4.signal-anchor.command.v1\\0" || I || u32be(len(B)) || B || u32be(len(R)) || R || u64be(n) || u64be(v) || parent_tag || P? || u64be(len(A)) || A`

The displayed `\\0` is exactly one `0x00` byte, not two UTF-8 characters.
`B` is exactly the UTF-8 bytes of fixed `bundle_id`, not the bundle payload.
`parent_tag` is `0x00` with no following parent or `0x01` followed by exactly
32 parent bytes. `A` is the strict ordered `Vec<BaseLoopActionV1>` encoding.

| Sequence | Expected revision | Expected parent | UTF-8 label used only by the frozen fixture registry | Atomic batch | Command ID hex | Required observation |
|---|---|---|---|---|---|---|
| 1 | 0 | `none` | `prepare` | `Prepare` | `6fa6a6d429003d91fb4f577486a34ff4bf174e16e659c966ccf3327e8dd2cc15` | resumable prepare stop |
| 2 | 1 | `b5f7b4d62d529354ae0de94469521cdef175a1ca8458dbcf0a78958bca02a66f` | `depart-and-choose-outcome` | `Depart + ChooseOutcome` | `287ccf55549997ecd90f3fd8fc202bd7c92be386923e1954a9324554b343fda1` | atomic encounter consequence stop |
| 3 | 2 | `95bd51fd8aa14ba9e67f9ad19193dee7ae613751c37d46973e51fe6e03a5d7e8` | `begin-return` | `BeginReturn` | `42a8db36764d01b533e7adf62c87fb226685471ca2b2f23ac3253cec330b9da1` | prefix restart before remembered response |
| 4 | 3 | `66e5b1d83cd5d0ad86dab09bcef57b72350437c50662a7587830a0965c0045ea` | `record-remembered-response` | `RecordRememberedResponse` | `d3dd8df1f02284cebeb677d1136ebaa1ceb095edb3704aedf7e6fe9a57344fff` | final terminal restart |

Expected revision/parent chain is exact: sequence 1 uses revision 0 and no
parent; sequence 2 uses revision 1 and parent
`b5f7b4d62d529354ae0de94469521cdef175a1ca8458dbcf0a78958bca02a66f`;
sequence 3 uses revision 2 and parent
`95bd51fd8aa14ba9e67f9ad19193dee7ae613751c37d46973e51fe6e03a5d7e8`;
sequence 4 uses revision 3 and parent
`66e5b1d83cd5d0ad86dab09bcef57b72350437c50662a7587830a0965c0045ea`.
The final head is
`32c341b292d4ddcabcb580f536041a1ad4c08ab7a430fdffed8cecbb2d8ccc4b`.

These are four C4V command batches containing five GP1 actions. Revision 3 is
restarted and snapshot-verified before revision 4 is appended. Revision 4 is
restarted again and must equal the terminal C3A-backed state.

## Exact GP2 projection

GP2 consumes only the authored-fixture shadow terminal state. The expected
rule is `gp2.s4.rescue`; the opened decision is `s4.rescue-next`.

Expected emitted record IDs, in order:

1. `knowledge.s4-temporary-rescue`
2. `knowledge.s4-temporary-rescue.grant-0`
3. `relationship.s4-temporary-rescue.0`
4. `construction.s4-temporary-rescue.0`
5. `asset.s4-temporary-rescue`
6. `liability.s4-temporary-rescue.0`

Expected GP2 receipt authority is real and exact:

| Field | Value |
|---|---|
| rule ID | `gp2.s4.rescue` |
| opened decision | `s4.rescue-next` |
| rule-registry digest | `ef4cd659fcfc59290288babb82935cc081c9a53a2796d0030a4a269496c62c07` |
| session-record digest | `1caea41670d9f63b3f454219a0b995d69ae35ebd800c4d6dd5f608bf3523f0d5` |
| terminal shadow-state digest | `febd62dc54cecd95ab7c7de6c95597b8880da7f756f9355997bbcf36ff369b87` |

These values must be read from the one real `ProgressionReceiptV1` inside the
strictly revalidated ledger, never reconstructed as a private GP2 rule.

Expected GP2 world-transition IDs, in order:

1. `anchor.brace.temporary`
2. `crossing.count.one`
3. `iven.location.returned`
4. `signal.coordinate.recorded`
5. `caravan.state.delayed`
6. `brace.state-at-return.expired`

The selected threat mutation `work-area.state.safe` is GP3/GP1 world-only
evidence. It creates no GP2 transition ID and no GP2 lane record.

## Fixed semantic presentation slots

The bundle contains exactly these twenty-five semantic slots:

`hub-status`, `player-actor`, `iven-absent`, `signal-anchor-opportunity`,
`anchor-broken-state`, `signal-window-evidence`, `wire-scavenger-threat`,
`anchor-collapse-risk`, `temporary-brace-tool`, `temporary-rescue-choice`,
`temporary-brace-intervention`, `work-area-safe`, `anchor-brace-temporary`,
`temporary-crossing`, `iven-returned`, `signal-coordinate-recorded`,
`caravan-delayed`, `brace-expired`, `permanent-repair-incomplete`,
`remembered-response`, `next-decision`, `rev1-prepared-stop`,
`rev2-consequence-stop`, `rev3-return-prefix`, `rev4-terminal`.

Each slot has a fixed semantic source, text equivalent, non-colour cue,
reduced-motion equivalent and screen-reader label. They are requirements and
inspectable semantics only; they prove no renderer, visual quality, audio,
timing, accessibility conformance or device performance.

`source_id_list_digest` values use exactly
`SHA256("mindwarp.gp4.presentation-source.v1\\0" || u32be(count) || each(u32be(len(source_id)) || source_id))`.
Again `\\0` is one NUL byte. For every row, `reduced_motion_equivalent` equals
the exact `text` cell and `screen_reader_label` equals
`slot_id + ": " + text`. This freezes all six typed slot fields.

| slot_id | exact ordered source_ids | source_id_list_digest | exact text | exact non_color_cue |
|---|---|---|---|---|
| `hub-status` | `c2.hub.dd1908707f8df16b609e109186a8d8e90dde2cc0836a5f00dd77a00161c7383e`; `gp0.s4.signal-anchor:caravan-leader` | `5b0a75a21e69c7939bd2c055821bb43310d6a57d5a456dbdbcc73012ddec4ff4` | Fixed hub frame: the caravan leader is waiting for a safe crossing. | square hub marker |
| `player-actor` | `c2.player.f7930c4ac3776c4aa4f7400c1b1050bc03a8c4edd82358925fd874d564822e2d`; `player` | `2970eff8074481183b3dd73137b88f0c102c820a71a75c97b6481527a4701a03` | The player is the sole actor for this vertical. | solid actor ring |
| `iven-absent` | `gp0.s4.signal-anchor:problem:iven-stranded` | `48c2b63120f4e2e2f2406a4572b9c025e59a297b48bc43625f8216a3f135526c` | Iven is stranded beyond the broken anchor. | empty person outline |
| `signal-anchor-opportunity` | `gp0.s4.signal-anchor:problem`; `gp0.s4.signal-anchor:core-tension` | `13fceddee77a076a56eabf09eb41ef9394ca9611fce04b3441abe8e689fe5951` | Rescue, signal evidence and permanent repair cannot all be completed in the window. | three-way fork glyph |
| `anchor-broken-state` | `gp0.s4.signal-anchor:problem:anchor-broken` | `2b7a8e146c2513b5ebeaeb66df65bc257f19317d261ca4f0467181166057e48e` | The Signal Anchor is broken. | split anchor shape |
| `signal-window-evidence` | `s4.timing` | `26381ccb785ab9119e59a0d9def3ecd07e0e69b78b7197f8c07079b8fc24913d` | The signal window is three actions; permanent repair needs four and a temporary brace needs two. | three ticks beside four ticks |
| `wire-scavenger-threat` | `wire-scavengers`; `9d9e3507f19953aef3c7a2013fac50c370d15e94013318c45d5b31fad33aa248` | `356839d4afcd203aca10e3cd63078ee4946e801fc24275b8dbca6ce50b692f97` | Wire scavengers block the work area but cannot resolve the rescue. | toothed obstacle outline |
| `anchor-collapse-risk` | `anchor-collapse` | `dd016ff9c3b0140f68130bcd2c00ffcc2aecb37b178cd137b1af7071304bb0aa` | Loading the failed anchor can cause collapse. | descending crack chevron |
| `temporary-brace-tool` | `temporary-brace-kit` | `d56db042020de315923e20ca76cdca388da5547e066595dc6e7b84efc921d1cb` | Prepared tool: temporary brace kit. | brace tool silhouette |
| `temporary-rescue-choice` | `s4.approach.temporary`; `21b33ac9883afa4df0667bd720af3c7f12b956726e8f2841f1eed1fd9fd0a24f` | `f3c96bb6103d66b9ff2db3f764f938332f6f37b57a450ec78c882d9abef93f74` | Choose temporary rescue and signal capture. | selected fork notch |
| `temporary-brace-intervention` | `s4.approach.temporary.step.1`; `s4.approach.temporary.step.2` | `4bdca3750ab72c30059679309d7611e62b0ee77b35e654dd4b38665bd87d5c14` | Fit the brace, cross once, return Iven and record the signal. | two numbered step blocks |
| `work-area-safe` | `wire-scavengers:mutation.0` | `ed36819d5ac8bb9a324a58bbdfe980e45e3db477505b91a596fa04475e28b488` | The diverted work area is safe; this is world-only threat evidence. | cleared obstacle outline |
| `anchor-brace-temporary` | `s4.temporary-rescue:mutation.0` | `bdb72d13efa4fbc5eeb47a4161ee0646bdd71ef4325c8db3928e9f2a50818a01` | The anchor brace is temporary. | temporary brace hatch |
| `temporary-crossing` | `s4.temporary-rescue:mutation.1` | `af8c99b775615f5cf082e17894b6038e596db41676d1f6bf09aa4e78cbae77a9` | One crossing was completed. | single crossing bar |
| `iven-returned` | `s4.temporary-rescue:mutation.2` | `ef5058b5b9ab39ccf10736b6d7cf4221714e6b4b5c7e8a4e3894cbcf6c9316e1` | Iven returned. | filled person outline |
| `signal-coordinate-recorded` | `s4.temporary-rescue:mutation.3` | `fcaac0c18ee51c124f23908f3a3f2b9a8f2dc6b28bf12f3520d6a9edf25df332` | The signal coordinate was recorded. | pinned signal cross |
| `caravan-delayed` | `s4.temporary-rescue:mutation.4` | `f53108ba45b9025e222c7e3c8d8fdb1f67138206e2c5eb92508a8e75cc36c74d` | The caravan remains delayed. | paused caravan bars |
| `brace-expired` | `s4.temporary-rescue:mutation.5` | `01b9a5166a20939541140c3fb5006ae5bfa4cdab97d2bf40297a1ed67150002c` | The temporary brace expired on return. | crossed brace outline |
| `permanent-repair-incomplete` | `s4.temporary-rescue:opportunity_cost.0`; `liability.s4-temporary-rescue.0` | `5c8c04dc03050922688ccf079a3e986e8d2de97fdec0cf46df7102e4615eea76` | Permanent anchor repair was not completed. | open repair bracket |
| `remembered-response` | `s4.temporary-rescue:memory.0`; `c4v.revision.4.ledger_after.history` | `b5ccf08714fbdbcf14f8c9e1e53bb4d3b33f3b8e0ad9cad0de36a8d9b5c50f40` | Iven remembers that rescue and evidence were chosen over permanent repair. | memory knot |
| `next-decision` | `s4.temporary-rescue:named_decision.0`; `s4.rescue-next` | `db309abf424f37103a307b6f30bae40ba308016dbb75b8f093516b4082c7938d` | Next decision: pursue the signal or return with a permanent repair crew. | two-arrow decision fork |
| `rev1-prepared-stop` | `c4v.revision.1.stable_stop` | `6d033456690d6429917c2b40178ea0ab026e0b5acaedf15d28f1c4e162d470cf` | Stable stop after preparation; depart is next. | stop marker one |
| `rev2-consequence-stop` | `c4v.revision.2.stable_stop` | `2c0be5970c5eb5132778cb632e36a4a73ffbfbdbdaccff3e4291ee77672f2a46` | Stable stop after consequence; begin return is next. | stop marker two |
| `rev3-return-prefix` | `c4v.revision.3.stable_stop` | `091a805fbfe08eb59203860b5e9ce48618972cd4a1f810d8b803b8e33ae2276a` | Restarted stable return prefix; record remembered response is next. | stop marker three |
| `rev4-terminal` | `c4v.revision.4.stable_stop` | `5cfdfe7bee6b929482bbf3f50ba9c3c7b2e7bfa03633a4491bcb461221e87e71` | Final restarted terminal remembered response. | terminal stop marker |

The list digest authenticates only the ordered source identifiers. It is not a
payload digest. Each identifier must resolve before the row is accepted:

- `c2.hub.<hex>` and `c2.player.<hex>` compare the suffix to the exact C4V
  `VerticalIdentityV1` hub/player fingerprint; `player` compares the command
  actor to that player fingerprint;
- `gp0.s4.signal-anchor:problem` and `:core-tension` resolve the strict fixed
  session's `player_problem` and `core_tension`; suffixes
  `:iven-stranded`, `:anchor-broken` and `:caravan-leader` require those named
  propositions inside the same validated fields, not free text;
- GP3 situation/facet/fact/risk/threat/approach and `.step.1`/`.step.2` resolve
  by exact ID after `fixed_encounter_grammar` validation;
- `wire-scavengers:mutation.0` resolves the sole exact GP3/GP1 threat mutation;
- `s4.temporary-rescue:mutation.N`, `opportunity_cost.0`, `memory.0` and
  `named_decision.0` resolve exact ordered elements of the fixed GP0 outcome;
- `liability.s4-temporary-rescue.0` and `s4.rescue-next` resolve the real GP2
  ledger liability and receipt opened decision;
- `c4v.revision.4.ledger_after.history` resolves the final replayed GP1 history;
- `c4v.revision.N.stable_stop` resolves the state produced by the real C4V log
  after exactly batch N and must match its replayed stable stop.

No unresolved identifier, substring-only substitute or caller-provided payload
may satisfy a semantic slot.

## Fixed unmeasured requirement rows

Exactly twenty-nine typed rows remain `unmeasured`: sixteen hard requirements
and thirteen compare requirements, listed exactly below. No row contains a
result or names an engine, executable, path or URI.

Every row has exact `status = Unmeasured`. The remaining typed values are:

| requirement_id | class | exact question | exact required_evidence | exact method | exact target |
|---|---|---|---|---|---|
| `hard.strict-bundle-roundtrip` | Hard | Does the adapter preserve strict bundle bytes and every digest? | canonical encode/decode and hostile codec receipt | byte comparison | pass required |
| `hard.exact-dependency-digests` | Hard | Does the adapter authenticate every pinned dependency digest? | C3A GP3 GP2 and C4V digest comparison receipt | fixed-vector comparison | pass required |
| `hard.c2-c3a-identity` | Hard | Does the adapter preserve exact C2 identity and C3A world authority? | identity and validated packet binding receipt | typed authority replay | pass required |
| `hard.gp1-action-stable-order` | Hard | Does the adapter preserve five GP1 actions in four stable C4V batches? | ordered action and stable-stop trace | deterministic trace comparison | pass required |
| `hard.gp3-approach-evidence-risk` | Hard | Does the adapter preserve the exact GP3 approach evidence risk and threat mapping? | fixed situation approach risk and threat receipt | GP3 registry resolution | pass required |
| `hard.c4v-append-restart` | Hard | Does the adapter preserve C4V append prefix restart and final restart semantics? | revision three and revision four replay receipts | semantic restart comparison | pass required |
| `hard.gp2-authored-shadow-isolation` | Hard | Is GP2 restricted to the authority-lowering authored shadow? | rejected C3A GP2 attempt and accepted shadow receipt | authority-negative test | pass required |
| `hard.no-duplicate-memory-progression` | Hard | Are memory and progression records emitted exactly once? | exact receipt emission and history cardinality | set and order comparison | pass required |
| `hard.semantic-slot-coverage` | Hard | Are all twenty-five decision-relevant semantic slots present? | exact fixed slot registry comparison | typed row equality | pass required |
| `hard.accessibility-equivalence` | Hard | Do text non-colour reduced-motion and screen-reader forms preserve each slot meaning? | per-slot equivalence review receipt | semantic equivalence review | pass required |
| `hard.no-canonical-mutation` | Hard | Does adapter execution leave canonical Forge and gameplay records unchanged? | before and after canonical hashes | mutation-negative comparison | pass required |
| `hard.no-ambient-authority` | Hard | Does the adapter avoid filesystem network process and hidden runtime authority? | capability and side-effect denial receipt | containment audit | pass required |
| `hard.headless-deterministic-tests` | Hard | Does the complete vertical replay byte-identically without presentation? | repeated isolated headless receipts | clean-process replay | pass required |
| `hard.clean-target-build` | Hard | Does the adapter build from a clean isolated target? | clean target build receipt | isolated build | pass required |
| `hard.runtime-provenance-licensing` | Hard | Are runtime and dependency provenance and licenses acceptable? | source license and dependency inventory | provenance review | owner approval required |
| `hard.containment-teardown` | Hard | Can the runtime trial be contained stopped and removed without residue? | launch boundary and teardown receipt | containment exercise | pass required |
| `compare.cold-build-import` | Compare | What is cold build and initial import cost? | measured clean build and import trace | timed clean trial | owner-set after measurement |
| `compare.incremental-iteration` | Compare | What is edit to verified incremental iteration cost? | measured incremental build and test trace | timed incremental trial | owner-set after measurement |
| `compare.bundle-validation-restart-latency` | Compare | What are bundle validation prefix restart and final restart latencies? | measured validation and both restart traces | monotonic timing | owner-set after measurement |
| `compare.input-semantic-feedback-latency` | Compare | What is input to semantic feedback latency? | measured input and semantic projection timestamps | event trace timing | owner-set after measurement |
| `compare.cpu-gpu-frame-pacing` | Compare | What CPU GPU and frame pacing cost does presentation add? | measured CPU GPU and frame pacing trace | representative scene profile | owner-set after measurement |
| `compare.peak-steady-memory` | Compare | What are peak and steady memory use? | measured peak and steady allocation trace | memory profile | owner-set after measurement |
| `compare.binary-asset-project-size` | Compare | What binary asset and project size does the adapter add? | measured clean artifact inventory | size inventory | owner-set after measurement |
| `compare.mobile-battery-thermal` | Compare | What mobile battery and thermal cost occurs? | measured supported-device battery and thermal trace | bounded device trial | owner-set after measurement |
| `compare.adapter-dependency-surface` | Compare | How large is the adapter and dependency surface? | counted public adapter and dependency inventory | interface inventory | owner-set after measurement |
| `compare.debugging-profiling` | Compare | How effective are debugging and profiling workflows? | timed fault isolation and profile exercise | controlled defect exercise | owner-set after measurement |
| `compare.platform-export-coverage` | Compare | Which target exports pass the exact vertical? | per-target build run and replay receipts | platform matrix | owner-set after measurement |
| `compare.upgrade-maintenance-risk` | Compare | What upgrade and maintenance risk is observed? | dependency update and migration exercise | bounded upgrade trial | owner-set after measurement |
| `compare.owner-play-comprehension` | Compare | Does the owner understand and enjoy the fixed vertical? | explicit owner-authored play observation | bounded owner play check | owner decision required |

## Bounds and authority

- complete canonical bundle: at most 8 MiB before parsing;
- C4V log: upstream 4 MiB bound;
- each snapshot: upstream 512 KiB bound;
- C4V receipt: upstream 64 KiB bound;
- authored shadow state: upstream 256 KiB bound;
- GP3 situation: upstream 32 KiB bound;
- GP2 ledger: upstream 1 MiB bound;
- exactly four command IDs, twenty-five semantic slots and twenty-nine requirement rows;
- no paths, URIs, executables, engine selection, broad C4, C3B, Companion,
  Greenfield, filesystem, network or Kernel mutation.

The later closeout sibling is exactly `G1-VERTICAL-CLOSEOUT` with
`broad_g1=false` and `runtime_containment_pending=true`. It does not close
broad `G1-CLOSEOUT`, modify R1 or promote a runtime.
