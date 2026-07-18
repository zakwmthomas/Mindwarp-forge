Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_POST_WHOLE_CELL_RECEIVER_COUPLING_CONSUMER_REASSESSMENT.md') -Raw
$program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json

foreach ($required in @(
  'whole-cell dimensionless transfer uniformity',
  'The operands do not yet describe the same subject',
  'Bind loss before source magnitude',
  'receiver-before-face truncation',
  'subdivision invariance',
  'disposable exact-rational oracle',
  'Add no crate, contract schema, dependency',
  'separate serious owner decision',
  'do not claim source magnitude, radiance, received power',
  'C3 closure'
)) {
  if ($audit -notlike "*$required*") {
    throw "Post receiver-coupling reassessment is missing: $required"
  }
}

$c3 = @($program.items | Where-Object id -eq 'C3')
if ($c3.Count -ne 1 -or
    -not ($c3[0].next_action -like '*whole-cell dimensionless transfer*code-free mathematical design*' -or
          $c3[0].next_action -like '*code-free mathematical design*whole-cell dimensionless transfer*' -or
          ($c3[0].next_action -like '*code-facing readiness/gap audit*whole-cell dimensionless-transfer*' -and
           $c3[0].sources -contains 'G1_C3_WHOLE_CELL_DIMENSIONLESS_TRANSFER_ORACLE_RESULT.md') -or
          $c3[0].next_action -like '*owner-authorized whole-cell dimensionless-transfer implementation*' -or
          $c3[0].next_action -like '*code-free post-result consumer reassessment*whole-cell dimensionless-transfer*' -or
          $c3[0].next_action -like '*code-free source-distribution*phase-space-measure compatibility*' -or
          $c3[0].next_action -like '*code-facing source-quantity-basis*schema gap audit*' -or
          $c3[0].next_action -like '*source-quantity-basis mathematical design audit*' -or
          $c3[0].next_action -like '*calibrated spectral/time basis mathematical design audit*' -or
          $c3[0].next_action -like '*calibrated-basis*transport-applicability schema gap audit*' -or
          $c3[0].next_action -like '*implementation-readiness audit*source-calibration sibling*' -or
          $c3[0].next_action -like '*calibrated source-energy distribution ownership*implementation-readiness audit*' -or
          $c3[0].next_action -like '*explicit owner decision*calibrated-source-energy-distribution*' -or
          $c3[0].next_action -like '*owner-approved bounded calibrated-source-energy-distribution*' -or
          $c3[0].next_action -like '*explicit owner decision*evidence-acquisition*mathematical-design audit*') -or
    $c3[0].sources -notcontains 'G1_C3_POST_WHOLE_CELL_RECEIVER_COUPLING_CONSUMER_REASSESSMENT.md') {
  throw 'C3 does not retain the selected post receiver-coupling code-free route.'
}

Write-Output 'Post whole-cell receiver-coupling reassessment verified: geometry and measure are closed for the bounded subject; whole-cell dimensionless transfer is the next code-free seam.'
