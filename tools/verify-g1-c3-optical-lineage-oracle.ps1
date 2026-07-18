Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$scriptPath = Join-Path $root 'tools\prove-g1-c3-optical-lineage.py'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LINEAGE_COUNTEREXAMPLE_ORACLE_RESULT.md'
$expectedSource = 'baeb6d315e422af8932d4aa5706d44d290d5b8028f548baf558fac94fc778385'
$actualSource = (Get-FileHash -LiteralPath $scriptPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actualSource -ne $expectedSource) { throw 'Optical-lineage oracle source checksum drifted.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$receipt = (& $python $scriptPath | Out-String | ConvertFrom-Json)
if ($LASTEXITCODE -ne 0) { throw 'Optical-lineage oracle failed.' }
if ($receipt.receipt_sha256 -ne '85b308953d2112a4bd2723c3e01ded96abdf4a33d16e9c1ff32cc4e3f0627937' -or
    $receipt.candidate -ne 'accepted_for_code_facing_readiness_audit_only' -or
    $receipt.hostile_rejection_count -ne 26 -or
    $receipt.typed_terminal_count -ne 10 -or
    $receipt.portfolios.one_lane_64_steps.total_steps -ne 64 -or
    $receipt.portfolios.three_lanes_64_steps.total_steps -ne 192 -or
    $receipt.portfolios.three_lanes_64_steps.manifest_bytes -ne 142834 -or
    $receipt.portfolios.three_lanes_64_steps.bundle_canonical_bytes -ne 387186 -or
    $receipt.portfolios.three_lanes_64_steps.conservative_validation_bytes -ne 530534 -or
    -not $receipt.portfolios.three_lanes_64_steps.manifest_under_1_mib -or
    -not $receipt.portfolios.three_lanes_64_steps.bundle_under_48_mib -or
    -not $receipt.portfolios.three_lanes_64_steps.conservative_validation_under_64_mib) {
  throw 'Optical-lineage oracle canonical receipt drifted.'
}
foreach ($name in @('resealed_hit_narrowing','resealed_source_alias','resealed_wrong_interface_cells','resealed_incident_direction','resealed_cross_band_direction','resealed_early_terminal')) {
  if (@($receipt.hostile_rejections | Where-Object { $_ -eq $name }).Count -ne 1) { throw "Missing resealed hostile lineage case: $name" }
}
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('thin per-band manifest plus explicit replayed bundle survives','26 hostile cases','six **resealed attacker** cases','Ten terminal families','unsupported interface model','41,102,834 bytes','Cumulative power','cannot claim receiver arrival','no source authorized')) {
  if ($result -notlike "*$required*") { throw "Optical-lineage oracle result drift: $required" }
}
if (Test-Path -LiteralPath (Join-Path $root 'crates\optical-lineage-binding')) {
  $checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
  $program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
  $c3 = @($program.items | Where-Object id -eq 'C3')
  $federatedContinuity = $c3.Count -eq 1 -and (& (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0])
  if (-not $federatedContinuity -and ($checkpoint.batch_id -notin @('G1-C3-OPTICAL-LINEAGE-BINDING-IMPLEMENTATION-V1','G1-C3-POST-OPTICAL-LINEAGE-CONSUMER-REASSESSMENT-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-MATHEMATICAL-DESIGN-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-READINESS-V1','G1-C3-CUMULATIVE-LANE-TRANSFER-IMPLEMENTATION-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-MATHEMATICAL-DESIGN-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-READINESS-V1','G1-C3-RECEIVER-ARRIVAL-GEOMETRY-IMPLEMENTATION-V1') -or
      ($checkpoint.authority_lane -notlike '*explicitly approved*' -and $checkpoint.authority_lane -notlike '*Audit and counterexample work only*' -and $checkpoint.authority_lane -notlike '*No crate*receiver arrival*visibility*C3 closure*' -and $checkpoint.authority_lane -notlike '*Mathematical design and independent oracle only*No crate*schema*source*visibility*C3 closure*' -and $checkpoint.authority_lane -notlike '*requires explicit owner approval*No crate*schema*source*visibility*C3 closure*' -and $checkpoint.authority_lane -notlike '*Readiness audit only and explicit owner gate*Existing owners remain unchanged*visibility*runtime*promotion*C3 closure*' -and $checkpoint.authority_lane -notlike '*Owner-approved exact additive package only*Existing owner source and behavior remain unchanged*visibility*runtime*promotion*C3 closure*' -and $checkpoint.authority_lane -notlike '*Owner-authorized implementation*optical-phase-space-transport-certificate*no coupling consumer*no arrival, power, visibility, runtime, promotion or C3-closure authority*'))) {
    throw 'Optical-lineage production crate appeared without the exact approved owner action.'
  }
}
Write-Output 'Optical-lineage oracle verified: 26 hostile rejections including six fully resealed attacks, ten typed terminals and bounded 1/3/64/192-step manifest-plus-bundle portfolios.'
