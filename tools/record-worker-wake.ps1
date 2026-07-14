param(
  [ValidateSet('started','checkpoint','completed','blocked','no_work','failed')][string]$Status,
  [string]$Detail
)
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'context\active\WORKER_WAKE_LOG.jsonl'
$record = [ordered]@{ timestamp=(Get-Date).ToUniversalTime().ToString('o'); status=$Status; detail=$Detail }
($record | ConvertTo-Json -Compress) | Add-Content -LiteralPath $path
Write-Output "Worker wake recorded: $Status"
