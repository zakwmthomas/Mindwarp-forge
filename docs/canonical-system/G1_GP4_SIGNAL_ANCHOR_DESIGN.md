# G1 GP4 Signal Anchor design

Status: frozen pre-source design; implementation remains blocked on independent
readiness acceptance.

## Isolated owner

One additive crate, `mindwarp-signal-anchor-vertical`, may depend on
`mindwarp-gameplay-foundation`, `mindwarp-vertical-persistence`, the exact C3A
records and only the identity/address crates needed to reproduce the fixed
fixture. It owns bundle composition and validation, not gameplay, persistence,
rendering, accessibility conformance, runtime choice or broad G1 closure.

## Exact `SignalAnchorBundleV1` schema

The strict top-level record contains exactly these fields, in this order:

1. `schema_version: u16` — exactly `1`;
2. `bundle_id: String` — fixed `gp4.signal-anchor.bundle-v1`;
3. `session_bytes: Vec<u8>` — exact canonical GP0 S4 record;
4. `c3a_input_bytes: Vec<u8>` — exact canonical fixed C3A input;
5. `c3a_packet_bytes: Vec<u8>` — exact canonical compiled C3A packet;
6. `c4v_log_bytes: Vec<u8>` — exact terminal V1 C4V log;
7. `return_prefix_snapshot_bytes: Vec<u8>` — revision-3 snapshot;
8. `final_snapshot_bytes: Vec<u8>` — revision-4 snapshot;
9. `persistence_receipt_bytes: Vec<u8>` — real final C4V receipt;
10. `command_ids: Vec<[u8; 32]>` — exactly the four registry IDs;
11. `authored_shadow_state_bytes: Vec<u8>` — strict terminal authored state;
12. `common_semantic_digest: [u8; 32]` — exhaustive 13-field projection;
13. `gp3_situation_bytes: Vec<u8>` — exact strict S4 situation;
14. `gp4_approach_ref_digest: [u8; 32]` — derived fixed temporary approach ref;
15. `gp3_threat_digest: String` — upstream exact threat digest;
16. `gp4_threat_ref_digest: [u8; 32]` — derived exact threat ref;
17. `threat_selected: bool` — exactly true;
18. `progression_ledger_bytes: Vec<u8>` — exact real GP2 output;
19. `presentation_slots: Vec<SemanticPresentationSlotV1>` — exactly 25 fixed rows;
20. `adapter_requirements: Vec<AdapterRequirementV1>` — exactly 29 fixed rows;
21. `bundle_digest: [u8; 32]` — domain-framed canonical body digest.

All records use `serde(deny_unknown_fields)`. The GP0 session digest is never a
bundle field: authority comes from strict session bytes and the pinned upstream
GP3 session reference. No invented `gp0_contract_digest` exists.

## Typed nested records

`SemanticPresentationSlotV1` contains exactly `slot_id`, `source_ids`,
`source_id_list_digest`, `text_equivalent`, `non_color_cue`,
`reduced_motion_equivalent` and `screen_reader_label`. The list digest covers
only the framed ordered IDs; every ID is separately resolved to validated
upstream data by the fixed registry rules. Each field equals the registry; no
free-form slot or optional payload is accepted.

`AdapterRequirementV1` contains exactly `requirement_id`, `class`, `status`,
`question`, `required_evidence`, `method` and `target`. `class` is `Hard` or
`Compare`; `status` has the sole V1 value `Unmeasured`. The sixteen hard and
thirteen compare IDs are fixed by the registry. There are no result fields.

`BaseLoopSemanticProjectionV1` contains exactly the thirteen
`BaseLoopStateV1` fields listed in readiness and omits only `world_context`.
Construction uses exhaustive destructuring without `..`.

## Construction and validation order

1. Reproduce the fixed C2/C3A fixture and compare canonical input/packet hashes.
2. Strictly resolve the exact GP0 S4 record and GP3 grammar/situation.
3. Start the C3A-backed GP1 run and initialize a real C4V V1 log.
4. Derive each 32-byte command ID from identity, bundle/run, revision, parent
   and canonical action-vector bytes, then append the four frozen batches.
5. After batch 3, restart semantically and build the exact return-prefix
   snapshot. Only then append batch 4, restart again, build the final snapshot
   and create the real persistence receipt.
6. Start the authored shadow from the identical run and `ledger_before`; replay
   the identical preparation/actions/threat/outcome; validate its own strict
   authority and compare all thirteen projected fields and both ledger byte sets.
7. Resolve and validate GP3 approach, consequence coverage and optional
   nonterminal threat; recompute both GP4 reference digests.
8. Run real GP2 only over the shadow, decode/revalidate the resulting ledger and
   compare its receipt/emissions/transitions to the fixed registry.
