function Test-G1C6ReconciliationReadinessRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)

    $expectedAuthority = 'Owner-authorized C6 semantic/construction and organism-ecology reconciliation and capability-free readiness only. Exact dependencies verified C4 and C5. Retain corrected C6 prerequisite evidence as non-closure evidence. No C6 implementation source, C3B, C7, broad G1 closure, runtime, product ontology or vocabulary, solver or AI generation, geometry, assets, animation, renderer, visual-quality claim, physiology or content constants, filesystem, network, process, Companion, Greenfield, promotion authority or Kernel mutation.'
    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        if ($Checkpoint.$field -isnot [string]) { return $false }
    }
    return $Checkpoint.batch_id -eq 'G1-C6-SEMANTIC-CONSTRUCTION-ORGANISM-ECOLOGY-READINESS-V1' -and
        $Checkpoint.master_program_item -eq 'C6' -and
        $Checkpoint.state -eq 'executing' -and
        $Checkpoint.substage_id -eq 'c6-reconciliation-readiness' -and
        $Checkpoint.authority_lane -eq $expectedAuthority
}

function Test-G1C6BodyPlanStructureImplementationRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)

    $expectedAuthority = 'Owner-authorized capability-free C6 body-plan family/topology V1 test-first implementation only. Exact dependencies verified C4 and C5. Authorizes the new body-plan-structure crate, one additive macro-lineage-binding family-reference validator, exact tests, governance projections and verification for this package. No ecology realization, physiology, reproduction, heredity, development, sex or dimorphism applicability, caste, species, individual or population semantics, personhood, product ontology, solver or AI generation, geometry, proportions, pose, assets, animation, renderer, visual-quality claim, runtime, filesystem, network, process, Companion, Greenfield, C7, broad G1 closure, promotion authority or Kernel mutation.'
    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        if ($Checkpoint.$field -isnot [string]) { return $false }
    }
    return $Checkpoint.batch_id -eq 'G1-C6-BODY-PLAN-STRUCTURE-IMPLEMENTATION-V1' -and
        $Checkpoint.master_program_item -eq 'C6' -and
        $Checkpoint.state -eq 'executing' -and
        $Checkpoint.substage_id -eq 'c6-body-plan-structure-test-first-implementation' -and
        $Checkpoint.authority_lane -eq $expectedAuthority
}

function Test-G1C6OrganismIdentityReadinessRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)

    $expectedAuthority = 'Owner-routed code-free C6 package-3 identity readiness only. Authorizes reconciliation of stale body-plan projections and design, adversarial review, fixtures, verifier and governance records for distinct lineage, organism-form, species-candidate, individual and population identity envelopes plus exact C4 lifecycle/history consumption. No production crate or source implementation; no asserted species membership, population members/count/distribution, ancestry/evolution inference, ecology, physiology, reproduction, heredity, development, sex, dimorphism, culture, representation, runtime, Companion, Greenfield, C7, promotion authority or Kernel mutation.'
    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        if ($Checkpoint.$field -isnot [string]) { return $false }
    }
    return $Checkpoint.batch_id -eq 'G1-C6-ORGANISM-IDENTITY-READINESS-V1' -and
        $Checkpoint.master_program_item -eq 'C6' -and
        $Checkpoint.state -eq 'executing' -and
        $Checkpoint.substage_id -eq 'c6-organism-identity-readiness' -and
        $Checkpoint.authority_lane -eq $expectedAuthority
}

function Test-G1C6OrganismSubjectIdentityImplementationRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)

    $expectedAuthority = 'Owner-authorized capability-free C6 organism subject identity V1 test-first implementation only. Exact dependencies verified C4, C5 and body-plan V1. Authorizes the organism-subject-identity crate, one additive person-form-eligibility bound-subject evaluator, exact 33-group implementation matrix, module/governance projections and verification. No asserted species membership, population members/count/distribution, ancestry/evolution inference, ecology, physiology, reproduction, heredity, development, sex, dimorphism, caste, culture, capacity truth, comparison, representation, runtime, filesystem, network, process, Companion, Greenfield, C7, broad G1 closure, promotion authority or Kernel mutation.'
    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        if ($Checkpoint.$field -isnot [string]) { return $false }
    }
    return $Checkpoint.batch_id -eq 'G1-C6-ORGANISM-SUBJECT-IDENTITY-IMPLEMENTATION-V1' -and
        $Checkpoint.master_program_item -eq 'C6' -and
        $Checkpoint.state -eq 'executing' -and
        $Checkpoint.substage_id -eq 'c6-organism-subject-identity-test-first-implementation' -and
        $Checkpoint.authority_lane -eq $expectedAuthority
}

function Test-G1C6EcologicalNicheSemanticsSchemaGapRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)

    $expectedAuthority = 'Owner-routed code-free C6 package-4 ecological-niche semantics schema-gap audit only. Authorizes canonical status reconciliation, exact upstream field and authority inventory, claim classification, adversarial source-negative fixtures, verifier and governance records for habitat, resource, hazard, trophic, competition, ecotone and prospective-occupancy prerequisites. No ecological contract schema or production crate/source; no habitat suitability, resource yield, organism hazard, trophic or competition fact, realized occupancy, species or population membership, physiology, viability, senses, locomotion, behavior, reproduction, heredity, development, evolution, dimorphism applicability, comparison, culture, representation, runtime, filesystem, network, process, Companion, Greenfield, C7, broad G1 closure, promotion authority or Kernel mutation.'
    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        if ($Checkpoint.$field -isnot [string]) { return $false }
    }
    return $Checkpoint.batch_id -eq 'G1-C6-ECOLOGICAL-NICHE-SEMANTICS-SCHEMA-GAP-AUDIT-V1' -and
        $Checkpoint.master_program_item -eq 'C6' -and
        $Checkpoint.state -eq 'executing' -and
        $Checkpoint.substage_id -eq 'c6-ecological-niche-semantics-schema-gap-audit' -and
        $Checkpoint.authority_lane -eq $expectedAuthority
}

function Test-G1C6AuthorizedCurrentRoute {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)]$Checkpoint)
    return (Test-G1C6ReconciliationReadinessRoute -Checkpoint $Checkpoint) -or
        (Test-G1C6BodyPlanStructureImplementationRoute -Checkpoint $Checkpoint) -or
        (Test-G1C6OrganismIdentityReadinessRoute -Checkpoint $Checkpoint) -or
        (Test-G1C6OrganismSubjectIdentityImplementationRoute -Checkpoint $Checkpoint) -or
        (Test-G1C6EcologicalNicheSemanticsSchemaGapRoute -Checkpoint $Checkpoint)
}
