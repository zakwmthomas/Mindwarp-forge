$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$tempBase = [IO.Path]::GetFullPath([IO.Path]::GetTempPath())
$fixture = [IO.Path]::GetFullPath((Join-Path $tempBase ('forge-metrics-fixture-' + $PID + '-' + [guid]::NewGuid().ToString('N'))))
if (!$fixture.StartsWith($tempBase,[StringComparison]::OrdinalIgnoreCase)) { throw 'Metrics fixture escaped the temporary root.' }
try {
  New-Item -ItemType Directory -Path (Join-Path $fixture 'governance'),(Join-Path $fixture 'context\active'),(Join-Path $fixture 'sessions') -Force | Out-Null
  @{batch_id='fixture-batch'} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $fixture 'context\active\WORKER_BATCH_STATE.json') -Encoding utf8
  $session='fixture-session'
  $sessionFile=Join-Path $fixture "sessions\rollout-$session.jsonl"
  @(
    (@{timestamp='2026-07-18T00:00:00Z';type='session_meta';payload=@{session_id=$session;originator='Codex Desktop'}}|ConvertTo-Json -Compress -Depth 8),
    (@{timestamp='2026-07-18T00:01:00Z';type='event_msg';payload=@{type='token_count';info=@{total_token_usage=@{input_tokens=100;cached_input_tokens=40;output_tokens=20;reasoning_output_tokens=5;total_tokens=120}}}}|ConvertTo-Json -Compress -Depth 8)
  ) | Set-Content -LiteralPath $sessionFile -Encoding utf8
  & (Join-Path $root 'tools\forge-batch-metrics.ps1') -Mode start -ProjectRoot $fixture -SessionId $session -SessionsRoot (Join-Path $fixture 'sessions') | Out-Null
  (@{timestamp='2026-07-18T00:02:00Z';type='event_msg';payload=@{type='token_count';info=@{total_token_usage=@{input_tokens=120;cached_input_tokens=50;output_tokens=25;reasoning_output_tokens=7;total_tokens=145}}}}|ConvertTo-Json -Compress -Depth 8) | Add-Content -LiteralPath $sessionFile -Encoding utf8
  & (Join-Path $root 'tools\forge-batch-metrics.ps1') -Mode finish -ProjectRoot $fixture -SessionId $session -SessionsRoot (Join-Path $fixture 'sessions') | Out-Null
  $events=@(Get-ChildItem (Join-Path $fixture '.local\forge-metrics\inbox') -Filter '*.json'|ForEach-Object{Get-Content -Raw $_.FullName|ConvertFrom-Json})
  if ($events.Count -ne 6) { throw 'Batch metric fixture did not emit five token deltas and one wall-time observation.' }
  $total=$events|Where-Object metric_name -eq 'total_tokens'
  $cached=$events|Where-Object metric_name -eq 'cached_input_tokens'
  if ($total.metric_value -ne 25 -or $cached.metric_value -ne 10) { throw 'Token delta or cached-input separation is incorrect.' }
  if (@($events|Where-Object metric_dimensions.value -match 'prompt|path|error').Count -ne 0) { throw 'Private text leaked into metric dimensions.' }

  Remove-Item (Join-Path $fixture '.local\forge-metrics\inbox\*.json') -Force
  & (Join-Path $root 'tools\forge-batch-metrics.ps1') -Mode start -ProjectRoot $fixture -SessionId $session -SessionsRoot (Join-Path $fixture 'sessions') | Out-Null
  (@{timestamp='2026-07-18T00:03:00Z';type='event_msg';payload=@{type='token_count';info=@{total_token_usage=@{input_tokens=1;cached_input_tokens=1;output_tokens=1;reasoning_output_tokens=1;total_tokens=1}}}}|ConvertTo-Json -Compress -Depth 8) | Add-Content -LiteralPath $sessionFile -Encoding utf8
  & (Join-Path $root 'tools\forge-batch-metrics.ps1') -Mode finish -ProjectRoot $fixture -SessionId $session -SessionsRoot (Join-Path $fixture 'sessions') | Out-Null
  $resetEvents=@(Get-ChildItem (Join-Path $fixture '.local\forge-metrics\inbox') -Filter '*.json'|ForEach-Object{Get-Content -Raw $_.FullName|ConvertFrom-Json})
  $unknownTokens=@($resetEvents|Where-Object {$_.outcome -eq 'unknown'})
  if ($unknownTokens.Count -ne 5 -or @($unknownTokens|Where-Object {$null -ne $_.metric_value}).Count -ne 0) { throw 'Reset token counters were not retained as unknown values.' }

  Remove-Item (Join-Path $fixture '.local\forge-metrics\inbox\*.json') -Force
  @{
    schema_version=1
    runs=@(@{id='fixture-run-v1';label='Fixture';runner='powershell';working_directory='.';arguments=@('-NoProfile','-Command','exit 0');module='forge-dashboard';verification_scope='focused';timeout_seconds=30})
  }|ConvertTo-Json -Depth 8|Set-Content -LiteralPath (Join-Path $fixture 'governance\routine-run-registry.json') -Encoding utf8
  & (Join-Path $root 'tools\invoke-measured-run.ps1') -RunId fixture-run-v1 -ProjectRoot $fixture | Out-Null
  $run=@(Get-ChildItem (Join-Path $fixture '.local\forge-metrics\inbox') -Filter '*.json'|ForEach-Object{Get-Content -Raw $_.FullName|ConvertFrom-Json})
  if ($run.Count -ne 1 -or $run[0].event_type -ne 'routine_run_completed' -or $run[0].outcome -ne 'passed' -or $run[0].metric_value -lt 0) { throw 'Registered routine-run receipt is invalid.' }
  Write-Output 'Forge metrics fixtures verified: exact and reset token deltas, cached subset, bounded dimensions, registered run and local receipts.'
} finally {
  if (Test-Path -LiteralPath $fixture) { Remove-Item -LiteralPath $fixture -Recurse -Force }
}
