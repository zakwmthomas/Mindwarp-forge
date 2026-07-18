Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$doc = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_POST_SOURCE_CALIBRATION_CONSUMER_REASSESSMENT.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_SOURCE_CALIBRATION_IMPLEMENTATION_RESULT.md') -Raw
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
foreach($required in @('physical spectral/time identity owner is verified','Actual consumer comparison','Reject as first consumer','Transport applicability is still blocked','Preserve channel-neutral geometry','Separate calibrated source-energy distribution','Select for code-free mathematical design and oracle only','Smallest next proof','4-, 16- and 64-way refinement','first and only consumer','Nothing broader is locked in')) {
  if($doc -notlike "*$required*"){throw "Post-source-calibration reassessment drift: $required"}
}
if($doc -notlike '*zero*consumers*') { throw 'Post-source-calibration reassessment drift: zero consumers' }
foreach($required in @('392.9 seconds','2,276','812 durable files','51 modules verified')) {
  if($doc -notlike "*$required*"){throw "Post-source-calibration verification receipt drift: $required"}
}
foreach($required in @('implemented and fully verified','417.9 seconds','2,283','810','51')) {
  if($result -notlike "*$required*"){throw "Source-calibration result receipt drift: $required"}
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$sourceCalibrationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*implementation-readiness audit*source-calibration sibling*verified implementation*code-free calibrated source-energy distribution*' -and
   $c3[0].proof -like '*calibrated-spectral-time-basis*fully verified*zero consumers*Transport applicability remains blocked*'
$distributionResultRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and
   $c3[0].proof -like '*closed-frontier additive calibrated radiant-energy measure*zero-consumer calibration owner*Transport applicability remains blocked*'
$distributionReadinessRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -and
   $c3[0].proof -like '*compact axis-bearing*zero downstream consumers*'
$distributionImplementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*' -and
   $c3[0].proof -like '*owner explicitly approved*zero downstream consumers*'
$transportGapRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*' -and $c3[0].proof -like '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*'
if(-not ($sourceCalibrationRoute -or $distributionResultRoute -or $distributionReadinessRoute -or $distributionImplementationRoute -or $transportGapRoute)) {
  throw 'C3 post-source-calibration route drift'
}
$sourceCalibrationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -eq 'source-calibration-result' -and
   $checkpoint.next_action -like '*code-free calibrated source-energy distribution*' -and $checkpoint.authority_lane -like '*Verified zero-consumer source-calibration result and code-free reassessment only*No consumer*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionResultCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and
   $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionReadinessCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
   $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(-not ($sourceCalibrationCheckpoint -or $distributionResultCheckpoint -or $distributionReadinessCheckpoint -or $distributionImplementationCheckpoint -or $transportGapCheckpoint -or $transportDesignCheckpoint -or $c3InterruptionRoute)) {
  throw 'Post-source-calibration checkpoint drift'
}
Write-Output 'Post-source-calibration reassessment verified: no current owner is a safe first consumer; calibrated source-energy distribution advances to code-free design only.'
