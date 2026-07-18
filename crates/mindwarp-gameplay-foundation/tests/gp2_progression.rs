use mindwarp_gameplay_foundation::{
    BaseLoopActionV1, BaseLoopLedgerV1, CapabilityRecordV1, GameplayError, PreparationV1,
    ProgressionLedgerV1, StrategyKindV1, apply_base_loop_action, apply_progression,
    conversion_rule_count, fixed_progression_rule_count, fixed_progression_rule_keys,
    fixed_sessions, pairwise_incomparable, reset_rule_count, simulate_strategies,
    start_authored_base_loop,
};

fn terminal(
    run: &str,
    tool: &str,
    outcome: &str,
    failures: usize,
) -> mindwarp_gameplay_foundation::BaseLoopStateV1 {
    let session = &fixed_sessions()[0];
    let mut state = start_authored_base_loop(session, run, BaseLoopLedgerV1::empty()).unwrap();
    state = apply_base_loop_action(
        session,
        &state,
        &BaseLoopActionV1::Prepare(PreparationV1 {
            session_id: session.session_id.clone(),
            intent_id: "restore-shared-cause".into(),
            tool_id: tool.into(),
            divert_threat: false,
        }),
    )
    .unwrap();
    state = apply_base_loop_action(session, &state, &BaseLoopActionV1::Depart).unwrap();
    for _ in 0..failures {
        state = apply_base_loop_action(
            session,
            &state,
            &BaseLoopActionV1::FailEncounter {
                reason_id: "caller-failure".into(),
                opportunity_cost: mindwarp_gameplay_foundation::TypedMutation {
                    subject_id: "caller".into(),
                    field_id: "claimed-power".into(),
                    value_id: "fabricated".into(),
                },
            },
        )
        .unwrap();
        state = apply_base_loop_action(session, &state, &BaseLoopActionV1::Recover).unwrap();
    }
    for action in [
        BaseLoopActionV1::ChooseOutcome {
            outcome_id: outcome.into(),
        },
        BaseLoopActionV1::BeginReturn,
        BaseLoopActionV1::RecordRememberedResponse,
    ] {
        state = apply_base_loop_action(session, &state, &action).unwrap();
    }
    state
}

fn progress(state: &mindwarp_gameplay_foundation::BaseLoopStateV1) -> ProgressionLedgerV1 {
    let prior = ProgressionLedgerV1::from_base_loop(&state.ledger_before).unwrap();
    apply_progression(&fixed_sessions()[0], state, &prior).unwrap()
}

#[test]
fn s1_rules_emit_distinct_nonfungible_records_and_exact_horizontal_capabilities() {
    let cases = [
        (
            "direct",
            "full-flow-kit",
            "s1.direct",
            "emergency-restoration",
        ),
        (
            "bypass",
            "colony-safe-kit",
            "s1.bypass",
            "bypass-installation",
        ),
        (
            "ration",
            "timed-controller",
            "s1.ration",
            "synchronized-scheduling",
        ),
    ];
    let mut ledgers = Vec::new();
    for (run, tool, outcome, capability) in cases {
        let ledger = progress(&terminal(run, tool, outcome, 0));
        assert!(!ledger.knowledge.is_empty());
        assert!(!ledger.access.is_empty());
        assert!(!ledger.relationship_events.is_empty());
        assert!(!ledger.constructions.is_empty());
        assert!(
            ledger
                .constructions
                .iter()
                .all(|item| item.predecessor_state_id.is_some())
        );
        assert!(!ledger.liabilities.is_empty());
        assert!(
            ledger
                .capabilities
                .iter()
                .any(|item| item.capability_id == capability)
        );
        ledgers.push(ledger);
    }
    assert_ne!(ledgers[0].capabilities, ledgers[1].capabilities);
    assert_ne!(ledgers[1].constructions, ledgers[2].constructions);
}

