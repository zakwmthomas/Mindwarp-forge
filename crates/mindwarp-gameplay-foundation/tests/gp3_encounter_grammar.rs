use mindwarp_gameplay_foundation::*;

fn grammar() -> EncounterGrammarV1 {
    fixed_encounter_grammar().expect("fixed GP3 registry")
}

#[test]
fn five_fixed_situations_cover_exact_multi_domain_facets() {
    let grammar = grammar();
    assert_eq!(grammar.situations.len(), 5);
    let kinds = |index: usize| grammar.situations[index].domain_kinds();
    assert_eq!(
        kinds(0),
        vec![
            DomainKindV1::Environment,
            DomainKindV1::Creature,
            DomainKindV1::Society,
            DomainKindV1::Construction
        ]
    );
    assert_eq!(
        kinds(1),
        vec![
            DomainKindV1::Environment,
            DomainKindV1::Creature,
            DomainKindV1::Society,
            DomainKindV1::Construction
        ]
    );
    assert_eq!(
        kinds(2),
        vec![
            DomainKindV1::Environment,
            DomainKindV1::Society,
            DomainKindV1::Construction
        ]
    );
    assert_eq!(
        kinds(3),
        vec![
            DomainKindV1::Environment,
            DomainKindV1::Creature,
            DomainKindV1::Society,
            DomainKindV1::Anomaly,
            DomainKindV1::Construction
        ]
    );
    assert_eq!(
        kinds(4),
        vec![
            DomainKindV1::Environment,
            DomainKindV1::Creature,
            DomainKindV1::Society,
            DomainKindV1::Construction
        ]
    );
}

#[test]
fn evidence_risks_and_sessions_bind_exact_gp0_authority() {
    let grammar = grammar();
    let sessions = fixed_sessions();
    grammar.validate_against(&sessions).unwrap();
    assert_eq!(
        grammar
            .situations
            .iter()
            .map(|s| s.evidence_refs.len())
            .sum::<usize>(),
        11
    );
    assert_eq!(
        grammar
            .situations
            .iter()
            .map(|s| s.risk_refs.len())
            .sum::<usize>(),
        5
    );
    assert!(
        grammar
            .situations
            .iter()
            .flat_map(|s| &s.evidence_refs)
            .all(|e| e.evidence_class == EvidenceClass::AuthoredGameplayNonC3B)
    );
}

#[test]
fn distinct_approaches_cover_all_gp0_outcomes_and_keep_force_partial() {
    let grammar = grammar();
    assert_eq!(
        grammar
            .situations
            .iter()
            .map(|s| s.approaches.len())
            .sum::<usize>(),
        18
    );
    for situation in &grammar.situations {
        assert!(
            situation
                .approaches
                .iter()
                .filter(|a| a.kind != ApproachKindV1::Retreat)
                .count()
                >= 2
        );
        assert_eq!(
            situation
                .approaches
                .iter()
                .filter(|a| a.kind == ApproachKindV1::Retreat)
                .count(),
            1
        );
        assert!(
            situation
                .approaches
                .iter()
                .all(|a| a.approach_id != a.outcome_id)
        );
    }
    let force = &grammar.situations[2].approaches[1];
    assert_eq!(force.approach_id, "s3.approach.force");
    assert_eq!(force.kind, ApproachKindV1::ForcePartial);
    assert!(
        !resolve_outcome(&fixed_sessions()[2], force)
            .unwrap()
            .resolves_core_tension
    );
}

#[test]
fn tools_and_gp4_s4_seam_are_exact() {
    let grammar = grammar();
    let s4 = &grammar.situations[3];
    assert_eq!(s4.approaches[0].approach_id, "s4.approach.temporary");
    assert_eq!(
        s4.approaches[0].prepared_tool_id.as_deref(),
        Some("temporary-brace-kit")
    );
    assert_eq!(s4.approaches[2].approach_id, "s4.approach.long");
}

