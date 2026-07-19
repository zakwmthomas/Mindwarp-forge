use body_plan_structure::*;

fn valid(report: ValidationReport) {
    assert_eq!(
        report.status,
        ValidationStatus::Valid,
        "{:?}",
        report.violations
    );
}

#[test]
fn g01_strict_codec_round_trip() {
    let f = reference_fixtures().unwrap().humanoid;
    let bytes = f.family.to_bytes().unwrap();
    assert_eq!(BodyPlanFamily::from_bytes(&bytes).unwrap(), f.family);
    let expression_bytes = f.expression.to_bytes().unwrap();
    assert_eq!(
        StructuralExpression::from_bytes(&expression_bytes).unwrap(),
        f.expression
    );
    let mut whitespace = b" ".to_vec();
    whitespace.extend(&bytes);
    assert!(BodyPlanFamily::from_bytes(&whitespace).is_err());
    let mut trailing = bytes.clone();
    trailing.push(b' ');
    assert!(BodyPlanFamily::from_bytes(&trailing).is_err());
    let unknown = String::from_utf8(bytes.clone())
        .unwrap()
        .replacen("{", "{\"sex\":1,", 1);
    assert!(BodyPlanFamily::from_bytes(unknown.as_bytes()).is_err());
    let duplicate = String::from_utf8(bytes)
        .unwrap()
        .replacen("{", "{\"schema_version\":1,", 1);
    assert!(BodyPlanFamily::from_bytes(duplicate.as_bytes()).is_err());
    let mut wrong_schema = f.family.clone();
    wrong_schema.schema_version = 2;
    assert!(wrong_schema.to_bytes().is_err());
    let mut reordered = f.family.clone();
    reordered.part_templates.reverse();
    let noncanonical = serde_json::to_vec(&reordered).unwrap();
    assert!(BodyPlanFamily::from_bytes(&noncanonical).is_err());
}

#[test]
fn g02_humanoid_vectors_are_deterministic() {
    let a = reference_fixtures().unwrap().humanoid;
    let b = reference_fixtures().unwrap().humanoid;
    assert_eq!(
        hex(a.family.family_id),
        "8b514e7a585efdd41f76479a0869e7907746c2b9f27b6cdcb7ef15df077549f6"
    );
    assert_eq!(
        hex(a.expression.expression_id),
        "8fc7a8940a765879fce56d3123cfd42a48b36c28953a253078d40b68dd1b1245"
    );
    let family_bytes_sha: Id = {
        use sha2::{Digest, Sha256};
        Sha256::digest(a.family.to_bytes().unwrap()).into()
    };
    assert_eq!(
        hex(family_bytes_sha),
        "f5dc902d4ad6c26cf1681a2b00aef4a655d2f3a4a92c8a0b3589ed4917d28a81"
    );
    assert_eq!(a, b);
    assert_eq!(a.family.family_id, a.family.fingerprint().unwrap());
    assert_eq!(
        a.expression.expression_id,
        a.expression.fingerprint().unwrap()
    );
}

#[test]
fn g03_radial_five_and_seven_share_family_only() {
    let r = reference_fixtures().unwrap().radial;
    assert_eq!(r.five.family_id, r.family.family_id);
    assert_eq!(r.seven.family_id, r.family.family_id);
    assert_ne!(r.five.expression_id, r.seven.expression_id);
    valid(validate_expression(
        &r.family,
        &r.five,
        MAX_VALIDATION_EXAMINATIONS,
    ));
    valid(validate_expression(
        &r.family,
        &r.seven,
        MAX_VALIDATION_EXAMINATIONS,
    ));
}

