Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$oracle = Join-Path $root 'tools\prove-g1-c3-cross-boundary-ecotone.py'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_CROSS_BOUNDARY_ECOTONE_ORACLE_RESULT.md'
$readinessPath = Join-Path $root 'docs\canonical-system\G1_C3_CROSS_BOUNDARY_ECOTONE_ORACLE_IMPLEMENTATION_READINESS.md'
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

if (!(Test-Path -LiteralPath $oracle) -or !(Test-Path -LiteralPath $resultPath)) {
  throw 'Disposable ecotone oracle package is incomplete'
}
$source = Get-Content -LiteralPath $oracle -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
$readiness = Get-Content -LiteralPath $readinessPath -Raw
$sourceHash = (Get-FileHash -LiteralPath $oracle -Algorithm SHA256).Hash.ToLowerInvariant()
if ($sourceHash -ne '60253d2fea9a06109df5ac81ae1f0f80846f872c5a4271af7fbb088f7cff201d') {
  throw "Ecotone oracle source hash drift: $sourceHash"
}

foreach ($required in @(
  'Fraction','ThreadPoolExecutor','tracemalloc','allow_nan=False','MAX_CELLS = 65_536',
  'MAX_EDGES = 130_560','mindwarp.disposable.ecotone.cell-result.v1',
  'mindwarp.disposable.ecotone.edge-result.v1','mindwarp.disposable.ecotone.fixture-result.v1',
  'mindwarp.disposable.ecotone.suite-receipt.v1','noncanonical_input','provenance_mismatch',
  'unavailable_evidence','unsupported_join','contradictory_evidence','sharp_cause_exact',
  'continuous_cause_exact','label_only_split','partition_evidence_poison',
  'aligned_refinement_and_moved_control','recursive_blend_order_drift',
  'heterogeneous_evidence_collapse','four_threads','oversize-257x256'
)) {
  if ($source -notlike "*$required*") { throw "Ecotone oracle source missing: $required" }
}
foreach ($forbidden in @('import os','import pathlib','import subprocess','import socket','import numpy','import torch','import cupy','from crates','forge-federate')) {
  if ($source -like "*$forbidden*") { throw "Ecotone oracle isolation drift: $forbidden" }
}
foreach ($required in @(
  'byte-identical','60253d2fea9a06109df5ac81ae1f0f80846f872c5a4271af7fbb088f7cff201d',
  '3b281dcaf2e6e8a9053c7cbeefd0dd8b971c07dc1f5da45fd51888fd20c39c2a',
  '8bcb3bd741ba6913a85a4ca3c26170943d6e54609bb44aa867c0cebff1f44b24',
  '19 hostile families','469,833','934,472','65,536','130,560','256 MiB','120 seconds',
  '444.7 seconds','2,508','853','52',
  'No production owner','Nothing broader is locked in','One consumer first, reassess before expanding'
)) {
  if ($result -notlike "*$required*") { throw "Ecotone oracle result missing: $required" }
}
if ($readiness -notlike '*ready for one exact disposable-oracle decision only*') {
  throw 'Ecotone oracle readiness continuity drift'
}

$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (!(Test-Path -LiteralPath $python)) { $python = 'python' }
function Invoke-Oracle {
  $startInfo = [Diagnostics.ProcessStartInfo]::new()
  $startInfo.FileName = $python
  $startInfo.UseShellExecute = $false
  $startInfo.RedirectStandardOutput = $true
  $startInfo.RedirectStandardError = $true
  $startInfo.Arguments = "-I -B `"$oracle`""
  $process = [Diagnostics.Process]::new()
  $process.StartInfo = $startInfo
  $clock = [Diagnostics.Stopwatch]::StartNew()
  [void]$process.Start()
  $stdoutTask = $process.StandardOutput.ReadToEndAsync()
  $stderrTask = $process.StandardError.ReadToEndAsync()
  if (!$process.WaitForExit(120000)) {
    try { $process.Kill($true) } catch {}
    throw 'Ecotone oracle exceeded the 120-second proof budget'
  }
  $clock.Stop()
  $stdout = $stdoutTask.GetAwaiter().GetResult()
  $stderr = $stderrTask.GetAwaiter().GetResult()
  if ($process.ExitCode -ne 0 -or $stderr.Length -ne 0) {
    throw "Ecotone oracle failed: exit=$($process.ExitCode) stderr=$stderr"
  }
  [pscustomobject]@{ Output = $stdout; Seconds = $clock.Elapsed.TotalSeconds }
}

$first = Invoke-Oracle
$second = Invoke-Oracle
if ($first.Output -cne $second.Output) { throw 'Ecotone oracle output is not byte-identical' }
$bytes = [Text.Encoding]::UTF8.GetBytes($first.Output)
$sha256 = [Security.Cryptography.SHA256]::Create()
try { $stdoutHash = ([BitConverter]::ToString($sha256.ComputeHash($bytes))).Replace('-','').ToLowerInvariant() }
finally { $sha256.Dispose() }
if ($stdoutHash -ne '3b281dcaf2e6e8a9053c7cbeefd0dd8b971c07dc1f5da45fd51888fd20c39c2a') {
  throw "Ecotone oracle stdout hash drift: $stdoutHash"
}
if ($bytes.Length -gt 65536 -or $first.Output -notmatch '^\{[^\r\n]+\}\r?\n$') {
  throw 'Ecotone oracle canonical stdout envelope drift'
}
$receipt = $first.Output | ConvertFrom-Json
$expectedModes = 'row_major,column_major,reverse,annotation_major,sha256_permutation,fixed_chunks,four_threads'
if (!$receipt.pass -or $receipt.receipt_hash -ne '8bcb3bd741ba6913a85a4ca3c26170943d6e54609bb44aa867c0cebff1f44b24' -or
    $receipt.observed_outcomes.hostile_families -ne 19 -or $receipt.expected_outcomes.hostile_families -ne 19 -or
    $receipt.evaluated_cell_count -ne 469833 -or $receipt.evaluated_edge_count -ne 934472 -or
    $receipt.maximum_product_bits -gt 40 -or -not $receipt.resource_ceiling.peak_below_limit -or
    (@($receipt.enumeration_modes) -join ',') -ne $expectedModes -or @($receipt.hostile_family_ids).Count -ne 19) {
  throw 'Ecotone oracle pinned receipt drift'
}

$active = $checkpoint.batch_id -eq 'G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1' -and
  $checkpoint.substage_id -in @('c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $checkpoint.authority_lane -like '*Owner-approved disposable C3 ecotone oracle implementation only*No crate*contract schema*dependency*production test*production source*downstream consumer*renderer*biome*organism*runtime*promotion*C3 closure*'
if (!$active -and !$c3InterruptionRoute) { throw 'Ecotone oracle checkpoint authority drift' }

Write-Output ('C3 disposable ecotone oracle verified: two byte-identical isolated runs; 19 hostile families, 469833 cell visits and 934472 edge visits pass. seconds={0:N1}/{1:N1} stdout={2}' -f $first.Seconds,$second.Seconds,$stdoutHash)