#[test]
fn caller_tool_failure_cost_combat_and_retreat_cannot_grant_capability() {
    let wrong = progress(&terminal(
        "wrong-tool",
        "emergency-restoration",
        "s1.direct",
        1,
    ));
    assert!(wrong.capabilities.is_empty());
    assert!(
        wrong
            .knowledge
            .iter()
            .all(|item| !item.proposition.contains("caller"))
    );
    let retreat = progress(&terminal("retreat", "full-flow-kit", "s1.retreat", 0));
    assert!(retreat.capabilities.is_empty() && retreat.knowledge.is_empty());
}

#[test]
fn recovery_is_attempt_local_and_does_not_change_durable_emissions() {
    let clean = progress(&terminal("clean", "colony-safe-kit", "s1.bypass", 0));
    let recovered = progress(&terminal("recovered", "colony-safe-kit", "s1.bypass", 2));
    assert_eq!(clean.knowledge, recovered.knowledge);
    assert_eq!(clean.access, recovered.access);
    assert_eq!(clean.relationship_events, recovered.relationship_events);
    assert_eq!(clean.constructions, recovered.constructions);
    let mut clean_capabilities = clean.capabilities.clone();
    let mut recovered_capabilities = recovered.capabilities.clone();
    for item in &mut clean_capabilities {
        item.source_run_id.clear();
    }
    for item in &mut recovered_capabilities {
        item.source_run_id.clear();
    }
    assert_eq!(clean_capabilities, recovered_capabilities);
    assert_eq!(clean.liabilities, recovered.liabilities);
}

#[test]
fn ledger_is_digest_bound_idempotent_strict_and_rejects_fabrication() {
    let state = terminal("digest", "timed-controller", "s1.ration", 0);
    let prior = ProgressionLedgerV1::from_base_loop(&state.ledger_before).unwrap();
    let ledger = apply_progression(&fixed_sessions()[0], &state, &prior).unwrap();
    assert!(apply_progression(&fixed_sessions()[0], &state, &ledger).is_err());
    let bytes = ledger.to_bytes().unwrap();
    assert_eq!(
        ProgressionLedgerV1::from_bytes(&state.ledger_after, &bytes).unwrap(),
        ledger
    );
    assert!(
        ProgressionLedgerV1::from_bytes(&state.ledger_after, &[bytes, b" ".to_vec()].concat())
            .is_err()
    );
    let mut forged = prior;
    forged.source_base_loop_ledger_digest = [9; 32];
    assert_eq!(
        apply_progression(&fixed_sessions()[0], &state, &forged),
        Err(GameplayError::Invalid(
            "progression source ledger digest mismatch"
        ))
    );
}

#[test]
fn external_world_context_fixed_registry_and_exact_history_are_required() {
    let state = terminal("authority", "full-flow-kit", "s1.direct", 0);
    let prior = ProgressionLedgerV1::from_base_loop(&state.ledger_before).unwrap();
    let foreign = mindwarp_gameplay_foundation::LoopWorldContextV1::ValidatedC3A(
        mindwarp_gameplay_foundation::C3AWorldReferenceV1 {
            schema_version: 1,
            reconstruction_id: [7; 32],
            input_id: "a".repeat(64),
            packet_id: "b".repeat(64),
        },
    );
    let mut foreign_state = state.clone();
    foreign_state.world_context = foreign;
    assert_eq!(
        apply_progression(&fixed_sessions()[0], &foreign_state, &prior),
        Err(GameplayError::Invalid(
            "GP2 V1 requires authored fixture context"
        ))
    );
    let ledger = progress(&state);
    let mut fabricated = ledger.clone();
    fabricated.processed_receipts[0].outcome_id = "s1.bypass".into();
    assert!(fabricated.validate_against(&state.ledger_after).is_err());
    let mut forged_digest = ledger.clone();
    forged_digest.processed_receipts[0].terminal_state_digest = [8; 32];
    assert!(forged_digest.validate_against(&state.ledger_after).is_err());
    let mut forged_lane = ledger.clone();
    forged_lane.capabilities[0].horizontal_scope = "fabricated-scope".into();
    assert!(forged_lane.validate_against(&state.ledger_after).is_err());
    let mut reordered = ledger.clone();
    reordered.processed_receipts[0].emitted_record_ids.reverse();
    assert!(reordered.validate_against(&state.ledger_after).is_err());
}

