Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\optical-phase-space-cell-binding'
$sourcePath = Join-Path $crate 'src\lib.rs'
$testPath = Join-Path $crate 'tests\provenance.rs'
$manifestPath = Join-Path $crate 'Cargo.toml'
$contractPath = Join-Path $root 'contracts\optical-phase-space-cell-binding-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_CELL_PROVENANCE_IMPLEMENTATION_RESULT.md'
foreach ($path in @($sourcePath,$testPath,$manifestPath,$contractPath,$resultPath)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Optical phase-space implementation artifact missing: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$tests = Get-Content -LiteralPath $testPath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'TransverseAreaDirection4d','PositiveRationalV1','CorrelatedAffineOutputV1',
  'OpticalPhaseSpaceSplitReceiptV1','OpticalPhaseSpaceProjectionReceiptV1',
  'MAX_DEPTH: u8 = 12','ROOT_BITS: u16 = 192','LIVE_BITS: u16 = 368',
  'mindwarp.optical-phase-space.root.v1','mindwarp.optical-phase-space.cell.v1',
  'mindwarp.optical-phase-space.split.v1','mindwarp.optical-phase-space.projection.v1',
  'div_floor','div_ceil','deny_unknown_fields','none_evidence_only'
)) {
  if ($source -notlike "*$required*") { throw "Optical phase-space implementation drift: $required" }
}
foreach ($required in @('correlation_depth_caps_and_hostile_forms_fail_typed','direction_projection_rejects_values_outside_q1_62','root_split_and_projection_are_exact_and_strict','sixty_four_leaf_conservation_bit_edges_and_identity_fixtures','8724e0219d44bc40dbcb7315369dabe3153710617def82854d1ad490a802141f')) {
  if ($tests -notlike "*$required*") { throw "Optical phase-space test shield missing: $required" }
}
foreach ($required in @('fixed-interval-arithmetic = { path = "../fixed-interval-arithmetic" }','serde = { version = "1", features = ["derive"] }','serde_json = "1"','sha2 = "0.10"')) {
  if ($manifest -notmatch [regex]::Escape($required)) { throw "Optical phase-space dependency drift: $required" }
}
foreach ($forbidden in @('physical-path-substrate','visible-radiance','optical-lineage','optical-lane-transfer','receiver-arrival','reqwest','tauri')) {
  if ($manifest -like "*$forbidden*") { throw "Forbidden optical phase-space dependency: $forbidden" }
}
foreach ($required in @('capability-free evidence owner','collective gcd one','368-bit live shield','deletion-only')) {
  if ($contract -notlike "*$required*") { throw "Optical phase-space contract drift: $required" }
}
foreach ($required in @('implemented and fully','verified; this result does not authorize','20 positive portfolios, 33 hostile rejections','234.1 seconds','8724e0219d44bc40dbcb7315369dabe3153710617def82854d1ad490a802141f','no current crate imports')) {
  if ($result -notlike "*$required*") { throw "Optical phase-space result drift: $required" }
}
$authorizedTransportManifest = Join-Path $root 'crates\optical-phase-space-transport-certificate\Cargo.toml'
$authorizedReceiverCouplingManifest = Join-Path $root 'crates\optical-phase-space-receiver-coupling\Cargo.toml'
$authorizedSourceDistributionManifest = Join-Path $root 'crates\calibrated-source-energy-distribution\Cargo.toml'
$otherManifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object { $_.FullName -ne $manifestPath -and $_.FullName -ne $authorizedTransportManifest -and $_.FullName -ne $authorizedReceiverCouplingManifest -and $_.FullName -ne $authorizedSourceDistributionManifest }
foreach ($other in $otherManifests) {
  $otherManifest = Get-Content -LiteralPath $other.FullName -Raw
  $productionDependencies = ($otherManifest -split '(?m)^\[dev-dependencies\]\s*$')[0]
  if ($productionDependencies -match '(?m)^optical-phase-space-cell-binding\s*=') {
    throw "Current owner imported the additive prerequisite: $($other.FullName)"
  }
}
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
$legacyRoute = $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-additive-source-implementation','optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and
  $checkpoint.authority_lane -like '*Owner-approved exact additive package only*no coupling consumer*'
$transportRoute = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
if (-not $legacyRoute -and -not $transportRoute -and -not $federatedContinuity) {
  throw 'Optical phase-space implementation checkpoint or authority boundary drifted.'
}
Push-Location $root
try {
  & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $root 'tools\verify-g1-c3-optical-phase-space-cell-provenance.ps1')
  if ($LASTEXITCODE -ne 0) { throw 'Pinned optical phase-space oracle failed.' }
  & cargo test -p optical-phase-space-cell-binding --all-targets
  if ($LASTEXITCODE -ne 0) { throw 'Native optical phase-space tests failed.' }
  & cargo test -p optical-phase-space-cell-binding --target i686-pc-windows-msvc
  if ($LASTEXITCODE -ne 0) { throw 'Executable i686 optical phase-space tests failed.' }
  & cargo check -p optical-phase-space-cell-binding --target aarch64-linux-android
  if ($LASTEXITCODE -ne 0) { throw 'Android ARM64 optical phase-space check failed.' }
} finally { Pop-Location }
Write-Output 'Optical phase-space cell implementation verified: pinned oracle, frozen schema, exact split/projection arithmetic, strict codecs, dependency direction, authority boundary and native/i686/Android gates pass.'
