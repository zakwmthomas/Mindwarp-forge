Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$scriptPath = Join-Path $root 'tools\prove-g1-c3-receiver-arrival-geometry.py'
$designPath = Join-Path $root 'docs\canonical-system\G1_C3_RECEIVER_ARRIVAL_GEOMETRY_MATHEMATICAL_DESIGN_AUDIT.md'
$resultPath = Join-Path $root 'docs\canonical-system\G1_C3_RECEIVER_ARRIVAL_GEOMETRY_ORACLE_RESULT.md'
$expectedSource = 'd1ea2e46e9e41e85b5523b629244b958b396903914fcf2f5dd70b7ad85f0a545'
$actualSource = (Get-FileHash -LiteralPath $scriptPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actualSource -ne $expectedSource) { throw 'Receiver-arrival geometry oracle source checksum drifted.' }
$python = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$first = (& $python $scriptPath | Out-String).Trim()
$second = (& $python $scriptPath | Out-String).Trim()
if ($LASTEXITCODE -ne 0 -or $first -ne $second) { throw 'Receiver-arrival geometry oracle failed or became nondeterministic.' }
$receipt = $first | ConvertFrom-Json
if ($receipt.receipt_sha256 -ne '25c31003ff4ee8d1be3b01a5a2203958238205e4adc80e6cc50623c27af69aea' -or
    $receipt.candidate -ne 'exact_ray_bounded_aabb_strict_interior_for_code_facing_readiness_only' -or
    $receipt.portfolio_count -ne 18 -or
    $receipt.hostile_rejection_count -ne 26 -or
    $receipt.conditional_policy -ne 'nondegenerate_point_direction_or_face_time_is_typed_unsupported' -or
    $receipt.point_receiver_policy -ne 'contact_only_never_strict_arrival' -or
    $receipt.face_tie_policy -ne 'contact_only_in_current_step' -or
    $receipt.authority_effect -ne 'none_evidence_only') {
  throw 'Receiver-arrival geometry oracle canonical receipt drifted.'
}
foreach ($name in @('before_face','after_face','start_inside','tangent_edge','point_receiver','face_tie','parallel_inside','parallel_outside','reverse_direction','fractional_entry','multi_cell_box','corner_contact','nondegenerate_point','nondegenerate_direction','nondegenerate_face_time','ambiguous_next_face','no_forward_progress','unavailable_current')) {
  if (@($receipt.portfolios | Where-Object name -eq $name).Count -ne 1) { throw "Missing receiver-arrival portfolio: $name" }
}
foreach ($name in @('conditional_midpoint_injection','face_tie_promoted_to_arrival','independently_resealed_step','step_65_cap_bypass')) {
  if (@($receipt.hostile_rejections | Where-Object { $_ -eq $name }).Count -ne 1) { throw "Missing receiver-arrival hostile case: $name" }
}
$design = Get-Content -LiteralPath $designPath -Raw
foreach ($required in @('exact-ray bounded-AABB strict-interior candidate','unsupported_conditional_evidence','arrival_at_start','contact_only','miss_before_face','0 <= t < t_face','point receiver','receiver spanning multiple cells','No float','no implementation is authorized')) {
  if ($design -notlike "*$required*") { throw "Receiver-arrival geometry design drift: $required" }
}
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @('code-free counterexample oracle','18','26','4/3, 8/3','unsupported_conditional_evidence','point receiver is','face parameter remains contact-only','Do not implement before explicit approval')) {
  if ($result -notlike "*$required*") { throw "Receiver-arrival geometry result drift: $required" }
}
Write-Output 'Receiver-arrival geometry oracle verified: exact-ray strict-interior AABB semantics, 18 portfolios, 26 hostile rejections, typed conditional exclusion and authority boundary are stable.'