#[test]
fn consequence_refs_cover_every_gp0_element_exactly_once() {
    let grammar = grammar();
    for (session, situation) in fixed_sessions().iter().zip(&grammar.situations) {
        for approach in &situation.approaches {
            validate_consequence_coverage(session, approach).unwrap();
            let outcome = resolve_outcome(session, approach).unwrap();
            let expected = outcome.exact_mutations.len()
                + outcome.opportunity_costs.len()
                + outcome.memories.len()
                + outcome.grants.len()
                + 1;
            assert_eq!(approach.consequence_refs.len(), expected);
        }
    }
}

#[test]
fn optional_threat_is_exact_nonterminal_and_never_prerequisite() {
    let grammar = grammar();
    let sessions = fixed_sessions();
    for index in [1usize, 3, 4] {
        let situation = &grammar.situations[index];
        assert!(situation.threat_ref.is_some());
        for approach in &situation.approaches {
            assert!(approach.prerequisites.iter().all(|p| {
                !["predator", "wire-scavengers", "food-scavengers"].contains(&p.reference_id())
            }));
            assert!(
                compose_optional_threat(&sessions[index], situation, approach, false)
                    .unwrap()
                    .is_empty()
            );
            assert_eq!(
                compose_optional_threat(&sessions[index], situation, approach, true)
                    .unwrap()
                    .len(),
                1
            );
        }
    }
    assert!(grammar.situations[0].threat_ref.is_none());
    assert!(grammar.situations[2].threat_ref.is_none());
}

#[test]
fn s5_requires_exact_latest_admitted_s1_predecessor() {
    let grammar = grammar();
    let sessions = fixed_sessions();
    let s1 = &sessions[0];
    let s5 = &grammar.situations[4];
    let approach = &s5.approaches[0];
    let mut history = WorldHistoryV1::empty();
    let direct = replay_actions(
        s1,
        &[
            Action::ObserveCause,
            Action::MakeFittingTool,
            Action::CommitOutcome {
                outcome_id: "s1.direct".into(),
            },
        ],
    )
    .unwrap();
    history = history.append(s1, &direct).unwrap();
    validate_approach_context(s5, approach, &history, Some("s1.direct")).unwrap();
    assert!(validate_approach_context(s5, approach, &history, Some("s1.ration")).is_err());
    let retreat = replay_actions(
        s1,
        &[Action::Retreat {
            outcome_id: "s1.retreat".into(),
        }],
    )
    .unwrap();
    history = history.append(s1, &retreat).unwrap();
    assert!(validate_approach_context(s5, approach, &history, Some("s1.retreat")).is_err());
}

#[test]
fn strict_codecs_reject_unknown_noncanonical_oversized_and_authored_drift() {
    let grammar = grammar();
    let bytes = grammar.to_bytes().unwrap();
    assert_eq!(EncounterGrammarV1::from_bytes(&bytes).unwrap(), grammar);
    let mut unknown = bytes.clone();
    unknown.insert(1, b' ');
    assert!(EncounterGrammarV1::from_bytes(&unknown).is_err());
    assert!(EncounterGrammarV1::from_bytes(&vec![b' '; MAX_GRAMMAR_BYTES + 1]).is_err());
    let mut drift = grammar.clone();
    drift.situations[0].domain_facets.swap(0, 1);
    assert!(drift.validate_against(&fixed_sessions()).is_err());
}

#[test]
fn whole_situation_and_grammar_digests_are_pinned() {
    let grammar = grammar();
    assert_eq!(grammar.grammar_digest, FIXED_GRAMMAR_DIGEST);
    for (situation, expected) in grammar.situations.iter().zip(FIXED_SITUATION_DIGESTS) {
        assert_eq!(situation.situation_digest, expected);
    }
}

fn rejects(mutator: impl FnOnce(&mut EncounterGrammarV1)) {
    let mut value = grammar();
    mutator(&mut value);
    assert!(value.validate_against(&fixed_sessions()).is_err());
}

