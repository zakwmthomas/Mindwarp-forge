use derived_world_rules::{
    ClimateContract, ClimateInput, GeologicalAtmosphericContract, GeologicalAtmosphericInput,
    HydrologicalContract, HydrologicalInput, RegionalEnvironmentContract, RegionalEnvironmentInput,
    SignalChannel, SignalPotential, StellarOrbitalContract, StellarOrbitalInput,
    WorldGenerationInput, compile_climate, compile_geological_atmospheric, compile_hydrological,
    compile_regional_environment, compile_stellar_orbital, compile_world,
};
use field_basis::{FieldRecipe, ONE, Term};
use hierarchy_history::DeltaEnvelope;
use mindwarp_signal_anchor_vertical::{
    AdapterRequirementClassV1, AdapterRequirementStatusV1, BaseLoopSemanticProjectionV1,
    SignalAnchorBundleV1, SignalAnchorError, build_signal_anchor_bundle,
    inspect_signal_anchor_bundle, validate_signal_anchor_command_rows,
};
use sha2::{Digest, Sha256};

fn authority() -> (WorldGenerationInput, derived_world_rules::CausalWorldPacket) {
    let input = world_input([0x4a; 32]);
    let packet = compile_world(&input).unwrap();
    (input, packet)
}

fn resign(value: &mut SignalAnchorBundleV1) {
    value.bundle_digest = [0; 32];
    let body = serde_json::to_vec(value).unwrap();
    let mut h = Sha256::new();
    h.update(b"mindwarp.gp4.signal-anchor.bundle.v1\0");
    h.update((body.len() as u64).to_be_bytes());
    h.update(body);
    value.bundle_digest = h.finalize().into();
}
fn reject(
    mut value: SignalAnchorBundleV1,
    input: &WorldGenerationInput,
    packet: &derived_world_rules::CausalWorldPacket,
    stage: SignalAnchorError,
) {
    resign(&mut value);
    assert_eq!(value.validate_against(input, packet).unwrap_err(), stage);
}
fn json_bytes(value: &serde_json::Value) -> Vec<u8> {
    serde_json::from_value(value.clone()).unwrap()
}
fn byte_json(bytes: Vec<u8>) -> serde_json::Value {
    serde_json::to_value(bytes).unwrap()
}
fn mutate_persisted_command(
    mut bundle: SignalAnchorBundleV1,
    index: usize,
    mutate: impl FnOnce(&mut serde_json::Value),
) -> SignalAnchorBundleV1 {
    let mut log: serde_json::Value = serde_json::from_slice(&bundle.c4v_log_bytes).unwrap();
    let delta = DeltaEnvelope::decode_strict(&json_bytes(&log["delta_bytes"][index])).unwrap();
    let mut stored: serde_json::Value = serde_json::from_slice(&delta.operation).unwrap();
    let mut command: serde_json::Value =
        serde_json::from_slice(&json_bytes(&stored["command_bytes"])).unwrap();
    mutate(&mut command);
    stored["command_bytes"] = byte_json(serde_json::to_vec(&command).unwrap());
    let rebuilt = DeltaEnvelope::new(
        delta.baseline_key,
        delta.target_logical_id,
        delta.sequence,
        delta.expected_parent,
        delta.command_id,
        delta.operation_schema,
        serde_json::to_vec(&stored).unwrap(),
    )
    .unwrap();
    log["delta_bytes"][index] = byte_json(rebuilt.encode_canonical().unwrap());
    bundle.c4v_log_bytes = serde_json::to_vec(&log).unwrap();
    bundle
}

