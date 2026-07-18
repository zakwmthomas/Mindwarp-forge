mod world_support;

use derived_world_rules::compile_world;
use mindwarp_gameplay_foundation::{
    BaseLoopActionV1, BaseLoopLedgerV1, BaseLoopStateV1, GameplayError, LoopPhaseV1,
    LoopWorldContextV1, PreparationV1, ResumeActionV1, TypedMutation, apply_base_loop_action,
    fixed_sessions, start_authored_base_loop, start_c3a_base_loop,
};

fn preparation(session_id: &str, divert_threat: bool) -> PreparationV1 {
    PreparationV1 {
        session_id: session_id.into(),
        intent_id: "restore-shared-cause".into(),
        tool_id: "fitting-field-tool".into(),
        divert_threat,
    }
}

fn drive_to_response(
    session: &mindwarp_gameplay_foundation::SessionRecordV1,
    run_id: &str,
    ledger: BaseLoopLedgerV1,
    outcome_id: &str,
) -> mindwarp_gameplay_foundation::BaseLoopStateV1 {
    let mut state = start_authored_base_loop(session, run_id, ledger).unwrap();
    for action in [
        BaseLoopActionV1::Prepare(preparation(
            &session.session_id,
            session.threat_contribution.is_some(),
        )),
        BaseLoopActionV1::Depart,
        BaseLoopActionV1::ChooseOutcome {
            outcome_id: outcome_id.into(),
        },
        BaseLoopActionV1::BeginReturn,
        BaseLoopActionV1::RecordRememberedResponse,
    ] {
        state = apply_base_loop_action(session, &state, &action).unwrap();
    }
    state
}

#[test]
fn all_five_sessions_share_one_six_phase_loop_and_outcome_is_not_prepared() {
    for (index, session) in fixed_sessions().iter().enumerate() {
        let predecessor = (index == 4).then(|| {
            let s1 = drive_to_response(
                &fixed_sessions()[0],
                "run-s1-seed",
                BaseLoopLedgerV1::empty(),
                "s1.bypass",
            );
            s1.ledger_after
        });
        let ledger = predecessor.unwrap_or_else(BaseLoopLedgerV1::empty);
        let mut state = start_authored_base_loop(session, &format!("run-{index}"), ledger).unwrap();
        assert_eq!(state.phase, LoopPhaseV1::Prepare);
        assert_eq!(
            state.stable_stop.resume_action,
            Some(ResumeActionV1::Prepare)
        );
        state = apply_base_loop_action(
            session,
            &state,
            &BaseLoopActionV1::Prepare(preparation(
                &session.session_id,
                session.threat_contribution.is_some(),
            )),
        )
        .unwrap();
        assert!(state.session_state.selected_outcome_id.is_none());
        assert_eq!(state.phase, LoopPhaseV1::Depart);
        state = apply_base_loop_action(session, &state, &BaseLoopActionV1::Depart).unwrap();
        assert_eq!(state.phase, LoopPhaseV1::Encounter);
        let outcome = session
            .outcomes
            .iter()
            .find(|item| {
                !matches!(
                    item.trigger,
                    mindwarp_gameplay_foundation::OutcomeTrigger::Retreat
                )
            })
            .unwrap();
        state = apply_base_loop_action(
            session,
            &state,
            &BaseLoopActionV1::ChooseOutcome {
                outcome_id: outcome.outcome_id.clone(),
            },
        )
        .unwrap();
        assert_eq!(state.phase, LoopPhaseV1::Consequence);
        state = apply_base_loop_action(session, &state, &BaseLoopActionV1::BeginReturn).unwrap();
        assert_eq!(state.phase, LoopPhaseV1::Return);
        state =
            apply_base_loop_action(session, &state, &BaseLoopActionV1::RecordRememberedResponse)
                .unwrap();
        assert_eq!(state.phase, LoopPhaseV1::RememberedResponse);
        assert!(state.stable_stop.terminal && state.stable_stop.resume_action.is_none());
    }
}

