use derived_world_rules::{
    ClimateContract, ClimateInput, GeologicalAtmosphericContract, GeologicalAtmosphericInput,
    HydrologicalContract, HydrologicalInput, RegionalEnvironmentContract, RegionalEnvironmentInput,
    SignalChannel, SignalPotential, StellarOrbitalContract, StellarOrbitalInput,
    WorldGenerationInput, compile_climate, compile_geological_atmospheric, compile_hydrological,
    compile_regional_environment, compile_stellar_orbital, compile_world,
};
use field_basis::{FieldRecipe, ONE, Term};
use mindwarp_gameplay_foundation::{
    Action, C3AWorldReferenceV1, EvidenceClass, FactKind, GameplayConceptRecordV1, GameplayError,
    SessionState, WorldHistoryV1, apply_action, fixed_concept, fixed_sessions, project_trace_text,
    replay_actions, replay_actions_after,
};

fn actions(outcome_id: &str, combat: bool) -> Vec<Action> {
    let mut actions = vec![Action::ObserveCause, Action::MakeFittingTool];
    if combat {
        actions.push(Action::DivertThreat);
    }
    actions.push(Action::CommitOutcome {
        outcome_id: outcome_id.into(),
    });
    actions
}

#[test]
fn concept_codec_is_strict_and_assumptions_remain_reversible() {
    let record = fixed_concept();
    let bytes = record.to_bytes().unwrap();
    assert_eq!(GameplayConceptRecordV1::from_bytes(&bytes).unwrap(), record);
    assert!(GameplayConceptRecordV1::from_bytes(&[bytes, b" ".to_vec()].concat()).is_err());
}

#[test]
fn five_authored_sessions_validate_and_round_trip_strictly() {
    let sessions = fixed_sessions();
    assert_eq!(sessions.len(), 5);
    for session in sessions {
        session.validate().unwrap();
        let bytes = session.to_bytes().unwrap();
        assert_eq!(
            mindwarp_gameplay_foundation::SessionRecordV1::from_bytes(&bytes).unwrap(),
            session
        );
        assert!(
            mindwarp_gameplay_foundation::SessionRecordV1::from_bytes(
                &[bytes, b"\n".to_vec()].concat()
            )
            .is_err()
        );
    }
}

#[test]
fn every_outcome_is_typed_terminal_and_has_its_own_later_decision() {
    for session in fixed_sessions() {
        assert_eq!(
            session
                .outcomes
                .iter()
                .filter(|outcome| matches!(
                    outcome.trigger,
                    mindwarp_gameplay_foundation::OutcomeTrigger::Retreat
                ))
                .count(),
            1
        );
        for outcome in &session.outcomes {
            assert!(!outcome.exact_mutations.is_empty());
            assert!(!outcome.opportunity_costs.is_empty());
            assert!(!outcome.memories.is_empty());
            assert!(!outcome.next_decision.decision_id.is_empty());
        }
    }
}

#[test]
fn combat_contributes_in_three_sessions_but_completes_none() {
    for (index, predecessor) in [(1_usize, None), (3, None), (4, Some("s1.bypass"))] {
        let session = &fixed_sessions()[index];
        let diverted = replay_actions_after(
            session,
            predecessor,
            &[
                Action::ObserveCause,
                Action::MakeFittingTool,
                Action::DivertThreat,
            ],
        )
        .unwrap();
        assert!(diverted.threat_diverted);
        assert!(!diverted.terminal);
        assert!(!diverted.core_tension_resolved);
    }
}

#[test]
fn threat_actions_are_session_specific_and_s4_only_clears_the_work_area() {
    let sessions = fixed_sessions();
    assert_eq!(
        replay_actions(&sessions[0], &[Action::DivertThreat]),
        Err(GameplayError::Invalid("session has no threat contribution"))
    );
    assert_eq!(
        replay_actions(&sessions[2], &[Action::DivertThreat]),
        Err(GameplayError::Invalid("session has no threat contribution"))
    );
    let s4 = replay_actions(
        &sessions[3],
        &[
            Action::ObserveCause,
            Action::MakeFittingTool,
            Action::DivertThreat,
        ],
    )
    .unwrap();
    assert!(
        s4.contributing_mutations
            .iter()
            .any(|mutation| { mutation.subject_id == "work-area" && mutation.value_id == "safe" })
    );
    assert!(!s4.terminal && !s4.core_tension_resolved);
    assert!(s4.exact_mutations.is_empty());
}

