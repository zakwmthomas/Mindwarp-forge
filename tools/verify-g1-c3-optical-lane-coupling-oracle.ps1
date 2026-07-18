Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'tools\prove-g1-c3-optical-lane-coupling-measure.py'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LANE_COUPLING_MEASURE_ORACLE_RESULT.md'
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LANE_COUPLING_MEASURE_MATHEMATICAL_DESIGN_AUDIT.md'
foreach ($path in @($sourcePath,$resultPath,$designPath)) { if (-not (Test-Path -LiteralPath $path)) { throw "Missing coupling oracle artifact: $path" } }
$sourceHash = (Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($sourceHash -ne '368064c569fee8da613b0463c5322e93d3bd2870c4fc06aac68d5b720f8dab87') { throw 'Coupling oracle source hash drifted.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$output = & $python $sourcePath
if ($LASTEXITCODE -ne 0 -or ($output -join "`n") -notlike '*19e9b252a965e5a154d6864a4a426d47015b987a4932d3694ffc04a15d722d84*') { throw 'Coupling oracle receipt drifted.' }
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('rejected as sufficient','coupling evidence','Twelve portfolios','twenty hostile','phase-space measure is invariant','central ray but produce','different exact footprints','central arrival is not full receiver acceptance','centre is inside','unsupported_caustic_or_fold','not implementation-ready','No crate')) {
  if ($result -notlike "*$required*") { throw "Coupling oracle result drift: $required" }
}
if (Test-Path -LiteralPath (Join-Path $root 'crates\optical-lane-coupling-measure')) { throw 'Coupling crate appeared after a rejected oracle candidate.' }
Write-Output 'Optical lane coupling oracle verified: 12 exact portfolios and 20 hostiles reject central/corner sufficiency, preserve refinement and passive bounds, and authorize no schema.'
