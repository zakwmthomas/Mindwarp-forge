param(
  [Parameter(Mandatory=$true)]$Checkpoint,
  $C3
)

if ($null -eq $C3) {
  $root = Split-Path -Parent $PSScriptRoot
  $program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
  $c3Items = @($program.items | Where-Object id -eq 'C3')
  if ($c3Items.Count -ne 1) { throw 'Canonical C3 program item is missing or ambiguous' }
  $C3 = $c3Items[0]
}

$implementationRoute =
  $Checkpoint.batch_id -eq 'G1-FEDERATED-PROJECT-ROUTING-AND-STORAGE-V1' -and
  $Checkpoint.substage_id -like 'federated-routing-storage-v1-*' -and
  $Checkpoint.authority_lane -like '*may not delete exact evidence*execute cleanup*grant game/runtime authority*'
$reconciliationRoute =
  $Checkpoint.batch_id -in @('G1-FEDERATED-LIVE-STATE-RECONCILIATION-AUDIT-V1','G1-FEDERATED-LIVE-STATE-DECISION-CHECKPOINT-V1') -and
  $Checkpoint.substage_id -in @('federated-live-state-read-only-reconciliation-audit','federated-live-state-owner-decision-checkpoint') -and
  $Checkpoint.authority_lane -like '*may not write the live database*delete evidence*execute cleanup*grant game/runtime authority*'
$acceptanceRoute =
  $Checkpoint.batch_id -eq 'G1-FEDERATED-V4-ACCEPTANCE-V1' -and
  $Checkpoint.substage_id -in @('federated-live-v4-acceptance-verification','federated-live-v4-accepted-result') -and
  $Checkpoint.authority_lane -like '*V4 selection*may not rewrite the live database*delete evidence*execute cleanup*grant game/runtime authority*'
$forgeMetricsRoute =
  $Checkpoint.batch_id -eq 'G1-FORGE-METRICS-DASHBOARD-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('forge-metrics-telemetry-and-projection-implementation','forge-metrics-dashboard-verification','forge-metrics-dashboard-recorded') -and
  $Checkpoint.authority_lane -like '*Owner-approved local Forge measurement*read-only dashboard*may not approve*execute*tune*promote*runtime*'
$controlPlaneConsolidationRoute =
  $Checkpoint.batch_id -eq 'G1-FORGE-CONSOLIDATION-CONTROL-PLANE-V1' -and
  $Checkpoint.master_program_item -eq 'B4' -and
  $Checkpoint.substage_id -in @('forge-consolidation-control-plane-implementation','forge-consolidation-control-plane-verification','forge-consolidation-control-plane-recorded') -and
  $Checkpoint.authority_lane -like '*Preserve accumulated Forge evidence*control-plane safety*No game feature*runtime*promotion*C3 closure*'
$gp0GameplayRoute =
  $Checkpoint.batch_id -eq 'G1-GP0-PLAYER-FANTASY-BOUNDARY-V1' -and
  $Checkpoint.master_program_item -eq 'GP0' -and
  (($Checkpoint.substage_id -eq 'gp0-product-player-fantasy-boundary' -and
   $Checkpoint.authority_lane -like '*Clean-room gameplay design*No historical source code*runtime*engine*network*monetization*') -or
  ($Checkpoint.substage_id -eq 'gp0-gameplay-foundation-verification' -and
   $Checkpoint.authority_lane -like '*Owner-authorized clean-room GP0 typed contracts*No GP1*runtime*engine*database*network*monetization*Greenfield dependency*C3B substitution*'))
$gp1GameplayRoute =
  $Checkpoint.batch_id -eq 'G1-GP1-FIXED-BASE-LOOP-V1' -and
  $Checkpoint.master_program_item -eq 'GP1' -and
  $Checkpoint.substage_id -in @('gp1-fixed-base-loop-readiness','gp1-fixed-base-loop-implementation','gp1-fixed-base-loop-verification','gp1-fixed-base-loop-verified-result') -and
  $Checkpoint.authority_lane -like '*Owner-authorized GP1 engine-neutral deterministic base loop*No GP2*graphics*runtime*procedural breadth*grind*network*Greenfield*C3B*'
