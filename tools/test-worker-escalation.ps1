$ErrorActionPreference = 'Stop'
$verifier = Join-Path $PSScriptRoot 'verify-worker-escalation.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-worker-escalation-$PID.jsonl"
try {
  @('{"type":"optimization_audit","result":"failed"}','{"type":"optimization_audit","result":"failed"}','{"type":"optimization_audit","result":"failed"}','{"type":"owner_escalation","reason":"three_failed_audits"}') | Set-Content $temp
  & $verifier -Path $temp | Out-Null
  @('{"type":"optimization_audit","result":"failed"}','{"type":"optimization_audit","result":"failed"}','{"type":"optimization_audit","result":"corrected"}') | Set-Content $temp
  & $verifier -Path $temp | Out-Null
  Add-Content $temp '{"type":"owner_escalation","reason":"invalid"}'
  $rejected = $false
  try { & $verifier -Path $temp | Out-Null } catch { $rejected = $true }
  if (!$rejected) { throw 'Spurious owner escalation was accepted.' }
  Write-Output 'Worker escalation fixtures verified: threshold, deduplication, and corrected-path suppression.'
} finally { Remove-Item $temp -Force -ErrorAction SilentlyContinue }