#[test]
fn hostile_registry_and_session_identity_matrix_rejects_every_drift() {
    rejects(|g| g.situations.clear());
    rejects(|g| g.situations.push(g.situations[0].clone()));
    rejects(|g| g.situations[1] = g.situations[0].clone());
    rejects(|g| g.situations.swap(0, 1));
    rejects(|g| g.situations[0].session_id = "gp0.foreign".into());
    rejects(|g| g.situations[0].session_digest.replace_range(0..1, "0"));
    rejects(|g| g.situations[0].situation_digest.replace_range(0..1, "0"));
    rejects(|g| g.grammar_digest.replace_range(0..1, "0"));
    rejects(|g| g.schema_version = 2);
}

#[test]
fn hostile_evidence_facet_and_risk_matrix_rejects_every_drift() {
    rejects(|g| {
        g.situations[0].evidence_refs.remove(0);
    });
    rejects(|g| {
        let x = g.situations[0].evidence_refs[0].clone();
        g.situations[0].evidence_refs.push(x);
    });
    rejects(|g| g.situations[0].evidence_refs[0].fact_id = "s2.exposure".into());
    rejects(|g| g.situations[0].evidence_refs[0].fact_id = "invented.fact".into());
    rejects(|g| g.situations[0].evidence_refs[0].kind = FactKind::Inference);
    rejects(|g| g.situations[0].evidence_refs[0].evidence_class = EvidenceClass::ObservedC3AOutput);
    rejects(|g| {
        g.situations[0].evidence_refs[0]
            .canonical_digest
            .replace_range(0..1, "0")
    });
    rejects(|g| match &mut g.situations[0].domain_facets[0] {
        DomainFacetV1::Environment {
            supporting_evidence_ids,
            ..
        } => supporting_evidence_ids[0] = "s2.exposure".into(),
        _ => unreachable!(),
    });
    rejects(|g| {
        g.situations[0].domain_facets.remove(0);
    });
    rejects(|g| {
        g.situations[0].risk_refs.clear();
    });
    rejects(|g| {
        let x = g.situations[0].risk_refs[0].clone();
        g.situations[0].risk_refs.push(x);
    });
    rejects(|g| g.situations[0].risk_refs[0].risk_id = "storm-arrival".into());
    rejects(|g| {
        g.situations[0].risk_refs[0]
            .canonical_digest
            .replace_range(0..1, "0")
    });
}

#[test]
fn hostile_approach_step_prerequisite_and_explanation_matrix_rejects_drift() {
    rejects(|g| {
        g.situations[1].approaches.remove(1);
    });
    rejects(|g| g.situations[0].approaches[0].approach_id = "s1.approach.invented".into());
    rejects(|g| g.situations[0].approaches[0].outcome_id = "s1.bypass".into());
    rejects(|g| g.situations[0].approaches[0].kind = ApproachKindV1::Retreat);
    rejects(|g| g.situations[0].approaches[0].prepared_tool_id = Some("wrong-tool".into()));
    rejects(|g| g.situations[0].approaches[0].intervention_steps[0].kind = StepKindV1::Coerce);
    rejects(|g| {
        g.situations[0].approaches[0].intervention_steps[0].subject_ids[0] = "foreign".into()
    });
    rejects(|g| g.situations[0].approaches[0].intervention_steps.swap(0, 1));
    rejects(|g| {
        g.situations[0].approaches[0].prerequisites.remove(0);
    });
    rejects(|g| {
        g.situations[0].approaches[0].prerequisites[0] = ApproachPrerequisiteV1::AuthoredState {
            reference_id: "invented".into(),
            expected_digest: None,
        }
    });
    rejects(|g| {
        if let ApproachPrerequisiteV1::ObservedFact {
            expected_digest, ..
        } = &mut g.situations[0].approaches[0].prerequisites[0]
        {
            expected_digest.as_mut().unwrap().replace_range(0..1, "0")
        }
    });
    rejects(|g| {
        g.situations[0].approaches[0]
            .causal_explanation
            .admitted_evidence_ids
            .remove(0);
    });
    rejects(|g| {
        g.situations[0].approaches[0]
            .causal_explanation
            .intervention_step_ids
            .swap(0, 1)
    });
    rejects(|g| {
        g.situations[0].approaches[0]
            .causal_explanation
            .consequence_ref_ids
            .remove(0);
    });
    rejects(|g| {
        g.situations[0].approaches[0]
            .causal_explanation
            .risk_disposition_ids[0] = "foreign".into()
    });
    rejects(|g| {
        g.situations[0].approaches[0].causal_explanation.explanation =
            "The player wants it, so it works".into()
    });
    rejects(|g| {
        g.situations[0].approaches[0].causal_explanation.limitation =
            "Nothing remains unresolved".into()
    });
}