$gp2GameplayRoute =
  $Checkpoint.batch_id -in @('G1-GP2-PROGRESSION-DESIGN-V1','G1-GP2-PROGRESSION-IMPLEMENTATION-V1') -and
  $Checkpoint.master_program_item -eq 'GP2' -and
  $Checkpoint.substage_id -in @('gp2-progression-design-intake','gp2-progression-readiness','gp2-progression-verification','gp2-progression-verified-result') -and
  (($Checkpoint.authority_lane -like '*Owner-authorized GP2 design intake*No source implementation*universal currency*positive cycle*dominant conversion*runtime*Greenfield*C3B*GP3*GP4*') -or
   ($Checkpoint.authority_lane -like '*Owner-authorized bounded GP2 implementation*No universal currency*positive cycle*dominant conversion*runtime*Greenfield*C3B*GP3*GP4*'))
$c4vGameplayRoute =
  $Checkpoint.batch_id -eq 'G1-C4V-VERTICAL-PERSISTENCE-V1' -and
  $Checkpoint.master_program_item -eq 'C4V' -and
  $Checkpoint.substage_id -in @('c4v-vertical-persistence-readiness','c4v-implementation-verification','c4v-verified-result') -and
  $Checkpoint.authority_lane -like '*Owner-authorized isolated C4V engine-neutral persistence proof only*No broad C4*production filesystem*runtime*GP2*GP3*C3B*Greenfield*Kernel mutation*'
$gp3GameplayRoute =
  $Checkpoint.batch_id -eq 'G1-GP3-ENCOUNTER-GRAMMAR-V1' -and
  $Checkpoint.master_program_item -eq 'GP3' -and
  $Checkpoint.substage_id -in @('gp3-encounter-grammar-readiness','gp3-encounter-grammar-implementation','gp3-encounter-grammar-verification','gp3-encounter-grammar-recorded') -and
  $Checkpoint.authority_lane -like '*Owner-authorized bounded GP3 authored encounter grammar only*Exactly five fixed situations*No procedural generation*weights*random selection*runtime*GP4*C3B*Greenfield*persistence expansion*combat resolver*economy mapping*automatic GP2 progression*Kernel mutation*'
$gp4GameplayRoute =
  $Checkpoint.batch_id -eq 'G1-GP4-SIGNAL-ANCHOR-VERTICAL-V1' -and
  $Checkpoint.master_program_item -eq 'GP4' -and
  $Checkpoint.substage_id -in @('gp4-signal-anchor-readiness','gp4-signal-anchor-implementation','gp4-signal-anchor-verification','gp4-signal-anchor-recorded') -and
  $Checkpoint.authority_lane -like '*Owner-authorized bounded GP4 Signal Anchor temporary-rescue vertical only*No engine selection*runtime implementation*promotion*broad C4*C3B*Companion*Greenfield*procedural generation*persistence expansion*filesystem*network*Kernel mutation*'
$g1VerticalCloseoutRoute =
  $Checkpoint.batch_id -eq 'G1-VERTICAL-CLOSEOUT-V1' -and
  $Checkpoint.master_program_item -eq 'G1-VERTICAL-CLOSEOUT' -and
  $Checkpoint.substage_id -eq 'g1-vertical-closeout-recorded' -and
  $Checkpoint.authority_lane -like '*Owner-authorized bounded G1 vertical closeout evidence receipt only*broad_g1=false*runtime_selected=false*runtime_containment_pending=true*evidence_only=true*promotion_authority=false*No runtime*broad C4*C3B*Companion*Greenfield*procedural generation*persistence expansion*filesystem*network*process*Kernel mutation*'
