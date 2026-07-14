$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$auditPath = Join-Path $root 'docs\canonical-system\FORGE_WIDE_NATURAL_FUNCTION_REASSESSMENT.md'
$resultPath = Join-Path $root 'docs\canonical-system\GROUNDED_SEARCH_CASE_RESULT.md'
$cratePath = Join-Path $root 'crates\grounded-search-protocol'
$manifestPath = Join-Path $cratePath 'Cargo.toml'
$sourcePath = Join-Path $cratePath 'src\lib.rs'

foreach ($path in @($auditPath, $resultPath, $manifestPath, $sourcePath)) {
    if (!(Test-Path -LiteralPath $path)) { throw "Natural-function reassessment artifact missing: $path" }
}

$audit = Get-Content -LiteralPath $auditPath -Raw
foreach ($required in @(
    'tested method library',
    'Full-plan consolidation',
    'Imported compendium disposition',
    'Forge manifestation atlas',
    'Game manifestation atlas',
    'Philosophy compatibility',
    'Selective aging trace',
    'H5 is',
    'no universal fitness score',
    'Real-money authority requires a server-side trusted boundary'
)) {
    if (!$audit.Contains($required)) { throw "Natural-function audit missing: $required" }
}

$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
    'Semantic deceptive minimum',
    'Forge Windows diagnosis',
    'Misleading resistance',
    'negative-transfer',
    'Selective aging state',
    'no production, worker-selection, authority, runtime',
    'not connect it to semantic construction'
)) {
    if (!$result.Contains($required)) { throw "Grounded-search result missing: $required" }
}

$source = Get-Content -LiteralPath $sourcePath -Raw
foreach ($required in @(
    'GroundingCase',
    'GroundingTrace',
    'ScopeDisposition',
    'GroundedBeam',
    'IndeterminateBudget',
    'misleading_natural_heuristic_is_negative_transfer',
    'adult_lock_changes_presentation_not_continuing_age'
)) {
    if (!$source.Contains($required)) { throw "Grounded-search proof surface missing: $required" }
}
foreach ($forbidden in @('forge_kernel', 'std::fs', 'std::process', 'std::net', 'reqwest', 'tauri')) {
    if ($source.Contains($forbidden)) { throw "Grounded-search proof acquired forbidden capability marker: $forbidden" }
}

$policy = Get-Content -LiteralPath (Join-Path $root 'governance\policy-registry.json') -Raw | ConvertFrom-Json
$p16 = @($policy.policies | Where-Object id -eq 'P16')
if ($p16.Count -ne 1 -or $p16[0].status -ne 'approved') { throw 'P16 natural-method policy is not uniquely approved.' }
foreach ($required in @('mathematical abstraction','simple baseline','falsifier','counterexample','non-applicable scope')) {
    if (!$p16[0].rule.Contains($required)) { throw "P16 missing gate field: $required" }
}

$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
if (!(Test-Path -LiteralPath $cargo)) { throw 'Cargo is required for grounded-search verification.' }
Push-Location $root
try {
    & $cargo test -p grounded-search-protocol
    if ($LASTEXITCODE -ne 0) { throw 'Grounded-search focused tests failed.' }
} finally {
    Pop-Location
}

Write-Output 'F5 natural-function reassessment verified: audit, P16, capability-free harness, 8 focused cases, and no-application gate pass.'
