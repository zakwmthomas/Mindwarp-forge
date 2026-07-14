$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-worker-feedback-$PID"
$sources = @('governance\policy-registry.json','governance\WORKER_GOVERNANCE_SYSTEM.md','governance\WORKER_PROMPT_SPEC.md','governance\WORKER_LEARNING_LEDGER.md','governance\SYSTEM_EFFICIENCY_AUDIT.md','governance\WORKER_METRIC_REGISTRY.md','governance\MEASUREMENT_AND_RECURSIVE_LEARNING_CONTRACT.md','context\active\WORKER_BATCH_STATE.json','docs\canonical-system\MASTER_CLOSURE_REGISTER.md')
try {
  foreach ($relative in $sources) {
    $destination = Join-Path $temp $relative
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $destination) | Out-Null
    Copy-Item -LiteralPath (Join-Path $root $relative) -Destination $destination
  }
  & (Join-Path $PSScriptRoot 'refresh-worker-feedback.ps1') -Root $temp | Out-Null
  & (Join-Path $PSScriptRoot 'verify-worker-feedback-freshness.ps1') -Root $temp | Out-Null
  $lineEndingFixture = Join-Path $temp $sources[0]
  $lineEndingText = [IO.File]::ReadAllText($lineEndingFixture).Replace("`r`n", "`n")
  [IO.File]::WriteAllText($lineEndingFixture, $lineEndingText.Replace("`n", "`r`n"), [Text.UTF8Encoding]::new($false))
  & (Join-Path $PSScriptRoot 'verify-worker-feedback-freshness.ps1') -Root $temp | Out-Null
  foreach ($index in @(0, 5, 7, 8)) {
    Add-Content -LiteralPath (Join-Path $temp $sources[$index]) -Value ' '
    $rejected = $false
    try { & (Join-Path $PSScriptRoot 'verify-worker-feedback-freshness.ps1') -Root $temp | Out-Null } catch { $rejected = $true }
    if (!$rejected) { throw "Stale worker feedback was accepted for $($sources[$index])." }
    & (Join-Path $PSScriptRoot 'refresh-worker-feedback.ps1') -Root $temp | Out-Null
    & (Join-Path $PSScriptRoot 'verify-worker-feedback-freshness.ps1') -Root $temp | Out-Null
  }
  Write-Output 'Worker feedback fixtures verified: canonical line endings, change propagation, and stale-source rejection.'
} finally { if (Test-Path $temp) { Remove-Item -LiteralPath $temp -Recurse -Force } }
