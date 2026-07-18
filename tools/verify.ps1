$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$ui = Join-Path $root 'apps\forge-desktop\ui'
$desktopBuildTarget = Join-Path $root 'target\verification\forge-desktop'
$vcvars = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat'
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'

& (Join-Path $PSScriptRoot 'verify-atlas.ps1')
if (!$?) { throw 'Project Atlas validation failed.' }
& (Join-Path $PSScriptRoot 'verify-operating-system.ps1')
if (!$?) { throw 'Forge operating-system validation failed.' }
& (Join-Path $PSScriptRoot 'verify-model-agnostic-handoff.ps1')
if (!$?) { throw 'Model-agnostic AI handoff validation failed.' }
& (Join-Path $PSScriptRoot 'verify-forge-startup-idempotency.ps1')
if (!$?) { throw 'Forge startup idempotency validation failed.' }
& (Join-Path $PSScriptRoot 'verify-canonical-system.ps1')
if (!$?) { throw 'Canonical system registry validation failed.' }
& (Join-Path $PSScriptRoot 'verify-conversation-compiler-continuity.ps1')
if (!$?) { throw 'Conversation compiler continuity validation failed.' }
& (Join-Path $PSScriptRoot 'verify-federated-routing-storage-v1.ps1')
if (!$?) { throw 'Federated routing and storage V1 validation failed.' }
& (Join-Path $PSScriptRoot 'verify-live-writer-lease-integration.ps1')
if (!$?) { throw 'Live writer lease integration validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h1-reference-intake.ps1')
if (!$?) { throw 'H1 reference-intake validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h2-neutral-humanoid.ps1')
if (!$?) { throw 'H2 neutral humanoid validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h3-humanoid-generation.ps1')
if (!$?) { throw 'H3 humanoid generation validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h4-control-calibration.ps1')
if (!$?) { throw 'H4 control calibration failed.' }
& (Join-Path $PSScriptRoot 'verify-h6-humanoid-recovery.ps1')
if (!$?) { throw 'H6 humanoid reproduction and recovery failed.' }
& (Join-Path $PSScriptRoot 'verify-h7-humanoid-promotion-readiness.ps1')
if (!$?) { throw 'H7 humanoid promotion readiness failed.' }
& (Join-Path $PSScriptRoot 'verify-record-roles.ps1')
if (!$?) { throw 'Forge record-role validation failed.' }
& (Join-Path $PSScriptRoot 'verify-coherence.ps1')
if (!$?) { throw 'Forge coherence validation failed.' }
& (Join-Path $PSScriptRoot 'verify-worker-governance.ps1')
if (!$?) { throw 'Worker governance validation failed.' }
& (Join-Path $PSScriptRoot 'verify-forge-metrics-dashboard.ps1')
if (!$?) { throw 'Forge metrics dashboard validation failed.' }
& (Join-Path $PSScriptRoot 'verify-whole-system-method-audit.ps1')
if (!$?) { throw 'Whole-system reusable-method audit validation failed.' }
& (Join-Path $PSScriptRoot 'verify-step-leader-framework.ps1')
if (!$?) { throw 'Step-leader framework validation failed.' }
& (Join-Path $PSScriptRoot 'verify-worker-proof-harness.ps1')
if (!$?) { throw 'Worker proof harness validation failed.' }
& (Join-Path $PSScriptRoot 'test-worker-feedback.ps1')
if (!$?) { throw 'Worker feedback fixtures failed.' }
& (Join-Path $PSScriptRoot 'verify-worker-batch-state.ps1')
if (!$?) { throw 'Worker batch-state validation failed.' }
& (Join-Path $PSScriptRoot 'test-worker-batch-state.ps1')
if (!$?) { throw 'Worker batch-state fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-stage-quality-gates.ps1')
if (!$?) { throw 'Stage-quality fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-active-context-projections.ps1')
if (!$?) { throw 'Active-context projection fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-verify-bootstrap-read-only.ps1')
if (!$?) { throw 'Read-only bootstrap fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-ensure-context-launch-paths.ps1')
if (!$?) { throw 'Forge context launch-path fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-worker-selector.ps1')
if (!$?) { throw 'Worker selector fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-worker-progress.ps1')
if (!$?) { throw 'Worker progress fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-worker-escalation.ps1')
if (!$?) { throw 'Worker escalation fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-owner-notification.ps1')
if (!$?) { throw 'Owner notification fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-forge-heartbeat-control.ps1')
if (!$?) { throw 'Forge heartbeat control fixtures failed.' }
& (Join-Path $PSScriptRoot 'test-forge-chat-visual.ps1')
if (!$?) { throw 'Forge chat visual fixtures failed.' }
& (Join-Path $PSScriptRoot 'verify-modularity.ps1')
if (!$?) { throw 'Modularity verification failed.' }
& (Join-Path $PSScriptRoot 'verify-module-context.ps1')
if (!$?) { throw 'Module context verification failed.' }
& (Join-Path $PSScriptRoot 'test-modularity.ps1')
if (!$?) { throw 'Modularity fixtures failed.' }
& (Join-Path $PSScriptRoot 'verify-f4-closeout.ps1')
if (!$?) { throw 'F4 closeout verification failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-proof-receipt.ps1')
if (!$?) { throw 'F5 ProofReceipt verification failed.' }
& (Join-Path $PSScriptRoot 'test-proof-receipt-system-id-projection.ps1')
if (!$?) { throw 'ProofReceipt system-ID projection fixtures failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-universe-identity-readiness.ps1')
if (!$?) { throw 'F5 universe identity readiness verification failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-field-basis-readiness.ps1')
if (!$?) { throw 'F5 field-basis readiness verification failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-hierarchy-history-readiness.ps1')
if (!$?) { throw 'F5 hierarchy/history readiness verification failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-significance-scheduler-readiness.ps1')
if (!$?) { throw 'F5 significance/scheduler readiness verification failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-semantic-construction-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-f5-representation-asset-animation-readiness.ps1')
if (!$?) { throw 'F5 semantic/construction readiness verification failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-natural-function-reassessment.ps1')
if (!$?) { throw 'F5 natural-function reassessment verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-derived-world.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-swept-aabb.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-optical-continuation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-incident-interface.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-numerical-certificate.ps1')
if (!$?) { throw 'G1 C3 derived-world verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-interface-implementation.ps1')
if (!$?) { throw 'G1 C3 fixed-160 interval-interface implementation verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-fixed160-consumer-reassessment.ps1')
if (!$?) { throw 'G1 C3 post-fixed160 consumer reassessment verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-optical-cell-step.ps1')
if (!$?) { throw 'G1 C3 interval optical cell-step verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-cell-step-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-bulk-transfer.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-interval-cell-step-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-fixed-interval-arithmetic-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-consolidation-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-bulk-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-interval-bulk-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-interval-bulk-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-lineage-design.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-lineage-oracle.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-lineage-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-lineage-binding.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-optical-lineage-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-source-receiver-prerequisite-audit.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-cumulative-lane-transfer-oracle.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-cumulative-lane-transfer-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-cumulative-lane-transfer-binding.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-cumulative-lane-transfer-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-receiver-arrival-geometry-oracle.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-receiver-arrival-geometry-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-receiver-arrival-geometry-binding.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-receiver-arrival-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-lane-coupling-design.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-lane-coupling-oracle.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-coupling-oracle-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-optical-coupling.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-coupling-provenance-gap.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-cell-provenance.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-cell-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-cell-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-optical-phase-space-provenance-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-transport-certificate.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-transport-width.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-origin-anchored.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-transport-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-optical-phase-space-transport-certificate-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-optical-phase-space-transport-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-receiver-coupling.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-receiver-coupling-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-receiver-coupling-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-whole-cell-receiver-coupling-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-dimensionless-transfer.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-dimensionless-transfer-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-whole-cell-dimensionless-transfer-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-whole-cell-dimensionless-transfer-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-source-distribution-measure.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-source-quantity-basis-schema-gap.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-source-quantity-basis.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-spectral-time-basis.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-basis-transport-applicability-gap.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-source-calibration-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-source-calibration-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-source-calibration-reassessment.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-source-energy-distribution.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-source-energy-distribution-readiness.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-source-energy-distribution-implementation.ps1')
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-calibrated-source-energy-distribution-reassessment.ps1')
if (!$?) { throw 'G1 C3 interval optical cell-step readiness verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-transport-applicability-witness-gap.ps1')
if (!$?) { throw 'G1 C3 calibrated transport-applicability witness gap verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-transport-applicability-witness-design.ps1')
if (!$?) { throw 'G1 C3 calibrated transport-applicability witness mathematical-design verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-calibrated-transport-applicability-physical-evidence-protocol.ps1')
if (!$?) { throw 'G1 C3 calibrated transport-applicability physical-evidence protocol verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-post-physical-evidence-residual-obligation-audit.ps1')
if (!$?) { throw 'G1 C3 post-physical-evidence residual-obligation audit verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-cross-boundary-ecotone-design.ps1')
if (!$?) { throw 'G1 C3 cross-boundary ecotone mathematical-design verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-cross-boundary-ecotone-oracle-readiness.ps1')
if (!$?) { throw 'G1 C3 cross-boundary ecotone oracle implementation-readiness verification failed.' }
& (Join-Path $PSScriptRoot 'verify-g1-c3-cross-boundary-ecotone-oracle.ps1')
if (!$?) { throw 'G1 C3 cross-boundary ecotone disposable oracle verification failed.' }
& (Join-Path $PSScriptRoot 'test-p7b1b-loader-surface.ps1')
if (!$?) { throw 'P7b-1b loader-surface proof verification failed.' }

