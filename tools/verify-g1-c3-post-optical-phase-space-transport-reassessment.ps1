Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_POST_OPTICAL_PHASE_SPACE_TRANSPORT_CONSUMER_REASSESSMENT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
foreach ($required in @(
  'same-medium ordered-face',
  'receiver-before-face ordering',
  'face-coincident special case',
  'exact accepted, zero and unresolved measure conservation',
  'current exact-ray receiver cannot supply this by reuse',
  'code-free mathematical design and disposable exact oracle only',
  'No crate, schema, dependency, production test',
  'or source is authorized',
  'no coupling, arrival, accepted fraction',
  'materially new owner'
)) {
  if ($audit -notlike "*$required*") { throw "Post-transport reassessment is missing: $required" }
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$selectedDesignRoute = $c3.Count -eq 1 -and (
    ($c3[0].next_action -like '*whole-cell receiver-coupling mathematical design*' -and
     ($c3[0].next_action -like '*Do not add a crate, schema, dependency, production test or source*' -or
      $c3[0].next_action -like '*exact serious owner decision*G1_C3_WHOLE_CELL_RECEIVER_COUPLING_IMPLEMENTATION_READINESS.md*Do not add*crate*dependency*contract schema*production test or production source*' -or
      $c3[0].next_action -like '*owner-approved*optical-phase-space-receiver-coupling*Modify no existing owner source or V1 behavior*')) -or
    ($c3[0].next_action -like '*code-free mathematical design*whole-cell dimensionless transfer*' -and
     $c3[0].sources -contains 'G1_C3_WHOLE_CELL_RECEIVER_COUPLING_IMPLEMENTATION_RESULT.md') -or
    ($c3[0].next_action -like '*code-facing readiness/gap audit*whole-cell dimensionless-transfer*' -and
     $c3[0].sources -contains 'G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_ORACLE_RESULT.md') -or
    $c3[0].next_action -like '*owner-authorized whole-cell dimensionless-transfer implementation*' -or
    $c3[0].next_action -like '*code-free post-result consumer reassessment*whole-cell dimensionless-transfer*' -or
    $c3[0].next_action -like '*code-free source-distribution*phase-space-measure compatibility*' -or
    $c3[0].next_action -like '*code-facing source-quantity-basis*schema gap audit*' -or
    $c3[0].next_action -like '*source-quantity-basis mathematical design audit*' -or
    $c3[0].next_action -like '*calibrated spectral/time basis mathematical design audit*' -or
    $c3[0].next_action -like '*calibrated-basis*transport-applicability schema gap audit*' -or
    $c3[0].next_action -like '*implementation-readiness audit*source-calibration sibling*' -or
    $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -or
    $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -or
    $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*' -or
    $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*')
if (-not $selectedDesignRoute -or
    $c3[0].sources -notcontains 'G1_C3_POST_OPTICAL_PHASE_SPACE_TRANSPORT_CONSUMER_REASSESSMENT.md') {
  throw 'C3 does not retain the selected post-transport code-free design route.'
}
$c3Checkpoint = $checkpoint.substage_id -eq 'post-optical-phase-space-transport-consumer-reassessment' -and
  $checkpoint.authority_lane -like '*Code-free reassessment only*No crate, schema, dependency, production test or source*coupling, arrival, power, visibility, runtime, promotion and C3 closure remain excluded*'
$preservedByLaterPackage = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
if (-not ($c3Checkpoint -or $preservedByLaterPackage)) {
  throw 'Post-transport reassessment checkpoint or authority boundary drifted.'
}
Write-Output 'Post optical phase-space transport reassessment verified: ordered correlated face derivation is consumable upstream, while general receiver ordering and coupling remain code-free and separately gated.'
