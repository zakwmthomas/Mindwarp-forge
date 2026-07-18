use super::*;

pub fn fixed_sessions() -> Vec<SessionRecordV1> {
    vec![
        session_one(),
        session_two(),
        session_three(),
        session_four(),
        session_five(),
    ]
}

pub fn fixed_concept() -> GameplayConceptRecordV1 {
    GameplayConceptRecordV1 {
        schema_version: CONTRACT_VERSION,
        concept_id: "mindwarp.gp0.causal-explorer-maker".into(),
        primary_fantasy: Fantasy::CausalExplorerMaker,
        player_promise: "Understand a strange local system, make a fitting intervention, and leave consequences the world remembers.".into(),
        non_goals: vec![
            NonGoal::CombatResolvesCoreTension,
            NonGoal::CurrencyShapedProgression,
            NonGoal::AuthoredFactsClaimC3AOrC3BProof,
            NonGoal::RuntimeOrEngineAuthority,
            NonGoal::NetworkOrMonetizationDependency,
        ],
        assumptions: vec![
            ("session_minutes_35_to_60".into(), AssumptionStatus::ReversibleProposed),
            ("stable_stop_minutes_10_to_15".into(), AssumptionStatus::ReversibleProposed),
            ("fixed_hub_or_vessel".into(), AssumptionStatus::ReversibleProposed),
            ("failure_is_repairable".into(), AssumptionStatus::ReversibleProposed),
        ],
    }
}

fn session_one() -> SessionRecordV1 {
    session(
        "gp0.s1.colony-conduit",
        "The Colony Conduit",
        "A failing conduit threatens clinic and fire water while pump vibration distresses a resident colony.",
        "Urgent full flow conflicts with preserving the colony and the greenhouse spare.",
        risk(
            "conduit-failure",
            "The conduit is nearing failure.",
            "red pressure bands pulse",
            "the pipe knocks in a shortening rhythm",
        ),
        vec![
            observed(
                "s1.flow-loss",
                "Flow loss is localized at the colony section.",
            ),
            inferred(
                "s1.colony-distress",
                "Pump restart would distress the colony, but observation does not choose a preferred solution.",
            ),
        ],
        None,
        vec![
            outcome(
                "s1.direct",
                OutcomeTrigger::CausalIntervention,
                vec![
                    m("clinic-water", "availability", "immediate"),
                    m("fire-water", "availability", "immediate"),
                    m("colony", "location", "displaced-to-spillway"),
                    m("pump", "flow", "full"),
                ],
                vec![m("colony", "habitat-security", "lost")],
                vec![memory(
                    "keeper-mara",
                    "The player restored urgent clinic and fire water and displaced the colony to the spillway.",
                )],
                vec![TypedGrant::Service {
                    provider_id: "clinic".into(),
                    proposition: "Immediate clinic water service is restored.".into(),
                }],
                decision(
                    "s1.direct-next",
                    "Choose whether the next habitat work restores the spillway colony or secures the orchard.",
                ),
                true,
                true,
            ),
            outcome(
                "s1.bypass",
                OutcomeTrigger::CausalIntervention,
                vec![
                    m("pump", "flow", "partial-bypass"),
                    m("water", "supply", "restricted-stable"),
                    m("colony", "state", "preserved"),
                    m("greenhouse-spare", "availability", "unavailable"),
                ],
                vec![m("orchard", "recovery", "delayed")],
                vec![memory(
                    "keeper-mara",
                    "The player protected both the water supply and the colony by consuming the greenhouse spare.",
                )],
                vec![TypedGrant::Permission {
                    grantor_id: "keeper-mara".into(),
                    proposition:
                        "Keeper Mara signs permission to install the colony-section bypass.".into(),
                }],
                decision(
                    "s1.bypass-next",
                    "Choose whether the next fabricated conduit serves the greenhouse or completes orchard recovery.",
                ),
                true,
                true,
            ),
            outcome(
                "s1.ration",
                OutcomeTrigger::CausalIntervention,
                vec![
                    m("water", "delivery", "timed-windows"),
                    m("colony", "light-cycle", "synchronized"),
                    m("conduit", "state", "failing-contained"),
                ],
                vec![m("supply", "capacity", "constrained")],
                vec![memory(
                    "keeper-mara",
                    "The player retained the colony and synchronized ration windows with its light cycle.",
                )],
                vec![TypedGrant::Service {
                    provider_id: "keeper-mara".into(),
                    proposition:
                        "Keeper Mara schedules named water windows for the clinic and fire crews."
                            .into(),
                }],
                decision(
                    "s1.ration-next",
                    "Choose whether to fabricate a bypass before the constrained supply reaches its next limit.",
                ),
                true,
                true,
            ),
            outcome(
                "s1.retreat",
                OutcomeTrigger::Retreat,
                vec![
                    m("water", "delivery", "emergency-ration"),
                    m("conduit", "repair", "still-required"),
                ],
                vec![m("orchard", "stress-stage", "plus-one")],
                vec![memory(
                    "keeper-mara",
                    "The player withdrew, leaving emergency rationing active and the repair unfinished.",
                )],
                vec![TypedGrant::Service {
                    provider_id: "keeper-mara".into(),
                    proposition:
                        "Keeper Mara maintains emergency ration service until another attempt."
                            .into(),
                }],
                decision(
                    "s1.retreat-next",
                    "Choose whether to return before orchard stress advances again.",
                ),
                false,
                false,
            ),
        ],
        vec![],
        vec![],
        vec![],
    )
}

