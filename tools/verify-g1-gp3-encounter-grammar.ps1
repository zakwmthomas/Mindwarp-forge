$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
& (Join-Path $root 'tools\verify-g1-gp3-encounter-grammar-readiness.ps1')

$required = @(
    'docs\canonical-system\G1_GP3_ENCOUNTER_GRAMMAR_FIXED_REGISTRY.md',
    'crates\mindwarp-gameplay-foundation\src\encounter_grammar.rs',
    'crates\mindwarp-gameplay-foundation\tests\gp3_encounter_grammar.rs'
)
foreach ($relative in $required) {
    if (!(Test-Path -LiteralPath (Join-Path $root $relative) -PathType Leaf)) {
        throw "GP3 implementation evidence is missing: $relative"
    }
}

$source = Get-Content -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\src\encounter_grammar.rs') -Raw
$tests = Get-Content -LiteralPath (Join-Path $root 'crates\mindwarp-gameplay-foundation\tests\gp3_encounter_grammar.rs') -Raw
foreach ($token in @(
    'FIXED_SITUATION_DIGESTS', 'FIXED_GRAMMAR_DIGEST',
    'DomainFacetV1', 'EncounterEvidenceRefV1', 'EncounterRiskRefV1',
    'EncounterApproachV1', 'ApproachPrerequisiteV1', 'RiskDispositionKindV1',
    'ThreatContributionKindV1', 'CausalExplanationV1',
    'resolve_outcome', 'resolve_consequence', 'compose_optional_threat',
    'validate_approach_context', 'MAX_GRAMMAR_BYTES', 'MAX_SITUATION_BYTES',
    'mindwarp.gp3.fixed-situation.v1', 'mindwarp.gp3.fixed-grammar.v1',
    'temporary-brace-kit', 's4.approach.temporary', 's4.approach.long'
)) {
    if (!$source.Contains($token)) { throw "GP3 source is missing: $token" }
}
foreach ($token in @(
    'hostile_registry_and_session_identity_matrix_rejects_every_drift',
    'hostile_evidence_facet_and_risk_matrix_rejects_every_drift',
    'hostile_approach_step_prerequisite_and_explanation_matrix_rejects_drift',
    'hostile_risk_disposition_and_force_matrix_rejects_drift',
    'hostile_consequence_exact_once_matrix_rejects_drift',
    'hostile_threat_matrix_rejects_presence_identity_elements_and_authority_drift',
    's5_hostile_history_matrix_rejects_missing_stale_reordered_foreign_and_fabricated',
    'strict_codec_hostile_matrix_rejects_real_structural_and_resource_attacks',
    'fixed_tools_session_digests_and_forbidden_surface_are_exact'
)) {
    if (!$tests.Contains($token)) { throw "GP3 hostile matrix is missing: $token" }
}
foreach ($forbidden in @(
    'PENDING-', 'ProgressionLedgerV1', 'apply_progression', 'rand::',
    'std::fs', 'std::net', 'tauri::', 'forge_kernel::', 'Greenfield'
)) {
    if ($source.Contains($forbidden)) { throw "GP3 crosses its capability boundary: $forbidden" }
}

Push-Location $root
try {
    cargo test -p mindwarp-gameplay-foundation --test gp3_encounter_grammar
    if ($LASTEXITCODE -ne 0) { throw 'GP3 focused encounter grammar tests failed.' }
    cargo test -p mindwarp-gameplay-foundation --test gp2_progression
    if ($LASTEXITCODE -ne 0) { throw 'GP2 retained regression failed.' }
    cargo test -p mindwarp-gameplay-foundation --test gp1_base_loop
    if ($LASTEXITCODE -ne 0) { throw 'GP1 retained regression failed.' }
    cargo test -p mindwarp-gameplay-foundation --test gp0_contract
    if ($LASTEXITCODE -ne 0) { throw 'GP0 retained regression failed.' }
} finally { Pop-Location }

Write-Output 'G1 GP3 encounter grammar verified: five exact multi-facet authored situations, complete GP0 consequence references, noncombat/retreat/threat boundaries, exact predecessor authority and hostile codecs pass.'