#[test]
fn fixed_signal_anchor_bundle_is_deterministic_strict_and_bounded() {
    let (input, packet) = authority();
    let first = build_signal_anchor_bundle(&input, &packet).expect("fixed bundle");
    let second = build_signal_anchor_bundle(&input, &packet).expect("repeat bundle");
    let bytes = first.to_bytes().expect("canonical bytes");
    assert_eq!(first, second);
    assert_eq!(bytes, second.to_bytes().unwrap());
    assert_eq!(
        SignalAnchorBundleV1::from_bytes(&bytes, &input, &packet).unwrap(),
        first
    );
    assert!(bytes.len() <= 8 * 1024 * 1024);
    assert_eq!(first.schema_version, 1);
    assert_eq!(first.bundle_id, "gp4.signal-anchor.bundle-v1");
    assert_eq!(first.command_ids.len(), 4);
    assert_eq!(first.presentation_slots.len(), 25);
    assert_eq!(first.adapter_requirements.len(), 29);
    assert!(first.threat_selected);
    assert_eq!(
        first.gp3_threat_digest,
        "9d9e3507f19953aef3c7a2013fac50c370d15e94013318c45d5b31fad33aa248"
    );
    assert_eq!(
        first
            .adapter_requirements
            .iter()
            .filter(|r| r.class == AdapterRequirementClassV1::Hard)
            .count(),
        16
    );
    assert_eq!(
        first
            .adapter_requirements
            .iter()
            .filter(|r| r.class == AdapterRequirementClassV1::Compare)
            .count(),
        13
    );
    assert!(
        first
            .adapter_requirements
            .iter()
            .all(|r| r.status == AdapterRequirementStatusV1::Unmeasured)
    );
}

#[test]
fn every_top_level_field_and_fixed_registry_are_authenticated() {
    let (input, packet) = authority();
    let original = build_signal_anchor_bundle(&input, &packet).unwrap();
    let mut v = original.clone();
    v.schema_version = 2;
    reject(v, &input, &packet, SignalAnchorError::Registry);
    let mut v = original.clone();
    v.bundle_id.push('x');
    reject(v, &input, &packet, SignalAnchorError::Registry);
    for field in [
        "session",
        "input",
        "packet",
        "receipt",
        "situation",
        "progression",
    ] {
        let mut v = original.clone();
        match field {
            "session" => v.session_bytes.push(0),
            "input" => v.c3a_input_bytes.push(0),
            "packet" => v.c3a_packet_bytes.push(0),
            "receipt" => v.persistence_receipt_bytes.push(0),
            "situation" => v.gp3_situation_bytes.push(0),
            _ => v.progression_ledger_bytes.push(0),
        }
        reject(v, &input, &packet, SignalAnchorError::Dependency);
    }
    for field in ["log", "prefix", "final", "shadow"] {
        let mut v = original.clone();
        match field {
            "log" => v.c4v_log_bytes.push(0),
            "prefix" => v.return_prefix_snapshot_bytes.push(0),
            "final" => v.final_snapshot_bytes.push(0),
            _ => v.authored_shadow_state_bytes.push(0),
        }
        reject(v, &input, &packet, SignalAnchorError::Replay);
    }
    let mut v = original.clone();
    v.command_ids.swap(0, 1);
    reject(v, &input, &packet, SignalAnchorError::Replay);
    let mut v = original.clone();
    v.common_semantic_digest[0] ^= 1;
    reject(v, &input, &packet, SignalAnchorError::Replay);
    let mut v = original.clone();
    v.gp4_approach_ref_digest[0] ^= 1;
    reject(v, &input, &packet, SignalAnchorError::Registry);
    let mut v = original.clone();
    v.gp3_threat_digest.push('0');
    reject(v, &input, &packet, SignalAnchorError::Registry);
    let mut v = original.clone();
    v.gp4_threat_ref_digest[0] ^= 1;
    reject(v, &input, &packet, SignalAnchorError::Registry);
    let mut v = original.clone();
    v.threat_selected = false;
    reject(v, &input, &packet, SignalAnchorError::Replay);
    let mut v = original.clone();
    v.presentation_slots[0].text_equivalent.push('x');
    reject(v, &input, &packet, SignalAnchorError::Registry);
    let mut v = original.clone();
    v.adapter_requirements[0].question.push('x');
    reject(v, &input, &packet, SignalAnchorError::Registry);
    let mut v = original.clone();
    v.bundle_digest[0] ^= 1;
    assert_eq!(
        v.validate_against(&input, &packet).unwrap_err(),
        SignalAnchorError::Digest
    );
}

#[test]
fn hostile_decode_rejects_unknown_fields_noncanonical_bytes_and_overflow_before_parse() {
    let (input, packet) = authority();
    let bundle = build_signal_anchor_bundle(&input, &packet).unwrap();
    let bytes = bundle.to_bytes().unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("ambient_runtime".into(), serde_json::json!(true));
    assert!(
        SignalAnchorBundleV1::from_bytes(&serde_json::to_vec(&value).unwrap(), &input, &packet)
            .is_err()
    );
    let mut whitespace = bytes.clone();
    whitespace.push(b' ');
    assert!(SignalAnchorBundleV1::from_bytes(&whitespace, &input, &packet).is_err());
    assert_eq!(
        SignalAnchorBundleV1::from_bytes(&vec![b' '; 8 * 1024 * 1024 + 1], &input, &packet)
            .unwrap_err(),
        SignalAnchorError::Bound
    );
}

