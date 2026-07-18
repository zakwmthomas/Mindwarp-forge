Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$doc = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_SOURCE_CALIBRATION_IMPLEMENTATION_READINESS.md') -Raw
$oraclePath = Join-Path $root 'tools\prove-g1-c3-source-calibration-readiness.py'
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw
$boundaries = Get-Content -LiteralPath (Join-Path $root 'governance\module-boundaries.json') -Raw

foreach($required in @('ready for one explicit owner decision only','calibrated-spectral-time-basis','Frozen input record','deny_unknown_fields','ExactUnsignedRationalV1','Frozen result record','Non-circular identity graph','mindwarp.calibrated-spectral-time-basis.basis.v1','mindwarp.calibrated-spectral-time-basis.legacy-time-commitment.v1','mindwarp.optical-phase-space.band-time.v1','Existing arbitrary V1 time IDs','Frozen resource ceilings','16 KiB','32 KiB','64 KiB','30 / 30','zero consumers','deletion-only','Transport applicability remains blocked','Exact owner decision','General continuation is not enough')) {
  if($doc -notlike "*$required*"){throw "Source-calibration readiness missing: $required"}
}
$oracleSha = (Get-FileHash -LiteralPath $oraclePath -Algorithm SHA256).Hash.ToLowerInvariant()
if($oracleSha -ne 'd8f1fd99fffea927c62642ecde46f4380ebb949ce279d0d940758a5ca31e5d22'){throw "Source-calibration oracle source drift: $oracleSha"}
$bundledPython = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if(Test-Path -LiteralPath $bundledPython){$bundledPython}else{'python'}
$first = (& $python $oraclePath | Out-String).Trim()
if($LASTEXITCODE -ne 0){throw 'Source-calibration readiness oracle failed'}
$second = (& $python $oraclePath | Out-String).Trim()
if($LASTEXITCODE -ne 0 -or $first -ne $second){throw 'Source-calibration readiness oracle is nondeterministic'}
$receipt = $first | ConvertFrom-Json
if($receipt.status -ne 'pass_ready_for_owner_decision_only' -or
   $receipt.checksum -ne '111245fd46c4b5639f5e63d1b3c6ea187c8dbed01bf786bb241157a64d576c3c' -or
   $receipt.hostile_rejections -ne 30 -or $receipt.distinct_identity_substitutions -ne 6 -or
   $receipt.observed_input_bytes -ne 896 -or $receipt.observed_result_bytes -ne 1786 -or
   $receipt.input_cap_bytes -ne 16384 -or $receipt.result_cap_bytes -ne 32768 -or
   $receipt.aggregate_cap_bytes -ne 65536 -or $receipt.production_artifacts -ne 'none' -or
   $receipt.transport_applicability -ne 'blocked_separately') {throw 'Source-calibration readiness receipt drift'}

$ownerGate = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1'
if($ownerGate -and ($cargo -like '*calibrated-spectral-time-basis*' -or $boundaries -like '*calibrated-spectral-time-basis*' -or
   (Test-Path -LiteralPath (Join-Path $root 'crates\calibrated-spectral-time-basis')) -or
   (Test-Path -LiteralPath (Join-Path $root 'contracts\calibrated-spectral-time-basis-contract.md')))) {
  throw 'Source-calibration production owner appeared before explicit approval'
}
$c3 = @($program.items | Where-Object id -eq 'C3')
$readinessRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*implementation-readiness audit*source-calibration sibling*' -and
   $c3[0].proof -like '*source-calibration implementation-readiness*stateless*zero consumers*Transport applicability remains blocked*'
$implementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*implementation-readiness audit*source-calibration sibling*verified*' -and
   $c3[0].proof -like '*owner-approved calibrated-spectral-time-basis*fully verified*stateless*zero consumers*Transport applicability remains blocked*'
$distributionResultRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and
   $c3[0].proof -like '*closed-frontier additive calibrated radiant-energy measure*zero-consumer calibration owner*Transport applicability remains blocked*'
$distributionReadinessRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -and
   $c3[0].proof -like '*compact axis-bearing*zero downstream consumers*'
$distributionImplementationRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*' -and
   $c3[0].proof -like '*owner explicitly approved*zero downstream consumers*'
$transportGapRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*' -and $c3[0].proof -like '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*'
if(!$readinessRoute -and !$implementationRoute -and !$distributionResultRoute -and !$distributionReadinessRoute -and !$distributionImplementationRoute -and !$transportGapRoute) {
  throw 'C3 source-calibration owner-gate route drift'
}
$implementationGate = $checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and
   $checkpoint.substage_id -in @('source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result') -and
   $checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$readinessGate = $ownerGate -and $checkpoint.substage_id -eq 'source-calibration-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*dependency*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$distributionResultGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and
   $checkpoint.authority_lane -like '*verified zero-consumer source-calibration owner remains frozen*No crate*consumer*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionReadinessGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$distributionImplementationGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
   $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignGate = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(!$readinessGate -and !$implementationGate -and !$distributionResultGate -and !$distributionReadinessGate -and !$distributionImplementationGate -and !$transportGapGate -and !$transportDesignGate -and !$c3InterruptionRoute) {
  throw 'Source-calibration owner-gate checkpoint drift'
}
Write-Output $(if($implementationGate){'Source-calibration readiness verified and explicitly released into the bounded zero-consumer implementation.'}else{'Source-calibration readiness verified: exact stateless schema, limits, hostiles, portability and deletion-only rollback are frozen; no production owner exists.'})