#[test]
fn recoverable_failure_preserves_choice_and_rejects_fourth_failure_without_mutation() {
    let session = &fixed_sessions()[0];
    let mut state =
        start_authored_base_loop(session, "run-recovery", BaseLoopLedgerV1::empty()).unwrap();
    let foreign_plan = preparation("gp0.s2.storm-nest", false);
    assert_eq!(
        apply_base_loop_action(session, &state, &BaseLoopActionV1::Prepare(foreign_plan)),
        Err(GameplayError::Invalid("preparation session mismatch"))
    );
    state = apply_base_loop_action(
        session,
        &state,
        &BaseLoopActionV1::Prepare(preparation(&session.session_id, false)),
    )
    .unwrap();
    state = apply_base_loop_action(session, &state, &BaseLoopActionV1::Depart).unwrap();
    let cost = TypedMutation {
        subject_id: "daylight".into(),
        field_id: "remaining".into(),
        value_id: "reduced".into(),
    };
    for expected in 0..3 {
        state = apply_base_loop_action(
            session,
            &state,
            &BaseLoopActionV1::FailEncounter {
                reason_id: "tool-misaligned".into(),
                opportunity_cost: cost.clone(),
            },
        )
        .unwrap();
        assert!(state.session_state.selected_outcome_id.is_none());
        assert_eq!(
            state.stable_stop.resume_action,
            Some(ResumeActionV1::Recover)
        );
        state = apply_base_loop_action(session, &state, &BaseLoopActionV1::Recover).unwrap();
        assert_eq!(state.recoveries_used, expected + 1);
    }
    let before = state.clone();
    assert_eq!(
        apply_base_loop_action(
            session,
            &state,
            &BaseLoopActionV1::FailEncounter {
                reason_id: "tool-misaligned".into(),
                opportunity_cost: cost
            }
        ),
        Err(GameplayError::Invalid("recovery limit exhausted"))
    );
    assert_eq!(state, before);
}

#[test]
fn authored_context_is_authority_negative_and_c3a_binding_is_exact() {
    let session = &fixed_sessions()[0];
    let authored =
        start_authored_base_loop(session, "run-authored", BaseLoopLedgerV1::empty()).unwrap();
    assert_eq!(authored.world_context, LoopWorldContextV1::AuthoredFixture);
    let input = world_support::world_input([31; 32]);
    let packet = compile_world(&input).unwrap();
    let bound = start_c3a_base_loop(
        session,
        "run-c3a",
        BaseLoopLedgerV1::empty(),
        &input,
        &packet,
    )
    .unwrap();
    assert!(matches!(
        bound.world_context,
        LoopWorldContextV1::ValidatedC3A(_)
    ));
    let foreign = compile_world(&world_support::world_input([32; 32])).unwrap();
    assert_eq!(
        start_c3a_base_loop(
            session,
            "run-foreign",
            BaseLoopLedgerV1::empty(),
            &input,
            &foreign
        ),
        Err(GameplayError::InvalidWorld)
    );
}

#[test]
fn remembered_response_is_atomic_idempotent_and_distinct_runs_can_repeat() {
    let session = &fixed_sessions()[0];
    let first = drive_to_response(session, "run-first", BaseLoopLedgerV1::empty(), "s1.direct");
    assert_eq!(first.ledger_after.completed_runs[0].run_id, "run-first");
    assert_eq!(first.ledger_after.completed_runs[0].event_sequence, 1);
    assert_eq!(
        apply_base_loop_action(session, &first, &BaseLoopActionV1::RecordRememberedResponse),
        Err(GameplayError::Invalid("loop is terminal"))
    );
    assert_eq!(
        start_authored_base_loop(session, "run-first", first.ledger_after.clone()),
        Err(GameplayError::Invalid("run already completed"))
    );
    let second = drive_to_response(session, "run-second", first.ledger_after, "s1.ration");
    assert_eq!(second.ledger_after.world_history.events.len(), 2);
    assert_eq!(
        second
            .ledger_after
            .completed_runs
            .iter()
            .map(|receipt| receipt.run_id.as_str())
            .collect::<Vec<_>>(),
        vec!["run-first", "run-second"]
    );
    let mut omitted = second.ledger_after.clone();
    omitted.completed_runs.pop();
    assert_eq!(
        omitted.validate(),
        Err(GameplayError::Invalid("GP1 append receipt coverage drift"))
    );
    let mut fabricated = second.ledger_after.clone();
    fabricated.completed_runs[1].event_sequence = 99;
    assert_eq!(
        fabricated.validate(),
        Err(GameplayError::Invalid("GP1 append receipt coverage drift"))
    );
    let mut mismatched = second.ledger_after.clone();
    mismatched.completed_runs[1].outcome_id = "s1.direct".into();
    assert_eq!(
        mismatched.validate(),
        Err(GameplayError::Invalid("GP1 append receipt event mismatch"))
    );
}

