$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$mapPath = Join-Path $root 'docs\canonical-system\COMPUTATIONAL_UNIVERSE_STEP_LEADER_CANDIDATE_MAP.md'
$contractPath = Join-Path $root 'contracts\step-leader-controller-contract.md'
$registryPath = Join-Path $root 'docs\canonical-system\system-registry.json'

$map = Get-Content $mapPath -Raw
$contract = Get-Content $contractPath -Raw
$registry = Get-Content $registryPath -Raw | ConvertFrom-Json
foreach ($system in @($registry.systems)) {
  if (!$map.Contains("| $($system.id) |")) { throw "Step-leader map misses registered system: $($system.id)" }
}
$mapped = [regex]::Matches($map, '(?m)^\| ([a-z0-9-]+) \|') | ForEach-Object { $_.Groups[1].Value } | Where-Object { $_ -ne 'System' }
if (@($mapped).Count -ne @($registry.systems).Count -or @($mapped | Sort-Object -Unique).Count -ne @($mapped).Count) {
  throw 'Step-leader map has duplicate or non-registry system rows.'
}
foreach ($required in @('VOI =','LocalNetGain =','ten percent','every result resumes the exact saved checkpoint','Ordinary edits and heartbeat wakes never trigger divergence')) {
  if (!$map.Contains($required) -and !$contract.Contains($required)) { throw "Step-leader invariant missing: $required" }
}
foreach ($rejected in @('universal P2P','observer-dependent truth','fixed execution radii','global utility')) {
  if (!$map.Contains($rejected)) { throw "Step-leader non-application missing: $rejected" }
}
Write-Output "Step-leader framework verified: $($registry.systems.Count) registered systems mapped."