#[test]
fn all_semantic_fields_and_both_authorities_are_independently_replayed() {
    let (input, packet) = authority();
    let bundle = build_signal_anchor_bundle(&input, &packet).unwrap();
    let inspection = inspect_signal_anchor_bundle(&bundle, &input, &packet).unwrap();
    assert_eq!(inspection.c3a_projection, inspection.authored_projection);
    assert_eq!(
        inspection.c3a_ledger_before_bytes,
        inspection.authored_ledger_before_bytes
    );
    assert_eq!(
        inspection.c3a_ledger_after_bytes,
        inspection.authored_ledger_after_bytes
    );
    assert_eq!(
        BaseLoopSemanticProjectionV1::FIELD_NAMES,
        [
            "schema_version",
            "run_id",
            "session_id",
            "phase",
            "preparation",
            "predecessor_outcome_id",
            "session_state",
            "ledger_before",
            "ledger_after",
            "failure",
            "recoveries_used",
            "stable_stop",
            "trace"
        ]
    );
    for field in BaseLoopSemanticProjectionV1::FIELD_NAMES {
        assert!(!inspection.semantic_field_bytes(field).unwrap().is_empty());
    }
    assert!(inspection.c3a_world_is_validated);
    assert!(inspection.shadow_world_is_authored);
    assert!(inspection.authored_c4v_rejected);
    assert_eq!(
        inspection
            .command_rows
            .iter()
            .map(|r| (r.sequence, r.expected_revision, r.action_count))
            .collect::<Vec<_>>(),
        [(1, 0, 1), (2, 1, 2), (3, 2, 1), (4, 3, 1)]
    );
    assert_eq!(
        inspection
            .command_rows
            .iter()
            .map(|r| r.expected_parent)
            .collect::<Vec<_>>(),
        [
            None,
            Some(hex32(
                "b5f7b4d62d529354ae0de94469521cdef175a1ca8458dbcf0a78958bca02a66f"
            )),
            Some(hex32(
                "95bd51fd8aa14ba9e67f9ad19193dee7ae613751c37d46973e51fe6e03a5d7e8"
            )),
            Some(hex32(
                "66e5b1d83cd5d0ad86dab09bcef57b72350437c50662a7587830a0965c0045ea"
            ))
        ]
    );
    assert!(
        inspection.prefix_restart_verified
            && inspection.final_restart_verified
            && inspection.stored_state_distrusted
    );
    let mut shadow: serde_json::Value =
        serde_json::from_slice(&bundle.authored_shadow_state_bytes).unwrap();
    for field in BaseLoopSemanticProjectionV1::FIELD_NAMES {
        let mut v = bundle.clone();
        let old = shadow[field].clone();
        shadow[field] = serde_json::Value::Null;
        v.authored_shadow_state_bytes = serde_json::to_vec(&shadow).unwrap();
        reject(v, &input, &packet, SignalAnchorError::Replay);
        shadow[field] = old;
    }
    let mut v = bundle.clone();
    shadow["world_context"] = serde_json::json!({"ValidatedC3A":{}});
    v.authored_shadow_state_bytes = serde_json::to_vec(&shadow).unwrap();
    reject(v, &input, &packet, SignalAnchorError::Authority);
    let mut v = bundle.clone();
    shadow = serde_json::from_slice(&v.authored_shadow_state_bytes).unwrap();
    shadow["trace"][0]["Prepare"]["divert_threat"] = serde_json::json!(false);
    v.authored_shadow_state_bytes = serde_json::to_vec(&shadow).unwrap();
    reject(v, &input, &packet, SignalAnchorError::Replay);
    for index in 0..inspection.command_rows.len() {
        for field in 0..5 {
            let mut rows = inspection.command_rows.clone();
            match field {
                0 => rows[index].actor_player_id[0] ^= 1,
                1 => rows[index].sequence += 1,
                2 => rows[index].expected_revision += 1,
                3 => rows[index].expected_parent = Some([0; 32]),
                _ => rows[index].action_count += 1,
            };
            assert_eq!(
                validate_signal_anchor_command_rows(&rows).unwrap_err(),
                SignalAnchorError::Replay
            );
        }
    }
    let mut rows = inspection.command_rows.clone();
    rows.push(rows[0].clone());
    assert_eq!(
        validate_signal_anchor_command_rows(&rows).unwrap_err(),
        SignalAnchorError::Replay
    );
}

