Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourcePath = Join-Path $root 'tools\prove-g1-c3-whole-cell-optical-coupling.py'
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_OPTICAL_COUPLING_MATHEMATICAL_DESIGN_AUDIT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_OPTICAL_COUPLING_ORACLE_RESULT.md'
foreach ($path in @($sourcePath,$designPath,$resultPath)) { if (-not (Test-Path -LiteralPath $path)) { throw "Missing whole-cell coupling artifact: $path" } }
if ((Get-FileHash -LiteralPath $sourcePath -Algorithm SHA256).Hash.ToLowerInvariant() -ne '5a6502863850d68c42d8cd5719455a937cb85690db14330ecc5a9624c0bf7bd2') { throw 'Whole-cell coupling oracle source drifted.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$output = & $python $sourcePath
if ($LASTEXITCODE -ne 0 -or ($output -join "`n") -notlike '*0899edb94e23d4d4aff684bbe4b44a11432a1cd1e26c9e394b8e1c4fd6a913b1*') { throw 'Whole-cell coupling oracle receipt drifted.' }
$design = Get-Content -LiteralPath $designPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('certified_full_cell_arrival','certified_zero_cell_arrival','unresolved_cell_coupling','correlation erasure','accepted measure','zero measure','unresolved measure','implementation-ready because','Do not add a crate')) { if ($design -notlike "*$required*") { throw "Whole-cell coupling design drift: $required" } }
foreach ($required in @('abstract full/zero/unresolved classifier survives','Sixteen exact portfolios','twenty-four hostile','1/4 + 1/4 + 1/2 = 1','u-u=0','independent box','implementation remains blocked','code-facing provenance/correlation gap audit')) { if ($result -notlike "*$required*") { throw "Whole-cell coupling result drift: $required" } }
if (Test-Path -LiteralPath (Join-Path $root 'crates\optical-lane-coupling-measure')) { throw 'Whole-cell coupling crate appeared before provenance/correlation readiness.' }
Write-Output 'Whole-cell optical coupling verified: 16 exact portfolios and 24 hostiles preserve strict full/zero/unresolved accounting, refinement measure and the provenance/correlation implementation blocker.'
