use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Valid,
    Invalid,
    IndeterminateBudget,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Violation {
    pub code: String,
    pub location: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationReport {
    pub status: ValidationStatus,
    pub examined: u32,
    pub violations: Vec<Violation>,
}

fn violation(code: &str, location: impl Into<String>) -> Violation {
    Violation {
        code: code.into(),
        location: location.into(),
    }
}

fn nonzero(id: &Id) -> bool {
    id.iter().any(|byte| *byte != 0)
}

fn valid_locator(reference: &LogicalReference) -> bool {
    let expected = format!("cid:sha256:{}", crate::hex(&reference.content_fingerprint));
    reference.locator == expected
        && reference.locator.len() == 75
        && reference.locator.is_ascii()
        && !reference.locator.contains("..")
        && !reference.locator.contains('\\')
        && !reference.locator.contains('/')
        && !reference.locator.contains('%')
        && !reference.locator.contains('$')
}

pub fn validate_package(package: &RepresentationContractPackage, budget: u32) -> ValidationReport {
    let mut report = ValidationReport {
        status: ValidationStatus::Valid,
        examined: 0,
        violations: Vec::new(),
    };
    macro_rules! examine {
        () => {{
            if report.examined >= budget {
                report.status = ValidationStatus::IndeterminateBudget;
                report.violations.sort();
                return report;
            }
            report.examined += 1;
        }};
    }

    examine!();
    if ensure_contract_version(package.schema_version).is_err()
        || ensure_contract_version(package.manifest.schema_version).is_err()
        || ensure_contract_version(package.materials.schema_version).is_err()
        || ensure_contract_version(package.articulation.schema_version).is_err()
        || ensure_contract_version(package.temporal.schema_version).is_err()
        || ensure_contract_version(package.review.schema_version).is_err()
    {
        report
            .violations
            .push(violation("unknown_schema", "package"));
    }
    for (name, id) in [
        ("semantic_package_ref", package.semantic_package_ref),
        ("recipe_ref", package.recipe_ref),
        (
            "importance_packet_ref",
            package.temporal.importance_packet_ref,
        ),
    ] {
        examine!();
        if !nonzero(&id) {
            report
                .violations
                .push(violation("missing_dependency", name));
        }
    }

    let feasible: Vec<&RepresentationOption> = package
        .portfolio
        .options
        .iter()
        .filter(|option| option.hard_constraints_satisfied)
        .collect();
    let mechanisms: BTreeSet<Vec<Id>> = feasible
        .iter()
        .map(|option| {
            let mut evidence = option.mechanism_evidence.clone();
            evidence.sort();
            evidence
        })
        .collect();
    if feasible.is_empty() {
        report
            .violations
            .push(violation("no_feasible_representation", "portfolio"));
    } else if mechanisms.len() < 2 && package.portfolio.single_feasible_representation.is_none() {
        report
            .violations
            .push(violation("fake_or_missing_diversity", "portfolio"));
    }
    let option_ids: BTreeSet<Id> = package
        .portfolio
        .options
        .iter()
        .map(|option| option.id)
        .collect();
    if option_ids.len() != package.portfolio.options.len() {
        report
            .violations
            .push(violation("duplicate_option", "portfolio.options"));
    }
    for option in &package.portfolio.options {
        examine!();
        if option.mechanism_evidence.is_empty() || option.requirement_refs.is_empty() {
            report.violations.push(violation(
                "ungrounded_option",
                format!("option:{:?}", option.id),
            ));
        }
        let dimensions: BTreeSet<Id> = option
            .trade_vector
            .iter()
            .map(|trade| trade.dimension_id)
            .collect();
        if option.trade_vector.is_empty() {
            report.violations.push(violation(
                "missing_trade_evidence",
                format!("option:{:?}", option.id),
            ));
        }
        if dimensions.len() != option.trade_vector.len() {
            report.violations.push(violation(
                "duplicate_trade_dimension",
                format!("option:{:?}", option.id),
            ));
        }
        for trade in &option.trade_vector {
            if trade.unit.is_empty() || trade.method.is_empty() || trade.uncertainty.is_empty() {
                report.violations.push(violation(
                    "incomplete_trade_evidence",
                    format!("option:{:?}", option.id),
                ));
            }
        }
    }
    if let Some(selected) = package.portfolio.selected_option {
        if !feasible.iter().any(|option| option.id == selected) {
            report.violations.push(violation(
                "selected_option_not_feasible",
                "portfolio.selected_option",
            ));
        }
    } else {
        report.violations.push(violation(
            "missing_selected_option",
            "portfolio.selected_option",
        ));
    }
    if package.portfolio.selection_rationale.is_empty() {
        report
            .violations
            .push(violation("missing_selection_rationale", "portfolio"));
    }

    let decision = package.portfolio.fingerprint().unwrap_or([0; 32]);
    let expected_artifact = artifact_identity(
        package.recipe_ref,
        decision,
        package.manifest.generator_profile,
    );
    if package.manifest.recipe_fingerprint != package.recipe_ref
        || package.manifest.decision_fingerprint != decision
        || package.manifest.artifact_id != expected_artifact
    {
        report
            .violations
            .push(violation("artifact_identity_mismatch", "manifest"));
    }
    let reference_ids: BTreeSet<Id> = package
        .manifest
        .references
        .iter()
        .map(|reference| reference.reference_id)
        .collect();
    if reference_ids.len() != package.manifest.references.len() {
        report
            .violations
            .push(violation("duplicate_reference", "manifest.references"));
    }
    for reference in &package.manifest.references {
        examine!();
        if !valid_locator(reference) {
            report.violations.push(violation(
                "hostile_or_unbound_reference",
                format!("reference:{:?}", reference.reference_id),
            ));
        }
    }
    let mut lineage = BTreeSet::from([package.manifest.artifact_id]);
    for derivative in &package.manifest.derivatives {
        examine!();
        if !lineage.contains(&derivative.parent_id) {
            report.violations.push(violation(
                "unknown_derivative_parent",
                format!("derivative:{:?}", derivative.id),
            ));
        }
        if !lineage.insert(derivative.id) {
            report.violations.push(violation(
                "duplicate_derivative",
                format!("derivative:{:?}", derivative.id),
            ));
        }
    }
    for repair in &package.manifest.repairs {
        examine!();
        if !lineage.contains(&repair.parent_id) {
            report.violations.push(violation(
                "unknown_repair_parent",
                format!("repair:{:?}", repair.id),
            ));
        }
        let allowed: BTreeSet<Id> = repair.allowed_scope.iter().copied().collect();
        if repair.changed_scope.iter().any(|id| !allowed.contains(id)) {
            report.violations.push(violation(
                "repair_scope_escape",
                format!("repair:{:?}", repair.id),
            ));
        }
        if repair.result == RepairResult::ValidatedCandidate && repair.candidate_id.is_none() {
            report.violations.push(violation(
                "missing_repair_candidate",
                format!("repair:{:?}", repair.id),
            ));
        }
        if let Some(candidate) = repair.candidate_id {
            if !lineage.contains(&candidate) {
                report.violations.push(violation(
                    "repair_candidate_outside_lineage",
                    format!("repair:{:?}", repair.id),
                ));
            }
        }
    }

    let region_ids: BTreeSet<Id> = package
        .materials
        .regions
        .iter()
        .map(|region| region.region_id)
        .collect();
    if region_ids.len() != package.materials.regions.len() {
        report
            .violations
            .push(violation("duplicate_material_region", "materials"));
    }
    for region in &package.materials.regions {
        examine!();
        if !nonzero(&region.source_role) || region.boundary_refs.is_empty() {
            report.violations.push(violation(
                "ungrounded_material_region",
                format!("region:{:?}", region.region_id),
            ));
        }
    }

    let frame_ids: BTreeSet<Id> = package
        .articulation
        .frames
        .iter()
        .map(|frame| frame.id)
        .collect();
    if frame_ids.len() != package.articulation.frames.len() {
        report
            .violations
            .push(violation("duplicate_frame", "articulation.frames"));
    }
    for frame in &package.articulation.frames {
        examine!();
        if !nonzero(&frame.source_role)
            || frame.linear_unit.is_empty()
            || frame.angular_unit.is_empty()
        {
            report.violations.push(violation(
                "incomplete_frame",
                format!("frame:{:?}", frame.id),
            ));
        }
    }
    for dof in &package.articulation.degrees_of_freedom {
        examine!();
        if !frame_ids.contains(&dof.frame_id)
            || !nonzero(&dof.source_role)
            || dof.unit.is_empty()
            || dof.minimum > dof.maximum
        {
            report.violations.push(violation(
                "invalid_degree_of_freedom",
                format!("dof:{:?}", dof.id),
            ));
        }
    }

    if package.temporal.importance_policy_version == 0 || package.temporal.request_epoch == 0 {
        report
            .violations
            .push(violation("invalid_importance_binding", "temporal"));
    }
    let mut previous: Option<&TemporalTierMapping> = None;
    let mut tiers = BTreeSet::new();
    if package.temporal.mappings.is_empty() {
        report
            .violations
            .push(violation("empty_temporal_map", "temporal.mappings"));
    }
    for mapping in &package.temporal.mappings {
        examine!();
        if !tiers.insert(mapping.tier) {
            report
                .violations
                .push(violation("duplicate_temporal_tier", "temporal.mappings"));
        }
        if let Some(prior) = previous {
            if mapping.tier <= prior.tier
                || mapping.fidelity_level < prior.fidelity_level
                || mapping.cadence_units < prior.cadence_units
            {
                report
                    .violations
                    .push(violation("non_monotone_temporal_map", "temporal.mappings"));
            }
        }
        if let Some(fallback) = mapping.fallback_tier {
            if fallback >= mapping.tier {
                report
                    .violations
                    .push(violation("invalid_temporal_fallback", "temporal.mappings"));
            }
        }
        previous = Some(mapping);
    }

    if package.review.artifact_ref != package.manifest.artifact_id
        || !nonzero(&package.review.conditions.renderer_profile_ref)
        || !nonzero(&package.review.conditions.camera_profile_ref)
        || !nonzero(&package.review.conditions.lighting_profile_ref)
        || !nonzero(&package.review.conditions.color_profile_ref)
        || package.review.conditions.assertion_refs.is_empty()
    {
        report
            .violations
            .push(violation("incomplete_review_conditions", "review"));
    }
    if !package.review.rendered_evidence_refs.is_empty() {
        report.violations.push(violation(
            "p7b_evidence_not_authorized",
            "review.rendered_evidence_refs",
        ));
    }

    report.violations.sort();
    if !report.violations.is_empty() {
        report.status = ValidationStatus::Invalid;
    }
    report
}