fn session_two() -> SessionRecordV1 {
    session(
        "gp0.s2.storm-nest",
        "The Storm Nest",
        "An exposed nest, an approaching predator, and a conductive crystal ridge make an authored storm dangerous.",
        "Defending the brood buys time but cannot replace relocation and stabilization.",
        risk("storm-arrival", "The storm arrives after two major actions.", "violet arcs advance along the ridge", "the warning whistle changes to a rapid triple note"),
        vec![
            observed("s2.exposure", "The nest is exposed and the predator is approaching."),
            inferred("s2.crystal-hazard", "Authored gameplay logic marks the named crystal ridge unsafe during the named storm; this is explicitly not C3B scientific proof."),
        ],
        Some(threat("predator", vec![m("predator", "state", "diverted")], "The brood remains exposed until relocation and stabilization.")),
        vec![
            outcome(
                "s2.relocate", OutcomeTrigger::CausalIntervention,
                vec![m("nest", "location", "sheltered"), m("brood", "state", "stabilized"), m("player-whistle", "recognition", "learned")],
                vec![m("old-nest", "occupancy", "abandoned")],
                vec![memory("nest-caretaker", "The player defended the brood without harvesting it and relocated it safely.")],
                vec![TypedGrant::Knowledge { knower_id: "player".into(), proposition: "The caretaker recognizes the player whistle and names the storm-crystal ridge hazard as authored gameplay knowledge, not C3B proof.".into() }],
                decision("s2.relocate-next", "Choose where to begin the second nesting-route survey."), true, true,
            ),
            outcome(
                "s2.harvest", OutcomeTrigger::CausalIntervention,
                vec![m("crystal-specimen", "custody", "player"), m("brood", "location", "displaced")],
                vec![m("nest-caretaker", "cooperation", "withdrawn")],
                vec![memory("nest-caretaker", "The player took the named ridge specimen and displaced the brood.")],
                vec![TypedGrant::Knowledge { knower_id: "player".into(), proposition: "The named ridge specimen is recorded as an authored object with no scientific authority.".into() }],
                decision("s2.harvest-next", "Choose whether to repair the displaced brood shelter before the next storm."), true, true,
            ),
            outcome(
                "s2.retreat", OutcomeTrigger::Retreat,
                vec![m("nest-caretaker", "dispatch", "sent"), m("nest-caretaker", "arrival", "before-storm")],
                vec![m("player", "direct-assistance", "foregone")],
                vec![memory("nest-caretaker", "The player withdrew after dispatching the caretaker, who arrived before the storm.")],
                vec![TypedGrant::Service { provider_id: "nest-caretaker".into(), proposition: "The caretaker performs emergency nest stabilization before the storm.".into() }],
                decision("s2.retreat-next", "Choose whether to return after the storm to inspect the exposed route."), false, false,
            ),
        ], vec![], vec![], vec![],
    )
}

