param(
  [string]$Root,
  [switch]$Check
)
$ErrorActionPreference = 'Stop'
$root = if ($Root) { $Root } else { Split-Path -Parent $PSScriptRoot }
$statePath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
$programPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
$atlasPath = Join-Path $root 'docs\project-atlas\project-model.json'
$policyPath = Join-Path $root 'governance\policy-registry.json'
foreach ($path in @($statePath,$programPath,$atlasPath,$policyPath)) {
  if (!(Test-Path -LiteralPath $path)) { throw "Active-context source missing: $path" }
}
$state = Get-Content -LiteralPath $statePath -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json
$atlas = Get-Content -LiteralPath $atlasPath -Raw | ConvertFrom-Json
$policy = Get-Content -LiteralPath $policyPath -Raw | ConvertFrom-Json
$item = @($program.items | Where-Object id -eq $state.master_program_item)[0]
$milestone = @($atlas.milestones | Where-Object id -eq $state.atlas_route.milestone)[0]
if ($null -eq $item -or $null -eq $milestone) { throw 'Active checkpoint route does not resolve.' }
$systems = @($state.atlas_route.systems) -join ', '
$evidence = @($state.evidence | Select-Object -First 8 | ForEach-Object { "- ``$_``" }) -join "`n"
if ([string]::IsNullOrWhiteSpace($evidence)) { $evidence = '- No durable package evidence recorded yet.' }
$risks = @($state.unresolved_risks | ForEach-Object { "- $_" }) -join "`n"
if ([string]::IsNullOrWhiteSpace($risks)) { $risks = '- No unresolved package risk recorded.' }
$resume = if ([string]::IsNullOrWhiteSpace($state.resume_after)) { 'None.' } else { $state.resume_after }
$current = @"
# Current State (Generated)

> Generated from ``context/active/WORKER_BATCH_STATE.json``. Do not edit this
> projection; change the canonical checkpoint and regenerate it.

## Active checkpoint

- Package: **$($state.batch_id)**
- Master item / milestone: **$($state.master_program_item) / $($milestone.id)**
- State / substage: **$($state.state) / $($state.substage_id)**
- Related systems: $systems
- Objective: $($state.objective)
- Context health: $($state.context_health)

## Durable evidence

$evidence

## Authority boundary

$($state.authority_lane)

## Exact next action

$($state.next_action)

## Unresolved risks

$risks

## Resume after this package

$resume
"@
$policies = (@($policy.policies | Where-Object status -eq 'approved') | ForEach-Object { "- **$($_.id):** $($_.rule)" }) -join "`n"
$briefing = @"
# Forge Task Briefing (Generated)

> Generated from ``context/active/WORKER_BATCH_STATE.json``. Do not edit.

## Active route

- Work package: **$($state.batch_id)**
- Objective: $($state.objective)
- Atlas milestone: **$($milestone.id) - $($milestone.name)**
- Related systems: $systems
- Risk/research gate: **$($state.risk_level) / $($state.research_gate)**
- Exact next action: $($state.next_action)

## Approved operating policies

$policies

## Navigation

Read raw transcript evidence only through ``tools\find-evidence.ps1`` when a
specific uncertainty remains. The master program selects work; the canonical
active checkpoint resumes it; generated projections never override either.
"@
$outputs = @{
  (Join-Path $root 'context\active\CURRENT_STATE.md') = $current
  (Join-Path $root 'context\bootstrap\BRIEFING.md') = $briefing
}
foreach ($entry in $outputs.GetEnumerator()) {
  $expected = $entry.Value.TrimEnd() + [Environment]::NewLine
  if ($Check) {
    if (!(Test-Path -LiteralPath $entry.Key)) { throw "Generated active-context projection missing: $($entry.Key)" }
    $actual = Get-Content -LiteralPath $entry.Key -Raw
    if ($actual -ne $expected) { throw "Generated active-context projection drifted: $($entry.Key)" }
  } else {
    Set-Content -LiteralPath $entry.Key -Value $expected -NoNewline
  }
}
Write-Output $(if ($Check) { 'Active-context projections verified.' } else { 'Active-context projections refreshed.' })
