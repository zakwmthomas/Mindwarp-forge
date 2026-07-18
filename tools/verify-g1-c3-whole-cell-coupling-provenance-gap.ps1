Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$auditPath = Join-Path $root 'docs\canonical-system\G1_C3_WHOLE_CELL_COUPLING_PROVENANCE_CORRELATION_GAP_AUDIT.md'
if (-not (Test-Path -LiteralPath $auditPath)) { throw 'Whole-cell coupling provenance/correlation gap audit is missing.' }
$audit = Get-Content -LiteralPath $auditPath -Raw
foreach ($required in @(
  'code-facing gap verified',
  'ConditionalIntervalCellStepInputV1',
  'VisibleRadianceIntervalInterfaceInputV1',
  'derive_optical_lane_id',
  'UnsupportedConditionalEvidence',
  'Adopt / adapt / build comparison',
  'optical-phase-space-cell-binding',
  'exact nonnegative rational measure',
  'correlation-preserving coordinate form',
  'receiver-independent',
  'deletion of the additive owner',
  '16-portfolio / 24-hostile',
  'Do not add a crate',
  'requires explicit owner approval'
)) {
  if ($audit -notlike "*$required*") { throw "Whole-cell provenance/correlation audit drift: $required" }
}

$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$c3 = @($program.items | Where-Object id -eq 'C3')
$gapRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*code-facing provenance and correlation gap audit*Add no crate, dependency, schema, test or production source*'
$proofRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*mathematical design and disposable exact-rational oracle*phase-space provenance prerequisite*Add no crate, dependency, schema, production test or production source*'
$readinessRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*code-facing implementation-readiness audit*optical phase-space provenance prerequisite*Add no crate, dependency, contract schema, production test or production source*'
$ownerRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*Owner: approve or reject*optical phase-space provenance prerequisite*Without explicit approval add no crate, dependency, contract schema, production test or production source*'
$implementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved optical phase-space provenance prerequisite package*optical-phase-space-cell-binding*Add no coupling consumer*'
$transportDesignRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*optical-phase-space-transport-certificate*Add no coupling consumer*production test or source*'
$transportOwnerRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*hold the exact serious owner decision*64-bit immutable-origin cap*Add no coupling consumer or production source*'
$transportImplementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*optical phase-space provenance prerequisite*owner-approved additive optical-phase-space-transport-certificate implementation*post-transport consumer reassessment*'
$postTransportRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*whole-cell receiver-coupling mathematical design*' -and $c3[0].sources -contains 'G1_C3_WHOLE_CELL_COUPLING_PROVENANCE_CORRELATION_GAP_AUDIT.md'
$postReceiverRoute = $c3.Count -eq 1 -and
  ($c3[0].next_action -like '*code-free mathematical design*whole-cell dimensionless transfer*' -or
   $c3[0].next_action -like '*code-facing readiness/gap audit*whole-cell dimensionless-transfer*' -or
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
   $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*') -and
  $c3[0].sources -contains 'G1_C3_WHOLE_CELL_COUPLING_PROVENANCE_CORRELATION_GAP_AUDIT.md'
if (-not ($gapRoute -or $proofRoute -or $readinessRoute -or $ownerRoute -or $implementationRoute -or $transportDesignRoute -or $transportOwnerRoute -or $transportImplementationRoute -or $postTransportRoute -or $postReceiverRoute) -or
    ($c3[0].proof -notlike '*16 exact portfolios and 24 hostiles*' -and $c3[0].proof -notlike '*sampling and whole-cell coupling oracles retain their negative and conservative results*' -and $c3[0].proof -notlike '*12 exact-rational portfolios*1,020 checks*' -and $c3[0].proof -notlike '*receiver-coupling sibling is implemented and verified*' -and $c3[0].proof -notlike '*exact-rational oracle passes 16 portfolios*' -and $c3[0].proof -notlike '*whole-cell dimensionless-transfer result*' -and $c3[0].proof -notlike '*source-distribution mathematical design*additive source-quantity measure*' -and $c3[0].proof -notlike '*whole-cell dimensionless-transfer result*source-distribution oracle*' -and $c3[0].proof -notlike '*source-quantity basis mathematical design*exact-rational oracle*' -and $c3[0].proof -notlike '*calibrated spectral/time mathematical design*34 hostile*' -and $c3[0].proof -notlike '*calibrated-basis and transport-applicability*no current owner or schema*' -and $c3[0].proof -notlike '*owner-approved calibrated-spectral-time-basis*fully verified*zero consumers*' -and $c3[0].proof -notlike '*closed-frontier additive calibrated radiant-energy measure*exact phase-space root*' -and $c3[0].proof -notlike '*compact axis-bearing*zero downstream consumers*' -and $c3[0].proof -notlike '*owner explicitly approved*zero downstream consumers*' -and $c3[0].proof -notlike '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*') -or
    ($c3[0].proof -notlike '*source phase-space parent measure*' -and $c3[0].proof -notlike '*boxes lack parent measure and correlation*' -and $c3[0].proof -notlike '*gap audit rejects current-owner reuse and mutation*' -and $c3[0].proof -notlike '*post-provenance reassessment closes measure/ancestry/correlation*' -and $c3[0].proof -notlike '*surviving immutable-origin oracle source*64-bit input cap*' -and $c3[0].proof -notlike '*immutable-origin shared-symbol*391-bit*' -and $c3[0].proof -notlike '*receiver-coupling sibling is implemented and verified*' -and $c3[0].proof -notlike '*per-step optical depth*' -and $c3[0].proof -notlike '*preserves exact accepted/zero/unresolved measure*' -and $c3[0].proof -notlike '*exact cell algebra*21 hostile rejections*' -and $c3[0].proof -notlike '*no current owner defines physical source quantity*' -and $c3[0].proof -notlike '*current RGB*no wavelength intervals*' -and $c3[0].proof -notlike '*physical received-energy composition remains blocked*' -and $c3[0].proof -notlike '*Transport applicability remains blocked*' -and $c3[0].proof -notlike '*compact axis-bearing*zero downstream consumers*' -and $c3[0].proof -notlike '*owner explicitly approved*zero downstream consumers*' -and $c3[0].proof -notlike '*Exact cell identity is necessary but insufficient*')) {
  throw 'C3 no longer retains the verified whole-cell provenance/correlation gap route.'
}

$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$gapCheckpoint = $checkpoint.substage_id -eq 'whole-cell-coupling-provenance-correlation-gap-audit' -and
  $checkpoint.authority_lane -like '*Code-facing audit only*Do not add or modify a crate, dependency, schema, test or production source*' -and
  $checkpoint.resume_after -like '*mathematical and disposable-oracle package*owner approval*'
$proofCheckpoint = $checkpoint.substage_id -eq 'optical-phase-space-cell-provenance-design-and-oracle' -and
  $checkpoint.authority_lane -like '*Mathematical design and disposable exact-rational oracle only*Do not add or modify a crate, dependency, schema, production test or production source*' -and
  $checkpoint.resume_after -like '*implementation-readiness audit*explicit owner approval*'
$readinessCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-implementation-readiness','optical-phase-space-cell-provenance-owner-gate') -and
  $checkpoint.authority_lane -like '*Readiness audit only and explicit owner gate*Do not add or modify a crate, dependency, contract schema, production test or production source*' -and
  ($checkpoint.resume_after -like '*Await explicit owner approval*' -or $checkpoint.resume_after -like '*On explicit approval*No source action while waiting*')
$implementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-cell-provenance-additive-source-implementation','optical-phase-space-cell-provenance-post-result-consumer-reassessment','optical-phase-space-transport-certificate-design-and-oracle','optical-phase-space-transport-certificate-arithmetic-width-spike','optical-phase-space-transport-origin-anchored-design-and-oracle','optical-phase-space-transport-origin-anchored-implementation-readiness','optical-phase-space-transport-origin-anchored-owner-gate') -and
  $checkpoint.authority_lane -like '*Owner-approved exact additive package only*no coupling consumer*'
$transportImplementationCheckpoint = $checkpoint.substage_id -in @('optical-phase-space-transport-origin-anchored-implementation','optical-phase-space-transport-origin-anchored-verification','optical-phase-space-transport-origin-anchored-result') -and
  $checkpoint.authority_lane -like '*Owner-authorized implementation*optical-phase-space-transport-certificate*no arrival*'
$federatedCheckpoint = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint -C3 $c3[0]
if (-not ($gapCheckpoint -or $proofCheckpoint -or $readinessCheckpoint -or $implementationCheckpoint -or $transportImplementationCheckpoint -or $federatedCheckpoint)) {
  throw 'Whole-cell provenance/correlation checkpoint or authority boundary drifted.'
}

if (-not $implementationCheckpoint -and -not $transportImplementationCheckpoint -and -not $postTransportRoute -and -not $postReceiverRoute) {
  foreach ($forbidden in @(
    'crates\optical-phase-space-cell-binding',
    'contracts\optical-phase-space-cell-binding-contract.md'
  )) {
    if (Test-Path -LiteralPath (Join-Path $root $forbidden)) { throw "Unauthorized phase-space schema appeared: $forbidden" }
  }
}

$physical = Get-Content -LiteralPath (Join-Path $root 'crates\physical-path-substrate\src\interval.rs') -Raw
$interface = Get-Content -LiteralPath (Join-Path $root 'crates\visible-radiance-interface-event\src\interval.rs') -Raw
$lineage = Get-Content -LiteralPath (Join-Path $root 'crates\optical-lineage-binding\src\lib.rs') -Raw
$receiver = Get-Content -LiteralPath (Join-Path $root 'crates\receiver-arrival-geometry-binding\src\lib.rs') -Raw
foreach ($check in @(
  @($physical, 'DeclaredConditionalPointDirectionBox'),
  @($interface, 'DeclaredConditionalDirectionBox'),
  @($lineage, 'initial_interval_cell_step_input_id'),
  @($receiver, 'UnsupportedConditionalEvidence')
)) {
  if ($check[0] -notlike "*$($check[1])*") { throw "Current owner seam drifted: $($check[1])" }
}

Write-Output 'Whole-cell coupling provenance gap verified: current axis boxes and lineage IDs do not own source measure/correlation; the later additive prerequisite is accepted only through its recorded owner-gated result.'
