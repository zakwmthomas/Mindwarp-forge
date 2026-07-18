//! G1-C5 evidence: several structurally different consumer domains share one
//! `ImportancePacket`/`SignificanceState` without inventing a private global
//! significance score. The specific fidelity curves below are illustrative
//! test fixtures only; they select no final product weights, engine timing,
//! or per-domain semantics.

use significance_scheduler::{
    ConsumerFidelityMap, HysteresisPolicy, ImportancePacket, ImportanceTier, PROTECT_THREAT,
    SignalVector, SignificanceState,
};

const CONSUMER_AI: u16 = 1;
const CONSUMER_ANIMATION: u16 = 2;
const CONSUMER_AUDIO: u16 = 3;
const CONSUMER_RENDERING: u16 = 4;

fn policy() -> HysteresisPolicy {
    HysteresisPolicy::new([100, 300, 600], [80, 250, 500], 2).unwrap()
}

fn packet(peak: u16, epoch: u64, protection: u8) -> ImportancePacket {
    ImportancePacket::new(
        [7; 32],
        epoch,
        SignalVector {
            focus: peak,
            visibility: peak,
            interaction: 0,
            threat: 0,
            prediction: 0,
        },
        0,
        protection,
    )
    .unwrap()
}

#[test]
fn structurally_different_domains_diverge_from_one_shared_tier() {
    // AI needs to notice distant/background targets early: front-loaded.
    let ai = ConsumerFidelityMap::new(CONSUMER_AI, [2, 8, 12, 16]).unwrap();
    // Animation is expensive: back-loaded, near-zero until Visible.
    let animation = ConsumerFidelityMap::new(CONSUMER_ANIMATION, [0, 0, 6, 16]).unwrap();
    // Audio needs only a coarse presence signal.
    let audio = ConsumerFidelityMap::new(CONSUMER_AUDIO, [0, 4, 4, 10]).unwrap();
    // Rendering scales smoothly across every tier.
    let rendering = ConsumerFidelityMap::new(CONSUMER_RENDERING, [1, 5, 10, 16]).unwrap();

    let mut state = SignificanceState::default();
    let tier = state.advance(&packet(650, 1, 0), policy(), 1).unwrap();
    assert_eq!(tier, ImportanceTier::Critical);

    let fidelities = [
        ai.fidelity(tier),
        animation.fidelity(tier),
        audio.fidelity(tier),
        rendering.fidelity(tier),
    ];
    assert!(
        fidelities
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            > 1,
        "no single universal scalar should describe every domain from one tier"
    );

    let mut low_state = SignificanceState::default();
    let low_tier = low_state.advance(&packet(150, 1, 0), policy(), 1).unwrap();
    assert_eq!(low_tier, ImportanceTier::Background);
    assert!(ai.fidelity(low_tier) > animation.fidelity(low_tier));
    assert_eq!(animation.fidelity(low_tier), 0);
}

#[test]
fn one_shared_hysteresis_protected_tier_keeps_every_consumer_stable_during_a_flap() {
    let policy = policy();
    let mut state = SignificanceState::default();

    // Cross into Critical, then flap just below/above the Critical exit
    // threshold on consecutive steps.
    let flapping_signal = [650u16, 260, 650, 260, 650];
    let mut tiers = Vec::new();
    for (index, &signal) in flapping_signal.iter().enumerate() {
        let step = index as u64 + 1;
        let tier = state
            .advance(&packet(signal, step, 0), policy, step)
            .unwrap();
        tiers.push(tier);
    }
    let transitions = tiers.windows(2).filter(|pair| pair[0] != pair[1]).count();
    assert!(
        transitions < flapping_signal.len() - 1,
        "minimum hold steps must absorb some of the raw flaps, not just replay them: {tiers:?}"
    );
}

#[test]
fn protected_threat_forces_every_domain_to_its_critical_fidelity_simultaneously() {
    let ai = ConsumerFidelityMap::new(CONSUMER_AI, [2, 8, 12, 16]).unwrap();
    let audio = ConsumerFidelityMap::new(CONSUMER_AUDIO, [0, 4, 4, 10]).unwrap();
    let mut state = SignificanceState::default();
    // Near-zero raw signal, but a protected threat flag must still force
    // Critical for every domain reading the same shared tier.
    let tier = state
        .advance(&packet(0, 1, PROTECT_THREAT), policy(), 1)
        .unwrap();
    assert_eq!(tier, ImportanceTier::Critical);
    assert_eq!(ai.fidelity(tier), 16);
    assert_eq!(audio.fidelity(tier), 10);
}

#[test]
fn independent_consumer_maps_do_not_interfere_with_each_other() {
    let a = ConsumerFidelityMap::new(CONSUMER_AI, [1, 2, 3, 4]).unwrap();
    let b = ConsumerFidelityMap::new(CONSUMER_ANIMATION, [1, 2, 3, 4]).unwrap();
    let before = (
        a.fidelity(ImportanceTier::Visible),
        b.fidelity(ImportanceTier::Visible),
    );
    // Constructing an unrelated third consumer must not mutate the other two;
    // they are plain values, never a shared mutable registry.
    let _ = ConsumerFidelityMap::new(CONSUMER_AUDIO, [16, 16, 16, 16]).unwrap();
    let after = (
        a.fidelity(ImportanceTier::Visible),
        b.fidelity(ImportanceTier::Visible),
    );
    assert_eq!(before, after);
}
