$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
if (!(Test-Path $path)) { throw 'Master program registry missing.' }
$program = Get-Content $path -Raw | ConvertFrom-Json
if ($program.schema_version -ne 1 -or @($program.items).Count -eq 0) { throw 'Master program registry is invalid.' }
$ids = @($program.items.id)
if (($ids | Group-Object | Where-Object Count -gt 1).Count) { throw 'Master program has duplicate IDs.' }
foreach ($item in $program.items) {
  if ([string]::IsNullOrWhiteSpace($item.id) -or [string]::IsNullOrWhiteSpace($item.next_action) -or [string]::IsNullOrWhiteSpace($item.proof)) { throw "Master program item is incomplete: $($item.id)" }
  foreach ($dependency in @($item.depends_on)) { if ($ids -notcontains $dependency) { throw "Unknown dependency $dependency on $($item.id)" } }
}
$active = @($program.items | Where-Object status -eq 'active')
if ($active.Count -ne 1) { throw 'Master program must have exactly one active item.' }
Write-Output "Master program verified: $(@($program.items).Count) items."
