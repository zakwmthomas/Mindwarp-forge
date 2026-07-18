$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$contract = Join-Path $root 'contracts\derived-world-rules-contract.md'
$result = Join-Path $root 'docs\canonical-system\G1_C3_DERIVED_WORLD_CONTRACT_RESULT.md'
$source = Join-Path $root 'crates\derived-world-rules\src\lib.rs'
$stellarContract = Join-Path $root 'contracts\stellar-orbital-contract.md'
$stellarResult = Join-Path $root 'docs\canonical-system\G1_C3_STELLAR_ORBITAL_RESULT.md'
$stellarSource = Join-Path $root 'crates\stellar-orbital\src\lib.rs'
$geologicalContract = Join-Path $root 'contracts\geological-atmospheric-contract.md'
$geologicalResult = Join-Path $root 'docs\canonical-system\G1_C3_GEOLOGICAL_ATMOSPHERIC_RESULT.md'
$geologicalSource = Join-Path $root 'crates\geological-atmospheric\src\lib.rs'
$hydrologicalContract = Join-Path $root 'contracts\hydrological-state-contract.md'
$hydrologicalResult = Join-Path $root 'docs\canonical-system\G1_C3_HYDROLOGICAL_STATE_RESULT.md'
$hydrologicalSource = Join-Path $root 'crates\hydrological-state\src\lib.rs'
$climateContract = Join-Path $root 'contracts\climate-state-contract.md'
$climateResult = Join-Path $root 'docs\canonical-system\G1_C3_CLIMATE_STATE_RESULT.md'
$climateSource = Join-Path $root 'crates\climate-state\src\lib.rs'
$surfaceMaterialContract = Join-Path $root 'contracts\surface-material-state-contract.md'
$surfaceMaterialResult = Join-Path $root 'docs\canonical-system\G1_C3_SURFACE_MATERIAL_STATE_RESULT.md'
$surfaceMaterialSource = Join-Path $root 'crates\surface-material-state\src\lib.rs'
$regionalContract = Join-Path $root 'contracts\regional-environment-state-contract.md'
$regionalResult = Join-Path $root 'docs\canonical-system\G1_C3_REGIONAL_EXPOSURE_RESULT.md'
$regionalSource = Join-Path $root 'crates\regional-environment-state\src\lib.rs'
$signalResult = Join-Path $root 'docs\canonical-system\G1_C3_SIGNAL_POTENTIAL_RESULT.md'
$opportunityContract = Join-Path $root 'contracts\environmental-opportunity-contract.md'
$opportunityResult = Join-Path $root 'docs\canonical-system\G1_C3_ENVIRONMENTAL_OPPORTUNITY_RESULT.md'
$opportunitySource = Join-Path $root 'crates\niche-graph-binding\src\lib.rs'
$consumerAudit = Join-Path $root 'docs\canonical-system\G1_C3_VISIBILITY_TRAVERSABILITY_CONSUMER_AUDIT.md'
$fieldReceiptResult = Join-Path $root 'docs\canonical-system\G1_C3_SECOND_LANGUAGE_FIELD_RECEIPT_RESULT.md'
$spatialContract = Join-Path $root 'contracts\spatial-domain-contract.md'
$spatialResult = Join-Path $root 'docs\canonical-system\G1_C3_SPATIAL_DOMAIN_RESULT.md'
$spatialSource = Join-Path $root 'crates\spatial-domain\src\lib.rs'
$partitionContract = Join-Path $root 'contracts\physical-region-partition-contract.md'
$partitionResult = Join-Path $root 'docs\canonical-system\G1_C3_PHYSICAL_REGION_PARTITION_RESULT.md'
$partitionSource = Join-Path $root 'crates\physical-region-partition\src\lib.rs'
$pathContract = Join-Path $root 'contracts\physical-path-substrate-contract.md'
$pathResult = Join-Path $root 'docs\canonical-system\G1_C3_PHYSICAL_PATH_SUBSTRATE_RESULT.md'
$pathSource = Join-Path $root 'crates\physical-path-substrate\src\lib.rs'
$radianceMathDesign = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_PATH_TRANSFER_MATHEMATICAL_DESIGN_AUDIT.md'
$radianceMathResult = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_BULK_TRANSFER_ORACLE_RESULT.md'
$radianceMathProof = Join-Path $root 'tools\prove-g1-c3-visible-radiance-math.py'
$radianceBulkContract = Join-Path $root 'contracts\visible-radiance-bulk-transfer-contract.md'
$radianceBulkReadiness = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_BULK_TRANSFER_IMPLEMENTATION_READINESS.md'
$radianceBulkResult = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_BULK_TRANSFER_RESULT.md'
$radianceBulkSource = Join-Path $root 'crates\visible-radiance-bulk-transfer\src\lib.rs'
$radianceInterfaceDesign = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_INTERFACE_EVENT_MATHEMATICAL_DESIGN_AUDIT.md'
$radianceInterfaceResult = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_INTERFACE_EVENT_ORACLE_RESULT.md'
$radianceInterfaceProof = Join-Path $root 'tools\prove-g1-c3-visible-radiance-interface-math.py'
$radianceInterfaceKernelDesign = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_INTERFACE_NUMERICAL_KERNEL_DESIGN_AUDIT.md'
$radianceInterfaceKernelResult = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_INTERFACE_STAGED_KERNEL_ORACLE_RESULT.md'
$radianceInterfaceKernelProof = Join-Path $root 'tools\prove-g1-c3-visible-radiance-interface-staged-kernel.py'
$radianceInterfaceAdaptiveDesign = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_INTERFACE_POST_ORACLE_STRATEGY_REASSESSMENT.md'
$radianceInterfaceAdaptiveResult = Join-Path $root 'docs\canonical-system\G1_C3_VISIBLE_RADIANCE_INTERFACE_ADAPTIVE_REFINEMENT_ORACLE_RESULT.md'
$radianceInterfaceAdaptiveProof = Join-Path $root 'tools\prove-g1-c3-visible-radiance-interface-adaptive.py'
$radianceInterfaceContract = Join-Path $root 'contracts\visible-radiance-interface-event-contract.md'
$radianceInterfaceSource = Join-Path $root 'crates\visible-radiance-interface-event\src\lib.rs'
$radianceInterfaceArithmetic = Join-Path $root 'crates\visible-radiance-interface-event\src\arithmetic.rs'
$registry = Get-Content (Join-Path $root 'docs\canonical-system\system-registry.json') -Raw | ConvertFrom-Json

