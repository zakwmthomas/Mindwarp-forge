[CmdletBinding()]
param(
  [Parameter(Mandatory=$true)][string]$RunId,
  [string]$ProjectRoot
)
$ErrorActionPreference = 'Stop'
if ([string]::IsNullOrWhiteSpace($ProjectRoot)) { $ProjectRoot = Split-Path -Parent $PSScriptRoot }
$ProjectRoot = [IO.Path]::GetFullPath($ProjectRoot)
$registryPath = Join-Path $ProjectRoot 'governance\routine-run-registry.json'
$checkpointPath = Join-Path $ProjectRoot 'context\active\WORKER_BATCH_STATE.json'
if (!(Test-Path -LiteralPath $registryPath -PathType Leaf)) { throw 'Routine-run registry is unavailable.' }
if (!(Test-Path -LiteralPath $checkpointPath -PathType Leaf)) { throw 'Canonical worker checkpoint is unavailable.' }
$registry = Get-Content -Raw -LiteralPath $registryPath | ConvertFrom-Json
if ($registry.schema_version -ne 1) { throw 'Unsupported routine-run registry schema.' }
$definition = @($registry.runs | Where-Object id -eq $RunId)
if ($definition.Count -ne 1) { throw "Unknown or duplicate registered routine run: $RunId" }
$definition = $definition[0]
if ($definition.timeout_seconds -lt 1 -or $definition.timeout_seconds -gt 1800) { throw 'Routine-run timeout is outside the bounded registry range.' }
$working = [IO.Path]::GetFullPath((Join-Path $ProjectRoot ([string]$definition.working_directory)))
if (!$working.StartsWith($ProjectRoot,[StringComparison]::OrdinalIgnoreCase)) { throw 'Routine-run working directory escapes the Forge repository.' }
$runner = switch ([string]$definition.runner) {
  'powershell' { 'powershell.exe' }
  'cargo' { Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe' }
  'npm' { 'npm.cmd' }
  default { throw 'Routine-run runner is not allowlisted.' }
}
$checkpoint = Get-Content -Raw -LiteralPath $checkpointPath | ConvertFrom-Json
$checkpointHash = (Get-FileHash -LiteralPath $checkpointPath -Algorithm SHA256).Hash.ToLowerInvariant()
$runIdValue = 'run-' + [guid]::NewGuid().ToString('N')
$started = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$exitCode = 1
$timedOut = $false
try {
  $process = Start-Process -FilePath $runner -ArgumentList @($definition.arguments) -WorkingDirectory $working -NoNewWindow -PassThru
  try {
    $process | Wait-Process -Timeout ([int]$definition.timeout_seconds) -ErrorAction Stop
    $exitCode = $process.ExitCode
  } catch [Microsoft.PowerShell.Commands.ProcessCommandException] {
    $timedOut = $true
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    $exitCode = 124
    Write-Error "Measured run exceeded its registered $($definition.timeout_seconds)-second timeout." -ErrorAction Continue
  }
} catch {
  Write-Error $_ -ErrorAction Continue
  $exitCode = 1
}
$ended = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
$inbox = Join-Path $ProjectRoot '.local\forge-metrics\inbox'
New-Item -ItemType Directory -Path $inbox -Force | Out-Null
$receipt = [ordered]@{
  schema_version = 2
  id = $runIdValue
  sequence = 0
  trace_id = $runIdValue
  parent_event_id = $null
  event_type = 'routine_run_completed'
  started_at_ms = $started
  ended_at_ms = $ended
  route_system = 'forge-dashboard'
  route_group = 'B4'
  route_contract = 'batch-event-v2'
  work_package_id = [string]$checkpoint.batch_id
  batch_id = [string]$checkpoint.batch_id
  outcome = if ($exitCode -eq 0) { 'passed' } else { 'failed' }
  evidence_ids = @("run-definition:$RunId","checkpoint-sha256:$checkpointHash") + $(if($timedOut){@("run-timeout-seconds:$($definition.timeout_seconds)")}else{@()})
  privacy_class = 'metadata_only'
  cardinality_class = 'bounded'
  metric_name = 'wall_duration_ms'
  metric_value = [long]($ended - $started)
  metric_unit = 'ms'
  metric_dimensions = @(
    @{name='module';value=[string]$definition.module},
    @{name='result_class';value=if($timedOut){'timed_out'}elseif($exitCode -eq 0){'passed'}else{'failed'}},
    @{name='measurement_source';value='registered-run-wrapper'},
    @{name='run_definition';value=$RunId},
    @{name='platform';value='windows'},
    @{name='verification_scope';value=[string]$definition.verification_scope},
    @{name='metric_version';value='v1'}
  )
}
$temporary = Join-Path $inbox ($runIdValue + '.tmp')
$destination = Join-Path $inbox ($runIdValue + '.json')
$receipt | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $temporary -Encoding utf8
Move-Item -LiteralPath $temporary -Destination $destination
Write-Output "Measured run $RunId completed with exit $exitCode$(if($timedOut){' (timed out)'}else{''}) in $($ended-$started) ms; local receipt $runIdValue queued."
exit $exitCode
