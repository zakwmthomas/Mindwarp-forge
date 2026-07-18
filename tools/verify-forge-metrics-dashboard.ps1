$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
foreach ($path in @(
  'docs\canonical-system\WORKER_TELEMETRY_MODULE.md',
  'governance\MEASUREMENT_AND_RECURSIVE_LEARNING_CONTRACT.md',
  'governance\WORKER_METRIC_REGISTRY.md',
  'governance\routine-run-registry.json',
  'tools\invoke-measured-run.ps1',
  'tools\test-registered-full-gate-launcher.ps1',
  'tools\forge-batch-metrics.ps1'
)) { if (!(Test-Path -LiteralPath (Join-Path $root $path) -PathType Leaf)) { throw "Metrics package file is missing: $path" } }
$registry=Get-Content -Raw (Join-Path $root 'governance\routine-run-registry.json')|ConvertFrom-Json
if ($registry.schema_version -ne 1 -or @($registry.runs).Count -lt 4 -or @($registry.runs.id|Sort-Object -Unique).Count -ne @($registry.runs).Count) { throw 'Routine-run registry is invalid or ambiguous.' }
$main=Get-Content -Raw (Join-Path $root 'apps\forge-desktop\src-tauri\src\main.rs')
$ui=Get-Content -Raw (Join-Path $root 'apps\forge-desktop\ui\index.html')
foreach($required in @('metrics_dashboard_snapshot','metrics_dashboard_projection_since','run_metrics_capture::scan_inbox')){if(!$main.Contains($required)){throw "Desktop metrics command is missing: $required"}}
foreach($required in @('data-nav="metrics"','data-page="metrics"','ADVISORY NEXT STEPS','Unknown values stay unknown')){if(!$ui.Contains($required)){throw "Metrics UI is missing: $required"}}
& (Join-Path $root 'tools\test-forge-metrics.ps1')
if (!$?) { throw 'Forge metrics PowerShell fixtures failed.' }
& (Join-Path $root 'tools\test-registered-full-gate-launcher.ps1')
if (!$?) { throw 'Registered full-gate launcher fixtures failed.' }
Write-Output 'Forge metrics dashboard static and PowerShell verification passed.'