$c4BroadRoute =
  $Checkpoint.batch_id -eq 'G1-C4-HIERARCHY-HISTORY-CLOSURE-V1' -and
  $Checkpoint.master_program_item -eq 'C4' -and
  $Checkpoint.substage_id -in @('c4-reconciliation-readiness','c4-hierarchy-history-hardening','c4-verification','c4-verified-result') -and
  $Checkpoint.authority_lane -like '*Owner-authorized broad C4 hierarchy/history reconciliation and capability-free closure proof only*Exact dependencies C2 and C3A*No C3B*C5*C6*C7*broad G1 closure*runtime*storage engine*filesystem*network*multiplayer*cross-target transactions*Companion*Greenfield*visual assets*Kernel mutation*'
$gameplayFoundationRoute = $gp0GameplayRoute -or $gp1GameplayRoute -or $gp2GameplayRoute -or $c4vGameplayRoute -or $gp3GameplayRoute -or $gp4GameplayRoute -or $g1VerticalCloseoutRoute -or $c4BroadRoute
$c3DesignRoute =
  $Checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-RECEIVER-COUPLING-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('whole-cell-receiver-coupling-mathematical-design','whole-cell-receiver-coupling-oracle-verification') -and
  $Checkpoint.authority_lane -like '*Mathematical design*disposable exact-rational oracle*Do not add or modify a crate*production source*runtime*promotion*C3 closure remain excluded*'
$c3ReceiverOwnerGate =
  $Checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-RECEIVER-COUPLING-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -eq 'whole-cell-receiver-coupling-owner-gate' -and
  $Checkpoint.authority_lane -like '*Serious owner gate*optical-phase-space-receiver-coupling*Do not add a crate*production source*runtime*promotion*C3 closure*'
$c3ReceiverImplementation =
  $Checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-RECEIVER-COUPLING-IMPLEMENTATION-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('whole-cell-receiver-coupling-test-first-implementation','whole-cell-receiver-coupling-verification','whole-cell-receiver-coupling-result') -and
  $Checkpoint.authority_lane -like '*Owner-approved exact additive package*optical-phase-space-receiver-coupling*Existing owner source*runtime*promotion*C3 closure*'
$c3PostReceiverReassessment =
  $Checkpoint.batch_id -eq 'G1-C3-POST-WHOLE-CELL-RECEIVER-COUPLING-CONSUMER-REASSESSMENT-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -eq 'post-whole-cell-receiver-coupling-consumer-reassessment' -and
  $Checkpoint.authority_lane -like '*Code-free reassessment*whole-cell dimensionless transfer*No crate*source magnitude*detector response*visibility*C3 closure*'
$c3WholeCellTransferDesign =
  $Checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('whole-cell-dimensionless-transfer-mathematical-design','whole-cell-dimensionless-transfer-oracle-verification','whole-cell-dimensionless-transfer-oracle-result') -and
  $Checkpoint.authority_lane -like '*Mathematical design and disposable exact-rational oracle only*No crate*source magnitude*detector response*visibility*C3 closure*'
$c3WholeCellTransferReadiness =
  $Checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-IMPLEMENTATION-READINESS-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('whole-cell-dimensionless-transfer-implementation-readiness','whole-cell-dimensionless-transfer-owner-gate') -and
  $Checkpoint.authority_lane -like '*Readiness audit and exact owner gate only*No crate*production source*source magnitude*detector response*visibility*C3 closure*'
$c3WholeCellTransferImplementation =
  $Checkpoint.batch_id -eq 'G1-C3-WHOLE-CELL-DIMENSIONLESS-TRANSFER-IMPLEMENTATION-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('bulk-optical-depth-evaluation-implementation','whole-cell-dimensionless-transfer-sibling-implementation','whole-cell-dimensionless-transfer-verification','whole-cell-dimensionless-transfer-result') -and
  $Checkpoint.authority_lane -like '*Owner-approved additive bulk evaluation*downstream dimensionless-transfer sibling only*No existing V1 migration*source magnitude*detector response*visibility*promotion*C3 closure*'
