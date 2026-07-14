$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$coverage = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM_COVERAGE.json') -Raw | ConvertFrom-Json
$ids = @(Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json).items.id
foreach ($source in $coverage.sources) {
  if (!(Test-Path (Join-Path $root $source.path))) { throw "Coverage source missing: $($source.path)" }
  if (@($source.items).Count -eq 0) { throw "Coverage source has no master-program item: $($source.path)" }
  foreach ($id in $source.items) { if ($ids -notcontains $id) { throw "Coverage references unknown master item: $id" } }
}
if (@($coverage.sources | Where-Object coverage -eq 'uncovered').Count -gt 0) { throw 'Master program has uncovered planning sources.' }
Write-Output "Master coverage verified: $(@($coverage.sources).Count) planning sources mapped."
