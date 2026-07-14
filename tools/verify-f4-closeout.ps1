$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$atlas = Get-Content (Join-Path $root 'docs\project-atlas\project-model.json') -Raw | ConvertFrom-Json
$items = @{}; foreach ($item in $program.items) { $items[$item.id] = $item }

$required = @('F4-MODULARITY','W1','W2','A1','A2','A3','A4','B1','B2','B3','B4','B5')
foreach ($id in $required) {
  if (!$items.ContainsKey($id) -or $items[$id].status -ne 'complete') { throw "F4 dependency is not complete: $id" }
  foreach ($source in @($items[$id].sources)) {
    $matches = @(Get-ChildItem -Path (Join-Path $root 'docs'),(Join-Path $root 'contracts'),(Join-Path $root 'governance') -Recurse -File -Filter $source -ErrorAction SilentlyContinue)
    if ($matches.Count -eq 0) { throw "F4 proof source is missing: $id -> $source" }
  }
}

if (!$items.ContainsKey('F4-CLOSEOUT') -or $items['F4-CLOSEOUT'].status -notin @('active','complete')) { throw 'F4 closeout item is unavailable.' }
if ($items['F4-CLOSEOUT'].status -eq 'complete') {
  if (!$items.ContainsKey('F5-OWNER-GATE') -or $items['F5-OWNER-GATE'].status -notin @('active','complete') -or $items['F5-OWNER-GATE'].gate -ne 'owner') { throw 'Completed F4 closeout lacks a retained owner gate.' }
}

$f4 = @($atlas.milestones | Where-Object id -eq 'F4')[0]
$f5 = @($atlas.milestones | Where-Object id -eq 'F5')[0]
$atlasDecision = @($atlas.decisions | Where-Object id -eq 'D3')[0]
$f4Items = @($program.items | Where-Object milestone -eq 'F4')
$f5Items = @($program.items | Where-Object milestone -eq 'F5')
$f4Status = if (@($f4Items | Where-Object status -ne 'complete').Count -eq 0) { 'verified' } elseif (@($f4Items | Where-Object status -eq 'active').Count -gt 0) { 'active' } else { 'gated' }
$f5Status = if (@($f5Items | Where-Object status -eq 'active').Count -gt 0) { 'active' } elseif (@($f5Items | Where-Object status -ne 'complete').Count -eq 0) { 'verified' } else { 'gated' }
$preTransition = $items['F5-OWNER-GATE'].status -eq 'active' -and $items['F5'].status -eq 'gated' -and $f4Status -eq 'active' -and $f5Status -eq 'gated'
$postTransition = $items['F5-OWNER-GATE'].status -eq 'complete' -and $items['F5'].status -in @('active','owner_gated') -and $f4Status -eq 'verified' -and $f5Status -eq 'active'
if ((!$preTransition -and !$postTransition) -or $atlasDecision.status -ne 'approved') { throw 'Atlas milestone owner boundary is inconsistent.' }

$requiredEvidence = @(
  'docs\canonical-system\F4_EXIT_AUDIT.md',
  'docs\canonical-system\MODULARITY_READINESS.md',
  'docs\canonical-system\BATCH_EVENT_READINESS.md',
  'docs\canonical-system\FEDERATED_IMPROVEMENT_READINESS.md',
  'contracts\module-boundary-contract.md',
  'contracts\batch-event-contract.md',
  'contracts\federated-improvement-contract.md'
)
foreach ($relative in $requiredEvidence) {
  if (!(Test-Path -LiteralPath (Join-Path $root $relative))) { throw "F4 closeout evidence missing: $relative" }
}
Write-Output "F4 closeout verified: $($required.Count) dependency items complete; Atlas owner transition is preserved."