#[test]
fn hostile_risk_disposition_and_force_matrix_rejects_drift() {
    rejects(|g| g.situations[0].approaches[0].risk_dispositions.clear());
    rejects(|g| {
        let x = g.situations[0].approaches[0].risk_dispositions[0].clone();
        g.situations[0].approaches[0].risk_dispositions.push(x);
    });
    rejects(|g| {
        g.situations[0].approaches[0].risk_dispositions[0].risk_id = "storm-arrival".into()
    });
    rejects(|g| {
        g.situations[1].approaches[0].risk_dispositions[0].disposition =
            RiskDispositionKindV1::Mitigated
    });
    rejects(|g| g.situations[2].approaches[0].kind = ApproachKindV1::ForcePartial);
    rejects(|g| g.situations[2].approaches[1].outcome_id = "s3.charter".into());
    rejects(|g| {
        g.situations[2].approaches[1].causal_explanation.limitation =
            "Force completely resolves ownership".into()
    });
}

#[test]
fn hostile_consequence_exact_once_matrix_rejects_drift() {
    rejects(|g| {
        g.situations[0].approaches[0].consequence_refs.remove(0);
    });
    rejects(|g| {
        let x = g.situations[0].approaches[0].consequence_refs[0].clone();
        g.situations[0].approaches[0].consequence_refs.push(x);
    });
    rejects(|g| g.situations[0].approaches[0].consequence_refs.swap(0, 1));
    rejects(|g| g.situations[0].approaches[0].consequence_refs[0].kind = ConsequenceKindV1::Memory);
    rejects(|g| g.situations[0].approaches[0].consequence_refs[0].ordinal = 9);
    rejects(|g| {
        g.situations[0].approaches[0].consequence_refs[0]
            .canonical_digest
            .replace_range(0..1, "0")
    });
    let grammar = grammar();
    let session = &fixed_sessions()[0];
    let approach = &grammar.situations[0].approaches[0];
    assert!(
        resolve_consequence(
            session,
            approach,
            &ConsequenceRefV1 {
                kind: ConsequenceKindV1::Mutation,
                ordinal: 99,
                canonical_digest: "0".repeat(64)
            }
        )
        .is_err()
    );
    let mut crafted_session = session.clone();
    crafted_session.outcomes[0].resolves_core_tension = false;
    assert!(resolve_outcome(&crafted_session, approach).is_err());
    assert!(
        resolve_consequence(&crafted_session, approach, &approach.consequence_refs[0]).is_err()
    );
}

#[test]
fn hostile_threat_matrix_rejects_presence_identity_elements_and_authority_drift() {
    rejects(|g| g.situations[0].threat_ref = g.situations[1].threat_ref.clone());
    rejects(|g| g.situations[2].threat_ref = g.situations[3].threat_ref.clone());
    rejects(|g| g.situations[1].threat_ref = None);
    rejects(|g| g.situations[1].threat_ref.as_mut().unwrap().threat_id = "foreign".into());
    rejects(|g| {
        g.situations[1]
            .threat_ref
            .as_mut()
            .unwrap()
            .canonical_digest
            .replace_range(0..1, "0")
    });
    rejects(|g| {
        g.situations[1]
            .threat_ref
            .as_mut()
            .unwrap()
            .contribution_refs[0]
            .ordinal = 1
    });
    rejects(|g| {
        g.situations[1]
            .threat_ref
            .as_mut()
            .unwrap()
            .contribution_refs[0]
            .canonical_digest
            .replace_range(0..1, "0")
    });
    rejects(|g| g.situations[1].threat_ref.as_mut().unwrap().nonterminal = false);
    rejects(|g| {
        g.situations[1].approaches[0]
            .prerequisites
            .push(ApproachPrerequisiteV1::AuthoredState {
                reference_id: "predator".into(),
                expected_digest: None,
            })
    });
    rejects(|g| g.situations[1].approaches[0].outcome_id = "predator".into());
}

