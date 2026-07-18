$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$auditPath = Join-Path $root 'docs\canonical-system\APERIODIC_MONOTILE_CANDIDATE_MAP.md'
$registryPath = Join-Path $root 'docs\canonical-system\system-registry.json'
$atlasPath = Join-Path $root 'docs\project-atlas\project-model.json'
$programPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'

foreach ($path in @($auditPath, $registryPath, $atlasPath, $programPath)) {
    if (!(Test-Path -LiteralPath $path)) { throw "Whole-system audit input missing: $path" }
}

$audit = Get-Content -LiteralPath $auditPath -Raw
$registry = Get-Content -LiteralPath $registryPath -Raw | ConvertFrom-Json
$atlas = Get-Content -LiteralPath $atlasPath -Raw | ConvertFrom-Json
$ids = @($registry.systems.id) + @($atlas.systems.id) | Sort-Object -Unique

foreach ($id in $ids) {
    $tick = [char]96
    $token = "${tick}${id}${tick}"
    if (!$audit.Contains($token)) {
        throw "Whole-system method audit does not cover registered system: $id"
    }
}

foreach ($required in @(
    'Reusable mechanism families',
    'Whole Forge audit',
    'Whole game-canonical audit',
    'Runtime and platform audit',
    'Cross-layer compounding opportunities',
    'Miscommunication analysis and engineered correction'
)) {
    if (!$audit.Contains($required)) { throw "Whole-system method audit lacks required section: $required" }
}

$program = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json
foreach ($itemId in @('C3','C4','C5','C6','C7','G1-CLOSEOUT','R1')) {
    $item = @($program.items | Where-Object id -eq $itemId)
    if ($item.Count -ne 1) { throw "Whole-system audit plan item missing or duplicated: $itemId" }
    if (@($item[0].sources) -notcontains 'APERIODIC_MONOTILE_CANDIDATE_MAP.md') {
        throw "Whole-system audit is not linked from future plan item: $itemId"
    }
}

Write-Output "Whole-system method audit verified: $($ids.Count) registered systems and 7 future plan links."