#[test]
fn afterlight_replays_distinct_direct_bypass_and_ration_meanings() {
    let s5 = &fixed_sessions()[4];
    let direct =
        replay_actions_after(s5, Some("s1.direct"), &actions("s5.nightway", true)).unwrap();
    let bypass =
        replay_actions_after(s5, Some("s1.bypass"), &actions("s5.nightway", true)).unwrap();
    let ration =
        replay_actions_after(s5, Some("s1.ration"), &actions("s5.nightway", true)).unwrap();
    assert_ne!(direct.exact_mutations, bypass.exact_mutations);
    assert_ne!(bypass.exact_mutations, ration.exact_mutations);
    assert!(project_trace_text(&direct).contains("colony.location=displaced-to-spillway"));
    assert!(project_trace_text(&bypass).contains("greenhouse-spare.availability=unavailable"));
    assert!(project_trace_text(&ration).contains("colony.light-cycle=synchronized"));
    assert!(
        direct
            .memories
            .iter()
            .any(|memory| memory.proposition.contains("displaced"))
    );
    assert!(
        bypass
            .memories
            .iter()
            .any(|memory| memory.proposition.contains("protected both"))
    );
    assert!(
        ration
            .memories
            .iter()
            .any(|memory| memory.proposition.contains("synchronized"))
    );
    assert_eq!(
        replay_actions_after(s5, Some("s1.retreat"), &actions("s5.nightway", true)),
        Err(GameplayError::Invalid(
            "afterlight predecessor not admitted"
        ))
    );
}

#[test]
fn deterministic_retreats_are_terminal_and_record_cost_memory_and_decision() {
    for session in fixed_sessions() {
        let retreat = session
            .outcomes
            .iter()
            .find(|outcome| {
                matches!(
                    outcome.trigger,
                    mindwarp_gameplay_foundation::OutcomeTrigger::Retreat
                )
            })
            .unwrap();
        let script = [Action::Retreat {
            outcome_id: retreat.outcome_id.clone(),
        }];
        let predecessor = (session.session_id == "gp0.s5.afterlight").then_some("s1.bypass");
        let first = replay_actions_after(&session, predecessor, &script).unwrap();
        let second = replay_actions_after(&session, predecessor, &script).unwrap();
        assert_eq!(first, second);
        assert!(first.terminal && first.stable_stop_available);
        assert!(!first.core_tension_resolved);
        assert!(!first.exact_mutations.is_empty());
        assert!(!first.opportunity_costs.is_empty());
        assert!(!first.memories.is_empty());
        assert!(first.next_decision.is_some());
    }
}

#[test]
fn frozen_primary_traces_preserve_exact_playtest_corrections() {
    let sessions = fixed_sessions();
    let s1 = replay_actions(&sessions[0], &actions("s1.bypass", false)).unwrap();
    let s2 = replay_actions(&sessions[1], &actions("s2.relocate", true)).unwrap();
    let s3 = replay_actions(&sessions[2], &actions("s3.charter", false)).unwrap();
    let s4 = replay_actions(&sessions[3], &actions("s4.temporary-rescue", true)).unwrap();
    let s5 = replay_actions_after(
        &sessions[4],
        Some("s1.bypass"),
        &actions("s5.nightway", true),
    )
    .unwrap();

    let joined = [&s1, &s2, &s3, &s4, &s5]
        .map(|state| project_trace_text(state))
        .join("\n");
    for exact in [
        "pump.flow=partial-bypass",
        "greenhouse-spare.availability=unavailable",
        "player-whistle.recognition=learned",
        "passage-charter.east-window=evening",
        "passage-charter.west-window=dawn",
        "iven.location=returned",
        "signal.coordinate=recorded",
        "nightway-boundary.state=marked",
        "habitat-buffer.state=protected",
    ] {
        assert!(joined.contains(exact), "missing {exact}");
    }
}

#[test]
fn state_codec_replays_and_rejects_fabricated_terminal_content() {
    let session = &fixed_sessions()[0];
    let state = replay_actions(session, &actions("s1.bypass", false)).unwrap();
    let bytes = state.to_bytes(session).unwrap();
    assert_eq!(SessionState::from_bytes(session, &bytes).unwrap(), state);

    let mut fabricated = state.clone();
    fabricated.exact_mutations[0].value_id = "fabricated".into();
    assert_eq!(
        fabricated.validate_against(session),
        Err(GameplayError::Invalid(
            "state does not match deterministic replay"
        ))
    );

    let mut fabricated_nonterminal = SessionState::new(session).unwrap();
    fabricated_nonterminal.observed_cause = true;
    assert_eq!(
        apply_action(session, &fabricated_nonterminal, &Action::MakeFittingTool),
        Err(GameplayError::Invalid(
            "state does not match deterministic replay"
        ))
    );
}

