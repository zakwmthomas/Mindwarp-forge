$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$federation = Get-Content -Raw (Join-Path $root 'crates\forge-kernel\src\federation.rs')
$persistence = Get-Content -Raw (Join-Path $root 'crates\forge-kernel\src\persistence.rs')
$cli = Get-Content -Raw (Join-Path $root 'crates\forge-kernel\src\bin\forge-federate.rs')
$wrapper = Get-Content -Raw (Join-Path $root 'tools\forge-writer-lease.ps1')
$contract = Get-Content -Raw (Join-Path $root 'contracts\live-writer-lease-contract.md')
$result = Get-Content -Raw (Join-Path $root 'docs\canonical-system\G1_FEDERATED_LIVE_WRITER_LEASE_INTEGRATION_RESULT.md')
$workstream = Get-Content -Raw (Join-Path $root 'governance\federation\workstreams\forge-live-mainline.json') | ConvertFrom-Json

foreach ($required in @('WRITER_LEASE_MAX_TTL_MS','claim_writer_lease','assert_writer_lease','release_writer_lease','checkpoint-sha256:')) {
    if (!$federation.Contains($required)) { throw "Writer lease domain shield missing: $required" }
}
foreach ($required in @('routed_workstream','session_route','FederatedRecordConflict','claim_writer_lease','assert_writer_lease','release_writer_lease')) {
    if (!$persistence.Contains($required)) { throw "Writer lease persistence shield missing: $required" }
}
foreach ($required in @('claim-workstream-writer','assert-workstream-writer','release-workstream-writer')) {
    if (!$cli.Contains($required)) { throw "Writer lease CLI shield missing: $required" }
}
foreach ($required in @('CODEX_THREAD_ID','registered_repository_root','AllowLiveDatabaseMutation','Get-FileHash','route-session')) {
    if (!$wrapper.Contains($required)) { throw "Writer lease wrapper shield missing: $required" }
}
foreach ($required in @('read-only by default','1,800 seconds','Checkpoint drift','Greenfield','separately gated')) {
    if (!$contract.Contains($required)) { throw "Writer lease contract drift: $required" }
}
foreach ($required in @('complete integration and additive live registration','103 tests','exit 0 in 282.7 seconds','only the additive `forge-live-mainline`','did not rewrite knowledge')) {
    if (!$result.Contains($required)) { throw "Writer lease result drift: $required" }
}
if ($workstream.id -ne 'forge-live-mainline' -or $workstream.status -ne 'active' -or $null -ne $workstream.lease -or $workstream.checkpoint_uri -ne 'context/active/WORKER_BATCH_STATE.json') {
    throw 'Canonical live writer workstream record drifted.'
}

& (Join-Path $PSScriptRoot 'test-forge-writer-lease.ps1')
if (!$?) { throw 'Disposable live writer lease integration failed.' }
Write-Output 'Live writer lease integration verified: exact routed holder, bounded checkpoint claim, conflict and drift rejection, release/takeover, live-mutation shield and authority-negative state pass.'