#[test]
fn rules_have_no_automatic_conversions_and_strategies_are_pairwise_incomparable() {
    assert_eq!(fixed_progression_rule_count(), 18);
    let expected_rules = fixed_sessions()
        .into_iter()
        .flat_map(|session| {
            session
                .outcomes
                .into_iter()
                .map(move |outcome| (session.session_id.clone(), outcome.outcome_id))
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        fixed_progression_rule_keys()
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>(),
        expected_rules
    );
    assert_eq!(conversion_rule_count(), 0);
    assert_eq!(reset_rule_count(), 0);
    let results = simulate_strategies().unwrap();
    assert_eq!(
        results.iter().map(|item| item.strategy).collect::<Vec<_>>(),
        vec![
            StrategyKindV1::StewardBuilder,
            StrategyKindV1::UrgencyDiscovery,
            StrategyKindV1::CautiousMapper
        ]
    );
    let decisions = results
        .iter()
        .map(|item| item.reachable_decisions.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        decisions[0],
        [
            "s1.bypass-next",
            "s2.relocate-next",
            "s3.charter-next",
            "s4.permanent-next",
            "s5.nightway-next"
        ]
        .into_iter()
        .map(String::from)
        .collect()
    );
    assert_eq!(
        decisions[1],
        [
            "s1.direct-next",
            "s2.harvest-next",
            "s3.force-next",
            "s4.rescue-next",
            "s5.dismantle-next"
        ]
        .into_iter()
        .map(String::from)
        .collect()
    );
    assert_eq!(
        decisions[2],
        [
            "s1.ration-next",
            "s2.retreat-next",
            "s3.alternate-next",
            "s4.long-next",
            "s5.nightway-next"
        ]
        .into_iter()
        .map(String::from)
        .collect()
    );
    assert_eq!(
        results[0].liabilities,
        [
            "both-sides.unrestricted-passage.foregone",
            "old-nest.occupancy.abandoned",
            "orchard.recovery.delayed",
            "signal.coordinate.missed",
            "travellers.unbounded-route.foregone"
        ]
        .into_iter()
        .map(String::from)
        .collect()
    );
    assert_eq!(
        results[1].liabilities,
        [
            "anchor.permanent-repair.not-completed",
            "colony.habitat-security.lost",
            "east-west.cooperation.damaged",
            "nest-caretaker.cooperation.withdrawn",
            "travellers.route.closed"
        ]
        .into_iter()
        .map(String::from)
        .collect()
    );
    assert_eq!(
        results[2].liabilities,
        [
            "caravan.delay.extended",
            "player.direct-assistance.foregone",
            "nonessential-travel.state.deferred",
            "supply.capacity.constrained",
            "travellers.unbounded-route.foregone"
        ]
        .into_iter()
        .map(String::from)
        .collect()
    );
    assert!(pairwise_incomparable(&results));
    assert!(results.iter().all(|item| !item.liabilities.is_empty()
        && !item.reachable_decisions.is_empty()
        && !item.active_affordances.is_empty()));
}

#[test]
fn public_capability_has_named_scope_and_no_magnitude_or_spend_surface() {
    let state = terminal("codec", "full-flow-kit", "s1.direct", 0);
    let ledger = progress(&state);
    let capability = &ledger.capabilities[0];
    let json = serde_json::to_string(capability).unwrap();
    assert!(json.contains("horizontal_scope"));
    for forbidden in ["amount", "balance", "level", "xp", "currency", "spend"] {
        assert!(!json.contains(forbidden));
    }
    let _: CapabilityRecordV1 = capability.clone();
}
