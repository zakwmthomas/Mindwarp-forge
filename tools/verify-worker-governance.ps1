$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $root 'governance\worker-governance-manifest.json'
$policyPath = Join-Path $root 'governance\policy-registry.json'
foreach ($path in @($manifestPath, $policyPath)) {
    if (!(Test-Path -LiteralPath $path)) { throw "Worker-governance record missing: $path" }
}
$manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
if ($manifest.schema_version -ne 1) { throw 'Unsupported worker-governance manifest schema.' }
foreach ($relative in @($manifest.canonical_prompt, $manifest.protocol, $manifest.governance_system, $manifest.learning_ledger, $manifest.batch_state, $manifest.closure_register, $manifest.integration_plan)) {
    if (!(Test-Path -LiteralPath (Join-Path $root $relative))) { throw "Worker-governance target missing: $relative" }
}
$policy = Get-Content -LiteralPath $policyPath -Raw | ConvertFrom-Json
$approved = @($policy.policies | Where-Object status -eq 'approved' | ForEach-Object id)
foreach ($id in @($manifest.required_policies)) {
    if ($approved -notcontains $id) { throw "Worker-governance required policy is not approved: $id" }
}
$prompt = Get-Content -LiteralPath (Join-Path $root $manifest.canonical_prompt) -Raw
if ($prompt -notmatch 'MASTER_PROGRAM.json' -or $prompt -notmatch 'Worker Batch State') { throw 'Canonical worker prompt lacks master-program navigation requirements.' }
foreach ($required in @('five consecutive heartbeat wakes','dependency closure','never cross the gate','Wakes never imply owner approval','recognized owner-input gate','one labelled side-by-side image','capture only','never send the whole desktop','Unrelated owner chat does not resume automation')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks owner-wait fallback guard: $required" }
}
$protocol = Get-Content -LiteralPath (Join-Path $root $manifest.protocol) -Raw
foreach ($required in @('Five-wake owner-wait safety fallback','full dependency closure is already satisfied','never infer owner input','Immediate owner-wait suspension','one labelled side-by-side image','never the whole desktop','does not resume the scheduler')) {
    if (!$protocol.Contains($required)) { throw "Worker optimization protocol lacks owner-wait fallback guard: $required" }
}
if ($protocol -notmatch 'never\s+reorder a dependency chain') { throw 'Worker optimization protocol lacks dependency-reordering guard.' }
Write-Output "Worker governance verified: $(@($manifest.required_policies).Count) required policies and $(@($manifest.PSObject.Properties).Count - 2) linked records."
