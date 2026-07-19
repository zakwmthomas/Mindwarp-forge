use serde::Serialize;

use crate::{
    BudgetEnvelope, DeadlineClass, ImportancePacket, ImportanceTier, ReferenceScheduler,
    ResourceClass, SignalVector, SignificanceSchedulerError, WorkTicket, hex,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SignificanceSchedulerProofEvidence {
    pub schema_version: u16,
    pub system_ids: Vec<String>,
    pub proof_id: String,
    pub fixture_id: String,
    pub measurement_classification: String,
    pub packet_fingerprint: String,
    pub queue_growth: Vec<(usize, u64, usize)>,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

pub fn reference_proof_evidence()
-> Result<SignificanceSchedulerProofEvidence, SignificanceSchedulerError> {
    let packet = ImportancePacket::new(
        [1; 32],
        1,
        SignalVector {
            focus: 500,
            visibility: 700,
            interaction: 100,
            threat: 0,
            prediction: 400,
        },
        1,
        0,
    )?;
    let mut queue_growth = Vec::new();
    for count in [1_usize, 16, 64, 256] {
        let tickets = (0..count)
            .map(|index| fixture_ticket(index, packet.fingerprint()?))
            .collect::<Result<Vec<_>, _>>()?;
        let mut scheduler =
            ReferenceScheduler::new(tickets, BudgetEnvelope::new(1, [0, 8, 0, 0], [0; 4], 4)?)?;
        let mut steps = 0_u64;
        let mut decisions = 0_usize;
        loop {
            let trace = scheduler.step();
            steps += 1;
            decisions += trace.decisions.len();
            if trace.remaining_tickets == 0 {
                break;
            }
        }
        queue_growth.push((count, steps, decisions));
    }
    Ok(SignificanceSchedulerProofEvidence {
        schema_version: 1,
        system_ids: vec!["significance-system".into(), "streaming-scheduler".into()],
        proof_id: "bounded-pressure-reference".into(),
        fixture_id: "significance-scheduler-v1/core".into(),
        measurement_classification: "simulated".into(),
        packet_fingerprint: hex(&packet.fingerprint()?),
        queue_growth,
        capabilities: Vec::new(),
        limitations: vec![
            "Integer reference simulation; not runtime, hardware, frame-time, or cache performance.".into(),
            "Signal thresholds, consumer fidelity meanings, and resource units are test fixtures, not product policy.".into(),
            "Evidence grants no approval, promotion, execution, spending, publishing, credential, engine, or Kernel authority.".into(),
        ],
    })
}

fn fixture_id(index: usize) -> [u8; 32] {
    let mut id = [0; 32];
    id[..8].copy_from_slice(&(index as u64 + 1).to_be_bytes());
    id
}

fn fixture_ticket(
    index: usize,
    packet: [u8; 32],
) -> Result<WorkTicket, SignificanceSchedulerError> {
    WorkTicket::new(
        fixture_id(index),
        [1; 32],
        1,
        (index % 8 + 1) as u16,
        1,
        ResourceClass::Cpu,
        1,
        vec![],
        DeadlineClass::QualityTarget,
        1000,
        None,
        None,
        packet,
        ImportanceTier::Background,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ConsumerDomainV1, ConsumerFidelityMap, DecisionKind, HysteresisPolicy, PROTECT_THREAT,
        SignificanceState, TicketState,
    };

    fn packet(signal: u16, protection: u8) -> ImportancePacket {
        ImportancePacket::new(
            [7; 32],
            1,
            SignalVector {
                focus: signal,
                visibility: 0,
                interaction: 0,
                threat: 0,
                prediction: 0,
            },
            1,
            protection,
        )
        .unwrap()
    }

    fn policy() -> HysteresisPolicy {
        HysteresisPolicy::new([100, 500, 900], [50, 400, 800], 3).unwrap()
    }

    #[allow(clippy::too_many_arguments)]
    fn ticket(
        id: u8,
        cost: u32,
        dependencies: Vec<u8>,
        class: DeadlineClass,
        due: u64,
        tier: ImportanceTier,
        fallback: Option<u8>,
        cancel_parent: Option<u8>,
    ) -> WorkTicket {
        WorkTicket::new(
            [id; 32],
            [7; 32],
            1,
            ConsumerDomainV1::Generation.code(),
            1,
            ResourceClass::Cpu,
            cost,
            dependencies
                .into_iter()
                .map(|dependency| [dependency; 32])
                .collect(),
            class,
            due,
            fallback.map(|value| [value; 32]),
            cancel_parent.map(|value| [value; 32]),
            [9; 32],
            tier,
        )
        .unwrap()
    }

    fn budget(cpu: u32, reserve: u32, debt: u16) -> BudgetEnvelope {
        BudgetEnvelope::new(1, [0, cpu, 0, 0], [0, reserve, 0, 0], debt).unwrap()
    }

    #[test]
    fn packet_bytes_are_strict_and_corruption_is_rejected() {
        let value = packet(700, 0);
        let bytes = value.encode_canonical().unwrap();
        assert_eq!(ImportancePacket::decode_strict(&bytes).unwrap(), value);
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(ImportancePacket::decode_strict(&trailing).is_err());
        let mut corrupt = bytes;
        corrupt[0] ^= 1;
        assert!(ImportancePacket::decode_strict(&corrupt).is_err());
    }

    #[test]
    fn hysteresis_resists_flapping_but_protection_promotes_immediately() {
        let mut state = SignificanceState::default();
        assert_eq!(
            state.advance(&packet(510, 0), policy(), 1).unwrap(),
            ImportanceTier::Visible
        );
        assert_eq!(
            state.advance(&packet(390, 0), policy(), 2).unwrap(),
            ImportanceTier::Visible
        );
        assert_eq!(
            state.advance(&packet(390, 0), policy(), 4).unwrap(),
            ImportanceTier::Background
        );
        assert_eq!(
            state
                .advance(&packet(0, PROTECT_THREAT), policy(), 5)
                .unwrap(),
            ImportanceTier::Critical
        );
        assert_eq!(
            state.advance(&packet(0, 0), policy(), 6).unwrap(),
            ImportanceTier::Critical
        );
        assert_eq!(
            state.advance(&packet(0, 0), policy(), 8).unwrap(),
            ImportanceTier::Dormant
        );
    }

    #[test]
    fn consumer_maps_share_tier_but_allow_monotone_fidelity() {
        let render = ConsumerFidelityMap::new(1, [0, 1, 3, 5]).unwrap();
        let audio = ConsumerFidelityMap::new(2, [0, 0, 1, 2]).unwrap();
        assert_ne!(
            render.fidelity(ImportanceTier::Visible),
            audio.fidelity(ImportanceTier::Visible)
        );
        assert!(ConsumerFidelityMap::new(3, [0, 2, 1, 3]).is_err());
    }

    #[test]
    fn ticket_bytes_are_strict_and_single_resource() {
        let value = ticket(
            1,
            4,
            vec![],
            DeadlineClass::QualityTarget,
            10,
            ImportanceTier::Background,
            None,
            None,
        );
        let bytes = value.encode_canonical().unwrap();
        assert_eq!(WorkTicket::decode_strict(&bytes).unwrap(), value);
        let mut trailing = bytes;
        trailing.push(0);
        assert!(WorkTicket::decode_strict(&trailing).is_err());
    }

    #[test]
    fn graph_rejects_duplicates_unknowns_cycles_and_bad_fallbacks() {
        let one = ticket(
            1,
            2,
            vec![],
            DeadlineClass::QualityTarget,
            10,
            ImportanceTier::Background,
            None,
            None,
        );
        assert_eq!(
            ReferenceScheduler::new(vec![one.clone(), one], budget(2, 0, 3)).unwrap_err(),
            SignificanceSchedulerError::DuplicateTicket
        );
        let unknown = ticket(
            2,
            1,
            vec![9],
            DeadlineClass::QualityTarget,
            10,
            ImportanceTier::Background,
            None,
            None,
        );
        assert_eq!(
            ReferenceScheduler::new(vec![unknown], budget(2, 0, 3)).unwrap_err(),
            SignificanceSchedulerError::UnknownDependency
        );
        let a = ticket(
            1,
            1,
            vec![2],
            DeadlineClass::QualityTarget,
            10,
            ImportanceTier::Background,
            None,
            None,
        );
        let b = ticket(
            2,
            1,
            vec![1],
            DeadlineClass::QualityTarget,
            10,
            ImportanceTier::Background,
            None,
            None,
        );
        assert_eq!(
            ReferenceScheduler::new(vec![a, b], budget(2, 0, 3)).unwrap_err(),
            SignificanceSchedulerError::DependencyCycle
        );
        let original = ticket(
            3,
            1,
            vec![],
            DeadlineClass::QualityTarget,
            10,
            ImportanceTier::Background,
            Some(4),
            None,
        );
        let expensive = ticket(
            4,
            2,
            vec![],
            DeadlineClass::QualityTarget,
            10,
            ImportanceTier::Background,
            None,
            None,
        );
        assert_eq!(
            ReferenceScheduler::new(vec![original, expensive], budget(2, 0, 3)).unwrap_err(),
            SignificanceSchedulerError::InvalidFallback
        );
    }

    #[test]
    fn safety_deadlines_require_reserved_admission_capacity() {
        let safety = ticket(
            1,
            4,
            vec![],
            DeadlineClass::InteractionSafety,
            1,
            ImportanceTier::Critical,
            None,
            None,
        );
        assert_eq!(
            ReferenceScheduler::new(vec![safety.clone()], budget(4, 1, 3)).unwrap_err(),
            SignificanceSchedulerError::AdmissionRejected
        );
        ReferenceScheduler::new(vec![safety], budget(4, 2, 3)).unwrap();
    }

    #[test]
    fn urgent_dependents_donate_priority_without_mutating_ticket() {
        let prerequisite = ticket(
            1,
            1,
            vec![],
            DeadlineClass::QualityTarget,
            100,
            ImportanceTier::Dormant,
            None,
            None,
        );
        let dependent = ticket(
            2,
            1,
            vec![1],
            DeadlineClass::InteractionSafety,
            2,
            ImportanceTier::Critical,
            None,
            None,
        );
        let mut scheduler =
            ReferenceScheduler::new(vec![prerequisite, dependent], budget(2, 1, 3)).unwrap();
        assert_eq!(
            scheduler.effective_priority([1; 32]).unwrap(),
            (
                DeadlineClass::InteractionSafety,
                ImportanceTier::Critical,
                2
            )
        );
        scheduler.request_cancel([2; 32], 1).unwrap();
        assert_eq!(
            scheduler.effective_priority([1; 32]).unwrap(),
            (DeadlineClass::QualityTarget, ImportanceTier::Dormant, 100)
        );
    }

    #[test]
    fn ties_and_traces_are_deterministic() {
        let tickets = vec![
            ticket(
                2,
                1,
                vec![],
                DeadlineClass::QualityTarget,
                10,
                ImportanceTier::Background,
                None,
                None,
            ),
            ticket(
                1,
                1,
                vec![],
                DeadlineClass::QualityTarget,
                10,
                ImportanceTier::Background,
                None,
                None,
            ),
        ];
        let mut left = ReferenceScheduler::new(tickets.clone(), budget(1, 0, 3)).unwrap();
        let mut right = ReferenceScheduler::new(tickets, budget(1, 0, 3)).unwrap();
        let a = left.step();
        let b = right.step();
        assert_eq!(a, b);
        assert_eq!(a.decisions[0].ticket_id, [1; 32]);
    }

    #[test]
    fn bounded_debt_prevents_background_starvation() {
        let urgent = ticket(
            1,
            10,
            vec![],
            DeadlineClass::VisibleMinimum,
            100,
            ImportanceTier::Critical,
            None,
            None,
        );
        let background = ticket(
            2,
            1,
            vec![],
            DeadlineClass::QualityTarget,
            100,
            ImportanceTier::Dormant,
            None,
            None,
        );
        let mut scheduler =
            ReferenceScheduler::new(vec![urgent, background], budget(1, 0, 3)).unwrap();
        for _ in 0..4 {
            scheduler.step();
        }
        assert_eq!(
            scheduler.state([2; 32]).unwrap(),
            TicketState::CompletedAccepted
        );
    }

    #[test]
    fn unused_safety_reserve_is_reclaimed_without_starving_safety() {
        let safety = ticket(
            1,
            1,
            vec![],
            DeadlineClass::InteractionSafety,
            2,
            ImportanceTier::Critical,
            None,
            None,
        );
        let quality = ticket(
            2,
            2,
            vec![],
            DeadlineClass::QualityTarget,
            20,
            ImportanceTier::Background,
            None,
            None,
        );
        let mut scheduler =
            ReferenceScheduler::new(vec![safety, quality], budget(2, 1, 3)).unwrap();
        let trace = scheduler.step();
        assert_eq!(
            scheduler.state([1; 32]).unwrap(),
            TicketState::CompletedAccepted
        );
        assert_eq!(scheduler.remaining([2; 32]).unwrap(), 1);
        assert_eq!(
            trace
                .decisions
                .iter()
                .filter(|decision| decision.kind == DecisionKind::Executed)
                .count(),
            2
        );
    }

    #[test]
    fn cancellation_cascades_settles_and_discards_late_output() {
        let original = ticket(
            1,
            4,
            vec![],
            DeadlineClass::VisibleMinimum,
            20,
            ImportanceTier::Visible,
            Some(3),
            None,
        );
        let child = ticket(
            2,
            2,
            vec![],
            DeadlineClass::QualityTarget,
            20,
            ImportanceTier::Background,
            None,
            Some(1),
        );
        let fallback = ticket(
            3,
            1,
            vec![],
            DeadlineClass::QualityTarget,
            20,
            ImportanceTier::Background,
            None,
            None,
        );
        let mut scheduler =
            ReferenceScheduler::new(vec![original, child, fallback], budget(1, 0, 3)).unwrap();
        scheduler.request_cancel([1; 32], 1).unwrap();
        assert_eq!(
            scheduler.state([2; 32]).unwrap(),
            TicketState::CancelRequested
        );
        scheduler.acknowledge_cancel([1; 32]).unwrap();
        scheduler.settle_cancel([1; 32]).unwrap();
        assert_eq!(scheduler.state([3; 32]).unwrap(), TicketState::Pending);
        let late = scheduler.record_external_completion([1; 32], 1).unwrap();
        assert_eq!(late.kind, DecisionKind::CompletedDiscarded);
    }

    #[test]
    fn stale_epochs_are_quarantined() {
        let work = ticket(
            1,
            2,
            vec![],
            DeadlineClass::QualityTarget,
            20,
            ImportanceTier::Background,
            None,
            None,
        );
        let mut scheduler = ReferenceScheduler::new(vec![work], budget(1, 0, 3)).unwrap();
        scheduler.advance_target_epoch([7; 32], 2).unwrap();
        assert_eq!(
            scheduler.state([1; 32]).unwrap(),
            TicketState::CancelRequested
        );
        let completion = scheduler.record_external_completion([1; 32], 1).unwrap();
        assert_eq!(completion.kind, DecisionKind::CompletedDiscarded);
    }

    #[test]
    fn dependency_failure_activates_validated_fallback() {
        let dependency = ticket(
            1,
            2,
            vec![],
            DeadlineClass::QualityTarget,
            20,
            ImportanceTier::Background,
            None,
            None,
        );
        let original = ticket(
            2,
            2,
            vec![1],
            DeadlineClass::VisibleMinimum,
            20,
            ImportanceTier::Visible,
            Some(3),
            None,
        );
        let fallback = ticket(
            3,
            1,
            vec![],
            DeadlineClass::QualityTarget,
            20,
            ImportanceTier::Background,
            None,
            None,
        );
        let mut scheduler =
            ReferenceScheduler::new(vec![dependency, original, fallback], budget(1, 0, 3)).unwrap();
        scheduler.request_cancel([1; 32], 1).unwrap();
        scheduler.acknowledge_cancel([1; 32]).unwrap();
        scheduler.settle_cancel([1; 32]).unwrap();
        let trace = scheduler.step();
        assert!(
            trace
                .decisions
                .iter()
                .any(|decision| decision.kind == DecisionKind::DependencyRejected)
        );
        assert!(matches!(
            scheduler.state([3; 32]).unwrap(),
            TicketState::Running | TicketState::CompletedAccepted
        ));
    }

    #[test]
    fn proof_growth_is_bounded_and_authority_negative() {
        let evidence = reference_proof_evidence().unwrap();
        assert_eq!(
            evidence
                .queue_growth
                .iter()
                .map(|row| row.0)
                .collect::<Vec<_>>(),
            vec![1, 16, 64, 256]
        );
        assert!(evidence.capabilities.is_empty());
        let value = serde_json::to_value(evidence).unwrap();
        for forbidden in [
            "approve",
            "promote",
            "execute",
            "publish",
            "spend",
            "credential",
            "engine",
        ] {
            assert!(value.get(forbidden).is_none());
        }
    }
}
