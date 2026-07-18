Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$audit = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\G1_C3_OPTICAL_LANE_COUPLING_MEASURE_MATHEMATICAL_DESIGN_AUDIT.md') -Raw
foreach ($required in @('no implementation candidate yet','phase-space measure','NIST defines radiance','dG = n^2 * dA_perpendicular * dOmega','declared scalar lane weight','first-order ray differential','finite correlated boundary lineages','free-space source/receiver solid angle','unsupported_topology_change','unsupported_caustic_or_fold','partial_receiver_coverage','Refining one phase-space cell','corner-only certification','Do not add a crate')) {
  if ($audit -notlike "*$required*") { throw "Optical lane coupling design drift: $required" }
}
foreach ($source in @('tsapps.nist.gov','nist.gov/pml/special-publication-330','doi.org/10.1145/311535.311555','graphics.stanford.edu/papers/trd','doi.org/10.1038/s41598-026-42509-9')) {
  if ($audit -notlike "*$source*") { throw "Optical lane coupling source drift: $source" }
}
if (Test-Path -LiteralPath (Join-Path $root 'crates\optical-lane-coupling-measure')) {
  throw 'Optical lane coupling production crate appeared during design-only work.'
}
Write-Output 'Optical lane coupling design verified: scalar/central-ray shortcuts are rejected, finite correlated boundary lineages are selected for counterexample proof, and no implementation authority is granted.'

