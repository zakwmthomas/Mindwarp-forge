Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$docPath = Join-Path $root 'docs\canonical-system\G1_C3_POST_CALIBRATED_SOURCE_ENERGY_DISTRIBUTION_CONSUMER_REASSESSMENT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_SOURCE_ENERGY_DISTRIBUTION_IMPLEMENTATION_RESULT.md'
foreach($path in @($docPath,$resultPath)) { if(!(Test-Path -LiteralPath $path)){throw "Post-distribution artifact missing: $path"} }
$doc = Get-Content -LiteralPath $docPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
foreach($required in @('zero downstream consumers','Import into whole-cell dimensionless transfer: reject','Import into source distribution: reject','Direct received-energy sibling: mathematically plausible but premature','declared per coordinate unit','metre mapping','Detector or visibility consumer: reject','zero source, zero coupling, opaque transfer','calibrated transport-applicability witness schema gap audit','pointwise, cell-wide or conservatively unresolved','adds no crate, contract schema, dependency, production test','Nothing broader is locked in')) {
  if($doc -notlike "*$required*"){throw "Post-distribution reassessment drift: $required"}
}
foreach($required in @('owner-approved bounded V1','300.5 seconds','2,385','828 durable','52 declared modules','Transport and perception remain blocked','Rollback is deletion-only')) {
  if($result -notlike "*$required*"){throw "Distribution result receipt drift: $required"}
}
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw
$manifests = Get-ChildItem -LiteralPath (Join-Path $root 'crates') -Filter Cargo.toml -Recurse | Where-Object { $_.Directory.Name -ne 'calibrated-source-energy-distribution' }
foreach($manifest in $manifests) {
  $production = ((Get-Content -LiteralPath $manifest.FullName -Raw) -split '(?m)^\[dev-dependencies\]\s*$')[0]
  if($production -match '(?m)^calibrated-source-energy-distribution\s*='){throw "Unauthorized downstream consumer appeared: $($manifest.FullName)"}
}
if($cargo -notlike '*"crates/calibrated-source-energy-distribution"*'){throw 'Verified source-distribution owner is absent'}
$c3 = @($program.items | Where-Object id -eq 'C3')
$implementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*' -and $c3[0].proof -like '*owner explicitly approved*zero downstream consumers*'
$selectedRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*calibrated transport-applicability witness schema gap audit*' -and $c3[0].proof -like '*source-energy distribution*fully verified*declared per coordinate unit*metre mapping*zero downstream consumers*'
$completedGapRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*' -and $c3[0].proof -like '*scale ambiguity*spectral/time ambiguity*subject mismatch*separate capability-free applicability sibling*'
if(!$implementationRoute -and !$selectedRoute -and !$completedGapRoute){throw 'C3 post-distribution route drift'}
$resultCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-result' -and $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*transport applicability*visibility*promotion*C3 closure*'
$reassessmentCheckpoint = $checkpoint.batch_id -eq 'G1-C3-POST-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-CONSUMER-REASSESSMENT-V1' -and $checkpoint.substage_id -eq 'post-calibrated-source-energy-distribution-consumer-reassessment' -and $checkpoint.authority_lane -like '*Code-free post-result reassessment only*No crate*schema*dependency*production test*production source*downstream consumer*transport applicability*visibility*promotion*C3 closure*'
$gapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$designCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(!$resultCheckpoint -and !$reassessmentCheckpoint -and !$gapCheckpoint -and !$designCheckpoint -and !$c3InterruptionRoute){throw 'Post-distribution checkpoint authority drift'}
Write-Output 'Post calibrated source-energy distribution reassessment verified: direct energy composition remains premature; a code-facing transport-applicability witness gap audit is selected with zero consumers.'