#[test]
fn g04_family_identity_is_expression_independent() {
    let fixtures = reference_fixtures().unwrap();
    let h = fixtures.humanoid;
    let mut alternate = StructuralExpressionDefinition::from(&h.expression);
    alternate.active_predicate_ids.clear();
    alternate
        .occurrences
        .retain(|o| o.template_id != id_from_u16(5));
    alternate.occurrences.push(PartOccurrence {
        occurrence_id: id_from_u16(104),
        template_id: id_from_u16(4),
    });
    alternate
        .relation_instances
        .retain(|r| r.rule_id != id_from_u16(14));
    alternate.relation_instances.push(RelationInstance {
        rule_id: id_from_u16(13),
        from_occurrence_id: id_from_u16(101),
        to_occurrence_id: id_from_u16(104),
    });
    let alternate = build_expression(&h.family, alternate).unwrap();
    assert_eq!(alternate.family_id, h.family.family_id);
    assert_ne!(alternate.expression_id, h.expression.expression_id);

    let r = fixtures.radial;
    assert_eq!(r.family.family_id, r.five.family_id);
    let mut changed = StructuralExpressionDefinition::from(&r.five);
    changed.limitations.push("non-semantic note".into());
    assert_eq!(
        build_expression(&r.family, changed).unwrap().expression_id,
        r.five.expression_id
    );
    let mut structural = StructuralExpressionDefinition::from(&r.five);
    let prior = structural.occurrences[0].occurrence_id;
    structural.occurrences[0].occurrence_id = id_from_u16(65000);
    for relation in &mut structural.relation_instances {
        if relation.from_occurrence_id == prior {
            relation.from_occurrence_id = id_from_u16(65000);
        }
        if relation.to_occurrence_id == prior {
            relation.to_occurrence_id = id_from_u16(65000);
        }
    }
    for position in &mut structural.symmetry_positions {
        if position.occurrence_id == prior {
            position.occurrence_id = id_from_u16(65000);
        }
    }
    assert_ne!(
        build_expression(&r.family, structural)
            .unwrap()
            .expression_id,
        r.five.expression_id
    );
}

#[test]
fn g05_permutations_canonicalize_duplicates_reject() {
    let f = reference_fixtures().unwrap().humanoid.family;
    let mut d = BodyPlanFamilyDefinition::from(&f);
    d.part_templates.reverse();
    d.relation_rules.reverse();
    let permuted = build_family(d).unwrap();
    assert_eq!(permuted.family_id, f.family_id);
    assert_eq!(permuted.to_bytes().unwrap(), f.to_bytes().unwrap());
    let mut public_permutation = f.clone();
    public_permutation.part_templates.reverse();
    public_permutation.relation_rules.reverse();
    assert_eq!(public_permutation.fingerprint().unwrap(), f.family_id);
    let expression = reference_fixtures().unwrap().humanoid.expression;
    let mut expression_permutation = expression.clone();
    expression_permutation.occurrences.reverse();
    expression_permutation.relation_instances.reverse();
    expression_permutation.symmetry_positions.reverse();
    assert_eq!(
        expression_permutation.fingerprint().unwrap(),
        expression.expression_id
    );
    let mut bad = BodyPlanFamilyDefinition::from(&f);
    bad.part_templates.push(bad.part_templates[0].clone());
    assert!(build_family(bad).is_err());
}

#[test]
fn g06_required_optional_conditional_selection() {
    let r = reference_fixtures().unwrap().radial;
    valid(validate_expression(
        &r.family,
        &r.five,
        MAX_VALIDATION_EXAMINATIONS,
    ));
    let mut bad = r.five.clone();
    bad.active_predicate_ids.clear();
    assert_eq!(
        validate_expression(&r.family, &bad, MAX_VALIDATION_EXAMINATIONS).status,
        ValidationStatus::Invalid
    );
    let h = reference_fixtures().unwrap().humanoid;
    let mut inactive = StructuralExpressionDefinition::from(&h.expression);
    inactive.active_predicate_ids.clear();
    inactive
        .occurrences
        .retain(|o| o.template_id != id_from_u16(5));
    inactive
        .relation_instances
        .retain(|r| r.rule_id != id_from_u16(14));
    assert!(build_expression(&h.family, inactive.clone()).is_ok());
    let mut active_missing = inactive;
    active_missing.active_predicate_ids.push(id_from_u16(501));
    assert!(build_expression(&h.family, active_missing).is_err());
    let mut required_missing = StructuralExpressionDefinition::from(&h.expression);
    required_missing
        .occurrences
        .retain(|o| o.template_id != id_from_u16(2));
    required_missing
        .relation_instances
        .retain(|r| r.rule_id != id_from_u16(11));
    required_missing
        .symmetry_positions
        .retain(|p| p.occurrence_id != id_from_u16(102));
    assert!(build_expression(&h.family, required_missing).is_err());
}

