$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$script = Join-Path $root 'tools\prove-g1-c3-interval-incident-interface.py'
$result = Join-Path $root 'docs\canonical-system\G1_C3_INTERVAL_INCIDENT_INTERFACE_ORACLE_RESULT.md'
$bundledPython = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundledPython) { $bundledPython } else { 'python' }

foreach ($path in @($script, $result)) {
    if (!(Test-Path -LiteralPath $path)) { throw "Interval-incident interface artifact missing: $path" }
}

$first = (& $python $script | Out-String)
if ($LASTEXITCODE -ne 0) { throw 'Interval-incident interface oracle failed.' }
$second = (& $python $script | Out-String)
if ($LASTEXITCODE -ne 0) { throw 'Interval-incident interface oracle repeat failed.' }
if ($first -ne $second) { throw 'Interval-incident interface oracle output is not deterministic.' }
$receipt = $first | ConvertFrom-Json

if ($receipt.status -ne 'pass' -or $receipt.total_cases -ne 265) { throw 'Interval-incident portfolio drift.' }
if ($receipt.branch_distribution.all_tir -ne 101 -or $receipt.branch_distribution.all_transmit -ne 162 -or $receipt.branch_distribution.ambiguous_interface_branch -ne 2) { throw 'Interval-incident branch distribution drift.' }
if ($receipt.stop_distribution.'96' -ne 263 -or $receipt.nonconvergent_cases -ne 0) { throw 'Interval-incident precision distribution drift.' }
if ($receipt.forced_cap_outcome -ne 'nonconvergent_enclosure' -or $receipt.forced_cap_numerical_excess_target_units -ne 2991) { throw 'Interval-incident forced-cap shield drift.' }
if ($receipt.max_candidate_live_integer_bits -gt 512 -or $receipt.repeated_event_portfolio.max_live_integer_bits -gt 512) { throw 'Interval-incident fixed-width ceiling exceeded.' }
foreach ($lane in @('red','green','blue')) {
    $laneReceipt = $receipt.repeated_event_portfolio.lanes.$lane
    if ($laneReceipt.events_attempted -ne 64 -or $laneReceipt.all_transmit_events -ne 64) { throw "Interval-incident repeated lane drift: $lane" }
}
$checksum = 'ff0da6f60432a42c10e45371459e1b2a44ea98dc0bba8d664879dc8c20eaa488'
if ($receipt.receipt_sha256 -ne $checksum) { throw 'Interval-incident receipt checksum drift.' }
if (!(Get-Content -LiteralPath $result -Raw).Contains($checksum)) { throw 'Interval-incident result does not bind the oracle checksum.' }

Write-Output "Interval-incident interface oracle verified: $($receipt.checks) checks, checksum $checksum."

