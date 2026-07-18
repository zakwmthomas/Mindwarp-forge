Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_POST_OPTICAL_LANE_COUPLING_ORACLE_REASSESSMENT.md') -Raw
$oracle = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LANE_COUPLING_MEASURE_ORACLE_RESULT.md') -Raw
foreach ($required in @('finite boundary/corner lineages are insufficient','whole-phase-space-cell interval certification','all-or-nothing receiver coverage','Adaptive point sampling','certified_full_cell_arrival','certified_zero_cell_arrival','partial receiver overlap remains typed unresolved','unresolved children retain their measure','correlation loss','Do not add a crate')) {
  if ($audit -notlike "*$required*") { throw "Post-coupling oracle reassessment drift: $required" }
}
if ($oracle -notlike '*not implementation-ready*' -or $oracle -notlike '*19e9b252a965e5a154d6864a4a426d47015b987a4932d3694ffc04a15d722d84*') { throw 'Rejected coupling oracle is not retained.' }
Write-Output 'Post-coupling oracle reassessment verified: sampling and boundary shortcuts stay rejected; only whole-cell all-or-nothing interval certification advances to code-free design.'

