Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$auditPath = Join-Path $root 'docs\canonical-system\G1_C3_POST_WHOLE_CELL_DIMENSIONLESS_TRANSFER_CONSUMER_REASSESSMENT.md'
$audit = Get-Content -LiteralPath $auditPath -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$closure = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_CLOSURE_REGISTER.md') -Raw

foreach ($required in @(
  'source-distribution and phase-space-',
  'Transfer is not brightness, darkness or visibility',
  'parent/child conservation',
  'coordinate reparameterization and Jacobian counterexamples',
  'Detector semantics cannot repair missing source physics',
  'Multi-interface whole-cell',
  'multiple-path/source aggregation and scattering',
  'Nothing broader is locked in. One consumer first, reassess before expanding.',
  'Do not add a',
  'crate, contract schema, dependency, production test or production source',
  'code-facing readiness audit',
  'exact serious owner decision',
  'C3 closure'
)) {
  if ($audit -notlike "*$required*") {
    throw "Post dimensionless-transfer reassessment is missing: $required"
  }
}

$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or
    ($c3[0].next_action -notlike '*code-free source-distribution*phase-space-measure compatibility*' -and
     $c3[0].next_action -notlike '*code-facing source-quantity-basis*schema gap audit*' -and
     $c3[0].next_action -notlike '*source-quantity-basis mathematical design audit*' -and
     $c3[0].next_action -notlike '*calibrated spectral/time basis mathematical design audit*' -and
     $c3[0].next_action -notlike '*calibrated-basis*transport-applicability schema gap audit*' -and
     $c3[0].next_action -notlike '*implementation-readiness audit*source-calibration sibling*' -and
     $c3[0].next_action -notlike '*calibrated source-energy distribution ownership*implementation-readiness audit*' -and
     $c3[0].next_action -notlike '*explicit owner decision*calibrated-source-energy-distribution*' -and
     $c3[0].next_action -notlike '*owner-approved bounded calibrated-source-energy-distribution*' -and
     $c3[0].next_action -notlike '*explicit owner decision*evidence-acquisition*mathematical-design audit*') -or
    $c3[0].sources -notcontains 'G1_C3_POST_WHOLE_CELL_DIMENSIONLESS_TRANSFER_CONSUMER_REASSESSMENT.md') {
  throw 'C3 does not select and retain the bounded source-distribution compatibility audit.'
}

$c3a = @($program.items | Where-Object id -eq 'C3A')
$c3b = @($program.items | Where-Object id -eq 'C3B')
$currentSplit = $c3.Count -eq 1 -and $c3[0].state -eq 'superseded' -and
    $c3[0].proof -like '*does not claim full C3 closure*' -and
    $c3a.Count -eq 1 -and $c3a[0].state -eq 'promoted' -and
    $c3b.Count -eq 1 -and $c3b[0].state -eq 'blocked' -and
    $c3b[0].next_action -like '*physical scale*coefficient*applicability*visibility*presentation fidelity*'

if ($currentSplit) {
  if ($closure -notlike '*Superseded umbrella*C3A is promoted*C3B remains independently evidence-blocked*retained ecotone oracle*proof-tool evidence only*no*C3 closure*') {
    throw 'The master closure register does not retain the superseded C3, promoted C3A and evidence-blocked C3B split.'
  }
} elseif ($closure -notlike '*whole-cell dimensionless transfer*source-distribution*phase-space-measure*' -and
    $closure -notlike '*whole-cell dimensionless transfer*additive source-quantity measure*quantity-basis and schema gap audit*' -and
    $closure -notlike '*Whole-cell dimensionless transfer*source-quantity basis/schema audit*mathematical design audit*' -and
    $closure -notlike '*whole-cell dimensionless transfer*source-quantity oracle*calibrated spectral/time basis mathematical design audit*' -and
    $closure -notlike '*source quantity is band/time-integrated radiant energy*calibration witness*pointwise spectral/time enclosure*' -and
    $closure -notlike '*source quantity and additive calibration witness*source-calibration sibling*implementation readiness*' -and
    $closure -notlike '*calibrated transport-applicability schema-gap audit*scale ambiguity*spectral/time ambiguity*' -and
    $closure -notlike '*physical-evidence*unavailable*material owner decision*' -and
    $closure -notlike '*residual-obligation audit*physical applicability remains evidence-blocked*ecotone*') {
  throw 'The master closure register does not retain the current C3 result and next gap.'
}

Write-Output 'Historical post whole-cell dimensionless-transfer reassessment verified and retained under promoted C3A and evidence-blocked C3B.'
