Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\optical-phase-space-transport-certificate'
$sourcePath = Join-Path $crate 'src\lib.rs'
$manifestPath = Join-Path $crate 'Cargo.toml'
$contractPath = Join-Path $root 'contracts\optical-phase-space-transport-certificate-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_PHASE_SPACE_TRANSPORT_CERTIFICATE_IMPLEMENTATION_RESULT.md'
$repairOracle = Join-Path $PSScriptRoot 'prove-g1-c3-optical-phase-space-transport-residual-containment.py'
foreach ($path in @($sourcePath,$manifestPath,$contractPath,$resultPath,$repairOracle)) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Transport-certificate implementation artifact missing: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'OriginAnchoredTransportInputV1','OriginAnchoredFaceStepV1','OriginAnchoredTransportCertificateV1',
  'InterfaceRequired','OuterDomainExit','UnavailableNeighbor','AmbiguousNextFace','NoForwardProgress',
  'ArithmeticShieldExceeded','ProjectionOutOfRange','WorkExhausted',
  'IMMUTABLE_ORIGIN_SCALAR_BITS: u16 = 64','DERIVED_MAXIMUM_LIVE_BITS: u16 = 490',
  'MAXIMUM_STEPS: u8 = 64','MAX_CHECKED_INTEGER_OPERATIONS: u32 = 24_576',
  'mindwarp.optical-phase-space.transport.input.v1','mindwarp.optical-phase-space.transport.form.v1',
  'mindwarp.optical-phase-space.transport.step.v1','mindwarp.optical-phase-space.transport.certificate.v1',
  'QuadraticPolynomial','polynomial_bounds','compile_conditional_interval_cell_step',
  'deny_unknown_fields','none_evidence_only','3e, 0x56, 0xfe, 0xc3'
)) {
  if ($source -notlike "*$required*") { throw "Transport-certificate source drift: $required" }
}
foreach ($required in @(
  'fixed-interval-arithmetic = { path = "../fixed-interval-arithmetic" }',
  'optical-phase-space-cell-binding = { path = "../optical-phase-space-cell-binding" }',
  'physical-path-substrate = { path = "../physical-path-substrate" }',
  'serde = { version = "1", features = ["derive"] }','serde_json = "1"','sha2 = "0.10"'
)) {
  if ($manifest -notmatch [regex]::Escape($required)) { throw "Transport-certificate dependency drift: $required" }
}
foreach ($forbidden in @('forge-kernel','tauri','reqwest','visible-radiance','optical-lineage-binding','optical-lane-transfer-binding','receiver-arrival-geometry-binding','crypto-bigint')) {
  if ($manifest -like "*$forbidden*") { throw "Forbidden transport-certificate dependency: $forbidden" }
}
foreach ($required in @('exact quadratic polynomial','D*S*V_endpoint*b^2','490 bits','deletion-only')) {
  if ($contract -notlike "*$required*") { throw "Transport-certificate contract drift: $required" }
}
foreach ($required in @('implemented and focused/platform verified','20,000','481 bits','8,691','3e56fec3951c34f7aa54dc3dc469bf5ba62d289da4785b03ea63e39b4f694f29','does not authorize coupling')) {
  if ($result -notlike "*$required*") { throw "Transport-certificate result drift: $required" }
}
if ((Get-FileHash -Algorithm SHA256 -LiteralPath $repairOracle).Hash.ToLowerInvariant() -ne '525f8a1a235812da50594de9cf1bac97fa5c86243402dd05f94e843a267d2c21') {
  throw 'Transport residual-containment oracle source drift.'
}
$authorizedReceiverCouplingManifest = Join-Path $root 'crates\optical-phase-space-receiver-coupling\Cargo.toml'
$authorizedDimensionlessTransferManifest = Join-Path $root 'crates\optical-phase-space-dimensionless-transfer\Cargo.toml'
$otherManifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object { $_.FullName -ne $manifestPath -and $_.FullName -ne $authorizedReceiverCouplingManifest -and $_.FullName -ne $authorizedDimensionlessTransferManifest }
foreach ($other in $otherManifests) {
  if ((Get-Content -LiteralPath $other.FullName -Raw) -match '(?m)^optical-phase-space-transport-certificate\s*=') {
    throw "Current owner imported transport-certificate sibling: $($other.FullName)"
  }
}
$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
Push-Location $root
try {
  $repair = & $python $repairOracle | ConvertFrom-Json
  if ($repair.receipt_sha256 -ne '70d171b5c377db29676b4d401a335a6f417124fa739665f62c27956c73daa987' -or $repair.residual_falsifiers -ne 20000 -or $repair.physical_face_containment_axes -ne 2) {
    throw 'Transport residual-containment receipt drift.'
  }
  & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-transport-readiness.ps1')
  if ($LASTEXITCODE -ne 0) { throw 'Pinned transport readiness/oracle family failed.' }
  & cargo test -p optical-phase-space-transport-certificate --all-targets
  if ($LASTEXITCODE -ne 0) { throw 'Native transport-certificate tests failed.' }
  & cargo test -p optical-phase-space-transport-certificate --target i686-pc-windows-msvc
  if ($LASTEXITCODE -ne 0) { throw 'Executable i686 transport-certificate tests failed.' }
  & cargo check -p optical-phase-space-transport-certificate --target aarch64-linux-android
  if ($LASTEXITCODE -ne 0) { throw 'Android ARM64 transport-certificate check failed.' }
  & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-modularity.ps1')
  if ($LASTEXITCODE -ne 0) { throw 'Transport-certificate modularity gate failed.' }
  & powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot 'verify-module-context.ps1')
  if ($LASTEXITCODE -ne 0) { throw 'Transport-certificate module-context gate failed.' }
} finally { Pop-Location }
Write-Output 'Optical phase-space transport-certificate implementation verified: immutable-origin algebra, quadratic residual containment, strict replay/codecs, frozen identity, native/i686/Android, modularity and authority-negative boundaries pass.'