$c3PostWholeCellTransferReassessment =
  $Checkpoint.batch_id -eq 'G1-C3-POST-WHOLE-CELL-DIMENSIONLESS-TRANSFER-CONSUMER-REASSESSMENT-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -eq 'post-whole-cell-dimensionless-transfer-consumer-reassessment' -and
  $Checkpoint.authority_lane -like '*Static reassessment*No crate*source distribution*physical quantity*detector*visibility*promotion*C3 closure*'
$c3SourceDistributionMeasure =
  $Checkpoint.batch_id -eq 'G1-C3-SOURCE-DISTRIBUTION-MEASURE-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('source-distribution-measure-design-and-oracle','source-distribution-measure-oracle-result') -and
  $Checkpoint.authority_lane -like '*Mathematical design*exact-rational oracle*No crate*production source*watts*radiance*detector*visibility*promotion*C3 closure*'
$c3SourceQuantityGap =
  $Checkpoint.batch_id -eq 'G1-C3-SOURCE-QUANTITY-BASIS-SCHEMA-GAP-AUDIT-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('source-quantity-basis-schema-gap-audit','source-quantity-basis-schema-gap-result') -and
  $Checkpoint.authority_lane -like '*read-only gap audit only*No crate*contract schema*production source*unit selection*watts*joules*radiance*detector*visibility*promotion*C3 closure*'
$c3SourceQuantityBasis =
  $Checkpoint.batch_id -eq 'G1-C3-SOURCE-QUANTITY-BASIS-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('source-quantity-basis-design-and-oracle','source-quantity-basis-oracle-result') -and
  $Checkpoint.authority_lane -like '*mathematical design*exact-rational oracle only*No crate*contract schema*production source*RGB boundaries*tick duration*detector*visibility*promotion*C3 closure*'
$c3CalibratedSpectralTimeBasis =
  $Checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SPECTRAL-TIME-BASIS-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('calibrated-spectral-time-basis-design-and-oracle','calibrated-spectral-time-basis-oracle-result') -and
  $Checkpoint.authority_lane -like '*Code-free mathematical design*exact-rational oracle only*No crate*contract schema*production source*normative wavelength*tick duration*spatial scale*visibility*promotion*C3 closure*'
$c3CalibratedBasisGap =
  $Checkpoint.batch_id -eq 'G1-C3-CALIBRATED-BASIS-TRANSPORT-APPLICABILITY-SCHEMA-GAP-AUDIT-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('calibrated-basis-transport-applicability-schema-gap-audit','calibrated-basis-transport-applicability-schema-gap-result') -and
  $Checkpoint.authority_lane -like '*Read-only code-facing gap audit only*No crate*contract schema*production source*registry*normative RGB*spatial scale*visibility*promotion*C3 closure*'
$c3SourceCalibrationOwnerGate =
  $Checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-READINESS-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -eq 'source-calibration-owner-gate' -and
  $Checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*production source*registry*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$c3SourceCalibrationImplementation =
  $Checkpoint.batch_id -eq 'G1-C3-SOURCE-CALIBRATION-IMPLEMENTATION-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('source-calibration-test-first-implementation','source-calibration-verification','source-calibration-result') -and
  $Checkpoint.authority_lane -like '*Owner-approved zero-consumer source-calibration implementation only*No existing owner import*consumer*normative calibration*transport applicability*visibility*promotion*C3 closure*'
$c3CalibratedSourceEnergyDistribution =
  $Checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('calibrated-source-energy-distribution-design-and-oracle','calibrated-source-energy-distribution-oracle-result') -and
  $Checkpoint.authority_lane -like '*Code-free calibrated source-energy distribution design and exact-rational oracle only*No crate*contract schema*consumer*production source*transport applicability*detector*visibility*promotion*C3 closure*'
$c3CalibratedSourceEnergyDistributionReadiness =
  $Checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-READINESS-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -eq 'calibrated-source-energy-distribution-owner-gate' -and
  $Checkpoint.authority_lane -like '*Serious owner gate*No crate*contract schema*dependency*consumer*production test*production source*transport applicability*visibility*promotion*C3 closure*'