fn session_three() -> SessionRecordV1 {
    session(
        "gp0.s3.memory-gate",
        "The Memory Gate",
        "A sealed gate divides an east ancestral claim from a west recent repair claim.",
        "Essential passage must be restored without pretending contradictory ownership testimony agrees.",
        risk(
            "channel-harm",
            "Opening the gate fully harms the west channel.",
            "west-channel markers turn amber",
            "a low gate resonance signals rising channel strain",
        ),
        vec![
            observed(
                "s3.ledger",
                "The old ledger records joint construction and the ownership mark is recently forged.",
            ),
            inferred(
                "s3.testimony",
                "East and west testimony remains contradictory and must be preserved separately.",
            ),
        ],
        None,
        vec![
            outcome(
                "s3.charter",
                OutcomeTrigger::CausalIntervention,
                vec![
                    m("passage-charter", "east-window", "evening"),
                    m("passage-charter", "west-window", "dawn"),
                    m("gate", "monitoring", "joint"),
                ],
                vec![m("both-sides", "unrestricted-passage", "foregone")],
                vec![
                    memory(
                        "east-keeper",
                        "The player preserved the east ancestral claim while accepting timed passage.",
                    ),
                    memory(
                        "west-keeper",
                        "The player preserved the west repair claim while accepting timed passage.",
                    ),
                ],
                vec![
                    TypedGrant::Right {
                        holder_id: "east-travellers".into(),
                        proposition: "Named passage is permitted during the evening window.".into(),
                    },
                    TypedGrant::Right {
                        holder_id: "west-travellers".into(),
                        proposition: "Named passage is permitted during the dawn window.".into(),
                    },
                ],
                decision(
                    "s3.charter-next",
                    "Choose how to adjudicate the first recorded charter breach.",
                ),
                true,
                true,
            ),
            outcome(
                "s3.force",
                OutcomeTrigger::CausalIntervention,
                vec![
                    m("gate", "crossings", "one-urgent-crossing"),
                    m("gate", "state", "resealed"),
                    m("ownership", "state", "unresolved"),
                ],
                vec![m("east-west", "cooperation", "damaged")],
                vec![
                    memory(
                        "east-keeper",
                        "The player coerced one crossing without resolving ownership.",
                    ),
                    memory(
                        "west-keeper",
                        "The player coerced one crossing without resolving ownership.",
                    ),
                ],
                vec![TypedGrant::Service {
                    provider_id: "gate".into(),
                    proposition: "One essential crossing is completed before resealing.".into(),
                }],
                decision(
                    "s3.force-next",
                    "Choose whether to return with evidence sufficient for a voluntary charter.",
                ),
                false,
                true,
            ),
            outcome(
                "s3.alternate",
                OutcomeTrigger::CausalIntervention,
                vec![
                    m("alternate-path", "travel", "essential-only"),
                    m("gate", "state", "sealed"),
                ],
                vec![m("nonessential-travel", "state", "deferred")],
                vec![
                    memory(
                        "east-keeper",
                        "The player preserved the sealed gate and enabled essential travel only.",
                    ),
                    memory(
                        "west-keeper",
                        "The player preserved the sealed gate and enabled essential travel only.",
                    ),
                ],
                vec![TypedGrant::Right {
                    holder_id: "essential-travellers".into(),
                    proposition: "Named essential journeys may use the alternate path.".into(),
                }],
                decision(
                    "s3.alternate-next",
                    "Choose whether to investigate the forged mark or negotiate timed passage.",
                ),
                true,
                true,
            ),
            outcome(
                "s3.retreat",
                OutcomeTrigger::Retreat,
                vec![
                    m("gate", "state", "sealed"),
                    m("essential-travel", "state", "waiting"),
                ],
                vec![m("channel", "inspection", "delayed")],
                vec![
                    memory(
                        "east-keeper",
                        "The player withdrew without altering either claim.",
                    ),
                    memory(
                        "west-keeper",
                        "The player withdrew without altering either claim.",
                    ),
                ],
                vec![TypedGrant::Service {
                    provider_id: "gate-watch".into(),
                    proposition:
                        "The gate watch holds essential travellers while evidence is reviewed."
                            .into(),
                }],
                decision(
                    "s3.retreat-next",
                    "Choose which independent evidence to obtain before returning.",
                ),
                false,
                false,
            ),
        ],
        vec![],
        vec![],
        vec![],
    )
}

