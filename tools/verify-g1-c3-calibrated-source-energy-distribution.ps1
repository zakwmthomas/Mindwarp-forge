Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$design = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_SOURCE_ENERGY_DISTRIBUTION_MATHEMATICAL_DESIGN_AUDIT.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_CALIBRATED_SOURCE_ENERGY_DISTRIBUTION_ORACLE_RESULT.md') -Raw
$oracle = Join-Path $root 'tools\prove-g1-c3-calibrated-source-energy-distribution.py'
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$c3InterruptionRoute = & (Join-Path $PSScriptRoot 'test-c3-federated-interruption.ps1') -Checkpoint $checkpoint

foreach($required in @('Independent leaf-energy records','Root total without a distribution','Density with respect to abstract cell measure','Closed-frontier additive radiant-energy measure','prefix-free','unresolved_within_cell','Source provenance cannot alias calibration provenance','4, 16 and 64 leaves','zero source with positive transfer','Add no crate','Nothing broader is locked in')) {
  if($design -notlike "*$required*"){throw "Calibrated source-energy design missing: $required"}
}
foreach($required in @('closed-frontier additive calibrated radiant-energy measure','byte-identical','e76be9bfcdf80543529baea94f70acf3455257e33c1e97871be4b2ecdc018553','33edaae6b5733b50f8c46592eee80664d5361a3613191844dcbd3ce58ed2edd6','Hostile rejections','Atomic split receipts','prefix_free_closed_frontier','derived_coordinate_local_average_only','403.1 seconds','2,277','816','51','ownership and implementation-readiness audit','Do not implement','Nothing broader is locked in')) {
  if($result -notlike "*$required*"){throw "Calibrated source-energy result missing: $required"}
}

$hash = (Get-FileHash -LiteralPath $oracle -Algorithm SHA256).Hash.ToLowerInvariant()
if($hash -ne 'e76be9bfcdf80543529baea94f70acf3455257e33c1e97871be4b2ecdc018553'){throw "Calibrated source-energy oracle hash drift: $hash"}
$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if(!(Test-Path $python)){$python='python'}
$a = & $python $oracle
if($LASTEXITCODE -ne 0){throw 'Calibrated source-energy oracle first run failed'}
$b = & $python $oracle
if($LASTEXITCODE -ne 0){throw 'Calibrated source-energy oracle second run failed'}
if($a -cne $b){throw 'Calibrated source-energy oracle output is nondeterministic'}
$receipt = $a | ConvertFrom-Json
if($receipt.status -ne 'pass' -or
   $receipt.candidate -ne 'closed_frontier_additive_calibrated_radiant_energy_measure' -or
   $receipt.checksum -ne '33edaae6b5733b50f8c46592eee80664d5361a3613191844dcbd3ce58ed2edd6' -or
   $receipt.hostile_rejections -ne 32 -or
   $receipt.split_receipts -ne 63 -or
   (@($receipt.conservation_leaf_counts)-join ',') -ne '4,16,64' -or
   $receipt.cell_measure_density -ne 'derived_coordinate_local_average_only' -or
   $receipt.unresolved_allocation -ne 'retained_at_coarser_frontier_cell') {
  throw 'Calibrated source-energy pinned receipt drift'
}

$c3 = @($program.items | Where-Object id -eq 'C3')
$oracleRoute = $c3.Count -eq 1 -and
   $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and
   $c3[0].proof -like '*closed-frontier additive calibrated radiant-energy measure*4, 16 and 64*63 atomic split*Thirty-two hostile*'
$ownerRoute = $c3.Count -eq 1 -and
   $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -and
   $c3[0].proof -like '*compact axis-bearing*63*64*zero downstream consumers*'
$implementationRoute = $c3.Count -eq 1 -and
   $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*test-first*' -and
   $c3[0].proof -like '*owner explicitly approved*zero downstream consumers*'
$transportGapRoute = $c3.Count -eq 1 -and $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*' -and $c3[0].proof -like '*scale ambiguity*spectral/time ambiguity*separate capability-free applicability sibling*'
if(!$oracleRoute -and !$ownerRoute -and !$implementationRoute -and !$transportGapRoute) {
  throw 'C3 calibrated source-energy route drift'
}
$oracleGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and
   $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-oracle-result' -and
   $checkpoint.next_action -like '*ownership and implementation-readiness audit*' -and
   $checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*contract schema*consumer*production source*transport applicability*detector*visibility*promotion*C3 closure*'
$ownerGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and
   $checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and
   $checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$implementationGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and
   $checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
   $checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$transportGapGate = $checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and $checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and $checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$transportDesignGate = $checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1','G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and $checkpoint.substage_id -in @('calibrated-transport-applicability-witness-mathematical-design-result','calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and $checkpoint.authority_lane -like '*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
if(!$oracleGate -and !$ownerGate -and !$implementationGate -and !$transportGapGate -and !$transportDesignGate -and !$c3InterruptionRoute) {
  throw 'Calibrated source-energy checkpoint authority drift'
}

$authorityStatus = if($implementationGate){'the bounded owner implementation is approved and downstream consumption remains unauthorized'}else{'implementation remains unauthorized'}
Write-Output "Calibrated source-energy distribution verified: the closed exact frontier survives; density stays derived-only and $authorityStatus. receipt=$a"
