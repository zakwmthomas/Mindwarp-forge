Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$python = 'C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
if (-not (Test-Path -LiteralPath $python)) { $python = 'python' }
$oracle = Join-Path $PSScriptRoot 'prove-g1-c3-interval-optical-cell-step.py'
$first = & $python $oracle | Out-String
if ($LASTEXITCODE -ne 0) { throw 'Interval optical cell-step oracle failed.' }
$second = & $python $oracle | Out-String
if ($LASTEXITCODE -ne 0 -or $first -ne $second) { throw 'Interval optical cell-step oracle is nondeterministic.' }
foreach ($required in @(
  '60984da9d8e4353852bc1831532c1026f5b12809325384b1f81e13bc520d7128',
  '"certified_next_face": 248',
  '"ambiguous_next_face": 8',
  '"certified_corner_containment_checks": 15872',
  '"maximum_observed_live_bits": 321',
  '"correlation_erasure_box"',
  '"maximum_coordinate_cell"',
  '"steps": 64',
  'implementation_readiness_audit_not_interval_bulk_or_composer'
)) {
  if ($first -notlike "*$required*") { throw "Interval cell-step receipt drift: $required" }
}
$source = Get-Content -LiteralPath $oracle -Raw
if ($source -match '\b(float|numpy|decimal)\b' -or $source -notmatch 'Fraction') {
  throw 'Interval cell-step oracle lost exact arithmetic isolation.'
}
Write-Output 'Interval optical cell-step oracle verified: 248/256 certified, 8 ambiguous, four 64-step lanes, 321 observed live bits.'
