Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\optical-lineage-binding'
$sourcePath = Join-Path $crate 'src\lib.rs'
$testPath = Join-Path $crate 'tests\lineage.rs'
$manifestPath = Join-Path $crate 'Cargo.toml'
$contractPath = Join-Path $root 'contracts\optical-lineage-binding-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LINEAGE_BINDING_IMPLEMENTATION_RESULT.md'
foreach ($path in @($sourcePath,$testPath,$manifestPath,$contractPath,$resultPath,(Join-Path $crate 'MODULE.md'))) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing optical-lineage implementation artifact: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$tests = Get-Content -LiteralPath $testPath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'MAX_LINEAGE_STEPS: usize = 64','MAX_LINEAGE_MANIFEST_BYTES: usize = 1024 * 1024',
  'MAX_LINEAGE_BUNDLE_BYTES: usize = 16 * 1024 * 1024','MAX_LINEAGE_OBJECTS: usize = 384',
  'mindwarp.optical-lineage.lane.v1','mindwarp.optical-lineage.derived-source.v1',
  'mindwarp.optical-lineage.step.v1','mindwarp.optical-lineage.bundle-receipt.v1',
  'mindwarp.optical-lineage.transcript.v1','pub fn derive_optical_lane_id',
  'pub fn derive_optical_lineage_source_id','pub fn derive_optical_lineage_step_id',
  'pub fn compile_optical_lane_manifest','pub fn validate_optical_lane_manifest',
  'OuterDomainExit','UnavailableNeighbor','UnavailableCurrent','AmbiguousNextFace',
  'NoForwardProgress','AllTir','AmbiguousInterfaceBranch','NonconvergentInterface',
  'UnsupportedInterfaceModel','WorkExhaustion','none_evidence_only'
)) {
  if ($source -notmatch [regex]::Escape($required)) { throw "Optical-lineage source drift: $required" }
}
foreach ($dependency in @('physical-path-substrate','visible-radiance-bulk-transfer','visible-radiance-interface-event','serde','serde_json','sha2')) {
  if ($manifest -notmatch [regex]::Escape($dependency)) { throw "Missing optical-lineage dependency: $dependency" }
}
foreach ($forbidden in @('fixed-interval-arithmetic','crypto-bigint','std::fs','std::net','std::process','Command::new','f32','f64')) {
  if (($manifest + $source) -match [regex]::Escape($forbidden)) { throw "Forbidden optical-lineage mechanism: $forbidden" }
}
foreach ($requiredTest in @('bulk_terminal_families_are_replayed_and_strictly_encoded','real_interface_owner_outputs_map_to_frozen_terminals','independently_resealed_local_objects_do_not_break_adjacency','sixty_four_replayed_steps_end_only_in_typed_work_exhaustion')) {
  if ($tests -notmatch [regex]::Escape($requiredTest)) { throw "Missing optical-lineage test shield: $requiredTest" }
}
foreach ($requiredContract in @('between one and 64','complete terminal taxonomy','384 objects','16 MiB','1 MiB','no dependency on fixed-interval arithmetic','none_evidence_only')) {
  if ($contract -notlike "*$requiredContract*") { throw "Optical-lineage contract drift: $requiredContract" }
}
foreach ($requiredResult in @('implemented and verified','26 hostile cases','i686-pc-windows-msvc','aarch64-linux-android','57 local-owner','232.9 seconds','Actual mobile-device performance','unmeasured','no cumulative-power')) {
  if ($result -notlike "*$requiredResult*") { throw "Optical-lineage implementation result drift: $requiredResult" }
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$oracleOutput = & $python (Join-Path $root 'tools\prove-g1-c3-optical-lineage.py')
if ($LASTEXITCODE -ne 0 -or ($oracleOutput -join "`n") -notlike '*85b308953d2112a4bd2723c3e01ded96abdf4a33d16e9c1ff32cc4e3f0627937*') {
  throw 'Pinned optical-lineage oracle receipt drifted.'
}
Write-Output 'Optical-lineage binding verified: frozen domains, exact owner replay, derived adjacency, ten typed terminals, hard caps, capability exclusions and pinned oracle are retained.'
