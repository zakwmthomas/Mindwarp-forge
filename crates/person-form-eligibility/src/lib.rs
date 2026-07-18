//! Capability-free prerequisite contract for future person-form evaluation.
//!
//! This module does not decide person-form eligibility. The master plan says
//! the *most compatible native lineage* is eligible, which requires real
//! macro-lineage identities, body plans, grounded capacities, structural
//! compatibility evidence, and comparison among candidate lineages. Those
//! inputs do not exist yet.
//!
//! The bounded contract here proves only that a future evaluator can fail
//! closed before comparison:
//!
//! - capacity evidence is bound to an explicit lineage identity rather than
//!   inferred from a world-packet evidence hash;
//! - evidence from another lineage cannot satisfy the assessed lineage;
//! - all five named comparison dimensions must be represented;
//! - a body-plan reference must exist;
//! - satisfying these checks yields structural binding completeness only,
//!   never grounded evidence, comparative readiness, or eligibility.
//!
//! "Grotesque retrofit" rejection remains a future structural-compatibility
//! proof. A lineage-ID mismatch alone cannot prove anatomy is grotesque.

use semantic_construction::{Claim, Id};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonFormCapacity {
    Manipulation,
    Locomotion,
    Communication,
    Cognition,
    ToolUse,
}

pub const COMPARISON_DIMENSIONS: [PersonFormCapacity; 5] = [
    PersonFormCapacity::Manipulation,
    PersonFormCapacity::Locomotion,
    PersonFormCapacity::Communication,
    PersonFormCapacity::Cognition,
    PersonFormCapacity::ToolUse,
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CapacityGrounding {
    pub capacity: PersonFormCapacity,
    pub lineage_id: Id,
    pub claim: Claim,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PrerequisiteStatus {
    IncompleteBindings,
    StructurallyCompleteBindings,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PersonFormPrerequisiteReport {
    pub status: PrerequisiteStatus,
    pub assessed_lineage_id: Id,
    pub body_plan_ref: Option<Id>,
    pub bound_dimensions: Vec<PersonFormCapacity>,
    pub missing_dimensions: Vec<PersonFormCapacity>,
    pub foreign_lineage_dimensions: Vec<PersonFormCapacity>,
    pub invalid_dimensions: Vec<PersonFormCapacity>,
    pub limitations: Vec<String>,
}

/// Checks whether one lineage has enough correctly bound evidence to enter a
/// later comparative person-form evaluation.
///
/// This function intentionally has no `eligible: bool`. The master-plan rule
/// is comparative ("most compatible"), and no score, threshold, structural
/// compatibility evaluator, or candidate portfolio has been approved.
pub fn evaluate_person_form_prerequisites(
    assessed_lineage_id: Id,
    body_plan_ref: Option<Id>,
    groundings: &[CapacityGrounding],
) -> PersonFormPrerequisiteReport {
    let mut bound = BTreeSet::new();
    let mut foreign = BTreeSet::new();
    let mut invalid = BTreeSet::new();
    let mut claim_ids = BTreeSet::new();

    for grounding in groundings {
        if grounding.lineage_id == assessed_lineage_id {
            let claim_valid = grounding.claim.id != [0; 32]
                && grounding.claim.evidence_ref != [0; 32]
                && grounding.claim.concept_id == capacity_concept_id(grounding.capacity)
                && claim_ids.insert(grounding.claim.id);
            if claim_valid {
                bound.insert(grounding.capacity);
            } else {
                invalid.insert(grounding.capacity);
            }
        } else {
            foreign.insert(grounding.capacity);
        }
    }

    let bound_dimensions: Vec<_> = COMPARISON_DIMENSIONS
        .into_iter()
        .filter(|capacity| bound.contains(capacity))
        .collect();
    let missing_dimensions: Vec<_> = COMPARISON_DIMENSIONS
        .into_iter()
        .filter(|capacity| !bound.contains(capacity))
        .collect();
    let foreign_lineage_dimensions: Vec<_> = COMPARISON_DIMENSIONS
        .into_iter()
        .filter(|capacity| foreign.contains(capacity))
        .collect();
    let invalid_dimensions: Vec<_> = COMPARISON_DIMENSIONS
        .into_iter()
        .filter(|capacity| invalid.contains(capacity))
        .collect();

    let structurally_complete = assessed_lineage_id != [0; 32]
        && body_plan_ref.is_some_and(|id| id != [0; 32])
        && missing_dimensions.is_empty()
        && invalid_dimensions.is_empty();
    PersonFormPrerequisiteReport {
        status: if structurally_complete {
            PrerequisiteStatus::StructurallyCompleteBindings
        } else {
            PrerequisiteStatus::IncompleteBindings
        },
        assessed_lineage_id,
        body_plan_ref,
        bound_dimensions,
        missing_dimensions,
        foreign_lineage_dimensions,
        invalid_dimensions,
        limitations: vec![
            "structurally complete means bindings are well-formed, never person-form eligible or ready for comparison".into(),
            "claim truth and evidence provenance still require validation by their future owning modules".into(),
            "structural compatibility and grotesque-retrofit rejection are not evaluated".into(),
            "no comparison score, weighting, threshold or winning lineage is selected".into(),
        ],
    }
}

pub fn capacity_concept_id(capacity: PersonFormCapacity) -> Id {
    let label = match capacity {
        PersonFormCapacity::Manipulation => "manipulation",
        PersonFormCapacity::Locomotion => "locomotion",
        PersonFormCapacity::Communication => "communication",
        PersonFormCapacity::Cognition => "cognition",
        PersonFormCapacity::ToolUse => "tool-use",
    };
    let mut hasher = Sha256::new();
    hasher.update(b"mindwarp.person-form-capacity-concept.v1\0");
    hasher.update(label.as_bytes());
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use semantic_construction::ClaimClass;
    use sha2::{Digest, Sha256};

    fn id(label: &str) -> Id {
        let mut hasher = Sha256::new();
        hasher.update(b"mindwarp.person-form-prerequisite.fixture.v1\0");
        hasher.update(label.as_bytes());
        hasher.finalize().into()
    }

    fn grounding(capacity: PersonFormCapacity, lineage_id: Id) -> CapacityGrounding {
        CapacityGrounding {
            capacity,
            lineage_id,
            claim: Claim {
                id: id(&format!("{capacity:?}-{lineage_id:?}-claim")),
                concept_id: capacity_concept_id(capacity),
                class: ClaimClass::Hypothesis,
                evidence_ref: id(&format!("{capacity:?}-{lineage_id:?}-evidence")),
            },
        }
    }

    #[test]
    fn no_body_plan_never_reaches_structural_completeness() {
        let lineage = id("lineage");
        let groundings: Vec<_> = COMPARISON_DIMENSIONS
            .into_iter()
            .map(|capacity| grounding(capacity, lineage))
            .collect();
        let report = evaluate_person_form_prerequisites(lineage, None, &groundings);
        assert_eq!(report.status, PrerequisiteStatus::IncompleteBindings);
        assert!(report.missing_dimensions.is_empty());
    }

    #[test]
    fn partial_dimensions_fail_closed_without_claiming_ineligibility() {
        let lineage = id("lineage");
        let report = evaluate_person_form_prerequisites(
            lineage,
            Some(id("body-plan")),
            &[grounding(PersonFormCapacity::Communication, lineage)],
        );
        assert_eq!(report.status, PrerequisiteStatus::IncompleteBindings);
        assert_eq!(
            report.bound_dimensions,
            vec![PersonFormCapacity::Communication]
        );
        assert_eq!(report.missing_dimensions.len(), 4);
    }

    #[test]
    fn foreign_lineage_evidence_is_reported_and_does_not_fill_a_dimension() {
        let assessed = id("assessed-lineage");
        let foreign = id("foreign-lineage");
        let report = evaluate_person_form_prerequisites(
            assessed,
            Some(id("body-plan")),
            &[grounding(PersonFormCapacity::Manipulation, foreign)],
        );
        assert_eq!(report.status, PrerequisiteStatus::IncompleteBindings);
        assert!(
            report
                .missing_dimensions
                .contains(&PersonFormCapacity::Manipulation)
        );
        assert_eq!(
            report.foreign_lineage_dimensions,
            vec![PersonFormCapacity::Manipulation]
        );
    }

    #[test]
    fn all_dimensions_and_body_plan_only_reach_structural_completeness() {
        let lineage = id("lineage");
        let groundings: Vec<_> = COMPARISON_DIMENSIONS
            .into_iter()
            .map(|capacity| grounding(capacity, lineage))
            .collect();
        let report =
            evaluate_person_form_prerequisites(lineage, Some(id("body-plan")), &groundings);
        assert_eq!(
            report.status,
            PrerequisiteStatus::StructurallyCompleteBindings
        );
        assert!(report.missing_dimensions.is_empty());
        assert!(
            report
                .limitations
                .iter()
                .any(|limit| limit.contains("never person-form eligible"))
        );
    }

    #[test]
    fn result_is_order_independent() {
        let lineage = id("lineage");
        let forward = vec![
            grounding(PersonFormCapacity::Manipulation, lineage),
            grounding(PersonFormCapacity::Communication, lineage),
        ];
        let mut reversed = forward.clone();
        reversed.reverse();
        assert_eq!(
            evaluate_person_form_prerequisites(lineage, Some(id("body-plan")), &forward),
            evaluate_person_form_prerequisites(lineage, Some(id("body-plan")), &reversed)
        );
    }

    #[test]
    fn zero_identities_and_unrelated_or_duplicate_claims_never_complete() {
        let shared = Claim {
            id: id("shared-claim"),
            concept_id: id("unrelated-concept"),
            class: ClaimClass::Derived,
            evidence_ref: id("evidence"),
        };
        let groundings: Vec<_> = COMPARISON_DIMENSIONS
            .into_iter()
            .map(|capacity| CapacityGrounding {
                capacity,
                lineage_id: [0; 32],
                claim: shared.clone(),
            })
            .collect();
        let report = evaluate_person_form_prerequisites([0; 32], Some([0; 32]), &groundings);
        assert_eq!(report.status, PrerequisiteStatus::IncompleteBindings);
        assert_eq!(report.invalid_dimensions.len(), COMPARISON_DIMENSIONS.len());
        assert!(report.bound_dimensions.is_empty());
    }
}
