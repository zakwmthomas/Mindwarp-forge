use addressable_world_binding::bind_addressable_world_package;
use derived_world_rules::compile_world;
use mindwarp_gameplay_foundation::{
    BaseLoopActionV1, BaseLoopLedgerV1, PreparationV1, ProgressionLedgerV1,
    apply_base_loop_action, apply_progression, fixed_encounter_grammar, fixed_sessions,
    start_authored_base_loop, start_c3a_base_loop,
};
use mindwarp_vertical_persistence::{
    VerticalCommandBatchV1, VerticalIdentityV1, VerticalLogV1, VerticalSnapshotV1,
};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use universe_identity::{AddressSegment, NodeKind, UniverseAddress};

mod world_support {
    include!(concat!(env!("FORGE_ROOT"), "/crates/mindwarp-gameplay-foundation/tests/world_support/mod.rs"));
}

#[derive(Serialize)]
struct CommandProof { sequence: u64, revision: u64, parent: Option<String>, command_id: String, label: String, action_mapping: String }
#[derive(Serialize)]
struct SemanticProof { slot_id:String, source_ids:Vec<String>, source_id_list_digest:String, text_equivalent:String, non_color_cue:String, reduced_motion_equivalent:String, screen_reader_label:String }
#[derive(Serialize)]
struct RequirementProof { requirement_id:String, class:String, status:String, question:String, required_evidence:String, method:String, target:String }
#[derive(Serialize)]
struct SchemaProof { field:String, rust_type:String, clause:String }

#[derive(Serialize)]
struct Proof {
    values: serde_json::Map<String, Value>,
    commands: Vec<CommandProof>,
    emitted: Vec<String>,
    transitions: Vec<String>,
    resolved_source_ids: Vec<String>,
    source_id_list_digests: serde_json::Map<String, Value>,
    semantic_fields: Vec<String>,
    semantic_rows: Vec<SemanticProof>,
    requirement_rows: Vec<RequirementProof>,
    schema_rows: Vec<SchemaProof>,
}

fn hex(bytes: impl AsRef<[u8]>) -> String { bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect() }
fn sha(bytes: &[u8]) -> String { hex(Sha256::digest(bytes)) }
fn be32(n: usize) -> [u8; 4] { u32::try_from(n).unwrap().to_be_bytes() }
fn address(seed: [u8; 32], kind: NodeKind, payload: &[u8]) -> UniverseAddress {
    UniverseAddress::new(seed, vec![AddressSegment::new(kind, payload).unwrap()]).unwrap()
}
fn ref_digest(domain: &[u8], situation: &[u8], entity: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new(); h.update(domain); h.update(be32(situation.len())); h.update(situation);
    h.update((entity.len() as u64).to_be_bytes()); h.update(entity); h.finalize().into()
}
fn command_id(identity: [u8;32], parent: Option<[u8;32]>, sequence: u64, revision: u64, actions: &[BaseLoopActionV1]) -> [u8;32] {
    let bundle = b"gp4.signal-anchor.bundle-v1"; let run = b"gp4.signal-anchor.vertical-1";
    let actions = serde_json::to_vec(actions).unwrap(); let mut h = Sha256::new();
    h.update(b"mindwarp.gp4.signal-anchor.command.v1\0"); h.update(identity);
    h.update(be32(bundle.len())); h.update(bundle); h.update(be32(run.len())); h.update(run);
    h.update(sequence.to_be_bytes()); h.update(revision.to_be_bytes());
    match parent { None => h.update([0]), Some(p) => { h.update([1]); h.update(p); } }
    h.update((actions.len() as u64).to_be_bytes()); h.update(actions); h.finalize().into()
}
fn source_digest(ids: &[&str]) -> String {
    let mut h = Sha256::new(); h.update(b"mindwarp.gp4.presentation-source.v1\0");
    h.update(be32(ids.len())); for id in ids { h.update(be32(id.len())); h.update(id.as_bytes()); } hex(h.finalize())
}
fn semantic(state: &mindwarp_gameplay_foundation::BaseLoopStateV1) -> (Vec<u8>, Vec<String>) {
    let mut value = serde_json::to_value(state).unwrap(); let object = value.as_object_mut().unwrap();
    let expected = ["schema_version","run_id","session_id","world_context","phase","preparation","predecessor_outcome_id","session_state","ledger_before","ledger_after","failure","recoveries_used","stable_stop","trace"];
    let mut actual=object.keys().map(String::as_str).collect::<Vec<_>>(); actual.sort(); let mut sorted=expected.to_vec(); sorted.sort(); assert_eq!(actual,sorted);
    object.remove("world_context").unwrap();
    let fields=expected.into_iter().filter(|f|*f!="world_context").map(str::to_owned).collect();
    (serde_json::to_vec(&value).unwrap(), fields)
}

