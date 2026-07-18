Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$crate = Join-Path $root 'crates\receiver-arrival-geometry-binding'
$sourcePath = Join-Path $crate 'src\lib.rs'
$testPath = Join-Path $crate 'tests\receiver.rs'
$manifestPath = Join-Path $crate 'Cargo.toml'
$contractPath = Join-Path $root 'contracts\receiver-arrival-geometry-binding-contract.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_RECEIVER_ARRIVAL_GEOMETRY_IMPLEMENTATION_RESULT.md'
foreach ($path in @($sourcePath,$testPath,$manifestPath,$contractPath,$resultPath,(Join-Path $crate 'MODULE.md'))) {
  if (-not (Test-Path -LiteralPath $path)) { throw "Missing receiver-arrival artifact: $path" }
}
$source = Get-Content -LiteralPath $sourcePath -Raw
$tests = Get-Content -LiteralPath $testPath -Raw
$manifest = Get-Content -LiteralPath $manifestPath -Raw
$contract = Get-Content -LiteralPath $contractPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('MAX_RECEIVER_INPUT_BYTES','18 * 1024 * 1024','MAX_RECEIVER_OUTPUT_BYTES','256 * 1024','MAX_VALIDATION_LIVE_CANONICAL_BYTES','32 * 1024 * 1024','MAX_RECEIVER_STEPS','MAXIMUM_LIVE_BITS','MAX_DIRECTED_DIVISIONS','MAX_BOUND_COMPARISONS','MAX_INTERSECTIONS','mindwarp.receiver-arrival.aabb.v1','mindwarp.receiver-arrival.result.v1','mindwarp.receiver-arrival.transcript.v1','pub fn compile_receiver_arrival_geometry','pub fn validate_receiver_arrival_geometry','UnsupportedConditionalEvidence','ArrivalAtStart','CertifiedStrictInteriorArrival','UpstreamTerminalWithoutFace','NoArrivalBeforeLineageTerminal','none_evidence_only')) {
  if ($source -notmatch [regex]::Escape($required)) { throw "Receiver-arrival source drift: $required" }
}
foreach ($dependency in @('optical-lineage-binding','physical-path-substrate','fixed-interval-arithmetic','serde','serde_json','sha2')) {
  if ($manifest -notmatch [regex]::Escape($dependency)) { throw "Missing receiver-arrival dependency: $dependency" }
}
foreach ($forbidden in @('optical-lane-transfer-binding','forge-kernel','tauri','reqwest','ureq','hyper','tokio','std::fs','std::net','std::process','Command::new','f32','f64')) {
  if (($manifest + $source) -match [regex]::Escape($forbidden)) { throw "Forbidden receiver-arrival mechanism: $forbidden" }
}
foreach ($requiredTest in @('strict_entry_start_inside_no_arrival_and_contact_are_distinct','conditional_evidence_is_typed_unsupported_and_not_sampled','reverse_direction_and_parallel_outside_are_ordered_exactly','receiver_face_tie_is_contact_then_successor_owned_arrival','receiver_identity_bounds_codecs_and_replay_fail_closed')) {
  if ($tests -notmatch [regex]::Escape($requiredTest)) { throw "Missing receiver-arrival test shield: $requiredTest" }
}
foreach ($requiredContract in @('positive-volume','unsupported_conditional_evidence','0 <= t < t_face','414-bit','384 directed divisions','768 comparisons','18 MiB','256 KiB','32 MiB','none_evidence_only','no source')) {
  if ($contract -notlike "*$requiredContract*") { throw "Receiver-arrival contract drift: $requiredContract" }
}
foreach ($requiredResult in @('implemented and verified','receiver-arrival-geometry-binding','18 exact-rational portfolios','26','hostile mutation families','i686-pc-windows-msvc','aarch64-linux-android','229.3 seconds','deletion-only','No real mobile-device performance')) {
  if ($result -notlike "*$requiredResult*") { throw "Receiver-arrival result drift: $requiredResult" }
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$oracleOutput = & $python (Join-Path $root 'tools\prove-g1-c3-receiver-arrival-geometry.py')
if ($LASTEXITCODE -ne 0 -or ($oracleOutput -join "`n") -notlike '*25c31003ff4ee8d1be3b01a5a2203958238205e4adc80e6cc50623c27af69aea*') {
  throw 'Pinned receiver-arrival geometry oracle receipt drifted.'
}
Push-Location $root
try {
  $env:RUSTFLAGS = '-D warnings'
  & cargo test -p receiver-arrival-geometry-binding --all-targets
  if ($LASTEXITCODE -ne 0) { throw 'Receiver-arrival focused tests failed.' }
} finally { Pop-Location }
Write-Output 'Receiver-arrival geometry binding verified: exact-ray strict-interior AABB ordering, typed conditional exclusion, face-tie ownership, strict replay, hard caps, authority exclusions and pinned oracle are retained.'
