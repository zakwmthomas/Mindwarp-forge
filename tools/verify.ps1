$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$ui = Join-Path $root 'apps\forge-desktop\ui'
$vcvars = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat'
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'

& (Join-Path $PSScriptRoot 'verify-atlas.ps1')
if (!$?) { throw 'Project Atlas validation failed.' }
& (Join-Path $PSScriptRoot 'verify-operating-system.ps1')
if (!$?) { throw 'Forge operating-system validation failed.' }
& (Join-Path $PSScriptRoot 'verify-canonical-system.ps1')
if (!$?) { throw 'Canonical system registry validation failed.' }
& (Join-Path $PSScriptRoot 'verify-conversation-compiler-continuity.ps1')
if (!$?) { throw 'Conversation compiler continuity validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h1-reference-intake.ps1')
if (!$?) { throw 'H1 reference-intake validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h2-neutral-humanoid.ps1')
if (!$?) { throw 'H2 neutral humanoid validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h3-humanoid-generation.ps1')
if (!$?) { throw 'H3 humanoid generation validation failed.' }
& (Join-Path $PSScriptRoot 'verify-h4-control-calibration.ps1')
if (!$?) { throw 'H4 control calibration failed.' }
& (Join-Path $PSScriptRoot 'verify-record-roles.ps1')
if (!$?) { throw 'Forge record-role validation failed.' }
& (Join-Path $PSScriptRoot 'verify-coherence.ps1')
if (!$?) { throw 'Forge coherence validation failed.' }
& (Join-Path $PSScriptRoot 'verify-worker-governance.ps1')
if (!$?) { throw 'Worker governance validation failed.' }
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
& (Join-Path $PSScriptRoot 'test-modularity.ps1')
if (!$?) { throw 'Modularity fixtures failed.' }
& (Join-Path $PSScriptRoot 'verify-f4-closeout.ps1')
if (!$?) { throw 'F4 closeout verification failed.' }
& (Join-Path $PSScriptRoot 'verify-f5-proof-receipt.ps1')
if (!$?) { throw 'F5 ProofReceipt verification failed.' }
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
    & $cargo build -p forge-desktop
    if ($LASTEXITCODE -ne 0) { throw 'Tauri desktop build failed.' }
    & git diff --check
    if ($LASTEXITCODE -ne 0) { throw 'Repository whitespace check failed.' }
} finally {
    Pop-Location
}
