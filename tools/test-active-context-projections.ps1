$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-active-projection-$PID"
$resolvedTempBase = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
$resolvedTemp = [System.IO.Path]::GetFullPath($temp)
if (!$resolvedTemp.StartsWith($resolvedTempBase,[System.StringComparison]::OrdinalIgnoreCase)) { throw 'Fixture temp path escaped the system temp directory.' }
try {
  foreach ($relative in @('context\active','context\bootstrap','docs\canonical-system','docs\project-atlas','governance','tools')) {
    New-Item -ItemType Directory -Path (Join-Path $temp $relative) -Force | Out-Null
  }
  foreach ($relative in @('context\active\WORKER_BATCH_STATE.json','docs\canonical-system\MASTER_PROGRAM.json','docs\project-atlas\project-model.json','governance\policy-registry.json','tools\refresh-active-context.ps1')) {
    Copy-Item -LiteralPath (Join-Path $root $relative) -Destination (Join-Path $temp $relative) -Force
  }
  $generator = Join-Path $temp 'tools\refresh-active-context.ps1'
  & $generator -Root $temp | Out-Null
  & $generator -Root $temp -Check | Out-Null
  Add-Content -LiteralPath (Join-Path $temp 'context\active\CURRENT_STATE.md') -Value 'manual drift'
  $failed = $false
  try { & $generator -Root $temp -Check | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Hand-edited generated current state was accepted.' }
  & $generator -Root $temp | Out-Null
  & $generator -Root $temp -Check | Out-Null
  $statePath = Join-Path $temp 'context\active\WORKER_BATCH_STATE.json'
  $state = Get-Content $statePath -Raw | ConvertFrom-Json
  $state.next_action = 'fixture single-source transition'
  $state | ConvertTo-Json -Depth 10 | Set-Content $statePath
  & $generator -Root $temp | Out-Null
  foreach ($relative in @('context\active\CURRENT_STATE.md','context\bootstrap\BRIEFING.md')) {
    if (!(Get-Content (Join-Path $temp $relative) -Raw).Contains('fixture single-source transition')) { throw "Canonical update did not reach projection: $relative" }
  }
  Write-Output 'Active-context projection fixtures verified: deterministic generation, drift rejection, repair, and single-source propagation.'
} finally {
  if (Test-Path -LiteralPath $resolvedTemp) { Remove-Item -LiteralPath $resolvedTemp -Recurse -Force }
}
