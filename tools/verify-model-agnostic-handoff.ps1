$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$handoff = Join-Path $root 'AI_HANDOFF.md'
if (!(Test-Path $handoff)) { throw 'Model-agnostic AI handoff entry point is missing.' }

$handoffText = Get-Content $handoff -Raw
foreach ($token in @('tools\ensure-context-current.ps1','AGENTS.md','BOOTSTRAP RECEIPT','context/active/WORKER_BATCH_STATE.json','git status --short','Do not reopen completed C1/C2')) {
  if (!$handoffText.Contains($token)) { throw "AI handoff is missing required routing or safety text: $token" }
}

$firstLayer = @(
  'AI_HANDOFF.md',
  'README.md',
  'AGENTS.md',
  'context\bootstrap\START_HERE.md',
  'governance\WORKING_COVENANT.md',
  'governance\RECORDING_PROTOCOL.md'
)
foreach ($relative in $firstLayer) {
  $text = Get-Content (Join-Path $root $relative) -Raw
  foreach ($stale in @('new Codex task','Codex is technical lead','future Codex task')) {
    if ($text.Contains($stale)) { throw "Model-specific first-layer wording remains in ${relative}: $stale" }
  }
}

$generator = Get-Content (Join-Path $root 'apps\forge-desktop\src-tauri\src\codex_capture.rs') -Raw
foreach ($token in @('../../AI_HANDOFF.md','any AI assistant','Forge AI Bootstrap Index')) {
  if (!$generator.Contains($token)) { throw "Generated local handoff does not preserve model-agnostic routing: $token" }
}

$roles = Get-Content (Join-Path $root 'governance\record-role-registry.json') -Raw | ConvertFrom-Json
$role = @($roles.rules | Where-Object { $_.pattern -eq 'AI_HANDOFF.md' -and $_.role -eq 'repository_navigation' })
if ($role.Count -ne 1) { throw 'AI_HANDOFF.md must be classified once as repository navigation.' }

Write-Output 'Model-agnostic AI handoff verified: one root entry point, canonical-state routing, dirty-tree preservation, authority limits, and generated-pack routing are present.'
