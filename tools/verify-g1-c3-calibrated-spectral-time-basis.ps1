Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$design = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_SPECTRAL_TIME_BASIS_MATHEMATICAL_DESIGN_AUDIT.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_SPECTRAL_TIME_BASIS_ORACLE_RESULT.md') -Raw
$oracle = Join-Path $root 'tools\prove-g1-c3-calibrated-spectral-time-basis.py'
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach($required in @('unchanged V1 band/time identity','vacuum_wavelength_metre','Overlapping response-weighted RGB channels','Band-centre or average-transfer calibration','Versioned disjoint spectral/time calibration witness','Alias and version rule','whole-cell pointwise enclosure','spatial calibration','Add no crate')) {
  if($design -notlike "*$required*"){throw "Calibrated basis design missing: $required"}
}
foreach($required in @('additive calibration witness survives','byte-identical','b8e94899d4b49f93416d4ffd054b7b028e3bdc5fd0c4b235d39db41592c6b8d5','0244df63a2ca1e2a58b3c035104c1d2997fcfe4a693fed872ae1262eb3b64c78','Hostile rejections:','pointwise','spatial calibration','code-facing calibrated-basis and transport-','Do not implement','Nothing broader is locked in')) {
  if($result -notlike "*$required*"){throw "Calibrated basis result missing: $required"}
}

$hash = (Get-FileHash -LiteralPath $oracle -Algorithm SHA256).Hash.ToLowerInvariant()
if($hash -ne 'b8e94899d4b49f93416d4ffd054b7b028e3bdc5fd0c4b235d39db41592c6b8d5'){throw "Calibrated basis oracle hash drift: $hash"}
$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if(!(Test-Path $python)){$python='python'}
$a = & $python $oracle
if($LASTEXITCODE -ne 0){throw 'Calibrated basis oracle first run failed'}
$b = & $python $oracle
if($LASTEXITCODE -ne 0){throw 'Calibrated basis oracle second run failed'}
if($a -cne $b){throw 'Calibrated basis oracle output is nondeterministic'}
$receipt = $a | ConvertFrom-Json
if($receipt.status -ne 'pass' -or
   $receipt.checksum -ne '0244df63a2ca1e2a58b3c035104c1d2997fcfe4a693fed872ae1262eb3b64c78' -or
   $receipt.hostile_rejections -ne 34 -or
   $receipt.additive_spectral_bands -ne 3 -or
   $receipt.allocation_portfolios -ne 5 -or
   $receipt.pointwise_transport_applicability -ne 'required') {
  throw 'Calibrated basis pinned receipt drift'
}

$c3 = @($program.items | Where-Object id -eq 'C3')
if($c3.Count -ne 1 -or
   ($c3[0].next_action -notlike '*calibrated-basis*transport-applicability schema gap audit*' -and $c3[0].next_action -notlike '*implementation-readiness audit*source-calibration sibling*' -and $c3[0].next_action -notlike '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and $c3[0].next_action -notlike '*explicit owner decision*calibrated-source-energy-distribution*' -and $c3[0].next_action -notlike '*owner-approved bounded calibrated-source-energy-distribution*' -and $c3[0].next_action -notlike '*explicit owner decision*evidence-acquisition*mathematical-design audit*') -or
   ($c3[0].proof -notlike '*separate versioned witness*pointwise*spatial*' -and $c3[0].proof -notlike '*stateless derived commitment*Transport applicability remains blocked*' -and $c3[0].proof -notlike '*closed-frontier additive calibrated radiant-energy measure*Transport applicability remains blocked*' -and $c3[0].proof -notlike '*compact axis-bearing*zero downstream consumers*' -and $c3[0].proof -notlike '*owner explicitly approved*zero downstream consumers*' -and $c3[0].proof -notlike '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*')) {
  throw 'C3 calibrated-basis route drift'
}
$calibratedDesignCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SPECTRAL-TIME-BASIS-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -in @('calibrated-spectral-time-basis-design-and-oracle','calibrated-spectral-time-basis-oracle-result') -and $checkpoint.authority_lane -like '*Code-free mathematical design and deterministic exact-rational oracle only*No crate*contract schema*production source*normative wavelength*tick duration*detector*visibility*promotion*C3 closure*'
$gapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -in @('calibrated-basis-transport-applicability-schema-gap-audit','calibrated-basis-transport-applicability-schema-gap-result') -and $checkpoint.authority_lane -like '*Read-only code-facing gap audit only*No crate*contract schema*production source*registry*normative RGB*spatial scale*visibility*promotion*C3 closure*'
$readinessCheckpoint = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1' -and $checkpoint.substage_id -eq 'source-calibration-owner-gate' -and $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$implementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result') -and $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$distributionResultCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionReadinessCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionImplementationCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapCheckpoint = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignCheckpoint = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(-not ($calibratedDesignCheckpoint -or $gapCheckpoint -or $readinessCheckpoint -or $implementationCheckpoint -or $distributionResultCheckpoint -or $distributionReadinessCheckpoint -or $distributionImplementationCheckpoint -or $transportGapCheckpoint -or $transportDesignCheckpoint -or $c3InterruptionRoute)) {
  throw 'Calibrated basis checkpoint authority drift'
}
Write-Output "Calibrated spectral/time basis verified: additive witness survives; physical composition remains blocked on pointwise transport applicability and spatial calibration. receipt=$a"