#[test]
fn g08_withheld_serial_rejects_family_transfer() {
    let x = reference_fixtures().unwrap();
    valid(validate_expression(
        &x.withheld.family,
        &x.withheld.expression,
        MAX_VALIDATION_EXAMINATIONS,
    ));
    let mut transferred = x.withheld.expression.clone();
    transferred.family_id = x.humanoid.family.family_id;
    transferred.expression_id = transferred.fingerprint().unwrap();
    assert_eq!(
        validate_expression(
            &x.humanoid.family,
            &transferred,
            MAX_VALIDATION_EXAMINATIONS
        )
        .status,
        ValidationStatus::Invalid
    );
}

#[test]
fn g09_h200_reference_laundering_rejects() {
    let f = reference_fixtures().unwrap().humanoid.family;
    assert_eq!(
        validate_body_plan_ref(id_from_u16(999), &f, MAX_VALIDATION_EXAMINATIONS).status,
        ValidationStatus::Invalid
    );
    valid(validate_body_plan_ref(
        f.family_id,
        &f,
        MAX_VALIDATION_EXAMINATIONS,
    ));
    let family_work = validate_family(&f, MAX_VALIDATION_EXAMINATIONS).examined;
    assert_eq!(
        validate_body_plan_ref(f.family_id, &f, family_work).status,
        ValidationStatus::IndeterminateBudget
    );
    assert_eq!(
        validate_body_plan_ref(f.family_id, &f, family_work + 1).status,
        ValidationStatus::Valid
    );
}

#[test]
fn g10_h201_dangling_and_disconnected_reject() {
    let h = reference_fixtures().unwrap().humanoid;
    let mut bad = h.expression.clone();
    bad.relation_instances.clear();
    assert_eq!(
        validate_expression(&h.family, &bad, MAX_VALIDATION_EXAMINATIONS).status,
        ValidationStatus::Invalid
    );
    let mut dangling = h.expression.clone();
    dangling.occurrences[0].template_id = id_from_u16(65500);
    assert_eq!(
        validate_expression(&h.family, &dangling, MAX_VALIDATION_EXAMINATIONS).status,
        ValidationStatus::Invalid
    );
    let mut dangling_relation = h.expression.clone();
    dangling_relation.relation_instances[0].to_occurrence_id = id_from_u16(65501);
    dangling_relation.expression_id = dangling_relation.fingerprint().unwrap();
    assert_eq!(
        validate_expression(&h.family, &dangling_relation, MAX_VALIDATION_EXAMINATIONS).status,
        ValidationStatus::Invalid
    );
    let mut dangling_symmetry = h.expression.clone();
    dangling_symmetry.symmetry_positions[0].declaration_id = id_from_u16(65502);
    dangling_symmetry.expression_id = dangling_symmetry.fingerprint().unwrap();
    assert_eq!(
        validate_expression(&h.family, &dangling_symmetry, MAX_VALIDATION_EXAMINATIONS).status,
        ValidationStatus::Invalid
    );
}

