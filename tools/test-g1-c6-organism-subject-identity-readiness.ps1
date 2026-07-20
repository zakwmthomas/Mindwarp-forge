$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$temp = Join-Path ([IO.Path]::GetTempPath()) ('c6-organism-subject-identity-readiness-' + [guid]::NewGuid().ToString('N'))

try {
    New-Item -ItemType Directory -Path @(
        (Join-Path $temp 'docs\canonical-system'),
        (Join-Path $temp 'contracts'),
        (Join-Path $temp 'context\active'),
        (Join-Path $temp 'crates\person-form-eligibility\src')
    ) -Force | Out-Null

    $programSource = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
    $checkpointSource = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
    $readinessSource = Join-Path $root 'docs\canonical-system\G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_READINESS.md'
    $contractSource = Join-Path $root 'contracts\organism-subject-identity-contract.md'
    $programPath = Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json'
    $checkpointPath = Join-Path $temp 'context\active\WORKER_BATCH_STATE.json'
    $readinessPath = Join-Path $temp 'docs\canonical-system\G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_READINESS.md'
    $contractPath = Join-Path $temp 'contracts\organism-subject-identity-contract.md'
    $verify = Join-Path $root 'tools\verify-g1-c6-organism-subject-identity-readiness.ps1'

    function Reset-Fixture {
        Copy-Item $programSource $programPath -Force
        Copy-Item $checkpointSource $checkpointPath -Force
        Copy-Item $readinessSource $readinessPath -Force
        Copy-Item $contractSource $contractPath -Force
        Copy-Item (Join-Path $root 'Cargo.toml') (Join-Path $temp 'Cargo.toml') -Force
        Copy-Item (Join-Path $root 'crates\person-form-eligibility\Cargo.toml') (Join-Path $temp 'crates\person-form-eligibility\Cargo.toml') -Force
        Copy-Item (Join-Path $root 'crates\person-form-eligibility\src\lib.rs') (Join-Path $temp 'crates\person-form-eligibility\src\lib.rs') -Force
        $program = Get-Content -Raw $programPath | ConvertFrom-Json
        $c6 = @($program.items | Where-Object id -eq C6)[0]
        $c6.gate = 'design'
        $program | ConvertTo-Json -Depth 100 | Set-Content $programPath -Encoding utf8
        $checkpoint = Get-Content -Raw $checkpointPath | ConvertFrom-Json
        $checkpoint.batch_id = 'G1-C6-ORGANISM-IDENTITY-READINESS-V1'
        $checkpoint.substage_id = 'c6-organism-identity-readiness'
        $checkpoint.authority_lane = 'Owner-routed code-free C6 package-3 identity readiness only. Authorizes reconciliation of stale body-plan projections and design, adversarial review, fixtures, verifier and governance records for distinct lineage, organism-form, species-candidate, individual and population identity envelopes plus exact C4 lifecycle/history consumption. No production crate or source implementation; no asserted species membership, population members/count/distribution, ancestry/evolution inference, ecology, physiology, reproduction, heredity, development, sex, dimorphism, culture, representation, runtime, Companion, Greenfield, C7, promotion authority or Kernel mutation.'
        $checkpoint.verification_receipts = @($checkpoint.verification_receipts | Where-Object { $_ -ne 'owner-authorization:c6-organism-subject-identity-v1:released' })
        $checkpoint | ConvertTo-Json -Depth 100 | Set-Content $checkpointPath -Encoding utf8
        Remove-Item -LiteralPath (Join-Path $temp 'crates\organism-subject-identity') -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath (Join-Path $temp 'docs\canonical-system\G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_RESULT.md') -Force -ErrorAction SilentlyContinue
    }
    function Invoke-Fixture {
        & $verify -ProgramPath $programPath -CheckpointPath $checkpointPath -ReadinessPath $readinessPath -ContractPath $contractPath -RootPath $temp | Out-Null
    }
    function Assert-Rejected([string]$label, [scriptblock]$mutate) {
        Reset-Fixture
        & $mutate
        $failed = $false
        try { Invoke-Fixture } catch { $failed = $true }
        if (!$failed) { throw "Organism-subject identity readiness hostile passed: $label" }
    }

    Reset-Fixture
    Invoke-Fixture
    Assert-Rejected 'readiness authority drift' {
        $json = Get-Content -Raw $checkpointPath | ConvertFrom-Json
        $json.authority_lane += ' forged'
        $json | ConvertTo-Json -Depth 100 | Set-Content $checkpointPath -Encoding utf8
    }
    Assert-Rejected 'identity type collapse' {
        (Get-Content -Raw $readinessPath).Replace('LineageSubjectRefV1', 'IndividualIdentityV1') | Set-Content $readinessPath -Encoding utf8
    }
    Assert-Rejected 'label-derived identity leakage' {
        (Get-Content -Raw $contractPath).Replace('Identity fields contain no free-form biological label.', 'Identity fields contain a biological label.') | Set-Content $contractPath -Encoding utf8
    }
    Assert-Rejected 'population semantics leakage' {
        (Get-Content -Raw $contractPath).Replace('V1 contains no members, counts, weights,', 'V1 contains members, counts, weights,') | Set-Content $contractPath -Encoding utf8
    }
    Assert-Rejected 'cross-world reuse laundering' {
        (Get-Content -Raw $readinessPath).Replace('foreign-world candidate, form, individual, population, cohort or history reuse rejects', 'foreign-world reuse is accepted') | Set-Content $readinessPath -Encoding utf8
    }
    Assert-Rejected 'lineage ancestry laundering' {
        (Get-Content -Raw $readinessPath).Replace('optional macro-lineage parent cannot become descent or ancestry proof', 'optional macro-lineage parent proves ancestry') | Set-Content $readinessPath -Encoding utf8
    }
    Assert-Rejected 'C4 target mismatch laundering' {
        (Get-Content -Raw $contractPath).Replace('The history target must equal the individual ID.', 'The history target may differ from the individual ID.') | Set-Content $contractPath -Encoding utf8
    }
    Assert-Rejected 'source authorization receipt leakage' {
        $json = Get-Content -Raw $checkpointPath | ConvertFrom-Json
        $json.verification_receipts = @($json.verification_receipts) + 'owner-authorization:c6-organism-subject-identity-v1:released'
        $json | ConvertTo-Json -Depth 100 | Set-Content $checkpointPath -Encoding utf8
    }
    Assert-Rejected 'prospective Rust source exists' {
        New-Item -ItemType Directory -Path (Join-Path $temp 'crates\organism-subject-identity') -Force | Out-Null
    }
    Assert-Rejected 'sex dimorphism reproduction leakage' {
        (Get-Content -Raw $readinessPath).Replace('no ecology, physiology, sex, dimorphism, caste, reproduction, heredity, development', 'ecology, physiology, sex, dimorphism, caste, reproduction, heredity, development') | Set-Content $readinessPath -Encoding utf8
    }
    Assert-Rejected 'resource drift' {
        (Get-Content -Raw $contractPath).Replace('identity-layer validation examinations | 2,048', 'identity-layer validation examinations | 2,049') | Set-Content $contractPath -Encoding utf8
    }
    Assert-Rejected 'test-count drift' {
        (Get-Content -Raw $readinessPath).Replace('Exactly 33 test groups are required:', 'Exactly 32 test groups are required:') | Set-Content $readinessPath -Encoding utf8
    }
    Assert-Rejected 'appended normative contradiction' {
        Add-Content -LiteralPath $contractPath -Value 'Normative override: individual_id MUST be reminted whenever form_template_id or species_candidate_id changes.'
    }
    Assert-Rejected 'relocated hostile anchor' {
        (Get-Content -Raw $readinessPath).Replace('21. `C6-H502` inherited or changed biological delta claims are outside the schema and reject;', '21. placeholder reserved;') | Set-Content $readinessPath -Encoding utf8
        Add-Content -LiteralPath $readinessPath -Value 'Lookup only: 21. `C6-H502` inherited or changed biological delta claims are outside the schema and reject;'
    }
    Write-Output 'G1 C6 organism-subject identity readiness hostiles verified: route, type separation, label independence, unresolved population semantics, world/C4 binding, lineage non-laundering, source-negative authority, biology exclusions, resource bounds and test count fail closed.'
}
finally {
    Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue
}
