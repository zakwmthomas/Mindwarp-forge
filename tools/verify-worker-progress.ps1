$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$batch = Get-Content (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$log = Join-Path $root 'context\active\WORKER_WAKE_LOG.jsonl'
if (!(Test-Path $log)) { throw 'Worker wake log missing.' }
$records = @(Get-Content $log | Where-Object { $_ } | ForEach-Object { $_ | ConvertFrom-Json })
if ($records.Count -lt 2) { Write-Output 'Worker progress verifier: insufficient wake history.'; exit }
$last = $records[-1]; $prior = $records[-2]
if ($last.status -eq 'checkpoint' -and $prior.status -eq 'checkpoint' -and $last.detail -eq $prior.detail) { throw 'Repeated checkpoint has no recorded progress.' }
Write-Output "Worker progress verified: latest outcome $($last.status)."
