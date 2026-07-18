Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$scriptPath = Join-Path $root 'tools\prove-g1-c3-cumulative-lane-transfer.py'
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_CUMULATIVE_LANE_TRANSFER_MATHEMATICAL_DESIGN_AUDIT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_CUMULATIVE_LANE_TRANSFER_ORACLE_RESULT.md'
$expectedSource = '62cdd6d36a2c74d315a9990a17b06641fbeb1f04ed747dab8c0d1e9f203d88fa'
if ((Get-FileHash -LiteralPath $scriptPath -Algorithm SHA256).Hash.ToLowerInvariant() -ne $expectedSource) {
  throw 'Cumulative lane-transfer oracle source checksum drifted.'
}
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$first = & $python $scriptPath
$second = & $python $scriptPath
if ($LASTEXITCODE -ne 0 -or $first -ne $second) { throw 'Cumulative lane-transfer oracle failed or drifted between runs.' }
$receipt = $first | ConvertFrom-Json
if ($receipt.receipt_sha256 -ne 'ee5f237fe1c8b7581372646e01ab12c7ddedfa1707d1b0e5dbf199e81b2ba09d' -or
    $receipt.candidate -ne 'accepted_for_implementation_readiness_audit_only' -or
    $receipt.fractional_bits -ne 160 -or $receipt.factor_ceiling -ne 128 -or
    $receipt.live_bit_shield -ne 209 -or $receipt.hostile_rejection_count -ne 26 -or
    $receipt.portfolios.one_twenty_eight_mixed.maximum_observed_bits -ne 209 -or
    $receipt.portfolios.sub_q160_positive.upper_q0_48 -ne 1 -or
    $receipt.portfolios.opaque_zero.upper_q0_48 -ne 0) {
  throw 'Cumulative lane-transfer oracle canonical receipt drifted.'
}
foreach ($name in @('factor_deletion','factor_duplication','factor_reordering','cross_band_substitution','resealed_endpoint_change','terminal_interface_factor_injection','repeated_q48_false_zero_policy','authority_mutation')) {
  if (@($receipt.hostile_rejections | Where-Object { $_ -eq $name }).Count -ne 1) { throw "Missing cumulative hostile case: $name" }
}
$design = Get-Content -LiteralPath $designPath -Raw
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('Q0.160','209-bit','128','Terminal interface outcomes','not contribute reflected','non-detectability','Do not add a crate')) {
  if ($design -notlike "*$required*") { throw "Cumulative transfer design drift: $required" }
}
foreach ($required in @('byte-identical','26 cases','[0,1]','implementation-readiness audit','No crate, dependency, schema or source')) {
  if ($result -notlike "*$required*") { throw "Cumulative transfer oracle result drift: $required" }
}
Write-Output 'Cumulative lane-transfer oracle verified: exact-rational containment, Q0.160 directed accumulation, 209-bit shield, 128-factor cap, 26 hostile rejections and no-arrival boundary are stable.'
