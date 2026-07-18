Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\optical-phase-space-receiver-coupling'
$source = Join-Path $crate 'src\lib.rs'
$tests = Join-Path $crate 'tests\coupling.rs'
$contract = Join-Path $root 'contracts\optical-phase-space-receiver-coupling-contract.md'
$result = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_RECEIVER_COUPLING_IMPLEMENTATION_RESULT.md'
foreach ($path in @($source,$tests,$contract,$result)) { if (-not (Test-Path -LiteralPath $path)) { throw "Missing receiver-coupling implementation artifact: $path" } }
if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash.ToLowerInvariant() -ne 'a3c2ec8a22587b27b59239f882320e4e6b0bac82ad5bc17712fa72b944e7fe93') { throw 'Receiver-coupling implementation source drifted.' }
$sourceText = Get-Content -LiteralPath $source -Raw
foreach ($required in @('MAXIMUM_LIVE_BITS: u16 = 391','MAX_CHECKED_INTEGER_OPERATIONS: u32 = 16_384','MAX_BOUND_COMPARISONS: u16 = 4_096','MAX_INPUT_BYTES: usize = 40 * 1024 * 1024','MAX_RESULT_BYTES: usize = 256 * 1024','MAX_AGGREGATE_LIVE_CANONICAL_BYTES: usize = 64 * 1024 * 1024','WholeCellReceiverCouplingInputV1','WholeCellReceiverCouplingOutcomeV1','CertifiedFullBeforeFace','CertifiedZeroBeforeFace','UnresolvedReceiverCoupling','compile_whole_cell_receiver_coupling','validate_whole_cell_receiver_coupling','mindwarp.optical-phase-space.receiver-coupling.input.v1','mindwarp.optical-phase-space.receiver-coupling.result.v1','none_evidence_only')) { if ($sourceText -notlike "*$required*") { throw "Receiver-coupling source drift: $required" } }
$manifest = Get-Content -LiteralPath (Join-Path $crate 'Cargo.toml') -Raw
foreach ($required in @('fixed-interval-arithmetic','optical-phase-space-cell-binding','optical-phase-space-transport-certificate','physical-path-substrate','receiver-arrival-geometry-binding')) { if ($manifest -notlike "*$required*") { throw "Receiver-coupling dependency drift: $required" } }
foreach ($forbidden in @('crypto-bigint','num-bigint','tokio','reqwest')) { if ($manifest -like "*$forbidden*") { throw "Receiver-coupling forbidden dependency: $forbidden" } }
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-receiver-coupling.ps1')
if (-not $?) { throw 'Receiver-coupling mathematical verifier failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-receiver-coupling-readiness.ps1')
if (-not $?) { throw 'Receiver-coupling readiness verifier failed.' }
& cargo test -p optical-phase-space-receiver-coupling --all-targets
if ($LASTEXITCODE -ne 0) { throw 'Receiver-coupling native tests failed.' }
& cargo test -p optical-phase-space-receiver-coupling --target i686-pc-windows-msvc
if ($LASTEXITCODE -ne 0) { throw 'Receiver-coupling executable i686 tests failed.' }
& cargo check -p optical-phase-space-receiver-coupling --target aarch64-linux-android
if ($LASTEXITCODE -ne 0) { throw 'Receiver-coupling Android compilation failed.' }
& (Join-Path $PSScriptRoot 'verify-module-context.ps1')
if (-not $?) { throw 'Receiver-coupling module context failed.' }
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or -not (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])) { throw 'Receiver-coupling implementation route drifted.' }
Write-Output 'Whole-cell receiver-coupling implementation verified: strict nested replay, 391-bit correlated arithmetic, conservative outcomes, exact measure, identities/codecs and native/i686/Android gates pass without existing-owner authority.'