$c3CalibratedSourceEnergyDistributionImplementation =
  $Checkpoint.batch_id -eq 'G1-C3-CALIBRATED-SOURCE-ENERGY-DISTRIBUTION-IMPLEMENTATION-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('calibrated-source-energy-distribution-test-first-implementation','calibrated-source-energy-distribution-verification','calibrated-source-energy-distribution-result') -and
  $Checkpoint.authority_lane -like '*Owner-approved bounded calibrated-source-energy-distribution implementation only*zero downstream consumers*Modify no existing owner behavior*transport applicability*visibility*promotion*C3 closure*'
$c3CalibratedTransportApplicabilityGap =
  $Checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-SCHEMA-GAP-AUDIT-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -eq 'calibrated-transport-applicability-witness-schema-gap-result' -and
  $Checkpoint.authority_lane -like '*Read-only code-facing schema gap audit only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$c3CalibratedTransportApplicabilityDesign =
  $Checkpoint.batch_id -eq 'G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-WITNESS-MATHEMATICAL-DESIGN-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('calibrated-transport-applicability-witness-evidence-and-mathematical-design','calibrated-transport-applicability-witness-mathematical-design-result') -and
  $Checkpoint.authority_lane -like '*Owner-authorized code-free primary research and mathematical design only*No crate*contract schema*dependency*production test*production source*downstream consumer*received energy*visibility*promotion*C3 closure*'