fn session_four() -> SessionRecordV1 {
    session(
        "gp0.s4.signal-anchor",
        "The Signal Anchor",
        "An authored event broke an anchor, stranding surveyor Iven while a caravan waits and a signal window closes.",
        "Permanent repair takes four actions, but a temporary brace and rescue take two before the three-action signal window closes.",
        risk("anchor-collapse", "The failed anchor can collapse under load.", "fracture lines brighten under weight", "the anchor emits a descending two-pulse vibration"),
        vec![
            observed("s4.timing", "The signal window is three actions, permanent repair is four, and a temporary brace is two."),
            observed("s4.wire-scavengers", "Wire scavengers block the anchor work area while Iven remains stranded."),
            inferred("s4.event", "The anchor failure is an authored event and makes no C3B physical claim."),
        ],
        Some(threat("wire-scavengers", vec![m("work-area", "state", "safe")], "Clearing the work area does not repair the anchor or rescue Iven.")),
        vec![
            outcome(
                "s4.temporary-rescue", OutcomeTrigger::CausalIntervention,
                vec![m("anchor", "brace", "temporary"), m("crossing", "count", "one"), m("iven", "location", "returned"), m("signal", "coordinate", "recorded"), m("caravan", "state", "delayed"), m("brace", "state-at-return", "expired")],
                vec![m("anchor", "permanent-repair", "not-completed")],
                vec![memory("iven", "The player chose rescue and signal evidence over permanent repair.")],
                vec![TypedGrant::Knowledge { knower_id: "player".into(), proposition: "Iven's recorder provides the named signal coordinate as authored session evidence.".into() }],
                decision("s4.rescue-next", "Choose whether to pursue the signal or return with a permanent repair crew."), true, true,
            ),
            outcome(
                "s4.permanent", OutcomeTrigger::CausalIntervention,
                vec![m("anchor", "repair", "permanent"), m("caravan", "state", "moving"), m("signal", "state", "expired")],
                vec![m("signal", "coordinate", "missed")],
                vec![memory("caravan-leader", "The player restored the permanent crossing and missed the signal window.")],
                vec![TypedGrant::Service { provider_id: "caravan".into(), proposition: "The reopened crossing resumes named caravan service.".into() }],
                decision("s4.permanent-next", "Choose whether to search for Iven by the named long route."), true, true,
            ),
            outcome(
                "s4.long-route", OutcomeTrigger::CausalIntervention,
                vec![m("iven", "location", "returned-by-north-detour"), m("north-detour", "state", "named"), m("signal", "state", "expired")],
                vec![m("caravan", "delay", "extended")],
                vec![memory("iven", "The player rescued Iven by the north detour after the signal expired.")],
                vec![TypedGrant::Knowledge { knower_id: "player".into(), proposition: "The north detour is recorded as a named rescue route.".into() }],
                decision("s4.long-next", "Choose whether the next crew repairs the anchor or maps the detour."), true, true,
            ),
            outcome(
                "s4.retreat", OutcomeTrigger::Retreat,
                vec![m("caravan", "schedule", "changed"), m("signal", "state", "expired-visible")],
                vec![m("iven", "rescue", "delayed")],
                vec![memory("caravan-leader", "The player withdrew; the caravan changed schedule and watched the signal expire.")],
                vec![TypedGrant::Service { provider_id: "caravan-leader".into(), proposition: "The caravan leader schedules a later rescue watch.".into() }],
                decision("s4.retreat-next", "Choose whether to return first for Iven or for the permanent crossing."), false, false,
            ),
        ], vec![], vec![], vec![],
    )
}

