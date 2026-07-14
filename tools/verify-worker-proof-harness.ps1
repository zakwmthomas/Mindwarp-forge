$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
& (Join-Path $PSScriptRoot 'verify-worker-feedback-freshness.ps1')
$brief = Get-Content (Join-Path $root 'governance\WORKER_FEEDBACK_BRIEF.md') -Raw
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$active = @($program.items | Where-Object status -eq 'active')
if (!$brief.Contains('Source fixity') -or !$brief.Contains('P10')) { throw 'Feedback brief lacks current governance data.' }
if ($active.Count -eq 0 -or @($active | Where-Object { [string]::IsNullOrWhiteSpace($_.next_action) }).Count -gt 0) { throw 'Active master-program item lacks executable next action.' }
if (@($program.items | Where-Object { $_.status -in @('gated','owner_gated','design_gated') -and $_.next_action -match 'Implement|Build' }).Count -gt 0) { throw 'Gated work is presented as implementation-ready.' }
foreach ($item in $program.items) {
  foreach ($source in @($item.sources)) {
    $matches = @(Get-ChildItem -Path (Join-Path $root 'docs'),(Join-Path $root 'governance'),(Join-Path $root 'contracts') -Recurse -File -ErrorAction SilentlyContinue | Where-Object Name -eq $source)
    if ($matches.Count -eq 0) { throw "Worker route has a missing source reference: $source on $($item.id)" }
  }
}
$protocol = Get-Content (Join-Path $root 'governance\WORKER_OPTIMIZATION_PROTOCOL.md') -Raw
if ($protocol -notmatch 'meaningful batches' -or $protocol -notmatch 'three') { throw 'Worker protocol lacks anti-micro-task audit controls.' }
$prompt = Get-Content (Join-Path $root 'governance\WORKER_PROMPT_SPEC.md') -Raw
if ($prompt -notmatch 'five consecutive heartbeat wakes' -or $prompt -notmatch 'independent of the waiting gate' -or $prompt -notmatch 'Wakes never imply owner approval' -or $prompt -notmatch 'forge-heartbeat-control.ps1 -Mode pause' -or $prompt -notmatch 'one labelled side-by-side image' -or $prompt -notmatch 'capture only' -or $prompt -notmatch 'never send the whole desktop' -or $prompt -notmatch 'Unrelated owner chat does not resume automation') { throw 'Worker prompt lacks dependency-safe owner-wait suspension controls.' }
Write-Output "Worker proof harness verified: feedback freshness, active routing, gate exclusion, and anti-micro-task controls."