#[test]
fn g11_h202_topology_homology_symmetry_contradiction() {
    let f = reference_fixtures().unwrap().humanoid.family;
    let mut bad = BodyPlanFamilyDefinition::from(&f);
    bad.homology_groups[0].member_template_ids.truncate(1);
    assert!(build_family(bad).is_err());
    let h = reference_fixtures().unwrap().humanoid;
    let mut symmetry = h.expression.clone();
    symmetry.symmetry_positions[1].position = 3;
    symmetry.expression_id = symmetry.fingerprint().unwrap();
    assert_eq!(
        validate_expression(&h.family, &symmetry, MAX_VALIDATION_EXAMINATIONS).status,
        ValidationStatus::Invalid
    );

    let cyclic_family = build_family(BodyPlanFamilyDefinition {
        part_templates: vec![
            PartTemplate {
                template_id: id_from_u16(8000),
                role_id: id_from_u16(8001),
                cardinality: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                presence: PresenceRule::Unconditional,
            },
            PartTemplate {
                template_id: id_from_u16(8002),
                role_id: id_from_u16(8003),
                cardinality: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                presence: PresenceRule::Unconditional,
            },
        ],
        relation_rules: vec![
            RelationRule {
                rule_id: id_from_u16(8010),
                kind: RelationKind::Containment,
                from_template_id: id_from_u16(8000),
                to_template_id: id_from_u16(8002),
                from_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
            RelationRule {
                rule_id: id_from_u16(8011),
                kind: RelationKind::Containment,
                from_template_id: id_from_u16(8002),
                to_template_id: id_from_u16(8000),
                from_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
        ],
        homology_groups: vec![],
        symmetry_declarations: vec![],
        limitations: vec![],
    })
    .unwrap();
    let cyclic = StructuralExpressionDefinition {
        family_id: cyclic_family.family_id,
        active_predicate_ids: vec![],
        occurrences: vec![
            PartOccurrence {
                occurrence_id: id_from_u16(8100),
                template_id: id_from_u16(8000),
            },
            PartOccurrence {
                occurrence_id: id_from_u16(8101),
                template_id: id_from_u16(8002),
            },
        ],
        relation_instances: vec![
            RelationInstance {
                rule_id: id_from_u16(8010),
                from_occurrence_id: id_from_u16(8100),
                to_occurrence_id: id_from_u16(8101),
            },
            RelationInstance {
                rule_id: id_from_u16(8011),
                from_occurrence_id: id_from_u16(8101),
                to_occurrence_id: id_from_u16(8100),
            },
        ],
        symmetry_positions: vec![],
        limitations: vec![],
    };
    assert!(build_expression(&cyclic_family, cyclic).is_err());

    let legal_non_tree_family = build_family(BodyPlanFamilyDefinition {
        part_templates: (0..4)
            .map(|n| PartTemplate {
                template_id: id_from_u16(8200 + n),
                role_id: id_from_u16(8210 + n),
                cardinality: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                presence: PresenceRule::Unconditional,
            })
            .collect(),
        relation_rules: vec![
            RelationRule {
                rule_id: id_from_u16(8230),
                kind: RelationKind::Containment,
                from_template_id: id_from_u16(8200),
                to_template_id: id_from_u16(8201),
                from_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
            RelationRule {
                rule_id: id_from_u16(8231),
                kind: RelationKind::Containment,
                from_template_id: id_from_u16(8202),
                to_template_id: id_from_u16(8203),
                from_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
            RelationRule {
                rule_id: id_from_u16(8232),
                kind: RelationKind::StructuralConnection,
                from_template_id: id_from_u16(8201),
                to_template_id: id_from_u16(8203),
                from_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
            RelationRule {
                rule_id: id_from_u16(8233),
                kind: RelationKind::StructuralConnection,
                from_template_id: id_from_u16(8203),
                to_template_id: id_from_u16(8200),
                from_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
            RelationRule {
                rule_id: id_from_u16(8234),
                kind: RelationKind::StructuralConnection,
                from_template_id: id_from_u16(8200),
                to_template_id: id_from_u16(8201),
                from_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
        ],
        homology_groups: vec![],
        symmetry_declarations: vec![],
        limitations: vec![],
    })
    .unwrap();
    let legal_non_tree = StructuralExpressionDefinition {
        family_id: legal_non_tree_family.family_id,
        active_predicate_ids: vec![],
        occurrences: (0..4)
            .map(|n| PartOccurrence {
                occurrence_id: id_from_u16(8250 + n),
                template_id: id_from_u16(8200 + n),
            })
            .collect(),
        relation_instances: vec![
            RelationInstance {
                rule_id: id_from_u16(8230),
                from_occurrence_id: id_from_u16(8250),
                to_occurrence_id: id_from_u16(8251),
            },
            RelationInstance {
                rule_id: id_from_u16(8231),
                from_occurrence_id: id_from_u16(8252),
                to_occurrence_id: id_from_u16(8253),
            },
            RelationInstance {
                rule_id: id_from_u16(8232),
                from_occurrence_id: id_from_u16(8251),
                to_occurrence_id: id_from_u16(8253),
            },
            RelationInstance {
                rule_id: id_from_u16(8233),
                from_occurrence_id: id_from_u16(8253),
                to_occurrence_id: id_from_u16(8250),
            },
            RelationInstance {
                rule_id: id_from_u16(8234),
                from_occurrence_id: id_from_u16(8250),
                to_occurrence_id: id_from_u16(8251),
            },
        ],
        symmetry_positions: vec![],
        limitations: vec![],
    };
    assert!(build_expression(&legal_non_tree_family, legal_non_tree).is_ok());
}

#[test]
fn g12_h203_capability_fields_reject() {
    let bytes = reference_fixtures()
        .unwrap()
        .humanoid
        .family
        .to_bytes()
        .unwrap();
    for key in ["capacity", "sense", "fitness", "locomotion"] {
        let injected = String::from_utf8(bytes.clone()).unwrap().replacen(
            "{",
            &format!("{{\"{key}\":true,"),
            1,
        );
        assert!(BodyPlanFamily::from_bytes(injected.as_bytes()).is_err());
    }
}

#[test]
fn g13_h204_geometry_and_root_fields_reject() {
    let bytes = reference_fixtures()
        .unwrap()
        .humanoid
        .family
        .to_bytes()
        .unwrap();
    for key in [
        "coordinates",
        "dimensions",
        "proportions",
        "pose",
        "root",
        "pelvis",
    ] {
        let injected =
            String::from_utf8(bytes.clone())
                .unwrap()
                .replacen("{", &format!("{{\"{key}\":0,"), 1);
        assert!(BodyPlanFamily::from_bytes(injected.as_bytes()).is_err());
    }
}

#[test]
fn g14_h205_reordering_stable_structural_change_sensitive() {
    let f = reference_fixtures().unwrap().humanoid.family;
    let mut reordered = BodyPlanFamilyDefinition::from(&f);
    reordered.part_templates.reverse();
    assert_eq!(build_family(reordered).unwrap().family_id, f.family_id);
    let mut changed = BodyPlanFamilyDefinition::from(&f);
    changed.part_templates[0].role_id = id_from_u16(64000);
    assert_ne!(build_family(changed).unwrap().family_id, f.family_id);
    let mut limitation_only = BodyPlanFamilyDefinition::from(&f);
    limitation_only
        .limitations
        .push("different non-semantic note".into());
    assert_eq!(
        build_family(limitation_only).unwrap().family_id,
        f.family_id
    );
}

#[test]
fn g15_dimorphism_and_social_label_injection_rejects() {
    let bytes = reference_fixtures()
        .unwrap()
        .humanoid
        .family
        .to_bytes()
        .unwrap();
    for key in [
        "sex",
        "gender",
        "reproductive_role",
        "dimorphism",
        "caste",
        "personality",
        "rank",
    ] {
        let injected = String::from_utf8(bytes.clone()).unwrap().replacen(
            "{",
            &format!("{{\"{key}\":null,"),
            1,
        );
        assert!(BodyPlanFamily::from_bytes(injected.as_bytes()).is_err());
    }
}

#[test]
fn g16_identity_fabrication_fields_reject() {
    let bytes = reference_fixtures()
        .unwrap()
        .humanoid
        .family
        .to_bytes()
        .unwrap();
    for key in ["lineage", "species", "individual", "population"] {
        let injected = String::from_utf8(bytes.clone()).unwrap().replacen(
            "{",
            &format!("{{\"{key}\":null,"),
            1,
        );
        assert!(BodyPlanFamily::from_bytes(injected.as_bytes()).is_err());
    }
}

#[test]
fn g17_resource_boundaries_and_budget() {
    let mut maximum_family = BodyPlanFamilyDefinition::empty();
    for n in 1..=MAX_PART_TEMPLATES {
        maximum_family
            .part_templates
            .push(optional_template(n as u16));
    }
    for n in 0..MAX_RELATION_RULES {
        maximum_family.relation_rules.push(RelationRule {
            rule_id: id_from_u16(1000 + n as u16),
            kind: RelationKind::StructuralConnection,
            from_template_id: id_from_u16(1),
            to_template_id: id_from_u16(2),
            from_degree: Cardinality {
                minimum: 0,
                maximum: 1,
            },
            to_degree: Cardinality {
                minimum: 0,
                maximum: 1,
            },
        });
    }
    for n in 0..MAX_HOMOLOGY_GROUPS {
        maximum_family.homology_groups.push(HomologyGroup {
            group_id: id_from_u16(2000 + n as u16),
            member_template_ids: vec![id_from_u16(1), id_from_u16(2)],
        });
    }
    for n in 0..MAX_SYMMETRY_DECLARATIONS {
        maximum_family
            .symmetry_declarations
            .push(SymmetryDeclaration {
                declaration_id: id_from_u16(3000 + n as u16),
                pattern: SymmetryPattern::NoDeclaredSymmetry,
                member_template_ids: vec![],
            });
    }
    maximum_family.limitations = (0..MAX_LIMITATIONS)
        .map(|n| format!("{n:02}{}", "x".repeat(MAX_LIMITATION_BYTES - 2)))
        .collect();
    let maximum_family_record = build_family(maximum_family.clone()).unwrap();
    valid(validate_family(
        &maximum_family_record,
        MAX_VALIDATION_EXAMINATIONS,
    ));

    let mut over = maximum_family.clone();
    over.part_templates.push(optional_template(65000));
    assert!(matches!(
        build_family(over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = maximum_family.clone();
    over.relation_rules.push(over.relation_rules[0].clone());
    assert!(matches!(
        build_family(over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = maximum_family.clone();
    over.homology_groups.push(over.homology_groups[0].clone());
    assert!(matches!(
        build_family(over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = maximum_family.clone();
    over.symmetry_declarations
        .push(over.symmetry_declarations[0].clone());
    assert!(matches!(
        build_family(over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = maximum_family.clone();
    over.limitations.push(String::new());
    assert!(matches!(
        build_family(over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = BodyPlanFamilyDefinition::empty();
    over.part_templates.push(optional_template(1));
    over.limitations.push("x".repeat(MAX_LIMITATION_BYTES + 1));
    assert!(matches!(
        build_family(over),
        Err(BodyPlanError::ResourceLimit(_))
    ));

    let expression_family = build_family(BodyPlanFamilyDefinition {
        part_templates: vec![
            PartTemplate {
                template_id: id_from_u16(4000),
                role_id: id_from_u16(4001),
                cardinality: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
                presence: PresenceRule::Unconditional,
            },
            PartTemplate {
                template_id: id_from_u16(4002),
                role_id: id_from_u16(4003),
                cardinality: Cardinality {
                    minimum: 255,
                    maximum: 255,
                },
                presence: PresenceRule::Unconditional,
            },
        ],
        relation_rules: vec![
            RelationRule {
                rule_id: id_from_u16(4010),
                kind: RelationKind::StructuralConnection,
                from_template_id: id_from_u16(4000),
                to_template_id: id_from_u16(4002),
                from_degree: Cardinality {
                    minimum: 255,
                    maximum: 255,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
            RelationRule {
                rule_id: id_from_u16(4011),
                kind: RelationKind::StructuralConnection,
                from_template_id: id_from_u16(4000),
                to_template_id: id_from_u16(4002),
                from_degree: Cardinality {
                    minimum: 255,
                    maximum: 255,
                },
                to_degree: Cardinality {
                    minimum: 1,
                    maximum: 1,
                },
            },
            RelationRule {
                rule_id: id_from_u16(4012),
                kind: RelationKind::StructuralConnection,
                from_template_id: id_from_u16(4000),
                to_template_id: id_from_u16(4002),
                from_degree: Cardinality {
                    minimum: 2,
                    maximum: 2,
                },
                to_degree: Cardinality {
                    minimum: 0,
                    maximum: 1,
                },
            },
        ],
        homology_groups: vec![],
        symmetry_declarations: vec![SymmetryDeclaration {
            declaration_id: id_from_u16(4020),
            pattern: SymmetryPattern::OtherDeclared {
                pattern_ref: id_from_u16(4021),
                positions: 256,
            },
            member_template_ids: vec![id_from_u16(4000), id_from_u16(4002)],
        }],
        limitations: vec![],
    })
    .unwrap();
    let mut maximum_expression = StructuralExpressionDefinition {
        family_id: expression_family.family_id,
        active_predicate_ids: vec![],
        occurrences: vec![PartOccurrence {
            occurrence_id: id_from_u16(5000),
            template_id: id_from_u16(4000),
        }],
        relation_instances: vec![],
        symmetry_positions: vec![SymmetryPosition {
            declaration_id: id_from_u16(4020),
            occurrence_id: id_from_u16(5000),
            position: 0,
        }],
        limitations: (0..MAX_LIMITATIONS)
            .map(|n| format!("{n:02}{}", "x".repeat(MAX_LIMITATION_BYTES - 2)))
            .collect(),
    };
    for n in 0..255u16 {
        let occurrence_id = id_from_u16(5001 + n);
        maximum_expression.occurrences.push(PartOccurrence {
            occurrence_id,
            template_id: id_from_u16(4002),
        });
        maximum_expression
            .relation_instances
            .push(RelationInstance {
                rule_id: id_from_u16(4010),
                from_occurrence_id: id_from_u16(5000),
                to_occurrence_id: occurrence_id,
            });
        maximum_expression
            .relation_instances
            .push(RelationInstance {
                rule_id: id_from_u16(4011),
                from_occurrence_id: id_from_u16(5000),
                to_occurrence_id: occurrence_id,
            });
        if n < 2 {
            maximum_expression
                .relation_instances
                .push(RelationInstance {
                    rule_id: id_from_u16(4012),
                    from_occurrence_id: id_from_u16(5000),
                    to_occurrence_id: occurrence_id,
                });
        }
        maximum_expression
            .symmetry_positions
            .push(SymmetryPosition {
                declaration_id: id_from_u16(4020),
                occurrence_id,
                position: n + 1,
            });
    }
    assert_eq!(maximum_expression.occurrences.len(), MAX_OCCURRENCES);
    assert_eq!(
        maximum_expression.relation_instances.len(),
        MAX_RELATION_INSTANCES
    );
    assert_eq!(
        maximum_expression.symmetry_positions.len(),
        MAX_SYMMETRY_POSITIONS
    );
    let maximum_expression_record =
        build_expression(&expression_family, maximum_expression.clone()).unwrap();
    let maximum_report = validate_expression(
        &expression_family,
        &maximum_expression_record,
        MAX_VALIDATION_EXAMINATIONS,
    );
    valid(maximum_report.clone());
    assert_eq!(
        validate_expression(
            &expression_family,
            &maximum_expression_record,
            maximum_report.examined - 1
        )
        .status,
        ValidationStatus::IndeterminateBudget
    );
    let mut over = maximum_expression.clone();
    over.occurrences.push(over.occurrences[0].clone());
    assert!(matches!(
        build_expression(&expression_family, over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = maximum_expression.clone();
    over.relation_instances
        .push(over.relation_instances[0].clone());
    assert!(matches!(
        build_expression(&expression_family, over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = maximum_expression.clone();
    over.symmetry_positions
        .push(over.symmetry_positions[0].clone());
    assert!(matches!(
        build_expression(&expression_family, over),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let mut over = maximum_expression.clone();
    over.limitations.push(String::new());
    assert!(matches!(
        build_expression(&expression_family, over),
        Err(BodyPlanError::ResourceLimit(_))
    ));

    let mut predicate_family = BodyPlanFamilyDefinition::empty();
    for n in 0..MAX_ACTIVE_PREDICATES {
        predicate_family.part_templates.push(PartTemplate {
            template_id: id_from_u16(6000 + n as u16),
            role_id: id_from_u16(6100 + n as u16),
            cardinality: Cardinality {
                minimum: 0,
                maximum: 1,
            },
            presence: PresenceRule::Conditional {
                predicate_id: id_from_u16(6200 + n as u16),
            },
        });
    }
    let predicate_family = build_family(predicate_family).unwrap();
    let predicates: Vec<_> = (0..MAX_ACTIVE_PREDICATES)
        .map(|n| id_from_u16(6200 + n as u16))
        .collect();
    let predicate_expression = StructuralExpressionDefinition {
        family_id: predicate_family.family_id,
        active_predicate_ids: predicates.clone(),
        occurrences: vec![PartOccurrence {
            occurrence_id: id_from_u16(6300),
            template_id: id_from_u16(6000),
        }],
        relation_instances: vec![],
        symmetry_positions: vec![],
        limitations: vec![],
    };
    assert!(build_expression(&predicate_family, predicate_expression.clone()).is_ok());
    let mut over = predicate_expression;
    over.active_predicate_ids.push(id_from_u16(65000));
    assert!(matches!(
        build_expression(&predicate_family, over),
        Err(BodyPlanError::ResourceLimit(_))
    ));

    assert!(maximum_family_record.to_bytes().unwrap().len() <= MAX_CANONICAL_BYTES);
    assert!(matches!(
        BodyPlanFamily::from_bytes(&vec![b' '; MAX_CANONICAL_BYTES + 1]),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    assert!(matches!(
        StructuralExpression::from_bytes(&vec![b' '; MAX_CANONICAL_BYTES + 1]),
        Err(BodyPlanError::ResourceLimit(_))
    ));
    let f = reference_fixtures().unwrap().humanoid.family;
    assert_eq!(
        validate_family(&f, 0).status,
        ValidationStatus::IndeterminateBudget
    );
}

#[test]
fn g18_receipt_is_authority_negative() {
    let r = reference_proof_receipt().unwrap();
    assert_eq!(r, reference_proof_receipt().unwrap());
    let fixtures = reference_fixtures().unwrap();
    assert_eq!(r.schema_version, CONTRACT_VERSION);
    assert_eq!(r.humanoid_family_id, fixtures.humanoid.family.family_id);
    assert_eq!(r.radial_family_id, fixtures.radial.family.family_id);
    assert_eq!(r.withheld_family_id, fixtures.withheld.family.family_id);
    let mut radial_ids = vec![
        fixtures.radial.five.expression_id,
        fixtures.radial.seven.expression_id,
    ];
    radial_ids.sort();
    assert_eq!(r.radial_expression_ids, radial_ids);
    assert_eq!(
        hex(r.fingerprint().unwrap()),
        "fdbdc35205fb0c955c7e436ab53bba60d0bcb61bbf1ca95d4ec7fb3c528ae529"
    );
    assert_eq!(
        hex(r.fixture_suite_id),
        "2828dc8b2e3ffa1c80ce1b1e047f8490474cca8ac8933385e9eb7cef293d01bb"
    );
    assert_eq!(
        hex(r.hostile_registry_digest),
        "7c1a4b981eb6471272a0fa39e3bacf6a41b6d2f4bab27dddd678deb03aac1257"
    );
    assert_eq!(r.validation_examinations, 266);
    assert!(r.capabilities.is_empty());
    assert!(
        !r.approved && !r.promoted && !r.runtime_authority && !r.geometry_truth && !r.biology_truth
    );
    let manifest = include_str!("../Cargo.toml");
    let dependencies = manifest
        .split("[dependencies]")
        .nth(1)
        .unwrap()
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('['))
        .collect::<Vec<_>>();
    assert_eq!(dependencies.len(), 3);
    for dependency in ["serde =", "serde_json =", "sha2 ="] {
        assert!(
            dependencies
                .iter()
                .any(|line| line.trim().starts_with(dependency))
        );
    }
    let source = include_str!("../src/lib.rs");
    let normalized = source.split_whitespace().collect::<String>();
    let forbidden = [
        ["std", "::", "fs"].concat(),
        ["std", "::", "net"].concat(),
        ["std", "::", "process"].concat(),
        ["std", "::", "time"].concat(),
        ["forge", "_kernel"].concat(),
        ["tau", "ri"].concat(),
        ["ra", "nd", "::"].concat(),
        ["get", "random"].concat(),
        ["render", "er"].concat(),
    ];
    for token in &forbidden {
        assert!(
            !normalized.contains(token),
            "forbidden capability surface: {token}"
        );
    }
    for alias_surface in [
        ["use", "std", "as"].concat(),
        ["use", "::", "std", "as"].concat(),
        ["extern", "crate", "std", "as"].concat(),
    ] {
        assert!(
            !normalized.contains(&alias_surface),
            "forbidden std alias surface"
        );
    }
    if normalized.contains(&["std", "::{"].concat()) {
        for member in ["fs", "net", "process", "time"] {
            assert!(
                !normalized.contains(member),
                "forbidden grouped std import: {member}"
            );
        }
    }
}