fn session_five() -> SessionRecordV1 {
    session(
        "gp0.s5.afterlight",
        "Afterlight",
        "Travellers follow the colony route and discarded food attracts scavengers after an admitted conduit outcome.",
        "A passage can remain open only if its cleanup and habitat boundary become remembered obligations.",
        risk("buffer-violation", "Scavengers and travellers are crossing the habitat buffer.", "boundary lanterns show broken violet segments", "Mara's whistle repeats the habitat-warning cadence"),
        vec![
            observed("s5.history", "Afterlight begins only after the direct, bypass, or ration conduit outcomes; retreat has no trigger."),
            inferred("s5.relocation", "Relocation means displaced spillway habitat after direct, preserved partial-flow habitat after bypass, and synchronized retained habitat after ration."),
        ],
        Some(threat("food-scavengers", vec![m("scavengers", "state", "repelled")], "Repelling scavengers does not charter or dismantle the passage.")),
        vec![
            outcome(
                "s5.nightway", OutcomeTrigger::CausalIntervention,
                vec![m("nightway-boundary", "state", "marked"), m("cleanup", "responsibility", "assigned"), m("habitat-buffer", "state", "protected"), m("passage", "state", "registered")],
                vec![m("travellers", "unbounded-route", "foregone")],
                vec![memory("travellers", "The player founded the charter and assigned cleanup around the habitat buffer.")],
                vec![TypedGrant::Right { holder_id: "travellers".into(), proposition: "Named Nightway passage is permitted while the habitat buffer and cleanup obligations are kept.".into() }, TypedGrant::Service { provider_id: "travellers".into(), proposition: "Named cleanup service is assigned after each passage.".into() }],
                decision("s5.nightway-next", "Choose how to adjudicate the first recorded habitat-buffer violation."), true, true,
            ),
            outcome(
                "s5.dismantle", OutcomeTrigger::CausalIntervention,
                vec![m("passage", "state", "dismantled"), m("history", "event", "dismantling-appended")],
                vec![m("travellers", "route", "closed")],
                vec![memory("keeper-mara", "The player dismantled the passage to preserve the habitat buffer."), memory("travellers", "The player closed the route after its habitat obligations failed.")],
                vec![TypedGrant::Service { provider_id: "keeper-mara".into(), proposition: "Keeper Mara marks an alternate journey for essential travellers.".into() }],
                decision("s5.dismantle-next", "Choose whether a future charter can meet the habitat obligations."), true, true,
            ),
            outcome(
                "s5.retreat", OutcomeTrigger::Retreat,
                vec![m("scavengers", "state", "active"), m("passage", "state", "unregistered")],
                vec![m("habitat-buffer", "protection", "delayed")],
                vec![memory("keeper-mara", "The player withdrew after Afterlight began, leaving the passage unresolved.")],
                vec![TypedGrant::Service { provider_id: "keeper-mara".into(), proposition: "Keeper Mara maintains a temporary habitat watch.".into() }],
                decision("s5.retreat-next", "Choose whether to return to charter or dismantle the passage."), false, true,
            ),
        ],
        vec!["s1.direct".into(), "s1.bypass".into(), "s1.ration".into()],
        vec!["s1.retreat".into()],
        vec![
            PredecessorInterpretation {
                outcome_id: "s1.direct".into(),
                exact_mutations: vec![m("colony", "location", "displaced-to-spillway"), m("pump", "flow", "full")],
                memories: vec![memory("keeper-mara", "The player restored full water flow and displaced the colony before founding the later passage.")],
            },
            PredecessorInterpretation {
                outcome_id: "s1.bypass".into(),
                exact_mutations: vec![m("colony", "state", "preserved"), m("pump", "flow", "partial-bypass"), m("greenhouse-spare", "availability", "unavailable")],
                memories: vec![memory("keeper-mara", "The player protected both water and colony before founding the later passage.")],
            },
            PredecessorInterpretation {
                outcome_id: "s1.ration".into(),
                exact_mutations: vec![m("colony", "state", "retained"), m("colony", "light-cycle", "synchronized"), m("water", "delivery", "timed-windows")],
                memories: vec![memory("keeper-mara", "The player retained the colony through synchronized water windows before founding the later passage.")],
            },
        ],
    )
}