foreach ($path in @($contract, $result, $source, $stellarContract, $stellarResult, $stellarSource, $geologicalContract, $geologicalResult, $geologicalSource, $hydrologicalContract, $hydrologicalResult, $hydrologicalSource, $climateContract, $climateResult, $climateSource, $surfaceMaterialContract, $surfaceMaterialResult, $surfaceMaterialSource, $regionalContract, $regionalResult, $regionalSource, $signalResult, $opportunityContract, $opportunityResult, $opportunitySource, $consumerAudit, $fieldReceiptResult, $spatialContract, $spatialResult, $spatialSource, $partitionContract, $partitionResult, $partitionSource, $pathContract, $pathResult, $pathSource, $radianceMathDesign, $radianceMathResult, $radianceMathProof, $radianceBulkContract, $radianceBulkReadiness, $radianceBulkResult, $radianceBulkSource, $radianceInterfaceDesign, $radianceInterfaceResult, $radianceInterfaceProof, $radianceInterfaceKernelDesign, $radianceInterfaceKernelResult, $radianceInterfaceKernelProof, $radianceInterfaceAdaptiveDesign, $radianceInterfaceAdaptiveResult, $radianceInterfaceAdaptiveProof)) {
  if (!(Test-Path $path)) { throw "C3 derived-world evidence missing: $path" }
}
foreach ($path in @($radianceInterfaceContract, $radianceInterfaceSource, $radianceInterfaceArithmetic)) {
  if (!(Test-Path $path)) { throw "C3 visible-radiance interface implementation missing: $path" }
}
$radianceBulkText = Get-Content $radianceBulkSource -Raw
foreach ($token in @('pub struct VisibleRadianceBulkProfileInputV1','pub struct VisibleRadianceBulkProfileV1','pub struct VisibleRadianceBulkQueryV1','pub struct FixedU128V1','pub enum BulkTransferOutcomeV1','pub struct VisibleRadianceBulkTransferV1','pub fn compile_visible_radiance_bulk_profile','pub fn validate_visible_radiance_bulk_profile','pub fn compile_visible_radiance_bulk_transfer','pub fn validate_visible_radiance_bulk_transfer','same_substance_subdivision_and_reversal_are_invariant','unavailable_ambiguous_and_interfaces_fail_as_typed_evidence','monotonicity_and_oracle_fixed_vectors_hold','maximum_profile_cost_receipt')) {
  if (!$radianceBulkText.Contains($token)) { throw "C3 visible-radiance bulk-transfer implementation shield missing: $token" }
}
$radianceProofText = Get-Content $radianceMathProof -Raw
foreach ($token in @('exp_base_q64','exp_neg_q64_bounds','merge_spans','assert_transfer_enclosure','ambiguous_boundary_lane','interface_model_required','random_transfer_enclosures=768','conservative_accumulator_product_bits')) {
  if (!$radianceProofText.Contains($token)) { throw "C3 visible-radiance mathematical proof shield missing: $token" }
}
$radianceInterfaceProofText = Get-Content $radianceInterfaceProof -Raw
foreach ($token in @('classify_geometry','sqrt_interval','band_event','normal analytic reflectance','critical equality is TIR','Snell tangent invariant','GENERATED_CASES = 1024','max_intermediate_numerator_bits','no downstream refractive path')) {
  if (!$radianceInterfaceProofText.Contains($token)) { throw "C3 visible-radiance interface oracle shield missing: $token" }
}
$radianceInterfaceKernelProofText = Get-Content $radianceInterfaceKernelProof -Raw
foreach ($token in @('exact_tir','REQUIRED_PRECISIONS = (72, 80, 96, 128)','SENSITIVITY_PRECISIONS = (160, 192, 256, 384)','coprime-wide-transmit','max_post_cancellation_tir_product_bits','pass_with_required_precision_counterexample','no schema implementation dependency')) {
  if (!$radianceInterfaceKernelProofText.Contains($token)) { throw "C3 visible-radiance staged-kernel oracle shield missing: $token" }
}
$radianceInterfaceAdaptiveProofText = Get-Content $radianceInterfaceAdaptiveProof -Raw
foreach ($token in @('ADAPTIVE_LADDER = (96, 128, 160, 192, 256, 384)','exact_fast_path_kind','normal_incidence','index_matched_perfect_square','tir_perfect_square','retained interval widened','nonconvergent_enclosure','forced_cap_outcome','fixed_160_fractional_bit_work_units','no schema implementation dependency')) {
  if (!$radianceInterfaceAdaptiveProofText.Contains($token)) { throw "C3 visible-radiance adaptive oracle shield missing: $token" }
}
$radianceInterfaceSourceText = Get-Content $radianceInterfaceSource -Raw
foreach ($token in @('pub struct VisibleRadianceInterfaceInputV1','pub struct FaceInteractionEvidenceV1','pub enum InterfaceEventOutcomeV1','NonconvergentEnclosure','pub fn compile_visible_radiance_interface_event','pub fn validate_visible_radiance_interface_event','exact_tir','exact_fast_outcome','wide_geometry_guard','retained_critical_and_coprime_wide_kernel_cases_certify_without_hidden_work','normal_incidence_matches_the_exact_analytic_power_fraction','independent_python_exact_fixed_portfolio_checksum_matches','deterministic_generated_kernel_portfolio_preserves_all_postconditions')) {
  if (!$radianceInterfaceSourceText.Contains($token)) { throw "C3 visible-radiance interface implementation shield missing: $token" }
}
$radianceInterfaceArithmeticText = Get-Content $radianceInterfaceArithmetic -Raw
foreach ($token in @('PRECISIONS: [u16; 3] = [96, 128, 160]','struct Signed512','div_floor','div_ceil','floor_sqrt_vartime','canonical_decimal','native_limbs_are_absent_from_target_neutral_decimal_codec')) {
  if (!$radianceInterfaceArithmeticText.Contains($token)) { throw "C3 visible-radiance interface arithmetic shield missing: $token" }
}
$pathText = Get-Content $pathSource -Raw
foreach ($token in @('pub struct PhysicalVolumeRecipeInputV1','pub struct PhysicalVolumeV1','pub struct PhysicalPathQueryV1','pub struct UnitRationalV1','pub struct PhysicalPathWitnessV1','pub fn compile_physical_volume','pub fn validate_physical_volume','pub fn compile_path_witness','pub fn validate_path_witness','face_edge_vertex_and_endpoint_contacts_are_conservative','zero_length_vertex_is_eight_points_with_full_parameter_preimage','thin_barrier_crossing_differs_from_point_contact','maximum_ceiling_cost_receipt')) {
  if (!$pathText.Contains($token)) { throw "C3 physical-path substrate implementation shield missing: $token" }
}
$partitionText = Get-Content $partitionSource -Raw
foreach ($token in @('pub struct PhysicalPartitionRecipeV1','pub struct PhysicalPartitionInputV1','pub struct PhysicalRegionPartitionV1','pub fn compile_physical_region_partition','pub fn validate_physical_region_partition','availability_is_climate_derived_and_distinct_from_numeric_zero','disconnected_equal_signatures_remain_distinct_without_edge_wrap','reconstruction_mismatch_and_proof_ceiling_fail_before_sampling','forged_membership_identity_boundary_and_authority_fail_closed')) {
  if (!$partitionText.Contains($token)) { throw "C3 physical-region partition implementation shield missing: $token" }
}
$spatialText = Get-Content $spatialSource -Raw
foreach ($token in @('pub struct SpatialDomainInput','pub struct SpatialDomain','pub struct SpatialCell','pub fn compile_spatial_domain','pub fn validate_spatial_domain','pub fn build_spatial_cell','pub fn validate_spatial_cell','neighbours_are_ordered_edge_only_and_never_wrap','forged_coordinate_neighbours_identity_and_claims_are_rejected','coordinate_overflow_and_out_of_bounds_fail_before_output')) {
  if (!$spatialText.Contains($token)) { throw "C3 spatial-domain implementation shield missing: $token" }
}
$opportunityText = Get-Content $opportunitySource -Raw
foreach ($token in @('pub enum EnvironmentalOpportunity','physical_regime_id','RadiantEnergy','SurfaceAccessibleLiquid','SurfaceMoisturePotential','SolidSubstrate','build_environmental_opportunity_graph','validate_environmental_opportunity_graph','absent_liquid_atmosphere_and_substrate_do_not_become_opportunities','fabricated_node_is_rejected_against_the_packet')) {
  if (!$opportunityText.Contains($token)) { throw "C3 environmental-opportunity implementation shield missing: $token" }
}
$text = Get-Content $source -Raw
foreach ($token in @('pub struct WorldGenerationInput','pub struct SignalPotential','pub struct CausalWorldPacket','pub fn compile_world','pub fn validate_world_packet','foreign_or_fabricated_stellar_orbital_state_fails_before_world_compile','a_valid_signal_ecology_need_not_have_visible_light_or_eyes','medium_dependent_signals_fail_closed','regional_coordinates_cause_palette_and_visible_signal_variation','baseline_potentials_do_not_claim_unimplemented_propagation')) {
  if (!$text.Contains($token)) { throw "C3 derived-world implementation shield missing: $token" }
}
$system = @($registry.systems | Where-Object id -eq 'derived-world-rules')
if ($system.Count -ne 1 -or $system[0].status -ne 'prototype_tested' -or $system[0].depends_on -notcontains 'field-basis' -or $system[0].depends_on -notcontains 'surface-material-state' -or $system[0].depends_on -notcontains 'regional-environment-state') {
  throw 'C3 derived-world registry state or dependency is invalid.'
}
$pathSystem = @($registry.systems | Where-Object id -eq 'physical-path-substrate')
if ($pathSystem.Count -ne 1 -or $pathSystem[0].status -ne 'prototype_tested' -or @($pathSystem[0].depends_on).Count -ne 0) {
  throw 'C3 physical-path substrate registry state or dependency is invalid.'
}
$radianceBulkSystem = @($registry.systems | Where-Object id -eq 'visible-radiance-bulk-transfer')
if ($radianceBulkSystem.Count -ne 1 -or $radianceBulkSystem[0].status -ne 'prototype_tested' -or $radianceBulkSystem[0].depends_on -notcontains 'physical-path-substrate') {
  throw 'C3 visible-radiance bulk-transfer registry state or dependency is invalid.'
}
$radianceInterfaceSystem = @($registry.systems | Where-Object id -eq 'visible-radiance-interface-event')
if ($radianceInterfaceSystem.Count -ne 1 -or $radianceInterfaceSystem[0].status -ne 'prototype_tested' -or $radianceInterfaceSystem[0].depends_on -notcontains 'physical-path-substrate') {
  throw 'C3 visible-radiance interface-event registry state or dependency is invalid.'
}
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
& $cargo test -p stellar-orbital
if ($LASTEXITCODE -ne 0) { throw 'C3 stellar/orbital focused tests failed.' }
& $cargo test -p geological-atmospheric
if ($LASTEXITCODE -ne 0) { throw 'C3 geological/atmospheric focused tests failed.' }
& $cargo test -p hydrological-state
if ($LASTEXITCODE -ne 0) { throw 'C3 hydrological focused tests failed.' }
& $cargo test -p climate-state
if ($LASTEXITCODE -ne 0) { throw 'C3 climate focused tests failed.' }
& $cargo test -p surface-material-state
if ($LASTEXITCODE -ne 0) { throw 'C3 surface material focused tests failed.' }
& $cargo test -p regional-environment-state
if ($LASTEXITCODE -ne 0) { throw 'C3 regional environment focused tests failed.' }
& $cargo test -p spatial-domain
if ($LASTEXITCODE -ne 0) { throw 'C3 spatial-domain focused tests failed.' }
& $cargo test -p physical-region-partition
if ($LASTEXITCODE -ne 0) { throw 'C3 physical-region partition focused tests failed.' }
& $cargo test -p physical-path-substrate
if ($LASTEXITCODE -ne 0) { throw 'C3 physical-path substrate focused tests failed.' }
& $cargo test -p visible-radiance-bulk-transfer
if ($LASTEXITCODE -ne 0) { throw 'C3 visible-radiance bulk-transfer focused tests failed.' }
& $cargo test -p visible-radiance-interface-event
if ($LASTEXITCODE -ne 0) { throw 'C3 visible-radiance interface-event focused tests failed.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (!(Test-Path $python)) {
  $python = (Get-Command python3 -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source -First 1)
}
if (!$python -or !(Test-Path $python)) { throw 'C3 visible-radiance mathematical proof requires a real Python runtime.' }
& $python $radianceMathProof
if ($LASTEXITCODE -ne 0) { throw 'C3 visible-radiance mathematical proof failed.' }
& $python $radianceInterfaceProof
if ($LASTEXITCODE -ne 0) { throw 'C3 visible-radiance interface-event oracle failed.' }
& $python $radianceInterfaceKernelProof
if ($LASTEXITCODE -ne 0) { throw 'C3 visible-radiance staged-kernel oracle failed.' }
& $python $radianceInterfaceAdaptiveProof
if ($LASTEXITCODE -ne 0) { throw 'C3 visible-radiance adaptive-refinement oracle failed.' }
& $cargo test -p derived-world-rules
if ($LASTEXITCODE -ne 0) { throw 'C3 derived-world focused tests failed.' }
& $cargo test -p niche-graph-binding
if ($LASTEXITCODE -ne 0) { throw 'C3 environmental-opportunity focused tests failed.' }
& powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $root 'tools\verify-f5-field-basis-readiness.ps1')
if ($LASTEXITCODE -ne 0) { throw 'C3 retained field-basis and second-language receipt gate failed.' }
Write-Output 'G1 C3 bounded physical-world foundation verified: focused physical suites, strict non-wrapping 2D sampling domains, corrected physical-region partition, isolated exact 3D occupancy/path evidence, single-medium visible-radiance bulk-transfer bounds, permanent local interface-event schema/code, independent bulk/interface/staged/adaptive Python math, and same-platform field vectors pass; downstream refractive paths, perception, traversability, spherical geometry, biome/ecotone consumers and real second-platform execution limits remain explicit.'