#[test]
fn s5_hostile_history_matrix_rejects_missing_stale_reordered_foreign_and_fabricated() {
    let grammar = grammar();
    let s5 = &grammar.situations[4];
    let approach = &s5.approaches[0];
    let sessions = fixed_sessions();
    let s1 = &sessions[0];
    assert!(
        validate_approach_context(s5, approach, &WorldHistoryV1::empty(), Some("s1.direct"))
            .is_err()
    );
    let direct = replay_actions(
        s1,
        &[
            Action::ObserveCause,
            Action::MakeFittingTool,
            Action::CommitOutcome {
                outcome_id: "s1.direct".into(),
            },
        ],
    )
    .unwrap();
    let ration = replay_actions(
        s1,
        &[
            Action::ObserveCause,
            Action::MakeFittingTool,
            Action::CommitOutcome {
                outcome_id: "s1.ration".into(),
            },
        ],
    )
    .unwrap();
    let mut history = WorldHistoryV1::empty().append(s1, &direct).unwrap();
    history = history.append(s1, &ration).unwrap();
    assert!(validate_approach_context(s5, approach, &history, Some("s1.direct")).is_err());
    let mut reordered = history.clone();
    reordered.events.swap(0, 1);
    assert!(validate_approach_context(s5, approach, &reordered, Some("s1.ration")).is_err());
    let mut foreign = history.clone();
    foreign.events[1].session_id = "gp0.s2.storm-nest".into();
    assert!(validate_approach_context(s5, approach, &foreign, Some("s1.ration")).is_err());
    let mut fabricated = history.clone();
    fabricated.events[1].exact_mutations.clear();
    assert!(validate_approach_context(s5, approach, &fabricated, Some("s1.ration")).is_err());
}

