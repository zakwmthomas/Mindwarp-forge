$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$atlas = Get-Content (Join-Path $root 'docs\project-atlas\project-model.json') -Raw | ConvertFrom-Json
$readme = Get-Content (Join-Path $root 'README.md') -Raw
$bootstrapContract = Get-Content (Join-Path $root 'contracts\task-bootstrap-contract.md') -Raw
$knowledgeContract = Get-Content (Join-Path $root 'contracts\knowledge-record-contract.md') -Raw

$active = @($program.items | Where-Object status -eq 'active')
if ($active.Count -ne 1) { throw 'Coherence requires exactly one active master-program item.' }
if ($checkpoint.master_program_item -ne $active[0].id) {
  throw "Active checkpoint $($checkpoint.master_program_item) does not match active master item $($active[0].id)."
}
$coherence = @($program.items | Where-Object id -eq 'F5-COHERENCE')
if ($coherence.Count -ne 1 -or $coherence[0].status -notin @('in_progress','complete')) {
  throw 'Owner-approved coherence package is missing or has an invalid lifecycle state.'
}
if (@($atlas.milestones | Where-Object { $_.PSObject.Properties.Name -contains 'status' }).Count -gt 0) {
  throw 'Atlas milestone status duplicates master-program execution truth.'
}
if ($readme -match 'intentionally in-memory') { throw 'Root README still claims persistence is absent.' }
if ($bootstrapContract -notmatch 'generated' -or $bootstrapContract -match 'agent-maintained') {
  throw 'Task bootstrap contract misstates generated context ownership.'
}
foreach ($token in @('evidence itself','never grants authority','idempotent','generated views')) {
  if ($knowledgeContract -notmatch [regex]::Escape($token)) { throw "Knowledge contract lacks invariant: $token" }
}

Write-Output "Forge coherence verified: active item $($active[0].id); checkpoint aligned; Atlas status is projected; knowledge authority remains evidence-only."