Push-Location $ui
try {
    & npm.cmd run build
    if ($LASTEXITCODE -ne 0) { throw 'UI build failed.' }
} finally {
    Pop-Location
}

if (!(Test-Path $vcvars)) { throw 'Visual C++ Build Tools are required to verify Rust code.' }
$environment = cmd.exe /c "call `"$vcvars`" >nul && set"
foreach ($line in $environment) {
    if ($line -match '^(.*?)=(.*)$') { Set-Item -Path "env:$($matches[1])" -Value $matches[2] }
}

Push-Location $root
try {
    $env:RUSTFLAGS = '-D warnings'
    & $cargo fmt --all -- --check
    if ($LASTEXITCODE -ne 0) { throw 'Rust formatting check failed.' }
    & $cargo test --workspace
    if ($LASTEXITCODE -ne 0) { throw 'Rust tests failed.' }
    # Forge can be running from target\debug while verification is active. Build
    # into an isolated target so Windows never needs to replace the live binary.
    & $cargo build -p forge-desktop --target-dir $desktopBuildTarget
    if ($LASTEXITCODE -ne 0) { throw 'Tauri desktop build failed.' }
    & git diff --check
    if ($LASTEXITCODE -ne 0) { throw 'Repository whitespace check failed.' }
} finally {
    Pop-Location
}