$c3CalibratedTransportPhysicalEvidenceProtocol =
  $Checkpoint.batch_id -in @('G1-C3-CALIBRATED-TRANSPORT-APPLICABILITY-PHYSICAL-EVIDENCE-ACQUISITION-PROTOCOL-V1','G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('calibrated-transport-applicability-physical-evidence-protocol','calibrated-transport-applicability-physical-evidence-protocol-result','post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $Checkpoint.authority_lane -like '*Owner-authorized code-free evidence inventory and acquisition-protocol design only*No crate*contract schema*dependency*production test*production source*downstream consumer*normative spatial scale*coefficient catalogue*received energy*visibility*promotion*C3 closure*'
$c3PostPhysicalEvidenceResidualObligation =
  $Checkpoint.batch_id -in @('G1-C3-POST-PHYSICAL-EVIDENCE-RESIDUAL-OBLIGATION-AUDIT-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('post-physical-evidence-residual-obligation-audit-result','c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $Checkpoint.authority_lane -like '*Owner-authorized code-free*C3 residual-obligation*No crate*contract schema*production source*physical calibration*promotion*C3 closure*'
$c3CrossBoundaryEcotoneDesign =
  $Checkpoint.batch_id -in @('G1-C3-CROSS-BOUNDARY-ECOTONE-MATHEMATICAL-DESIGN-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1','G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1') -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('c3-cross-boundary-ecotone-mathematical-design','c3-cross-boundary-ecotone-mathematical-design-result','c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $Checkpoint.authority_lane -like '*Owner-authorized code-free*C3 cross-boundary ecotone mathematical design*No crate*contract schema*dependency*production test*production source*downstream consumer*physical calibration*received energy*visibility*renderer*biome*organism*runtime*promotion*C3 closure*'
$c3CrossBoundaryEcotoneReadiness =
  $Checkpoint.batch_id -eq 'G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-IMPLEMENTATION-READINESS-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('c3-cross-boundary-ecotone-oracle-implementation-readiness','c3-cross-boundary-ecotone-oracle-owner-gate','c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $Checkpoint.authority_lane -like '*Serious owner gate*disposable*C3 ecotone oracle*No Python oracle*crate*contract schema*dependency*production test*production source*downstream consumer*physical calibration*received energy*visibility*renderer*biome*organism*runtime*promotion*C3 closure*'
$c3CrossBoundaryEcotoneOracle =
  $Checkpoint.batch_id -eq 'G1-C3-CROSS-BOUNDARY-ECOTONE-ORACLE-V1' -and
  $Checkpoint.master_program_item -eq 'C3' -and
  $Checkpoint.substage_id -in @('c3-cross-boundary-ecotone-oracle-implementation','c3-cross-boundary-ecotone-oracle-verification','c3-cross-boundary-ecotone-oracle-result') -and
  $Checkpoint.authority_lane -like '*Owner-approved disposable C3 ecotone oracle implementation only*No crate*contract schema*dependency*production test*production source*downstream consumer*physical calibration*received energy*visibility*renderer*biome*organism*runtime*promotion*C3 closure*'
$valid =
  ($C3.next_action -like '*Use C3A*Keep C3B*full C3 closure*' -or
   $C3.next_action -like '*whole-cell receiver-coupling mathematical design*receiver-before-face ordering*' -or
   $C3.next_action -like '*code-free mathematical design*whole-cell dimensionless transfer*' -or
   $C3.next_action -like '*code-facing readiness/gap audit*whole-cell dimensionless-transfer*' -or
   $C3.next_action -like '*owner-authorized*whole-cell dimensionless-transfer*' -or
   $C3.next_action -like '*code-free post-result consumer reassessment*whole-cell dimensionless-transfer*' -or
   $C3.next_action -like '*code-free source-distribution*phase-space-measure compatibility*' -or
   $C3.next_action -like '*code-facing source-quantity-basis*schema gap audit*' -or
   $C3.next_action -like '*source-quantity-basis mathematical design audit*' -or
   $C3.next_action -like '*calibrated spectral/time basis mathematical design audit*' -or
   $C3.next_action -like '*calibrated-basis*transport-applicability schema gap audit*' -or
   $C3.next_action -like '*implementation-readiness audit*source-calibration sibling*' -or
   $C3.next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -or
   $C3.next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -or
   $C3.next_action -like '*owner-approved bounded calibrated-source-energy-distribution*test-first*' -or
   $C3.next_action -like '*calibrated transport-applicability witness schema gap audit*' -or
   $C3.next_action -like '*owner-authorized code-free primary-evidence*mathematical-design audit*separate capability-free calibrated transport-applicability witness*' -or
   $C3.next_action -like '*code-free physical-evidence acquisition protocol*spatial calibration*coefficient evidence*stop before schema*' -or
   $C3.next_action -like '*material owner-direction gate*authoritative project-specific spatial and material evidence*leave physical applicability explicitly blocked*different dependency-ready C3 route*' -or
   $C3.next_action -like '*physical applicability*blocked*code-free*C3*ecotone*mathematical-design*' -or
   $C3.next_action -like '*physical applicability*blocked*ecotone oracle implementation verification*') -and
  (($Checkpoint.master_program_item -eq 'G1-FEDERATED-CONTINUITY' -and ($implementationRoute -or $reconciliationRoute -or $acceptanceRoute)) -or $forgeMetricsRoute -or $controlPlaneConsolidationRoute -or $gameplayFoundationRoute -or $c3DesignRoute -or $c3ReceiverOwnerGate -or $c3ReceiverImplementation -or $c3PostReceiverReassessment -or $c3WholeCellTransferDesign -or $c3WholeCellTransferReadiness -or $c3WholeCellTransferImplementation -or $c3PostWholeCellTransferReassessment -or $c3SourceDistributionMeasure -or $c3SourceQuantityGap -or $c3SourceQuantityBasis -or $c3CalibratedSpectralTimeBasis -or $c3CalibratedBasisGap -or $c3SourceCalibrationOwnerGate -or $c3SourceCalibrationImplementation -or $c3CalibratedSourceEnergyDistribution -or $c3CalibratedSourceEnergyDistributionReadiness -or $c3CalibratedSourceEnergyDistributionImplementation -or $c3CalibratedTransportApplicabilityGap -or $c3CalibratedTransportApplicabilityDesign -or $c3CalibratedTransportPhysicalEvidenceProtocol -or $c3PostPhysicalEvidenceResidualObligation -or $c3CrossBoundaryEcotoneDesign -or $c3CrossBoundaryEcotoneReadiness -or $c3CrossBoundaryEcotoneOracle)

Write-Output ([bool]$valid)