fn main() {
    let record = fixed_sessions().into_iter().find(|r| r.session_id == "gp0.s4.signal-anchor").unwrap();
    let grammar = fixed_encounter_grammar().unwrap();
    let situation = grammar.situations.iter().find(|s| s.situation_id == "gp3.s4.signal-anchor").unwrap();
    let approach = situation.approaches.iter().find(|a| a.approach_id == "s4.approach.temporary").unwrap();
    let threat = situation.threat_ref.as_ref().unwrap();
    let input = world_support::world_input([0x4a; 32]); let packet = compile_world(&input).unwrap();
    let hub = address([0x0e;32], NodeKind::Site, b"signal-hub");
    let place = address([0x0e;32], NodeKind::Site, b"signal-anchor");
    let player = address([0x0e;32], NodeKind::Entity, b"signal-player");
    let descriptor = bind_addressable_world_package(place.logical_fingerprint().unwrap(), Some(hub.logical_fingerprint().unwrap()), input.reconstruction_id, &input, &packet, b"gp4-signal-anchor-place-v1".to_vec()).unwrap();
    let identity = VerticalIdentityV1::new(&hub,&place,&player,&descriptor,&record.session_id,"gp4.signal-anchor.vertical-1").unwrap();
    let initial = start_c3a_base_loop(&record,"gp4.signal-anchor.vertical-1",BaseLoopLedgerV1::empty(),&input,&packet).unwrap();
    let mut log = VerticalLogV1::initialize(identity.clone(),&initial,&record,&input,&packet).unwrap();
    let baseline_sha = sha(&log.baseline_bytes); let snapshot0 = VerticalSnapshotV1::build(&log,&record,&input,&packet).unwrap();
    let batches = vec![
        vec![BaseLoopActionV1::Prepare(PreparationV1 { session_id: record.session_id.clone(), intent_id:"rescue-before-anchor-collapse".into(), tool_id:"temporary-brace-kit".into(), divert_threat:true })],
        vec![BaseLoopActionV1::Depart, BaseLoopActionV1::ChooseOutcome { outcome_id:"s4.temporary-rescue".into() }],
        vec![BaseLoopActionV1::BeginReturn], vec![BaseLoopActionV1::RecordRememberedResponse],
    ];
    let mut commands = vec![]; let mut states = vec![];
    for (index, actions) in batches.into_iter().enumerate() {
        let sequence = index as u64 + 1; let revision = log.revision(); let parent = log.head().unwrap();
        let id = command_id(identity.fingerprint().unwrap(), parent, sequence, revision, &actions);
        let label=["prepare","depart-and-choose-outcome","begin-return","record-remembered-response"][index].to_owned();
        let action_mapping=["Prepare","Depart + ChooseOutcome","BeginReturn","RecordRememberedResponse"][index].to_owned();
        commands.push(CommandProof { sequence, revision, parent: parent.map(hex), command_id: hex(id), label, action_mapping });
        let command = VerticalCommandBatchV1 { actor_player_id:identity.player_id, command_id:id, expected_revision:revision, sequence, expected_parent:parent, actions };
        let appended = log.append(&record,&input,&packet,&command).unwrap(); log = appended.0; states.push(appended.1);
        if sequence == 3 { assert_eq!(log.restart(&record,&input,&packet).unwrap(), states[2]); VerticalSnapshotV1::build(&log,&record,&input,&packet).unwrap(); }
    }
    let c3a = log.restart(&record,&input,&packet).unwrap(); assert_eq!(c3a, states[3]);
    let mut shadow = start_authored_base_loop(&record,&c3a.run_id,c3a.ledger_before.clone()).unwrap();
    for action in &c3a.trace { shadow = apply_base_loop_action(&record,&shadow,action).unwrap(); }
    let (c3a_semantic, semantic_fields) = semantic(&c3a); let (shadow_semantic, _) = semantic(&shadow);
    assert_eq!(c3a_semantic, shadow_semantic); assert_eq!(c3a.ledger_before.to_bytes().unwrap(),shadow.ledger_before.to_bytes().unwrap()); assert_eq!(c3a.ledger_after.to_bytes().unwrap(),shadow.ledger_after.to_bytes().unwrap());
    let mut semantic_hasher=Sha256::new(); semantic_hasher.update(b"mindwarp.gp4.base-loop-semantics.v1\0"); semantic_hasher.update((c3a_semantic.len() as u64).to_be_bytes()); semantic_hasher.update(&c3a_semantic);
    let gp2 = apply_progression(&record,&shadow,&ProgressionLedgerV1::from_base_loop(&shadow.ledger_before).unwrap()).unwrap(); let receipt=&gp2.processed_receipts[0];
    let source_rows: Vec<(&str,Vec<String>)> = vec![
      ("hub-status",vec![format!("c2.hub.{}",hex(identity.hub_id)),"gp0.s4.signal-anchor:caravan-leader".into()]),
      ("player-actor",vec![format!("c2.player.{}",hex(identity.player_id)),"player".into()]),
      ("iven-absent",vec!["gp0.s4.signal-anchor:problem:iven-stranded".into()]),
      ("signal-anchor-opportunity",vec!["gp0.s4.signal-anchor:problem".into(),"gp0.s4.signal-anchor:core-tension".into()]),
      ("anchor-broken-state",vec!["gp0.s4.signal-anchor:problem:anchor-broken".into()]),
      ("signal-window-evidence",vec!["s4.timing".into()]), ("wire-scavenger-threat",vec!["wire-scavengers".into(),threat.canonical_digest.clone()]),
      ("anchor-collapse-risk",vec!["anchor-collapse".into()]), ("temporary-brace-tool",vec!["temporary-brace-kit".into()]),
      ("temporary-rescue-choice",vec!["s4.approach.temporary".into(),hex(ref_digest(b"mindwarp.gp4.gp3-approach-ref.v1\0",situation.situation_id.as_bytes(),&serde_json::to_vec(approach).unwrap()))]),
      ("temporary-brace-intervention",vec![approach.intervention_steps[0].step_id.clone(),approach.intervention_steps[1].step_id.clone()]),
      ("work-area-safe",vec!["wire-scavengers:mutation.0".into()]),
      ("anchor-brace-temporary",vec!["s4.temporary-rescue:mutation.0".into()]), ("temporary-crossing",vec!["s4.temporary-rescue:mutation.1".into()]),
      ("iven-returned",vec!["s4.temporary-rescue:mutation.2".into()]), ("signal-coordinate-recorded",vec!["s4.temporary-rescue:mutation.3".into()]),
      ("caravan-delayed",vec!["s4.temporary-rescue:mutation.4".into()]), ("brace-expired",vec!["s4.temporary-rescue:mutation.5".into()]),
      ("permanent-repair-incomplete",vec!["s4.temporary-rescue:opportunity_cost.0".into(),"liability.s4-temporary-rescue.0".into()]),
      ("remembered-response",vec!["s4.temporary-rescue:memory.0".into(),"c4v.revision.4.ledger_after.history".into()]),
      ("next-decision",vec!["s4.temporary-rescue:named_decision.0".into(),"s4.rescue-next".into()]),
      ("rev1-prepared-stop",vec!["c4v.revision.1.stable_stop".into()]), ("rev2-consequence-stop",vec!["c4v.revision.2.stable_stop".into()]),
      ("rev3-return-prefix",vec!["c4v.revision.3.stable_stop".into()]), ("rev4-terminal",vec!["c4v.revision.4.stable_stop".into()]),
    ];
    assert!(record.player_problem.contains("stranding surveyor Iven") && record.player_problem.contains("broke an anchor") && record.player_problem.contains("caravan waits"));
    assert_eq!(approach.intervention_steps.iter().map(|s|s.step_id.as_str()).collect::<Vec<_>>(),["s4.approach.temporary.step.1","s4.approach.temporary.step.2"]);
    assert_eq!(record.outcome("s4.temporary-rescue").unwrap().exact_mutations.len(),6); assert_eq!(states.len(),4);
    let semantic_content="hub-status|Fixed hub frame: the caravan leader is waiting for a safe crossing.|square hub marker
player-actor|The player is the sole actor for this vertical.|solid actor ring
iven-absent|Iven is stranded beyond the broken anchor.|empty person outline
signal-anchor-opportunity|Rescue, signal evidence and permanent repair cannot all be completed in the window.|three-way fork glyph
anchor-broken-state|The Signal Anchor is broken.|split anchor shape
signal-window-evidence|The signal window is three actions; permanent repair needs four and a temporary brace needs two.|three ticks beside four ticks
wire-scavenger-threat|Wire scavengers block the work area but cannot resolve the rescue.|toothed obstacle outline
anchor-collapse-risk|Loading the failed anchor can cause collapse.|descending crack chevron
temporary-brace-tool|Prepared tool: temporary brace kit.|brace tool silhouette
temporary-rescue-choice|Choose temporary rescue and signal capture.|selected fork notch
temporary-brace-intervention|Fit the brace, cross once, return Iven and record the signal.|two numbered step blocks
work-area-safe|The diverted work area is safe; this is world-only threat evidence.|cleared obstacle outline
anchor-brace-temporary|The anchor brace is temporary.|temporary brace hatch
temporary-crossing|One crossing was completed.|single crossing bar
iven-returned|Iven returned.|filled person outline
signal-coordinate-recorded|The signal coordinate was recorded.|pinned signal cross
caravan-delayed|The caravan remains delayed.|paused caravan bars
brace-expired|The temporary brace expired on return.|crossed brace outline
permanent-repair-incomplete|Permanent anchor repair was not completed.|open repair bracket
remembered-response|Iven remembers that rescue and evidence were chosen over permanent repair.|memory knot
next-decision|Next decision: pursue the signal or return with a permanent repair crew.|two-arrow decision fork
rev1-prepared-stop|Stable stop after preparation; depart is next.|stop marker one
rev2-consequence-stop|Stable stop after consequence; begin return is next.|stop marker two
rev3-return-prefix|Restarted stable return prefix; record remembered response is next.|stop marker three
rev4-terminal|Final restarted terminal remembered response.|terminal stop marker";
    let content=semantic_content.lines().map(|line|{let p=line.split('|').collect::<Vec<_>>();(p[0],p[1],p[2])}).collect::<Vec<_>>();
    assert_eq!(content.len(),source_rows.len()); let mut source_digests=serde_json::Map::new(); let mut resolved=vec![]; let mut semantic_rows=vec![];
    for ((slot,ids),(content_slot,text,cue)) in source_rows.into_iter().zip(content) { assert_eq!(slot,content_slot); let refs=ids.iter().map(String::as_str).collect::<Vec<_>>(); let digest=source_digest(&refs); source_digests.insert(slot.into(),json!(digest)); resolved.extend(ids.clone()); semantic_rows.push(SemanticProof{slot_id:slot.into(),source_ids:ids,source_id_list_digest:digest,text_equivalent:text.into(),non_color_cue:cue.into(),reduced_motion_equivalent:text.into(),screen_reader_label:format!("{slot}: {text}")}); }
    resolved.sort(); resolved.dedup();
    let mut values=serde_json::Map::new();
    for (k,v) in [
      ("input_sha",sha(&input.to_bytes().unwrap())), ("packet_id",packet.packet_id.clone()), ("packet_sha",sha(&packet.to_bytes().unwrap())),
      ("identity",hex(identity.fingerprint().unwrap())), ("baseline_sha",baseline_sha), ("reducer",hex(snapshot0.reducer_fingerprint)), ("codec",hex(snapshot0.codec_fingerprint)),
      ("grammar",grammar.grammar_digest.clone()), ("situation",situation.situation_digest.clone()), ("session",situation.session_digest.clone()),
      ("upstream_threat",threat.canonical_digest.clone()), ("approach_ref",hex(ref_digest(b"mindwarp.gp4.gp3-approach-ref.v1\0",situation.situation_id.as_bytes(),&serde_json::to_vec(approach).unwrap()))),
      ("threat_ref",hex(ref_digest(b"mindwarp.gp4.gp3-threat-ref.v1\0",situation.situation_id.as_bytes(),&serde_json::to_vec(threat).unwrap()))),
      ("final_head",hex(log.head().unwrap().unwrap())), ("semantic_digest",hex(semantic_hasher.finalize())),
      ("gp2_registry",hex(receipt.rule_registry_digest)), ("gp2_session",hex(receipt.session_record_digest)), ("gp2_terminal",hex(receipt.terminal_state_digest)),
      ("gp2_rule",receipt.rule_id.clone()), ("gp2_decision",receipt.opened_decision_id.clone())
    ] { values.insert(k.into(),json!(v)); }
    let requirement_data="hard.strict-bundle-roundtrip|Hard|Does the adapter preserve strict bundle bytes and every digest?|canonical encode/decode and hostile codec receipt|byte comparison|pass required
hard.exact-dependency-digests|Hard|Does the adapter authenticate every pinned dependency digest?|C3A GP3 GP2 and C4V digest comparison receipt|fixed-vector comparison|pass required
hard.c2-c3a-identity|Hard|Does the adapter preserve exact C2 identity and C3A world authority?|identity and validated packet binding receipt|typed authority replay|pass required
hard.gp1-action-stable-order|Hard|Does the adapter preserve five GP1 actions in four stable C4V batches?|ordered action and stable-stop trace|deterministic trace comparison|pass required
hard.gp3-approach-evidence-risk|Hard|Does the adapter preserve the exact GP3 approach evidence risk and threat mapping?|fixed situation approach risk and threat receipt|GP3 registry resolution|pass required
hard.c4v-append-restart|Hard|Does the adapter preserve C4V append prefix restart and final restart semantics?|revision three and revision four replay receipts|semantic restart comparison|pass required
hard.gp2-authored-shadow-isolation|Hard|Is GP2 restricted to the authority-lowering authored shadow?|rejected C3A GP2 attempt and accepted shadow receipt|authority-negative test|pass required
hard.no-duplicate-memory-progression|Hard|Are memory and progression records emitted exactly once?|exact receipt emission and history cardinality|set and order comparison|pass required
hard.semantic-slot-coverage|Hard|Are all twenty-five decision-relevant semantic slots present?|exact fixed slot registry comparison|typed row equality|pass required
hard.accessibility-equivalence|Hard|Do text non-colour reduced-motion and screen-reader forms preserve each slot meaning?|per-slot equivalence review receipt|semantic equivalence review|pass required
hard.no-canonical-mutation|Hard|Does adapter execution leave canonical Forge and gameplay records unchanged?|before and after canonical hashes|mutation-negative comparison|pass required
hard.no-ambient-authority|Hard|Does the adapter avoid filesystem network process and hidden runtime authority?|capability and side-effect denial receipt|containment audit|pass required
hard.headless-deterministic-tests|Hard|Does the complete vertical replay byte-identically without presentation?|repeated isolated headless receipts|clean-process replay|pass required
hard.clean-target-build|Hard|Does the adapter build from a clean isolated target?|clean target build receipt|isolated build|pass required
hard.runtime-provenance-licensing|Hard|Are runtime and dependency provenance and licenses acceptable?|source license and dependency inventory|provenance review|owner approval required
hard.containment-teardown|Hard|Can the runtime trial be contained stopped and removed without residue?|launch boundary and teardown receipt|containment exercise|pass required
compare.cold-build-import|Compare|What is cold build and initial import cost?|measured clean build and import trace|timed clean trial|owner-set after measurement
compare.incremental-iteration|Compare|What is edit to verified incremental iteration cost?|measured incremental build and test trace|timed incremental trial|owner-set after measurement
compare.bundle-validation-restart-latency|Compare|What are bundle validation prefix restart and final restart latencies?|measured validation and both restart traces|monotonic timing|owner-set after measurement
compare.input-semantic-feedback-latency|Compare|What is input to semantic feedback latency?|measured input and semantic projection timestamps|event trace timing|owner-set after measurement
compare.cpu-gpu-frame-pacing|Compare|What CPU GPU and frame pacing cost does presentation add?|measured CPU GPU and frame pacing trace|representative scene profile|owner-set after measurement
compare.peak-steady-memory|Compare|What are peak and steady memory use?|measured peak and steady allocation trace|memory profile|owner-set after measurement
compare.binary-asset-project-size|Compare|What binary asset and project size does the adapter add?|measured clean artifact inventory|size inventory|owner-set after measurement
compare.mobile-battery-thermal|Compare|What mobile battery and thermal cost occurs?|measured supported-device battery and thermal trace|bounded device trial|owner-set after measurement
compare.adapter-dependency-surface|Compare|How large is the adapter and dependency surface?|counted public adapter and dependency inventory|interface inventory|owner-set after measurement
compare.debugging-profiling|Compare|How effective are debugging and profiling workflows?|timed fault isolation and profile exercise|controlled defect exercise|owner-set after measurement
compare.platform-export-coverage|Compare|Which target exports pass the exact vertical?|per-target build run and replay receipts|platform matrix|owner-set after measurement
compare.upgrade-maintenance-risk|Compare|What upgrade and maintenance risk is observed?|dependency update and migration exercise|bounded upgrade trial|owner-set after measurement
compare.owner-play-comprehension|Compare|Does the owner understand and enjoy the fixed vertical?|explicit owner-authored play observation|bounded owner play check|owner decision required";
    let requirement_rows=requirement_data.lines().map(|line|{let p=line.split('|').collect::<Vec<_>>();assert_eq!(p.len(),6);RequirementProof{requirement_id:p[0].into(),class:p[1].into(),status:"Unmeasured".into(),question:p[2].into(),required_evidence:p[3].into(),method:p[4].into(),target:p[5].into()}}).collect();
    let schema_data="schema_version|u16|exactly `1`
bundle_id|String|fixed `gp4.signal-anchor.bundle-v1`
session_bytes|Vec<u8>|exact canonical GP0 S4 record
c3a_input_bytes|Vec<u8>|exact canonical fixed C3A input
c3a_packet_bytes|Vec<u8>|exact canonical compiled C3A packet
c4v_log_bytes|Vec<u8>|exact terminal V1 C4V log
return_prefix_snapshot_bytes|Vec<u8>|revision-3 snapshot
final_snapshot_bytes|Vec<u8>|revision-4 snapshot
persistence_receipt_bytes|Vec<u8>|real final C4V receipt
command_ids|Vec<[u8; 32]>|exactly the four registry IDs
authored_shadow_state_bytes|Vec<u8>|strict terminal authored state
common_semantic_digest|[u8; 32]|exhaustive 13-field projection
gp3_situation_bytes|Vec<u8>|exact strict S4 situation
gp4_approach_ref_digest|[u8; 32]|derived fixed temporary approach ref
gp3_threat_digest|String|upstream exact threat digest
gp4_threat_ref_digest|[u8; 32]|derived exact threat ref
threat_selected|bool|exactly true
progression_ledger_bytes|Vec<u8>|exact real GP2 output
presentation_slots|Vec<SemanticPresentationSlotV1>|exactly 25 fixed rows
adapter_requirements|Vec<AdapterRequirementV1>|exactly 29 fixed rows
bundle_digest|[u8; 32]|domain-framed canonical body digest";
    let schema_rows=schema_data.lines().map(|line|{let p=line.split('|').collect::<Vec<_>>();assert_eq!(p.len(),3);SchemaProof{field:p[0].into(),rust_type:p[1].into(),clause:p[2].into()}}).collect();
    println!("{}",serde_json::to_string(&Proof { values,commands,emitted:receipt.emitted_record_ids.clone(),transitions:receipt.world_transition_ids.clone(),resolved_source_ids:resolved,source_id_list_digests:source_digests,semantic_fields,semantic_rows,requirement_rows,schema_rows }).unwrap());
}