#[test]
fn s1_remembered_response_materially_changes_later_s5_state() {
    let mut results = Vec::new();
    for (suffix, outcome) in [
        ("direct", "s1.direct"),
        ("bypass", "s1.bypass"),
        ("ration", "s1.ration"),
    ] {
        let s1 = drive_to_response(
            &fixed_sessions()[0],
            &format!("run-s1-{suffix}"),
            BaseLoopLedgerV1::empty(),
            outcome,
        );
        let s5 = drive_to_response(
            &fixed_sessions()[4],
            &format!("run-s5-{suffix}"),
            s1.ledger_after,
            "s5.nightway",
        );
        assert_eq!(
            s5.ledger_after
                .world_history
                .events
                .iter()
                .map(|event| event.session_id.as_str())
                .collect::<Vec<_>>(),
            vec!["gp0.s1.colony-conduit", "gp0.s5.afterlight"]
        );
        results.push((s5.session_state.exact_mutations, s5.session_state.memories));
    }
    assert_ne!(results[0], results[1]);
    assert_ne!(results[1], results[2]);
}

#[test]
fn phase_skips_unsafe_stops_and_fabricated_or_noncanonical_state_fail_closed() {
    let session = &fixed_sessions()[0];
    let mut state =
        start_authored_base_loop(session, "run-hostile", BaseLoopLedgerV1::empty()).unwrap();
    assert!(apply_base_loop_action(session, &state, &BaseLoopActionV1::Depart).is_err());
    state = apply_base_loop_action(
        session,
        &state,
        &BaseLoopActionV1::Prepare(preparation(&session.session_id, false)),
    )
    .unwrap();
    state = apply_base_loop_action(session, &state, &BaseLoopActionV1::Depart).unwrap();
    assert!(state.stable_stop.resume_action.is_none());
    assert!(apply_base_loop_action(session, &state, &BaseLoopActionV1::BeginReturn).is_err());
    state = apply_base_loop_action(
        session,
        &state,
        &BaseLoopActionV1::ChooseOutcome {
            outcome_id: "s1.bypass".into(),
        },
    )
    .unwrap();
    let bytes = state.to_bytes(session).unwrap();
    assert_eq!(
        BaseLoopStateV1::from_bytes(session, &state.world_context, &bytes).unwrap(),
        state
    );
    assert!(
        BaseLoopStateV1::from_bytes(
            session,
            &state.world_context,
            &[bytes.clone(), b"\n".to_vec()].concat()
        )
        .is_err()
    );
    let input = world_support::world_input([41; 32]);
    let packet = compile_world(&input).unwrap();
    let foreign_context = start_c3a_base_loop(
        session,
        "run-context",
        BaseLoopLedgerV1::empty(),
        &input,
        &packet,
    )
    .unwrap()
    .world_context;
    assert_eq!(
        BaseLoopStateV1::from_bytes(session, &foreign_context, &bytes),
        Err(GameplayError::Invalid(
            "world context does not match expected authority"
        ))
    );
    let mut fabricated = state.clone();
    fabricated.session_state.exact_mutations[0].value_id = "fabricated".into();
    assert!(fabricated.validate_against(session).is_err());
    let ledger_bytes = state.ledger_after.to_bytes().unwrap();
    assert_eq!(
        BaseLoopLedgerV1::from_bytes(&ledger_bytes).unwrap(),
        state.ledger_after
    );
    assert!(BaseLoopLedgerV1::from_bytes(&[ledger_bytes, b" ".to_vec()].concat()).is_err());
}

#[test]
fn afterlight_rejects_missing_or_retreat_and_uses_latest_legitimate_repeat() {
    let s1 = &fixed_sessions()[0];
    let s5 = &fixed_sessions()[4];
    assert!(start_authored_base_loop(s5, "run-missing", BaseLoopLedgerV1::empty()).is_err());
    let retreat = drive_to_response(s1, "run-retreat", BaseLoopLedgerV1::empty(), "s1.retreat");
    assert!(start_authored_base_loop(s5, "run-after-retreat", retreat.ledger_after).is_err());
    let first = drive_to_response(s1, "run-one", BaseLoopLedgerV1::empty(), "s1.direct");
    let second = drive_to_response(s1, "run-two", first.ledger_after, "s1.ration");
    let latest = start_authored_base_loop(s5, "run-latest", second.ledger_after).unwrap();
    assert_eq!(latest.predecessor_outcome_id.as_deref(), Some("s1.ration"));
}
