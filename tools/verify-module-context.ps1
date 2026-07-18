$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
& (Join-Path $PSScriptRoot 'refresh-module-context.ps1') -Check
if (!$?) { throw 'Module context projections are missing or stale.' }
$agents = Get-Content (Join-Path $root 'AGENTS.md') -Raw
$prompt = Get-Content (Join-Path $root 'governance\WORKER_PROMPT_SPEC.md') -Raw
$policy = Get-Content (Join-Path $root 'governance\policy-registry.json') -Raw | ConvertFrom-Json
if (!$agents.Contains('read its root `MODULE.md` first') -or !$prompt.Contains('read its root `MODULE.md`')) {
  throw 'Module-first reading is not enforced in startup and worker instructions.'
}
$p17 = @($policy.policies | Where-Object { $_.id -eq 'P17' -and $_.status -eq 'approved' })
if ($p17.Count -ne 1) { throw 'Approved P17 module-context policy is missing.' }
Write-Output 'Module context policy verified: first-read, update and regeneration rules are enforced.'
