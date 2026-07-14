$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\project-atlas\project-model.json'
if (!(Test-Path -LiteralPath $path)) { throw 'Project Atlas is missing.' }
$atlas = Get-Content -LiteralPath $path -Raw | ConvertFrom-Json
if ($atlas.schema_version -ne 1) { throw 'Unsupported Project Atlas schema.' }
if ([string]::IsNullOrWhiteSpace($atlas.project.vision)) { throw 'Project Atlas has no vision.' }
$ids = @($atlas.systems.id) + @($atlas.milestones.id) + @($atlas.decisions.id)
if (($ids | Group-Object | Where-Object Count -gt 1).Count -gt 0) { throw 'Project Atlas contains duplicate IDs.' }
$milestones = @($atlas.milestones)
$known = @{}; foreach ($milestone in $milestones) { $known[$milestone.id] = $true }
foreach ($milestone in $milestones) {
    if (@($milestone.exit_criteria).Count -eq 0) { throw "Milestone $($milestone.id) has no exit criteria." }
    foreach ($dependency in @($milestone.depends_on)) {
        if (!$known.ContainsKey($dependency)) { throw "Milestone $($milestone.id) references missing dependency $dependency." }
    }
    foreach ($reference in @($milestone.references)) {
        if (!(Test-Path -LiteralPath (Join-Path $root $reference))) { throw "Milestone $($milestone.id) references missing path $reference." }
    }
}
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$derived = foreach ($milestone in $milestones) {
    $items = @($program.items | Where-Object milestone -eq $milestone.id)
    $status = if ($items.Count -eq 0) { 'verified' } elseif (@($items | Where-Object status -eq 'active').Count -gt 0) { 'active' } elseif (@($items | Where-Object status -ne 'complete').Count -eq 0) { 'verified' } else { 'gated' }
    [pscustomobject]@{ id=$milestone.id; name=$milestone.name; status=$status }
}
$active = @($derived | Where-Object status -eq 'active')
if ($active.Count -ne 1) { throw 'Master-program projection must produce exactly one active Atlas milestone.' }
$roadmap = Get-Content (Join-Path $root 'docs\project-atlas\ROADMAP.md') -Raw
if (!$roadmap.Contains("The current active milestone is **$($active[0].id):")) { throw 'Roadmap active milestone is stale relative to Project Atlas.' }
Write-Output "Atlas verified from master-program status: active milestone $($active[0].id) ($($active[0].name)); $($milestones.Count) milestones, $($atlas.systems.Count) systems."
