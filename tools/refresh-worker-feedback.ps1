param([string]$Root)
$ErrorActionPreference = 'Stop'
$root = if ($Root) { $Root } else { Split-Path -Parent $PSScriptRoot }
$sources = @('governance\policy-registry.json','governance\WORKER_GOVERNANCE_SYSTEM.md','governance\WORKER_PROMPT_SPEC.md','governance\WORKER_LEARNING_LEDGER.md','governance\SYSTEM_EFFICIENCY_AUDIT.md','governance\WORKER_METRIC_REGISTRY.md','governance\MEASUREMENT_AND_RECURSIVE_LEARNING_CONTRACT.md','context\active\WORKER_BATCH_STATE.json','docs\canonical-system\MASTER_CLOSURE_REGISTER.md')
foreach ($relative in $sources) { if (!(Test-Path -LiteralPath (Join-Path $root $relative))) { throw "Worker feedback source missing: $relative" } }
$policy = Get-Content -LiteralPath (Join-Path $root 'governance\policy-registry.json') -Raw | ConvertFrom-Json
$approved = @($policy.policies | Where-Object status -eq 'approved' | ForEach-Object id) -join ', '
$hashes = $sources | ForEach-Object { "| ``$_`` | ``$((Get-FileHash -LiteralPath (Join-Path $root $_) -Algorithm SHA256).Hash.ToLowerInvariant())`` |" }
$content = @"
# Worker Feedback Brief (Generated)

This is the mandatory closed-loop handoff from Forge records to every worker. It is generated during bootstrap; do not edit it directly.

## Required action

Apply the latest approved policies ($approved), active batch exit criteria, learning-ledger regressions, metric definitions, and diminishing-return stop/refocus rule. A previous prompt is never fresher than these source records.

## Closed-loop rule

Every material observation, improvement, failure, metric result, and stop/refocus decision is recorded in its canonical Forge record. The next bootstrap regenerates this brief from those records before work selection. Captured chat remains evidence, never authority.

## Source fixity

| Source | SHA-256 |
|---|---|
$($hashes -join "`n")
"@
Set-Content -LiteralPath (Join-Path $root 'governance\WORKER_FEEDBACK_BRIEF.md') -Value $content -NoNewline
Write-Output "Worker feedback brief refreshed from $($sources.Count) canonical records."
