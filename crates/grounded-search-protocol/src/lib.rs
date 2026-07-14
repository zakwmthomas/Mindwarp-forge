//! Capability-free evidence harness for bounded goal-grounded search.
//!
//! This crate is a proof surface, not a production solver or authority path.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CONTRACT_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Exhaustive,
    GreedyLocal,
    GroundedBeam,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Grounded,
    Exhausted,
    IndeterminateBudget,
    InvalidCase,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDisposition {
    CaseLocal,
    DomainLocal,
    CrossDomainCandidate,
    UniversalProtocolCandidate,
    Rejected,
    Inconclusive,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeNode {
    pub id: u16,
    pub hard_valid: bool,
    pub objective: i32,
    /// A domain-owned estimate only. It is never treated as proof of fitness.
    pub target_resistance: i32,
    pub neighbours: Vec<u16>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroundingCase {
    pub schema_version: u16,
    pub case_id: String,
    pub domain: String,
    pub start: u16,
    pub acceptable_objective: i32,
    pub budget: u16,
    pub beam_width: u16,
    pub nodes: Vec<ProbeNode>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeRecord {
    pub node_id: u16,
    pub hard_valid: bool,
    pub objective: i32,
    pub target_resistance: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroundingTrace {
    pub case_id: String,
    pub domain: String,
    pub method: Method,
    pub outcome: Outcome,
    pub selected: Option<u16>,
    pub probes: Vec<ProbeRecord>,
    pub fingerprint: [u8; 32],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CaseComparison {
    pub case_id: String,
    pub baseline: GroundingTrace,
    pub candidate: GroundingTrace,
    pub disposition: ScopeDisposition,
    pub rationale: String,
}

fn valid_case(case: &GroundingCase) -> bool {
    if case.schema_version != CONTRACT_VERSION
        || case.budget == 0
        || case.beam_width == 0
        || case.nodes.is_empty()
    {
        return false;
    }
    let ids: BTreeSet<u16> = case.nodes.iter().map(|node| node.id).collect();
    ids.len() == case.nodes.len()
        && ids.contains(&case.start)
        && case
            .nodes
            .iter()
            .all(|node| node.neighbours.iter().all(|id| ids.contains(id)))
}

fn finish(
    case: &GroundingCase,
    method: Method,
    outcome: Outcome,
    selected: Option<u16>,
    probes: Vec<ProbeRecord>,
) -> GroundingTrace {
    let mut hasher = Sha256::new();
    hasher.update(b"mindwarp.grounded-search.trace.v1");
    hasher.update(case.case_id.as_bytes());
    hasher.update(case.domain.as_bytes());
    hasher.update([method as u8, outcome as u8]);
    hasher.update(selected.unwrap_or(u16::MAX).to_le_bytes());
    for probe in &probes {
        hasher.update(probe.node_id.to_le_bytes());
        hasher.update([probe.hard_valid as u8]);
        hasher.update(probe.objective.to_le_bytes());
        hasher.update(probe.target_resistance.to_le_bytes());
    }
    GroundingTrace {
        case_id: case.case_id.clone(),
        domain: case.domain.clone(),
        method,
        outcome,
        selected,
        probes,
        fingerprint: hasher.finalize().into(),
    }
}

fn record(node: &ProbeNode) -> ProbeRecord {
    ProbeRecord {
        node_id: node.id,
        hard_valid: node.hard_valid,
        objective: node.objective,
        target_resistance: node.target_resistance,
    }
}

pub fn run(case: &GroundingCase, method: Method) -> GroundingTrace {
    if !valid_case(case) {
        return finish(case, method, Outcome::InvalidCase, None, Vec::new());
    }
    match method {
        Method::Exhaustive => exhaustive(case),
        Method::GreedyLocal => greedy(case),
        Method::GroundedBeam => grounded_beam(case),
    }
}

fn exhaustive(case: &GroundingCase) -> GroundingTrace {
    let mut probes = Vec::new();
    let mut best: Option<&ProbeNode> = None;
    for node in case.nodes.iter().take(case.budget as usize) {
        probes.push(record(node));
        if node.hard_valid
            && best
                .map(|old| (node.objective, node.id) < (old.objective, old.id))
                .unwrap_or(true)
        {
            best = Some(node);
        }
    }
    let selected = best.filter(|node| node.objective <= case.acceptable_objective);
    let outcome = if selected.is_some() {
        Outcome::Grounded
    } else if probes.len() < case.nodes.len() {
        Outcome::IndeterminateBudget
    } else {
        Outcome::Exhausted
    };
    finish(
        case,
        Method::Exhaustive,
        outcome,
        selected.map(|n| n.id),
        probes,
    )
}

fn greedy(case: &GroundingCase) -> GroundingTrace {
    let by_id: BTreeMap<u16, &ProbeNode> = case.nodes.iter().map(|n| (n.id, n)).collect();
    let mut current = by_id[&case.start];
    let mut visited = BTreeSet::new();
    let mut probes = Vec::new();
    while probes.len() < case.budget as usize && visited.insert(current.id) {
        probes.push(record(current));
        if current.hard_valid && current.objective <= case.acceptable_objective {
            return finish(
                case,
                Method::GreedyLocal,
                Outcome::Grounded,
                Some(current.id),
                probes,
            );
        }
        let next = current
            .neighbours
            .iter()
            .filter_map(|id| by_id.get(id).copied())
            .filter(|node| !visited.contains(&node.id) && node.hard_valid)
            .min_by_key(|node| (node.objective, node.id));
        match next {
            Some(node) if node.objective < current.objective => current = node,
            _ => return finish(case, Method::GreedyLocal, Outcome::Exhausted, None, probes),
        }
    }
    finish(
        case,
        Method::GreedyLocal,
        Outcome::IndeterminateBudget,
        None,
        probes,
    )
}

fn grounded_beam(case: &GroundingCase) -> GroundingTrace {
    let by_id: BTreeMap<u16, &ProbeNode> = case.nodes.iter().map(|n| (n.id, n)).collect();
    let mut frontier = VecDeque::from([case.start]);
    let mut queued = BTreeSet::from([case.start]);
    let mut probes = Vec::new();
    while probes.len() < case.budget as usize {
        let Some(id) = frontier.pop_front() else {
            return finish(case, Method::GroundedBeam, Outcome::Exhausted, None, probes);
        };
        let node = by_id[&id];
        probes.push(record(node));
        if node.hard_valid && node.objective <= case.acceptable_objective {
            return finish(
                case,
                Method::GroundedBeam,
                Outcome::Grounded,
                Some(id),
                probes,
            );
        }
        let mut next: Vec<&ProbeNode> = node
            .neighbours
            .iter()
            .filter_map(|id| by_id.get(id).copied())
            .filter(|child| queued.insert(child.id))
            .collect();
        next.sort_by_key(|child| (!child.hard_valid, child.target_resistance, child.id));
        for child in next.into_iter().take(case.beam_width as usize) {
            frontier.push_back(child.id);
        }
    }
    finish(
        case,
        Method::GroundedBeam,
        Outcome::IndeterminateBudget,
        None,
        probes,
    )
}

pub fn compare(
    case: &GroundingCase,
    disposition: ScopeDisposition,
    rationale: impl Into<String>,
) -> CaseComparison {
    CaseComparison {
        case_id: case.case_id.clone(),
        baseline: run(case, Method::Exhaustive),
        candidate: run(case, Method::GroundedBeam),
        disposition,
        rationale: rationale.into(),
    }
}

/// Pure presentation rule for the owner-approved selective-aging design.
/// Biological age is never mutated by this function.
pub fn presented_age(biological_age: u16, adult_age: u16, adult_lock: bool) -> u16 {
    if adult_lock && biological_age >= adult_age {
        adult_age
    } else {
        biological_age
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: u16, objective: i32, resistance: i32, neighbours: &[u16]) -> ProbeNode {
        ProbeNode {
            id,
            hard_valid: true,
            objective,
            target_resistance: resistance,
            neighbours: neighbours.to_vec(),
        }
    }

    fn case(id: &str, domain: &str, budget: u16, nodes: Vec<ProbeNode>) -> GroundingCase {
        GroundingCase {
            schema_version: 1,
            case_id: id.into(),
            domain: domain.into(),
            start: 0,
            acceptable_objective: 0,
            budget,
            beam_width: 2,
            nodes,
        }
    }

    #[test]
    fn semantic_case_escapes_a_deceptive_local_minimum() {
        let fixture = case(
            "semantic-deceptive",
            "semantic-construction",
            8,
            vec![
                node(0, 9, 9, &[1, 2]),
                node(1, 3, 8, &[]),
                node(2, 7, 2, &[3, 4]),
                node(3, 5, 1, &[5]),
                node(4, 6, 4, &[]),
                node(5, 0, 0, &[]),
            ],
        );
        assert_eq!(
            run(&fixture, Method::GreedyLocal).outcome,
            Outcome::Exhausted
        );
        let grounded = run(&fixture, Method::GroundedBeam);
        assert_eq!(
            (grounded.outcome, grounded.selected),
            (Outcome::Grounded, Some(5))
        );
    }

    #[test]
    fn forge_diagnosis_finds_the_relevant_failure_with_fewer_probes() {
        let fixture = case(
            "windows-failure-diagnosis",
            "forge-diagnosis",
            8,
            vec![
                node(0, 8, 7, &[1, 4]),
                node(1, 7, 8, &[2]),
                node(2, 6, 9, &[3]),
                node(3, 5, 9, &[]),
                node(4, 4, 2, &[5]),
                node(5, 0, 0, &[]),
                node(6, 9, 9, &[]),
            ],
        );
        let comparison = compare(
            &fixture,
            ScopeDisposition::CrossDomainCandidate,
            "bounded diagnosis ordering",
        );
        assert_eq!(comparison.baseline.outcome, Outcome::Grounded);
        assert_eq!(comparison.candidate.outcome, Outcome::Grounded);
        assert!(comparison.candidate.probes.len() < comparison.baseline.probes.len());
    }

    #[test]
    fn morphology_and_spatial_cases_are_independent_local_adapters() {
        for (id, domain) in [
            ("morphology", "organism-morphology"),
            ("route", "spatial-network"),
        ] {
            let fixture = case(
                id,
                domain,
                7,
                vec![
                    node(0, 7, 7, &[1, 2]),
                    node(1, 5, 6, &[3]),
                    node(2, 6, 2, &[4]),
                    node(3, 4, 5, &[]),
                    node(4, 2, 1, &[5]),
                    node(5, 0, 0, &[]),
                ],
            );
            assert_eq!(
                run(&fixture, Method::GroundedBeam).outcome,
                Outcome::Grounded
            );
        }
    }

    #[test]
    fn misleading_natural_heuristic_is_negative_transfer() {
        let fixture = case(
            "misleading-resistance",
            "negative-transfer",
            5,
            vec![
                node(0, 8, 8, &[1, 2]),
                node(1, 6, 0, &[3, 4]),
                node(2, 4, 8, &[5]),
                node(3, 5, 0, &[]),
                node(4, 5, 1, &[]),
                node(5, 0, 9, &[]),
            ],
        );
        let baseline = run(&fixture, Method::Exhaustive);
        let candidate = run(&fixture, Method::GroundedBeam);
        assert_eq!(baseline.outcome, Outcome::IndeterminateBudget);
        assert_eq!(candidate.outcome, Outcome::IndeterminateBudget);
        let full_baseline = run(
            &GroundingCase {
                budget: 6,
                ..fixture.clone()
            },
            Method::Exhaustive,
        );
        assert_eq!(full_baseline.outcome, Outcome::Grounded);
    }

    #[test]
    fn hard_constraints_kill_attractive_invalid_probes() {
        let mut fixture = case(
            "hard-constraint",
            "semantic-construction",
            4,
            vec![
                node(0, 5, 5, &[1, 2]),
                node(1, -9, 0, &[]),
                node(2, 0, 1, &[]),
            ],
        );
        fixture.nodes[1].hard_valid = false;
        let trace = run(&fixture, Method::GroundedBeam);
        assert_eq!(
            (trace.outcome, trace.selected),
            (Outcome::Grounded, Some(2))
        );
    }

    #[test]
    fn budget_and_malformed_cases_fail_closed() {
        let fixture = case(
            "budget",
            "test-selection",
            1,
            vec![node(0, 5, 5, &[1]), node(1, 0, 0, &[])],
        );
        assert_eq!(
            run(&fixture, Method::GroundedBeam).outcome,
            Outcome::IndeterminateBudget
        );
        let malformed = GroundingCase {
            schema_version: 2,
            ..fixture
        };
        assert_eq!(
            run(&malformed, Method::GroundedBeam).outcome,
            Outcome::InvalidCase
        );
    }

    #[test]
    fn traces_are_deterministic_and_method_specific() {
        let fixture = case(
            "replay",
            "forge-research-routing",
            3,
            vec![node(0, 3, 3, &[1]), node(1, 0, 0, &[])],
        );
        let first = run(&fixture, Method::GroundedBeam);
        assert_eq!(first, run(&fixture, Method::GroundedBeam));
        assert_ne!(
            first.fingerprint,
            run(&fixture, Method::Exhaustive).fingerprint
        );
    }

    #[test]
    fn adult_lock_changes_presentation_not_continuing_age() {
        assert_eq!(presented_age(3, 10, true), 3);
        assert_eq!(presented_age(10, 10, true), 10);
        assert_eq!(presented_age(18, 10, true), 10);
        assert_eq!(presented_age(18, 10, false), 18);
        assert_eq!(presented_age(22, 10, true), 10);
        assert_eq!(presented_age(23, 10, false), 23);
    }
}
