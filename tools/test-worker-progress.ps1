$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$verifier = Join-Path $PSScriptRoot 'verify-worker-progress.ps1'
$target = Join-Path $root 'context\active\WORKER_WAKE_LOG.jsonl'
$backup = "$target.fixture-backup"
Copy-Item $target $backup -Force
try {
  @('{"timestamp":"2026-01-01T00:00:00Z","status":"checkpoint","detail":"same"}','{"timestamp":"2026-01-01T00:01:00Z","status":"checkpoint","detail":"same"}') | Set-Content $target
  $rejected = $false
  try { & $verifier } catch { $rejected = $true }
  if (!$rejected) { throw 'Repeated no-progress checkpoint was accepted.' }
  Write-Output 'Worker progress fixture verified: repeated checkpoint rejected.'
} finally { Move-Item $backup $target -Force }
