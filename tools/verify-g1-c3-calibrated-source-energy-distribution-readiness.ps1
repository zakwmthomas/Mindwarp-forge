Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$doc = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_SOURCE_ENERGY_DISTRIBUTION_IMPLEMENTATION_READINESS.md') -Raw
$oraclePath = Join-Path $root 'tools\prove-g1-c3-calibrated-source-energy-distribution-readiness.py'
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint
$cargo = Get-Content -LiteralPath (Join-Path $root 'Cargo.toml') -Raw
$boundaries = Get-Content -LiteralPath (Join-Path $root 'governance\module-boundaries.json') -Raw

foreach($required in @(
  'ready for one explicit owner decision only',
  'calibrated-source-energy-distribution',
  'Corrected upstream replay boundary',
  'split_optical_phase_space_cell',
  'Frozen V1 query records',
  'SourceEnergyRefinementDirectiveV1',
  'Frozen derived records',
  'Non-circular identity graph',
  'Correction and supersession',
  'V1 has no',
  'Exact conservation and density boundary',
  '64',
  '63',
  '128 KiB',
  '256 KiB',
  '4 MiB',
  '385',
  '19 / 19',
  'Strict codecs and typed errors',
  'i686-pc-windows-msvc',
  'aarch64-linux-android',
  'zero downstream consumers',
  'deletion-only rollback',
  'Transport and perception remain blocked',
  'Exact owner decision',
  'General continuation is not enough'
)) {
  if($doc -notlike "*$required*") { throw "Calibrated source-energy readiness missing: $required" }
}

$oracleSha = (Get-FileHash -LiteralPath $oraclePath -Algorithm SHA256).Hash.ToLowerInvariant()
if($oracleSha -ne '2910e3c9836968b8fdc0accba271873a1c84efa7d1e60e5e6993373f061888a7') {
  throw "Calibrated source-energy readiness oracle source drift: $oracleSha"
}
$bundledPython = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if(Test-Path -LiteralPath $bundledPython){$bundledPython}else{'python'}
$first = (& $python $oraclePath | Out-String).Trim()
if($LASTEXITCODE -ne 0) { throw 'Calibrated source-energy readiness oracle failed' }
$second = (& $python $oraclePath | Out-String).Trim()
if($LASTEXITCODE -ne 0 -or $first -ne $second) { throw 'Calibrated source-energy readiness oracle is nondeterministic' }
$receipt = $first | ConvertFrom-Json
if($receipt.status -ne 'pass_ready_for_owner_decision_only' -or
   $receipt.checksum -ne 'e39db1445baf6a069760f1742d267427e38f885177fc6c50e83fb65e222c1d1c' -or
   $receipt.hostile_rejections -ne 19 -or
   $receipt.frontier_cap -ne 64 -or $receipt.split_directive_cap -ne 63 -or
   $receipt.energy_live_bit_cap -ne 385 -or
   $receipt.input_cap_bytes -ne 131072 -or $receipt.result_cap_bytes -ne 262144 -or
   $receipt.live_cap_bytes -ne 4194304 -or
   $receipt.maximum_observed_input_bytes -ne 18737 -or
   $receipt.maximum_observed_result_bytes -ne 66834 -or
   $receipt.maximum_observed_live_upper_bound -ne 2248259 -or
   $receipt.axis_bearing_replay -ne 'compact_parent_allocation_plus_upstream_axis' -or
   $receipt.production_artifacts -ne 'none') {
  throw 'Calibrated source-energy readiness receipt drift'
}

$ownerGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1'
if($ownerGate -and (
  $cargo -like '*calibrated-source-energy-distribution*' -or
  $boundaries -like '*calibrated-source-energy-distribution*' -or
  (Test-Path -LiteralPath (Join-Path $root 'crates\calibrated-source-energy-distribution')) -or
  (Test-Path -LiteralPath (Join-Path $root 'contracts\calibrated-source-energy-distribution-contract.md'))
)) {
  throw 'Calibrated source-energy production owner appeared before explicit approval'
}

$c3 = @($program.items | Where-Object id -eq 'C3')
$resultRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and
  $c3[0].proof -like '*closed-frontier additive calibrated radiant-energy measure*zero-consumer calibration owner*'
$ownerRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -and
  $c3[0].proof -like '*compact axis-bearing*63*64*zero downstream consumers*'
$implementationRoute = $c3.Count -eq 1 -and
  $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*test-first*' -and
  $c3[0].proof -like '*owner explicitly approved*zero downstream consumers*'
$transportGapRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*' -and $c3[0].proof -like '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*'
if(!$resultRoute -and !$ownerRoute -and !$implementationRoute -and !$transportGapRoute) { throw 'C3 calibrated source-energy readiness route drift' }

$resultGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and
  $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result'
$readinessGate = $ownerGate -and
  $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and
  $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$implementationGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and
  $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
  $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignGate = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(!$resultGate -and !$readinessGate -and !$implementationGate -and !$transportGapGate -and !$transportDesignGate -and !$c3InterruptionRoute) { throw 'Calibrated source-energy readiness checkpoint drift' }

$artifactStatus = if($implementationGate){'the approved bounded production owner is present with zero downstream consumers'}else{'zero production artifacts remain before the owner gate'}
Write-Output "Calibrated source-energy distribution readiness verified: compact axis-bearing replay, bounded resources and $artifactStatus."