#[test]
fn strict_codec_hostile_matrix_rejects_real_structural_and_resource_attacks() {
    let grammar = grammar();
    let bytes = grammar.to_bytes().unwrap();
    let text = String::from_utf8(bytes.clone()).unwrap();
    let unknown = text.replacen('{', "{\"unknown\":true,", 1);
    assert!(EncounterGrammarV1::from_bytes(unknown.as_bytes()).is_err());
    let duplicate = text.replacen(
        "\"schema_version\":1",
        "\"schema_version\":1,\"schema_version\":1",
        1,
    );
    assert!(EncounterGrammarV1::from_bytes(duplicate.as_bytes()).is_err());
    let mut trailing = bytes.clone();
    trailing.extend_from_slice(b" ");
    assert!(EncounterGrammarV1::from_bytes(&trailing).is_err());
    let reordered =
        serde_json::to_vec(&serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()).unwrap();
    assert!(EncounterGrammarV1::from_bytes(&reordered).is_err());
    let situation = &grammar.situations[0];
    let situation_bytes = situation.to_bytes().unwrap();
    assert_eq!(
        EncounterSituationV1::from_bytes(&situation_bytes).unwrap(),
        *situation
    );
    assert!(EncounterSituationV1::from_bytes(&vec![b' '; MAX_SITUATION_BYTES + 1]).is_err());
    let situation_text = String::from_utf8(situation_bytes.clone()).unwrap();
    let situation_unknown = situation_text.replacen('{', "{\"unknown\":true,", 1);
    assert!(EncounterSituationV1::from_bytes(situation_unknown.as_bytes()).is_err());
    let situation_duplicate = situation_text.replacen(
        "\"schema_version\":1",
        "\"schema_version\":1,\"schema_version\":1",
        1,
    );
    assert!(EncounterSituationV1::from_bytes(situation_duplicate.as_bytes()).is_err());
    let mut situation_trailing = situation_bytes.clone();
    situation_trailing.push(b' ');
    assert!(EncounterSituationV1::from_bytes(&situation_trailing).is_err());
    let situation_reordered =
        serde_json::to_vec(&serde_json::from_slice::<serde_json::Value>(&situation_bytes).unwrap())
            .unwrap();
    assert!(EncounterSituationV1::from_bytes(&situation_reordered).is_err());
    let mut long_nested = situation.clone();
    long_nested.approaches[0].causal_explanation.explanation = "x".repeat(1_025);
    let long_nested_bytes = serde_json::to_vec(&long_nested).unwrap();
    assert!(long_nested_bytes.len() < MAX_SITUATION_BYTES);
    assert!(EncounterSituationV1::from_bytes(&long_nested_bytes).is_err());
    let mut wide_nested = situation.clone();
    let facet = wide_nested.domain_facets[0].clone();
    wide_nested
        .domain_facets
        .extend(std::iter::repeat_n(facet, 33));
    let wide_nested_bytes = serde_json::to_vec(&wide_nested).unwrap();
    assert!(wide_nested_bytes.len() < MAX_SITUATION_BYTES);
    assert!(EncounterSituationV1::from_bytes(&wide_nested_bytes).is_err());
    let mut duplicate_nested = situation.clone();
    duplicate_nested
        .evidence_refs
        .push(duplicate_nested.evidence_refs[0].clone());
    assert!(
        EncounterSituationV1::from_bytes(&serde_json::to_vec(&duplicate_nested).unwrap()).is_err()
    );
    rejects(|g| g.situations[0].situation_id = "BAD ID".into());
    rejects(|g| g.situations[0].schema_version = 2);
    rejects(|g| g.situations[0].approaches[0].causal_explanation.explanation = "x".repeat(1_025));
    rejects(|g| {
        let x = g.situations[0].domain_facets[0].clone();
        g.situations[0]
            .domain_facets
            .extend(std::iter::repeat_n(x, 33));
    });
}

#[test]
fn fixed_tools_session_digests_and_forbidden_surface_are_exact() {
    let grammar = grammar();
    let tools = grammar
        .situations
        .iter()
        .flat_map(|s| &s.approaches)
        .filter_map(|a| a.prepared_tool_id.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(
        tools,
        vec![
            "full-flow-kit",
            "colony-safe-kit",
            "timed-controller",
            "sheltered-nest-kit",
            "insulated-specimen-kit",
            "joint-ledger-kit",
            "urgent-crossing-kit",
            "essential-path-kit",
            "temporary-brace-kit",
            "permanent-anchor-kit",
            "north-route-kit",
            "nightway-charter-kit",
            "passage-dismantling-kit"
        ]
    );
    assert_eq!(
        grammar
            .situations
            .iter()
            .map(|s| s.session_digest.as_str())
            .collect::<Vec<_>>(),
        vec![
            "e7726be13efcf68e875e538103252aa46b3fd6c9e4ef86af95fc4622c160c274",
            "84c95d330549ba4d48ee2d47320558a29ec47dc5634cb450c383f9b5bd8bb0be",
            "3084d7d21fdb248cc3e83082a77164528d550ac2a15a6e51174cf4505e01dc68",
            "b6428f7febdfab0560b975f02b47bb3dcbd7e940a4a88fd65028aa6c685a4033",
            "a301a3d5cbfb73951cb4e6e0b453a1971ebc05fc351ab09563d7950d9820ab6b"
        ]
    );
    let source = include_str!("../src/encounter_grammar.rs");
    for forbidden in [
        "ProgressionLedgerV1",
        "apply_progression",
        "rand::",
        "weight:",
        "generation",
        "C3BWorld",
        "Greenfield",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden GP3 surface: {forbidden}"
        );
    }
}
