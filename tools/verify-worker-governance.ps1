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
foreach ($required in @('refresh and record both context scales','actual pixels of every visual asset','human reference must','cheapest sufficient proof tier','Still run the required final integration gate')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks stage-quality guard: $required" }
}
foreach ($required in @('nature-inspired mechanisms as scoped hypotheses','baseline, cost, falsifier','counterexample','Never infer correctness')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks natural-method guard: $required" }
}
foreach ($required in @('whole-system discovery audit before localization','every registered Forge, game-canonical, and runtime system','Broad mapping is not universal adoption')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks whole-system discovery guard: $required" }
}
$protocol = Get-Content -LiteralPath (Join-Path $root $manifest.protocol) -Raw
foreach ($required in @('Five-wake owner-wait safety fallback','full dependency closure is already satisfied','never infer owner input','Immediate owner-wait suspension','one labelled side-by-side image','never the whole desktop','does not resume the scheduler')) {
    if (!$protocol.Contains($required)) { throw "Worker optimization protocol lacks owner-wait fallback guard: $required" }
}
if ($protocol -notmatch 'never\s+reorder a dependency chain') { throw 'Worker optimization protocol lacks dependency-reordering guard.' }
foreach ($required in @('Simulation-first execution ladder','cheapest tier capable','never replaces a required final integrated verification','Visual reference fitness','actual rendered pixels','anatomically incoherent')) {
    if (!$protocol.Contains($required)) { throw "Worker optimization protocol lacks stage-quality guard: $required" }
}
foreach ($required in @('Natural-method candidate gate','abstraction before treating it as a method','target-local transfer gate','objectives, resistance heuristics')) {
    if (!$protocol.Contains($required)) { throw "Worker optimization protocol lacks natural-method guard: $required" }
}
foreach ($required in @('whole-system discovery audit','complete registry pass','Broad discovery is not universal adoption')) {
    if (!$protocol.Contains($required)) { throw "Worker optimization protocol lacks whole-system discovery guard: $required" }
}
foreach ($required in @('Hybrid step-leader divergence','ten percent','Ordinary edits and heartbeat wakes are not triggers','Two independent successful modules')) {
    if (!$protocol.Contains($required)) { throw "Worker optimization protocol lacks bounded step-leader guard: $required" }
}
foreach ($required in @('hybrid step-leader divergence','every third meaningful','ten percent','Any target regression blocks reuse')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks bounded step-leader guard: $required" }
}
foreach ($required in @('read-only by default','CODEX_THREAD_ID','forge-writer-lease.ps1','Reassert the','Release the lease')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks sole-writer lease guard: $required" }
}
foreach ($required in @('one parent writer and zero specialists by default','at most two','never duplicate an inventory or broad review','smallest focused gate set','route-shield inventory','one complete Forge gate','concise checksums, counts, timings','reduces bootstrap, hostile, platform, owner, provenance, or final')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks token-efficiency guard: $required" }
}
foreach ($required in @('forge-batch-metrics.ps1 -Mode start','invoke-measured-run.ps1','Metrics are advisory evidence only')) {
    if (!$prompt.Contains($required)) { throw "Canonical worker prompt lacks metrics-boundary guard: $required" }
}
foreach ($required in @('Token-efficient package execution','zero specialists by default','Never commission duplicate inventories or broad reviews','smallest focused gate set','route-shield inventory','one complete Forge gate','weakens bootstrap, hostile, platform, owner, provenance, or final integration')) {
    if (!$protocol.Contains($required)) { throw "Worker optimization protocol lacks token-efficiency guard: $required" }
}
Write-Output "Worker governance verified: $(@($manifest.required_policies).Count) required policies and $(@($manifest.PSObject.Properties).Count - 2) linked records."