fn session(
    session_id: &str,
    title: &str,
    player_problem: &str,
    core_tension: &str,
    risk: RiskCommunication,
    facts: Vec<SessionFact>,
    threat_contribution: Option<ThreatContribution>,
    outcomes: Vec<OutcomeRecordV1>,
    admitted_predecessor_outcomes: Vec<String>,
    rejected_predecessor_outcomes: Vec<String>,
    predecessor_interpretations: Vec<PredecessorInterpretation>,
) -> SessionRecordV1 {
    SessionRecordV1 {
        schema_version: CONTRACT_VERSION,
        session_id: session_id.into(),
        title: title.into(),
        player_problem: player_problem.into(),
        core_tension: core_tension.into(),
        risks: vec![risk],
        facts,
        threat_contribution,
        outcomes,
        admitted_predecessor_outcomes,
        rejected_predecessor_outcomes,
        predecessor_interpretations,
    }
}

fn risk(id: &str, meaning: &str, visual: &str, other: &str) -> RiskCommunication {
    RiskCommunication {
        risk_id: id.into(),
        meaning: meaning.into(),
        visual_cue: visual.into(),
        audio_or_haptic_cue: other.into(),
    }
}

fn threat(id: &str, exact_mutations: Vec<TypedMutation>, limitation: &str) -> ThreatContribution {
    ThreatContribution {
        threat_id: id.into(),
        exact_mutations,
        limitation: limitation.into(),
    }
}

fn observed(id: &str, proposition: &str) -> SessionFact {
    SessionFact {
        fact_id: id.into(),
        kind: FactKind::Observation,
        proposition: proposition.into(),
        evidence_class: EvidenceClass::AuthoredGameplayNonC3B,
        world_reference: None,
    }
}

fn inferred(id: &str, proposition: &str) -> SessionFact {
    SessionFact {
        fact_id: id.into(),
        kind: FactKind::Inference,
        proposition: proposition.into(),
        evidence_class: EvidenceClass::AuthoredGameplayNonC3B,
        world_reference: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn outcome(
    id: &str,
    trigger: OutcomeTrigger,
    exact_mutations: Vec<TypedMutation>,
    opportunity_costs: Vec<TypedMutation>,
    memories: Vec<MemoryProposition>,
    grants: Vec<TypedGrant>,
    next_decision: NamedDecision,
    resolves: bool,
    afterlight: bool,
) -> OutcomeRecordV1 {
    OutcomeRecordV1 {
        outcome_id: id.into(),
        trigger,
        exact_mutations,
        opportunity_costs,
        memories,
        grants,
        next_decision,
        resolves_core_tension: resolves,
        afterlight_trigger: afterlight,
    }
}

fn m(subject: &str, field: &str, value: &str) -> TypedMutation {
    TypedMutation {
        subject_id: subject.into(),
        field_id: field.into(),
        value_id: value.into(),
    }
}

fn memory(rememberer: &str, proposition: &str) -> MemoryProposition {
    MemoryProposition {
        rememberer_id: rememberer.into(),
        proposition: proposition.into(),
    }
}

fn decision(id: &str, proposition: &str) -> NamedDecision {
    NamedDecision {
        decision_id: id.into(),
        proposition: proposition.into(),
    }
}