#[test]
fn s1_to_s5_history_is_append_only_strict_and_replay_bound() {
    let sessions = fixed_sessions();
    let s1 = replay_actions(&sessions[0], &actions("s1.bypass", false)).unwrap();
    let s5 = replay_actions_after(
        &sessions[4],
        Some("s1.bypass"),
        &actions("s5.nightway", true),
    )
    .unwrap();
    let after_s1 = WorldHistoryV1::empty().append(&sessions[0], &s1).unwrap();
    let complete = after_s1.append(&sessions[4], &s5).unwrap();
    assert_eq!(complete.events.len(), 2);
    assert_eq!(complete.events[0].outcome_id, "s1.bypass");
    assert_eq!(complete.events[1].outcome_id, "s5.nightway");
    assert!(!complete.events[1].grants.is_empty());
    assert!(!complete.events[1].opportunity_costs.is_empty());
    let bytes = complete.to_bytes().unwrap();
    assert_eq!(WorldHistoryV1::from_bytes(&bytes).unwrap(), complete);

    let retreat = replay_actions(
        &sessions[0],
        &[Action::Retreat {
            outcome_id: "s1.retreat".into(),
        }],
    )
    .unwrap();
    let after_retreat = WorldHistoryV1::empty()
        .append(&sessions[0], &retreat)
        .unwrap();
    assert_eq!(
        after_retreat.append(&sessions[4], &s5),
        Err(GameplayError::Invalid(
            "afterlight predecessor not admitted"
        ))
    );
}

#[test]
fn history_codec_rejects_nested_and_predecessor_corruption() {
    let sessions = fixed_sessions();
    let s1 = replay_actions(&sessions[0], &actions("s1.bypass", false)).unwrap();
    let s5 = replay_actions_after(
        &sessions[4],
        Some("s1.bypass"),
        &actions("s5.nightway", true),
    )
    .unwrap();
    let valid = WorldHistoryV1::empty()
        .append(&sessions[0], &s1)
        .unwrap()
        .append(&sessions[4], &s5)
        .unwrap();

    let mut malformed = valid.clone();
    malformed.events[1].exact_mutations[0].subject_id = "INVALID ID".into();
    assert_eq!(
        malformed.validate(),
        Err(GameplayError::Invalid("malformed identifier"))
    );

    let mut currency = valid.clone();
    currency.events[1].memories[0].proposition = "Gain bond after the charter.".into();
    assert_eq!(
        currency.validate(),
        Err(GameplayError::Invalid("disguised currency language"))
    );

    let mut predecessor = valid;
    predecessor.events[1].predecessor_outcome_id = Some("s1.direct".into());
    assert_eq!(
        predecessor.validate(),
        Err(GameplayError::Invalid("history predecessor mismatch"))
    );
}

#[test]
fn negative_matrix_rejects_authority_currency_accessibility_and_order_drift() {
    let mut drifted_concept = fixed_concept();
    drifted_concept.non_goals.pop();
    assert_eq!(
        drifted_concept.validate(),
        Err(GameplayError::Invalid("non-goal boundary drift"))
    );

    let mut session = fixed_sessions()[0].clone();
    session.facts[0].proposition = "C3B proves this authored valve behavior.".into();
    assert_eq!(
        session.validate(),
        Err(GameplayError::Invalid(
            "authored fact claims scientific authority"
        ))
    );

    let mut session = fixed_sessions()[0].clone();
    session.outcomes[0].memories[0].proposition = "Mara grants reputation.".into();
    assert_eq!(
        session.validate(),
        Err(GameplayError::Invalid("disguised currency language"))
    );

    let mut session = fixed_sessions()[0].clone();
    session.risks[0].audio_or_haptic_cue.clear();
    assert_eq!(
        session.validate(),
        Err(GameplayError::Invalid("risk lacks two equivalent cues"))
    );

    let session = &fixed_sessions()[0];
    assert_eq!(
        replay_actions(session, &[Action::MakeFittingTool]),
        Err(GameplayError::Invalid("tool precedes causal observation"))
    );
}

