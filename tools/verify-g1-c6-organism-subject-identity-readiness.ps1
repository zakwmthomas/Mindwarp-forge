param(
    [string]$ProgramPath,
    [string]$CheckpointPath,
    [string]$ReadinessPath,
    [string]$ContractPath,
    [string]$RootPath
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')

if (!$RootPath) { $RootPath = $root }
if (!$ProgramPath) { $ProgramPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json' }
if (!$CheckpointPath) { $CheckpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json' }
if (!$ReadinessPath) { $ReadinessPath = Join-Path $root 'docs\canonical-system\G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_READINESS.md' }
if (!$ContractPath) { $ContractPath = Join-Path $root 'contracts\organism-subject-identity-contract.md' }

$program = Get-Content -Raw -LiteralPath $ProgramPath | ConvertFrom-Json
$checkpoint = Get-Content -Raw -LiteralPath $CheckpointPath | ConvertFrom-Json
$readiness = Get-Content -Raw -LiteralPath $ReadinessPath
$contract = Get-Content -Raw -LiteralPath $ContractPath

$readinessRoute = Test-G1C6OrganismIdentityReadinessRoute -Checkpoint $checkpoint
$implementationRoute = Test-G1C6OrganismSubjectIdentityImplementationRoute -Checkpoint $checkpoint
$ecologySchemaGapRoute = Test-G1C6EcologicalNicheSemanticsSchemaGapRoute -Checkpoint $checkpoint
if (!$readinessRoute -and !$implementationRoute -and !$ecologySchemaGapRoute) {
    throw 'C6 organism-subject identity readiness route is not exact.'
}

$c4 = @($program.items | Where-Object id -eq C4)
$c5 = @($program.items | Where-Object id -eq C5)
$c6 = @($program.items | Where-Object id -eq C6)
if ($c4.Count -ne 1 -or $c5.Count -ne 1 -or $c6.Count -ne 1) {
    throw 'C4-C6 program items are missing or ambiguous.'
}
if ($c4[0].state -ne 'verified' -or $c4[0].status -ne 'complete' -or
    $c5[0].state -ne 'verified' -or $c5[0].status -ne 'complete') {
    throw 'C6 organism-subject identity prerequisites are not verified and complete.'
}
$expectedGate = if ($readinessRoute -or $ecologySchemaGapRoute) { 'design' } else { 'implementation' }
if ($c6[0].state -ne 'executing' -or $c6[0].status -ne 'active' -or
    $c6[0].gate -ne $expectedGate -or (@($c6[0].depends_on) -join ',') -ne 'C4,C5') {
    throw 'C6 organism-subject identity master-program route is not exact.'
}
$active = @($program.items | Where-Object { $_.state -eq 'executing' -and $_.status -eq 'active' })
if ($active.Count -ne 1 -or $active[0].id -ne 'C6') { throw 'C6 is not the sole active cursor.' }

foreach ($source in @(
    'G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_READINESS.md',
    'organism-subject-identity-contract.md'
)) {
    if (@($c6[0].sources) -notcontains $source) { throw "C6 organism-subject identity source record missing: $source" }
}
foreach ($receipt in @(
    'receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded',
    'owner-route:c6-organism-identity-readiness:authorized'
)) {
    if (@($checkpoint.verification_receipts) -notcontains $receipt) { throw "C6 organism-subject identity readiness receipt missing: $receipt" }
}
if ($readinessRoute -and @($checkpoint.verification_receipts | Where-Object { $_ -match '^(owner-authorization|source-authorization):c6-organism-subject-identity' }).Count -ne 0) {
    throw 'C6 organism-subject identity source authorization exists during code-free readiness.'
}

foreach ($token in @(
    'Status: **implementation-ready behind one exact owner action; source absent.**',
    'Exactly 33 test groups are required:',
    'LineageSubjectRefV1', 'OrganismFormTemplateIdentityV1',
    'SpeciesCandidateIdentityV1', 'IndividualIdentityV1',
    'IndividualSubjectBindingV1', 'PopulationIdentityV1',
    'LifecycleHistorySubjectBindingV1',
    '`membership_status = unresolved`', '`person-form-eligibility`',
    '`C6-H400`', '`C6-H405`', '`C6-H500`', '`C6-H505`',
    '`C6-H1100`', '`C6-H1101`', '`C6-H1102`', '`C6-H1103`',
    '`C6-H1105`', '`C6-H1106`', '`C6-H1108`',
    'foreign-world candidate, form, individual, population, cohort or history reuse rejects',
    'optional macro-lineage parent cannot become descent or ancestry proof',
    'no ecology, physiology, sex, dimorphism, caste, reproduction, heredity, development',
    'No existing person-form bytes or test vectors may change.',
    'Implementation requires one new exact owner authorization',
    'Stop after the bounded implementation result is verified and recorded.'
)) {
    if (!$readiness.Contains($token)) { throw "C6 organism-subject identity readiness token missing: $token" }
}
$groupAnchors = @(
    '1. strict codec round trip',
    '2. all identity kinds and the receipt use distinct domains',
    '3. lineage-subject construction replays the exact macro-lineage',
    '4. form-template construction requires an exactly validated expression',
    '5. radial five/seven forms share lineage',
    '6. the withheld serial control validates alone',
    '7. species-candidate identity is label-free',
    '8. two individuals associated to one form template retain stable distinct individual identities',
    '9. population identities are species-candidate-neutral',
    '10. the exact ambient cohort binding must name the individual identity',
    '11. encoded C4 deltas replay to the exact received head',
    '12. the additive person-form consumer accepts only exact lineage/family bindings',
    '13. `C6-H400` lineage, template, species-candidate, individual and population cross-type collapse rejects',
    '14. `C6-H401` labels, aliases and presentation text cannot derive',
    '15. `C6-H402` absent, opaque-nonzero or asserted membership policy',
    '16. `C6-H403` a form template or species candidate cannot be used as an individual',
    '17. `C6-H404` population member/count/distribution injection',
    '18. `C6-H405` foreign-world candidate',
    '19. `C6-H500` optional macro-lineage parent cannot become descent',
    '20. `C6-H501` ancestry edge, cycle or time-order claims',
    '21. `C6-H502` inherited or changed biological delta claims',
    '22. `C6-H503` similarity of family, expression or codec cannot derive ancestry',
    '23. `C6-H504` hypothetical opportunity occupancy cannot become evolution',
    '24. `C6-H505` biological event identity without separately versioned provenance',
    '25. `C6-H1100` disconnected green component receipts',
    '26. `C6-H1101` substituted lineage, family, expression, cohort, baseline, delta, head',
    '27. `C6-H1102` any failure emits no partial identity',
    '28. `C6-H1103` exhausted identity/body examination budget is typed indeterminate',
    '29. `C6-H1105` native/i686/Android canonical vectors',
    '30. `C6-H1106` a desktop or reference receipt cannot claim runtime',
    '31. `C6-H1108` filesystem, network, process, clock, RNG, database',
    '32. every exact record, receipt, grounding, examination and inherited C4 maximum passes',
    '33. a static dependency, capability and vocabulary audit confirms no ecology'
)
foreach ($anchor in $groupAnchors) {
    if (!$readiness.Contains($anchor)) { throw "C6 organism-subject identity focused group drift: $anchor" }
}
$groupCount = [regex]::Matches($readiness, '(?m)^\d+\. ').Count
if ($groupCount -ne 33) { throw "C6 organism-subject identity readiness must contain exactly 33 numbered test groups, found $groupCount." }

foreach ($token in @(
    'Status: **code-free implementation candidate; no source authority.**',
    'mindwarp/c6-lineage-subject-ref/v1',
    'mindwarp/c6-organism-form-template/v1',
    'mindwarp/c6-species-candidate/v1',
    'mindwarp/c6-individual-identity/v1',
    'mindwarp/c6-individual-subject-binding/v1',
    'mindwarp/c6-population-identity/v1',
    'mindwarp/c6-lifecycle-history-subject-binding/v1',
    'mindwarp/c6-organism-subject-reference-receipt/v1',
    'canonical bytes per identity/binding record | 4,096',
    'canonical reference-receipt bytes | 32,768',
    'person-form capacity groundings at the sole consumer | 5',
    'identity-layer validation examinations | 2,048',
    'body-plan validation examinations | 4,096 inherited',
    'C4 baseline dependencies | 32 inherited',
    'C4 operation bytes | 65,536 inherited',
    'C4 recovery records | 1,024 inherited',
    'C4 recovery bytes | 16 MiB inherited',
    '`membership_status = unresolved`',
    'V1 contains no members, counts, weights,',
    'Identity fields contain no free-form biological label.',
    'The history target must equal the individual ID.',
    'stable identity is form-, expression-, lineage-',
    'species-candidate association or persistence policy.',
    'SHA-256(UTF8(domain) || 0x00 || semantic_preimage_bytes)',
    '## One consumer'
)) {
    if (!$contract.Contains($token)) { throw "C6 organism-subject identity contract token missing: $token" }
}

$preSourcePins = @{
    'contracts\organism-subject-identity-contract.md' = '9f831d6a9b429aedca9d611dbefe84a268a8fe94c55b8c86a341cb7fbca09180'
    'docs\canonical-system\G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_READINESS.md' = 'ac9b7bcb07e3ca05920f19e483afd7d8b6ead255a712ba82ccc5f59aad659e72'
}
foreach ($relative in $preSourcePins.Keys) {
    $path = Join-Path $RootPath $relative
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "C6 identity pre-source pin missing: $relative" }
    $actual = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -ne $preSourcePins[$relative]) { throw "C6 identity pre-source pin drift: $relative" }
}

if ($readinessRoute -and (Test-Path -LiteralPath (Join-Path $RootPath 'crates\organism-subject-identity'))) {
    throw 'Prospective organism-subject identity source exists before separate authorization.'
}
if ($readinessRoute -and (Test-Path -LiteralPath (Join-Path $RootPath 'docs\canonical-system\G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_RESULT.md'))) {
    throw 'Organism-subject identity implementation result exists during code-free readiness.'
}

Write-Output 'G1 C6 organism-subject identity readiness verified: exact route, seven typed records, unresolved membership, 33 test groups, resource envelope, C4 subject binding, one consumer and source-negative scope are frozen.'