#[test]
fn exact_gp3_and_gp2_evidence_is_ordered_and_threat_does_not_leak() {
    let (input, packet) = authority();
    let bundle = build_signal_anchor_bundle(&input, &packet).unwrap();
    let i = inspect_signal_anchor_bundle(&bundle, &input, &packet).unwrap();
    assert_eq!(
        (
            i.situation_id.as_str(),
            i.approach_id.as_str(),
            i.tool_id.as_str(),
            i.risk_id.as_str(),
            i.threat_id.as_str(),
            i.outcome_id.as_str()
        ),
        (
            "gp3.s4.signal-anchor",
            "s4.approach.temporary",
            "temporary-brace-kit",
            "anchor-collapse",
            "wire-scavengers",
            "s4.temporary-rescue"
        )
    );
    assert_eq!(
        i.intervention_step_ids,
        [
            "s4.approach.temporary.step.1",
            "s4.approach.temporary.step.2"
        ]
    );
    assert_eq!(
        (
            i.progression_rule_id.as_str(),
            i.progression_decision_id.as_str()
        ),
        ("gp2.s4.rescue", "s4.rescue-next")
    );
    assert_eq!(
        i.progression_emitted_ids,
        [
            "knowledge.s4-temporary-rescue",
            "knowledge.s4-temporary-rescue.grant-0",
            "relationship.s4-temporary-rescue.0",
            "construction.s4-temporary-rescue.0",
            "asset.s4-temporary-rescue",
            "liability.s4-temporary-rescue.0"
        ]
    );
    assert_eq!(
        i.progression_transition_ids,
        [
            "anchor.brace.temporary",
            "crossing.count.one",
            "iven.location.returned",
            "signal.coordinate.recorded",
            "caravan.state.delayed",
            "brace.state-at-return.expired"
        ]
    );
    assert!(
        !i.progression_transition_ids
            .iter()
            .any(|id| id == "work-area.state.safe")
    );
    assert!(
        !i.progression_lane_ids
            .iter()
            .any(|id| id == "work-area.state.safe")
    );
    assert!(
        i.progression_cross_identity_rejected
            && i.progression_duplicate_rejected
            && i.c3a_progression_rejected
    );
    let mut ledger: serde_json::Value =
        serde_json::from_slice(&bundle.progression_ledger_bytes).unwrap();
    ledger["processed_receipts"][0]["emitted_record_ids"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    let mut v = bundle.clone();
    v.progression_ledger_bytes = serde_json::to_vec(&ledger).unwrap();
    reject(v, &input, &packet, SignalAnchorError::Dependency);
    ledger = serde_json::from_slice(&bundle.progression_ledger_bytes).unwrap();
    ledger["processed_receipts"][0]["world_transition_ids"]
        .as_array_mut()
        .unwrap()
        .swap(0, 1);
    let mut v = bundle.clone();
    v.progression_ledger_bytes = serde_json::to_vec(&ledger).unwrap();
    reject(v, &input, &packet, SignalAnchorError::Dependency);
    ledger = serde_json::from_slice(&bundle.progression_ledger_bytes).unwrap();
    ledger["processed_receipts"][0]["world_transition_ids"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!("work-area.state.safe"));
    let mut v = bundle.clone();
    v.progression_ledger_bytes = serde_json::to_vec(&ledger).unwrap();
    reject(v, &input, &packet, SignalAnchorError::Dependency);
    for key in [
        "processed_receipts",
        "knowledge",
        "relationship_events",
        "constructions",
        "named_assets",
        "liabilities",
    ] {
        let mut l: serde_json::Value =
            serde_json::from_slice(&bundle.progression_ledger_bytes).unwrap();
        let array = l[key].as_array_mut().unwrap();
        array.push(array[0].clone());
        let mut v = bundle.clone();
        v.progression_ledger_bytes = serde_json::to_vec(&l).unwrap();
        reject(v, &input, &packet, SignalAnchorError::Dependency);
    }
}

#[test]
fn every_semantic_slot_channel_source_and_requirement_tuple_is_fixed() {
    let (input, packet) = authority();
    let original = build_signal_anchor_bundle(&input, &packet).unwrap();
    for index in 0..original.presentation_slots.len() {
        for channel in 0..6 {
            let mut v = original.clone();
            let row = &mut v.presentation_slots[index];
            match channel {
                0 => row.slot_id.push('x'),
                1 => row.source_ids.push("foreign.source".into()),
                2 => row.source_id_list_digest[0] ^= 1,
                3 => row.text_equivalent.push('x'),
                4 => row.non_color_cue.push('x'),
                _ => row.screen_reader_label.push('x'),
            };
            reject(v, &input, &packet, SignalAnchorError::Registry);
        }
        let mut v = original.clone();
        v.presentation_slots[index]
            .reduced_motion_equivalent
            .push('x');
        reject(v, &input, &packet, SignalAnchorError::Registry);
    }
    for index in 0..original.adapter_requirements.len() {
        for field in 0..7 {
            let mut v = original.clone();
            let row = &mut v.adapter_requirements[index];
            match field {
                0 => row.requirement_id.push('x'),
                1 => {
                    row.class = match row.class {
                        AdapterRequirementClassV1::Hard => AdapterRequirementClassV1::Compare,
                        AdapterRequirementClassV1::Compare => AdapterRequirementClassV1::Hard,
                    }
                }
                2 => {}
                3 => row.question.push('x'),
                4 => row.required_evidence.push('x'),
                5 => row.method.push('x'),
                _ => row.target.push('x'),
            };
            if field == 2 {
                row.status = AdapterRequirementStatusV1::Measured;
            }
            reject(v, &input, &packet, SignalAnchorError::Registry);
        }
    }
    for kind in 0..3 {
        let stage = if kind < 2 {
            SignalAnchorError::Bound
        } else {
            SignalAnchorError::Registry
        };
        let mut v = original.clone();
        match kind {
            0 => {
                v.presentation_slots.pop();
            }
            1 => {
                v.presentation_slots.push(v.presentation_slots[0].clone());
            }
            _ => v.presentation_slots.swap(0, 1),
        }
        reject(v, &input, &packet, stage);
        let mut v = original.clone();
        match kind {
            0 => {
                v.adapter_requirements.pop();
            }
            1 => {
                v.adapter_requirements
                    .push(v.adapter_requirements[0].clone());
            }
            _ => v.adapter_requirements.swap(0, 1),
        }
        reject(v, &input, &packet, stage);
    }
    for kind in 0..2 {
        let mut v = original.clone();
        if kind == 0 {
            v.command_ids.pop();
        } else {
            v.command_ids.push(v.command_ids[0]);
        }
        reject(v, &input, &packet, SignalAnchorError::Bound);
    }
}

#[test]
fn nested_caps_and_hostile_text_runtime_claims_fail_closed() {
    let (input, packet) = authority();
    let original = build_signal_anchor_bundle(&input, &packet).unwrap();
    for (field, cap) in [
        ("session", 262144),
        ("input", 262144),
        ("packet", 262144),
        ("log", 4194304),
        ("prefix", 524288),
        ("final", 524288),
        ("receipt", 65536),
        ("shadow", 262144),
        ("situation", 32768),
        ("ledger", 1048576),
    ] {
        let mut v = original.clone();
        let bytes = vec![0; cap + 1];
        match field {
            "session" => v.session_bytes = bytes,
            "input" => v.c3a_input_bytes = bytes,
            "packet" => v.c3a_packet_bytes = bytes,
            "log" => v.c4v_log_bytes = bytes,
            "prefix" => v.return_prefix_snapshot_bytes = bytes,
            "final" => v.final_snapshot_bytes = bytes,
            "receipt" => v.persistence_receipt_bytes = bytes,
            "shadow" => v.authored_shadow_state_bytes = bytes,
            "situation" => v.gp3_situation_bytes = bytes,
            _ => v.progression_ledger_bytes = bytes,
        };
        reject(v, &input, &packet, SignalAnchorError::Bound);
    }
    for hostile in [
        "https://runtime.invalid",
        "C:\\runtime.exe",
        "\\\\server\\runtime.dll",
        "adapter.ps1",
        "engine selected",
        "runtime promoted",
        "broad_g1=true",
        "runtime_selected=true",
        "runtime_containment_pending=false",
        "evidence_only=false",
        "promotion_authority=true",
        "G1-CLOSEOUT",
        "R1 promoted",
    ] {
        let mut v = original.clone();
        v.adapter_requirements[0].question = hostile.into();
        reject(v, &input, &packet, SignalAnchorError::Registry);
    }
    let mut v = original.clone();
    v.presentation_slots[0].source_ids = vec!["x".into(); 17];
    reject(v, &input, &packet, SignalAnchorError::Bound);
    let mut v = original.clone();
    v.presentation_slots[0].source_ids.clear();
    reject(v, &input, &packet, SignalAnchorError::Bound);
    let mut v = original.clone();
    v.presentation_slots[0].slot_id = "x".repeat(129);
    reject(v, &input, &packet, SignalAnchorError::Bound);
    let mut v = original.clone();
    v.presentation_slots[0].text_equivalent = "x".repeat(513);
    reject(v, &input, &packet, SignalAnchorError::Bound);
    for field in 0..4 {
        let mut v = original.clone();
        let row = &mut v.adapter_requirements[0];
        match field {
            0 => row.question = "x".repeat(513),
            1 => row.required_evidence = "x".repeat(513),
            2 => row.method = "x".repeat(513),
            _ => row.target = "x".repeat(513),
        }
        reject(v, &input, &packet, SignalAnchorError::Bound);
    }
}

#[test]
fn external_expected_c3a_authority_rejects_crossed_input_packet_and_dependency_bytes() {
    let (input, packet) = authority();
    let bundle = build_signal_anchor_bundle(&input, &packet).unwrap();
    let bytes = bundle.to_bytes().unwrap();
    let wrong_input = world_input([0x4b; 32]);
    let wrong_packet = compile_world(&wrong_input).unwrap();
    assert_eq!(
        SignalAnchorBundleV1::from_bytes(&bytes, &wrong_input, &wrong_packet).unwrap_err(),
        SignalAnchorError::Authority
    );
    assert_eq!(
        SignalAnchorBundleV1::from_bytes(&bytes, &input, &wrong_packet).unwrap_err(),
        SignalAnchorError::Authority
    );
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["c3a_packet_bytes"][0] = serde_json::json!(0);
    assert_eq!(
        SignalAnchorBundleV1::from_bytes(&serde_json::to_vec(&value).unwrap(), &input, &packet)
            .unwrap_err(),
        SignalAnchorError::Codec
    );
    let text = String::from_utf8(bytes).unwrap();
    let duplicate = text.replacen(
        "{\"schema_version\":1",
        "{\"schema_version\":1,\"schema_version\":1",
        1,
    );
    assert!(SignalAnchorBundleV1::from_bytes(duplicate.as_bytes(), &input, &packet).is_err());
    let missing = text.replacen("\"bundle_id\":\"gp4.signal-anchor.bundle-v1\",", "", 1);
    assert!(SignalAnchorBundleV1::from_bytes(missing.as_bytes(), &input, &packet).is_err());
    let wrong = text.replacen("\"schema_version\":1", "\"schema_version\":[]", 1);
    assert!(SignalAnchorBundleV1::from_bytes(wrong.as_bytes(), &input, &packet).is_err());
    assert!(
        SignalAnchorBundleV1::from_bytes(&text.as_bytes()[..text.len() - 1], &input, &packet)
            .is_err()
    );
}

#[test]
fn nested_dependency_command_snapshot_gp3_and_receipt_hostiles_are_direct() {
    let (input, packet) = authority();
    let original = build_signal_anchor_bundle(&input, &packet).unwrap();
    let inspection = inspect_signal_anchor_bundle(&original, &input, &packet).unwrap();
    assert_eq!(
        inspection
            .command_rows
            .iter()
            .flat_map(|r| r.action_labels.clone())
            .collect::<Vec<_>>(),
        [
            "Prepare",
            "Depart",
            "ChooseOutcome",
            "BeginReturn",
            "RecordRememberedResponse"
        ]
    );
    for field in 0..3 {
        let mut rows = inspection.command_rows.clone();
        match field {
            0 => rows[0].command_id[0] ^= 1,
            1 => rows[1].action_labels.reverse(),
            _ => rows.swap(0, 1),
        }
        assert_eq!(
            validate_signal_anchor_command_rows(&rows).unwrap_err(),
            SignalAnchorError::Replay
        );
    }
    for field in 0..6 {
        let v = mutate_persisted_command(
            original.clone(),
            if field == 5 { 1 } else { 0 },
            |command| match field {
                0 => command["actor_player_id"][0] = serde_json::json!(0),
                1 => command["command_id"][0] = serde_json::json!(0),
                2 => command["sequence"] = serde_json::json!(99),
                3 => command["expected_revision"] = serde_json::json!(99),
                4 => command["expected_parent"] = serde_json::json!(vec![0; 32]),
                _ => command["actions"].as_array_mut().unwrap().reverse(),
            },
        );
        reject(v, &input, &packet, SignalAnchorError::Replay);
    }
    let v = mutate_persisted_command(original.clone(), 0, |command| {
        command["actions"][0]["Prepare"]["divert_threat"] = serde_json::json!(false)
    });
    reject(v, &input, &packet, SignalAnchorError::Replay);
    let mut v = original.clone();
    v.threat_selected = false;
    reject(v, &input, &packet, SignalAnchorError::Replay);
    for mutation in 0..2 {
        let mut v = original.clone();
        let mut log: serde_json::Value = serde_json::from_slice(&v.c4v_log_bytes).unwrap();
        if mutation == 0 {
            log["delta_bytes"].as_array_mut().unwrap().swap(0, 1)
        } else {
            let row = log["delta_bytes"][0].clone();
            log["delta_bytes"].as_array_mut().unwrap().push(row)
        }
        v.c4v_log_bytes = serde_json::to_vec(&log).unwrap();
        reject(v, &input, &packet, SignalAnchorError::Replay);
    }
    for (field, stage) in [
        ("log", SignalAnchorError::Replay),
        ("prefix", SignalAnchorError::Replay),
        ("final", SignalAnchorError::Replay),
        ("receipt", SignalAnchorError::Dependency),
    ] {
        let mut v = original.clone();
        let target = match field {
            "log" => &mut v.c4v_log_bytes,
            "prefix" => &mut v.return_prefix_snapshot_bytes,
            "final" => &mut v.final_snapshot_bytes,
            _ => &mut v.persistence_receipt_bytes,
        };
        let mut nested: serde_json::Value = serde_json::from_slice(target).unwrap();
        let key = match field {
            "log" => "final_state_bytes",
            "prefix" | "final" => "revision",
            _ => "evidence_only",
        };
        nested[key] = match field {
            "prefix" | "final" => serde_json::json!(99),
            "receipt" => serde_json::json!(false),
            _ => serde_json::json!([0]),
        };
        *target = serde_json::to_vec(&nested).unwrap();
        reject(v, &input, &packet, stage);
    }
    for mutation in 0..6 {
        let mut v = original.clone();
        let mut s: serde_json::Value = serde_json::from_slice(&v.gp3_situation_bytes).unwrap();
        match mutation {
            0 => s["situation_id"] = serde_json::json!("gp3.s1.failing-world"),
            1 => s["approaches"][0]["approach_id"] = serde_json::json!("foreign.approach"),
            2 => s["threat_ref"]["nonterminal"] = serde_json::json!(false),
            3 => {
                s["evidence_refs"].as_array_mut().unwrap().pop();
            }
            4 => {
                s["approaches"][0]["consequence_refs"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            }
            _ => {
                let row = s["approaches"][0]["consequence_refs"][0].clone();
                s["approaches"][0]["consequence_refs"]
                    .as_array_mut()
                    .unwrap()
                    .push(row);
            }
        };
        v.gp3_situation_bytes = serde_json::to_vec(&s).unwrap();
        reject(v, &input, &packet, SignalAnchorError::Dependency);
    }
    for field in ["run_id", "session_id", "outcome_id", "rule_id"] {
        let mut v = original.clone();
        let mut l: serde_json::Value = serde_json::from_slice(&v.progression_ledger_bytes).unwrap();
        l["processed_receipts"][0][field] = serde_json::json!("foreign.id");
        v.progression_ledger_bytes = serde_json::to_vec(&l).unwrap();
        reject(v, &input, &packet, SignalAnchorError::Dependency);
    }
    for field in [
        "session_bytes",
        "gp3_situation_bytes",
        "progression_ledger_bytes",
    ] {
        let mut v = original.clone();
        let target = match field {
            "session_bytes" => &mut v.session_bytes,
            "gp3_situation_bytes" => &mut v.gp3_situation_bytes,
            _ => &mut v.progression_ledger_bytes,
        };
        let mut nested: serde_json::Value = serde_json::from_slice(target).unwrap();
        nested
            .as_object_mut()
            .unwrap()
            .insert("ambient_runtime".into(), serde_json::json!(true));
        *target = serde_json::to_vec(&nested).unwrap();
        reject(v, &input, &packet, SignalAnchorError::Dependency);
    }
}

fn hex32(value: &str) -> [u8; 32] {
    let mut out = [0; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&value[i * 2..i * 2 + 2], 16).unwrap();
    }
    out
}

fn world_input(reconstruction_id: [u8; 32]) -> WorldGenerationInput {
    WorldGenerationInput {
        schema_version: 1,
        field_contract_version: field_basis::CONTRACT_VERSION,
        reconstruction_id,
        surface_material: surface_contract(reconstruction_id),
        regional_environment: regional_contract(reconstruction_id),
        signal_potentials: vec![SignalPotential {
            channel: SignalChannel::VisibleRadiance,
            baseline_strength_permille: 900,
        }],
    }
}
fn regional_contract(r: [u8; 32]) -> RegionalEnvironmentContract {
    compile_regional_environment(&RegionalEnvironmentInput {
        schema_version: 1,
        reconstruction_id: r,
        regional_source_id: [8; 32],
        field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(ONE)], 0)
            .unwrap()
            .encode_canonical()
            .unwrap(),
        moisture_source_id: [9; 32],
        moisture_field_recipe_bytes: FieldRecipe::new(vec![Term::Constant(0)], 0)
            .unwrap()
            .encode_canonical()
            .unwrap(),
        coordinate_q32_32: [0, 0],
    })
    .unwrap()
}
fn stellar_contract(r: [u8; 32]) -> StellarOrbitalContract {
    compile_stellar_orbital(&StellarOrbitalInput {
        schema_version: 1,
        reconstruction_id: r,
        stellar_source_id: [3; 32],
        primary_mass_milli_solar: 1000,
        stellar_luminosity_millionths_solar: 1_000_000,
        stellar_spectrum_rgb_permille: [400, 350, 250],
        semi_major_axis_milli_au: 1000,
        eccentricity_millionths: 0,
    })
    .unwrap()
}
fn geological_contract(r: [u8; 32]) -> GeologicalAtmosphericContract {
    compile_geological_atmospheric(&GeologicalAtmosphericInput {
        schema_version: 1,
        reconstruction_id: r,
        planetary_body_id: [4; 32],
        stellar_orbital: stellar_contract(r),
        planet_mass_milli_earth: 1000,
        planet_radius_milli_earth: 1000,
        internal_heat_flux_milli_w_m2: 87,
        solid_surface_fraction_permille: 600,
        atmospheric_column_mass_g_m2: 10_332_000,
        gas_transmission_rgb_permille: [800, 900, 950],
        aerosol_transmission_rgb_permille: [1000; 3],
    })
    .unwrap()
}
fn hydrological_contract(r: [u8; 32]) -> HydrologicalContract {
    compile_hydrological(&HydrologicalInput {
        schema_version: 1,
        reconstruction_id: r,
        hydrological_source_id: [5; 32],
        geological_atmospheric: geological_contract(r),
        total_water_column_g_m2: 2_000_000,
        phase_partition_permille: [100, 850, 50],
        surface_accessible_liquid_fraction_permille: 700,
    })
    .unwrap()
}
fn climate_contract(r: [u8; 32]) -> ClimateContract {
    compile_climate(&ClimateInput {
        schema_version: 1,
        reconstruction_id: r,
        climate_source_id: [6; 32],
        hydrological: hydrological_contract(r),
        bond_albedo_permille: 300,
        outgoing_longwave_fraction_of_incident_permille: 700,
    })
    .unwrap()
}
fn surface_contract(r: [u8; 32]) -> derived_world_rules::SurfaceMaterialContract {
    derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
        schema_version: 1,
        reconstruction_id: r,
        material_source_id: [7; 32],
        climate: climate_contract(r),
        dominant_surface_reflectance_rgb_permille: [500, 400, 300],
    })
    .unwrap()
}