#[test]
fn c3_a_reference_codec_is_path_free_and_observations_require_it() {
    let reference = C3AWorldReferenceV1 {
        schema_version: 1,
        reconstruction_id: [7; 32],
        input_id: "11".repeat(32),
        packet_id: "22".repeat(32),
    };
    let bytes = reference.to_bytes().unwrap();
    assert_eq!(C3AWorldReferenceV1::from_bytes(&bytes).unwrap(), reference);
    let json = String::from_utf8(bytes).unwrap();
    assert!(!json.to_ascii_lowercase().contains("path"));

    let mut session = fixed_sessions()[0].clone();
    session.facts[0].evidence_class = EvidenceClass::ObservedC3AOutput;
    session.facts[0].kind = FactKind::Observation;
    assert_eq!(
        session.validate(),
        Err(GameplayError::Invalid(
            "C3A observation lacks typed binding"
        ))
    );
}

#[test]
fn c3_a_binding_accepts_exact_pair_and_rejects_foreign_packet() {
    let input = world_input([1; 32]);
    let packet = compile_world(&input).unwrap();
    let reference =
        mindwarp_gameplay_foundation::bind_validated_c3a_world(&input, &packet).unwrap();
    assert_eq!(reference.reconstruction_id, [1; 32]);
    assert_eq!(reference.input_id, packet.content.input_id);
    assert_eq!(reference.packet_id, packet.packet_id);

    let foreign = compile_world(&world_input([2; 32])).unwrap();
    assert_eq!(
        mindwarp_gameplay_foundation::bind_validated_c3a_world(&input, &foreign),
        Err(GameplayError::InvalidWorld)
    );
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

fn regional_contract(reconstruction_id: [u8; 32]) -> RegionalEnvironmentContract {
    compile_regional_environment(&RegionalEnvironmentInput {
        schema_version: 1,
        reconstruction_id,
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

fn stellar_contract(reconstruction_id: [u8; 32]) -> StellarOrbitalContract {
    compile_stellar_orbital(&StellarOrbitalInput {
        schema_version: 1,
        reconstruction_id,
        stellar_source_id: [3; 32],
        primary_mass_milli_solar: 1_000,
        stellar_luminosity_millionths_solar: 1_000_000,
        stellar_spectrum_rgb_permille: [400, 350, 250],
        semi_major_axis_milli_au: 1_000,
        eccentricity_millionths: 0,
    })
    .unwrap()
}

fn geological_contract(reconstruction_id: [u8; 32]) -> GeologicalAtmosphericContract {
    compile_geological_atmospheric(&GeologicalAtmosphericInput {
        schema_version: 1,
        reconstruction_id,
        planetary_body_id: [4; 32],
        stellar_orbital: stellar_contract(reconstruction_id),
        planet_mass_milli_earth: 1_000,
        planet_radius_milli_earth: 1_000,
        internal_heat_flux_milli_w_m2: 87,
        solid_surface_fraction_permille: 600,
        atmospheric_column_mass_g_m2: 10_332_000,
        gas_transmission_rgb_permille: [800, 900, 950],
        aerosol_transmission_rgb_permille: [1_000; 3],
    })
    .unwrap()
}

fn hydrological_contract(reconstruction_id: [u8; 32]) -> HydrologicalContract {
    compile_hydrological(&HydrologicalInput {
        schema_version: 1,
        reconstruction_id,
        hydrological_source_id: [5; 32],
        geological_atmospheric: geological_contract(reconstruction_id),
        total_water_column_g_m2: 2_000_000,
        phase_partition_permille: [100, 850, 50],
        surface_accessible_liquid_fraction_permille: 700,
    })
    .unwrap()
}

fn climate_contract(reconstruction_id: [u8; 32]) -> ClimateContract {
    compile_climate(&ClimateInput {
        schema_version: 1,
        reconstruction_id,
        climate_source_id: [6; 32],
        hydrological: hydrological_contract(reconstruction_id),
        bond_albedo_permille: 300,
        outgoing_longwave_fraction_of_incident_permille: 700,
    })
    .unwrap()
}

fn surface_contract(reconstruction_id: [u8; 32]) -> derived_world_rules::SurfaceMaterialContract {
    derived_world_rules::compile_surface_material(&derived_world_rules::SurfaceMaterialInput {
        schema_version: 1,
        reconstruction_id,
        material_source_id: [7; 32],
        climate: climate_contract(reconstruction_id),
        dominant_surface_reflectance_rgb_permille: [500, 400, 300],
    })
    .unwrap()
}
