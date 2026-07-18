Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$source = Join-Path $root 'tools\prove-g1-c3-whole-cell-receiver-coupling.py'
$design = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_RECEIVER_COUPLING_MATHEMATICAL_DESIGN_AUDIT.md'
$result = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_RECEIVER_COUPLING_ORACLE_RESULT.md'
foreach ($path in @($source,$design,$result)) { if (-not (Test-Path -LiteralPath $path)) { throw "Missing receiver-coupling artifact: $path" } }
if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash.ToLowerInvariant() -ne '9ca84a39370ee32c5e29823ff8d4282e17cdaafd38286da43059001754a9460c') { throw 'Receiver-coupling oracle source drifted.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$receipt = (& $python $source | Out-String | ConvertFrom-Json)
if ($LASTEXITCODE -ne 0 -or $receipt.status -ne 'pass' -or $receipt.checks -ne 1020 -or $receipt.positive_cases -ne 12 -or $receipt.hostile_cases -ne 7 -or $receipt.hostile_rejections -ne 3 -or $receipt.checksum -ne '8c9c2c6d5f5d6ab38483d1e7c769b833d5d0373378cacbf86ff59ccfba4a91aa') { throw 'Receiver-coupling oracle receipt drifted.' }
$designText = Get-Content -LiteralPath $design -Raw
foreach ($required in @('receiver-before-face ordering','D - N > 0','exact quadratic polynomials','strict separating-axis zero','4-, 16- and 64-child','implementation-readiness audit','Do not add a crate')) { if ($designText -notlike "*$required*") { throw "Receiver-coupling design drift: $required" } }
$resultText = Get-Content -LiteralPath $result -Raw
foreach ($required in @('Portfolios: **12**','Hostile non-full portfolios: **7**','Checks: **1,020**','no production') ) { if ($resultText -notlike "*$required*") { throw "Receiver-coupling result drift: $required" } }
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or -not (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])) { throw 'Receiver-coupling checkpoint or preserved C3 route drifted.' }
Write-Output 'Whole-cell receiver coupling verified: exact correlated receiver-before-face proofs, strict zero, unresolved conservation, 12 portfolios and 1,020 checks pass without source authority.'