9. Attach the exact presentation and unmeasured requirement registries.
10. Compute the bundle digest with its field zeroed, encode, enforce the 8 MiB
    ceiling and immediately decode/validate the emitted bytes.

Decoding performs the 8 MiB check before top-level JSON parsing. Because serde
may allocate nested byte arrays during parsing, V1 claims nested caps before
semantic traversal, not before allocation. After parse it checks every nested
cap before dependency decoding or fixed-registry equality.

## Exact bounds

| Item | Maximum |
|---|---:|
| complete bundle | 8,388,608 bytes |
| session bytes | 262,144 bytes |
| C3A input bytes | 262,144 bytes |
| C3A packet bytes | 262,144 bytes |
| C4V log bytes | 4,194,304 bytes |
| each snapshot | 524,288 bytes |
| persistence receipt | 65,536 bytes |
| authored shadow state | 262,144 bytes |
| GP3 situation | 32,768 bytes |
| progression ledger | 1,048,576 bytes |
| command IDs | exactly 4 |
| presentation slots | exactly 25 |
| adapter requirements | exactly 29 |
| source IDs per presentation slot | 1 through 16 |
| any ID | 128 UTF-8 bytes |
| any presentation-equivalent text | 512 UTF-8 bytes |
| any requirement question/evidence text | 512 UTF-8 bytes |

Vectors reject both under-count and over-count. Text rejects empty, surrounding
whitespace, control characters, `://`, drive/UNC/absolute-path forms and the
fixed executable suffixes `.exe`, `.dll`, `.so`, `.dylib`, `.bat`, `.cmd`,
`.ps1`, `.sh`. Fixed registry equality remains the final authority.

## Presentation registry

The 25 exact slot IDs are those in the fixed registry. Source IDs bind them to
the fixed hub/player identities; S4 facts `s4.timing` and
`s4.wire-scavengers`; risk `anchor-collapse`; preparation tool; GP3 approach,
steps and consequence refs; GP1 stable-stop projections at revisions 1, 2, 3
and 4; terminal exact mutations/cost/memory; and decision `s4.rescue-next`.

Each row uses plain fixed text, a redundant non-colour symbol/shape word, a
zero-motion state-change description and a screen-reader phrase containing the
same decision-relevant fact. This is semantic inspectability, not visual proof.

## Adapter matrix

Hard IDs:

- `hard.strict-bundle-roundtrip`
- `hard.exact-dependency-digests`
- `hard.c2-c3a-identity`
- `hard.gp1-action-stable-order`
- `hard.gp3-approach-evidence-risk`
- `hard.c4v-append-restart`
- `hard.gp2-authored-shadow-isolation`
- `hard.no-duplicate-memory-progression`
- `hard.semantic-slot-coverage`
- `hard.accessibility-equivalence`
- `hard.no-canonical-mutation`
- `hard.no-ambient-authority`
- `hard.headless-deterministic-tests`
- `hard.clean-target-build`
- `hard.runtime-provenance-licensing`
- `hard.containment-teardown`

Compare IDs:

- `compare.cold-build-import`
- `compare.incremental-iteration`
- `compare.bundle-validation-restart-latency`
- `compare.input-semantic-feedback-latency`
- `compare.cpu-gpu-frame-pacing`
- `compare.peak-steady-memory`
- `compare.binary-asset-project-size`
- `compare.mobile-battery-thermal`
- `compare.adapter-dependency-surface`
- `compare.debugging-profiling`
- `compare.platform-export-coverage`
- `compare.upgrade-maintenance-risk`
- `compare.owner-play-comprehension`

Every row is `Unmeasured` and names the evidence a future owner-authorized
runtime trial would need. GP4 supplies no measurements or runtime preference.

## Digest authority

Command, GP3 reference, semantic and bundle digest domains/framing are exact in
the fixed registry/readiness package. Displayed `\\0` always denotes one NUL
byte. Unsigned lengths are big-endian and are included before their payloads.
Bundle digest covers the complete ordered bundle with `bundle_digest` set to
32 zero bytes.

## Hostile implementation matrix

Red tests must cover all twenty readiness families plus one mutation for every
top-level field, every semantic projection field, every command parent/revision
and every fixed registry row. Direct decoders must reject top-level overflow
before parse and nested overflow before dependency traversal. Tests must prove
the threat mutation is present in GP1 world state yet absent from the six GP2
transition IDs and all GP2 lane records.

## Closeout

After focused, retained, independent, workspace and registered verification,
the bounded closeout receipt may bind C3A, C4V and GP0-GP4 proof IDs. It must
state `broad_g1=false`, `runtime_selected=false`,
`runtime_containment_pending=true`, `evidence_only=true` and
`promotion_authority=false`. Broad G1-CLOSEOUT and R1 remain untouched.
