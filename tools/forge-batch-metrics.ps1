[CmdletBinding()]
param(
  [Parameter(Mandatory=$true)][ValidateSet('start','finish')][string]$Mode,
  [string]$ProjectRoot,
  [string]$SessionId = $env:CODEX_THREAD_ID,
  [string]$SessionsRoot = (Join-Path $env:USERPROFILE '.codex\sessions')
)
$ErrorActionPreference = 'Stop'
if ([string]::IsNullOrWhiteSpace($SessionId)) { throw 'Batch metrics require CODEX_THREAD_ID or an explicit SessionId.' }
if ([string]::IsNullOrWhiteSpace($ProjectRoot)) { $ProjectRoot = Split-Path -Parent $PSScriptRoot }
$ProjectRoot = [IO.Path]::GetFullPath($ProjectRoot)
$checkpointPath = Join-Path $ProjectRoot 'context\active\WORKER_BATCH_STATE.json'
$checkpoint = Get-Content -Raw -LiteralPath $checkpointPath | ConvertFrom-Json
$metricsRoot = Join-Path $ProjectRoot '.local\forge-metrics'
$active = Join-Path $metricsRoot ('active-' + $SessionId + '.json')
$inbox = Join-Path $metricsRoot 'inbox'
New-Item -ItemType Directory -Path $inbox -Force | Out-Null

function Get-TokenSnapshot {
  $files = @(Get-ChildItem -LiteralPath $SessionsRoot -Recurse -File -Filter '*.jsonl' | Where-Object Name -Like "*$SessionId*" | Sort-Object LastWriteTime -Descending)
  if ($files.Count -eq 0) { return $null }
  $snapshot = $null
  foreach ($line in Get-Content -LiteralPath $files[0].FullName -Tail 8000) {
    if ($line -notmatch '"type":"token_count"') { continue }
    try { $record = $line | ConvertFrom-Json } catch { continue }
    if ($record.type -eq 'event_msg' -and $record.payload.type -eq 'token_count' -and $null -ne $record.payload.info.total_token_usage) {
      $usage = $record.payload.info.total_token_usage
      $snapshot = [ordered]@{
        captured_at_ms = [DateTimeOffset]::Parse([string]$record.timestamp).ToUnixTimeMilliseconds()
        input_tokens = [long]$usage.input_tokens
        cached_input_tokens = [long]$usage.cached_input_tokens
        output_tokens = [long]$usage.output_tokens
        reasoning_output_tokens = [long]$usage.reasoning_output_tokens
        total_tokens = [long]$usage.total_tokens
      }
    }
  }
  return $snapshot
}

if ($Mode -eq 'start') {
  $snapshot = Get-TokenSnapshot
  [ordered]@{
    schema_version=1; session_id=$SessionId; batch_id=[string]$checkpoint.batch_id
    started_at_ms=[DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds(); token_snapshot=$snapshot
  } | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $active -Encoding utf8
  Write-Output "Batch metric boundary started for $($checkpoint.batch_id); token state $($(if($null -eq $snapshot){'unknown'}else{'measured'}))."
  exit
}
if (!(Test-Path -LiteralPath $active -PathType Leaf)) { throw 'No matching batch metric start boundary exists.' }
$baseline = Get-Content -Raw -LiteralPath $active | ConvertFrom-Json
$current = Get-TokenSnapshot
$ended = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$checkpointHash = (Get-FileHash -LiteralPath $checkpointPath -Algorithm SHA256).Hash.ToLowerInvariant()
$metrics = @('input_tokens','cached_input_tokens','output_tokens','reasoning_output_tokens','total_tokens')
foreach ($name in $metrics) {
  $known = $null -ne $baseline.token_snapshot -and $null -ne $current
  $delta = if ($known) { [long]$current.$name - [long]$baseline.token_snapshot.$name } else { $null }
  if ($known -and $delta -lt 0) { $known = $false; $delta = $null }
  $id = 'metric-' + [guid]::NewGuid().ToString('N')
  $event = [ordered]@{
    schema_version=2; id=$id; sequence=0; trace_id=('batch-'+$baseline.batch_id); parent_event_id=$null
    event_type='metric_observed'; started_at_ms=[long]$baseline.started_at_ms; ended_at_ms=$ended
    route_system='forge-dashboard'; route_group='B4'; route_contract='batch-event-v2'
    work_package_id=[string]$baseline.batch_id; batch_id=[string]$baseline.batch_id
    outcome=if($known){'completed'}else{'unknown'}
    evidence_ids=@("session-ref:$SessionId","checkpoint-sha256:$checkpointHash")
    privacy_class='metadata_only'; cardinality_class='bounded'
    metric_name=if($known){$name}else{$null}; metric_value=if($known){$delta}else{$null}; metric_unit=if($known){'tokens'}else{$null}
    metric_dimensions=@(@{name='measurement_source';value='codex-local-token-count'},@{name='metric_version';value='v1'})
  }
  $temp=Join-Path $inbox ($id+'.tmp'); $dest=Join-Path $inbox ($id+'.json')
  $event|ConvertTo-Json -Depth 8|Set-Content -LiteralPath $temp -Encoding utf8; Move-Item -LiteralPath $temp -Destination $dest
}
$wallId='metric-'+[guid]::NewGuid().ToString('N')
$wall=[ordered]@{schema_version=2;id=$wallId;sequence=0;trace_id=('batch-'+$baseline.batch_id);parent_event_id=$null;event_type='metric_observed';started_at_ms=[long]$baseline.started_at_ms;ended_at_ms=$ended;route_system='forge-dashboard';route_group='B4';route_contract='batch-event-v2';work_package_id=[string]$baseline.batch_id;batch_id=[string]$baseline.batch_id;outcome='completed';evidence_ids=@("checkpoint-sha256:$checkpointHash");privacy_class='metadata_only';cardinality_class='bounded';metric_name='wall_duration_ms';metric_value=[long]($ended-[long]$baseline.started_at_ms);metric_unit='ms';metric_dimensions=@(@{name='measurement_source';value='worker-boundary'},@{name='metric_version';value='v1'})}
$temp=Join-Path $inbox ($wallId+'.tmp');$dest=Join-Path $inbox ($wallId+'.json');$wall|ConvertTo-Json -Depth 8|Set-Content -LiteralPath $temp -Encoding utf8;Move-Item -LiteralPath $temp -Destination $dest
Remove-Item -LiteralPath $active -Force
Write-Output "Batch metric boundary finished for $($baseline.batch_id); token deltas $($(if($null -eq $current -or $null -eq $baseline.token_snapshot){'unknown'}else{'queued'}))."
