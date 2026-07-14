$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
if (!(Test-Path $path)) { throw 'Master program registry missing.' }
$program = Get-Content $path -Raw | ConvertFrom-Json
if ($program.schema_version -ne 2 -or @($program.items).Count -eq 0) { throw 'Master program registry is invalid.' }
$allowedStates = @('proposed','researched','designed','ready_for_owner','authorized','executing','blocked','verified','promoted','superseded','rejected','retired')
$compatibility = @{}
foreach ($property in $program.compatibility_projection.mapping.psobject.Properties) {
  $compatibility[$property.Name] = [string]$property.Value
}
if ($program.canonical_state_field -ne 'state' -or $program.compatibility_projection.field -ne 'status' -or $compatibility.Count -eq 0) {
  throw 'Master program lifecycle projection is invalid.'
}
$ids = @($program.items.id)
if (($ids | Group-Object | Where-Object Count -gt 1).Count) { throw 'Master program has duplicate IDs.' }
foreach ($item in $program.items) {
  if ([string]::IsNullOrWhiteSpace($item.id) -or [string]::IsNullOrWhiteSpace($item.next_action) -or [string]::IsNullOrWhiteSpace($item.proof)) { throw "Master program item is incomplete: $($item.id)" }
  if ($item.state -notin $allowedStates) { throw "Master program item has an invalid lifecycle state: $($item.id)" }
  if (!$compatibility.ContainsKey([string]$item.state) -or $compatibility[[string]$item.state] -ne [string]$item.status) {
    throw "Master program compatibility status drifted from canonical state: $($item.id)"
  }
  if (@($item.sources).Count -eq 0) { throw "Master program item has no source evidence: $($item.id)" }
  foreach ($dependency in @($item.depends_on)) { if ($ids -notcontains $dependency) { throw "Unknown dependency $dependency on $($item.id)" } }
}
$active = @($program.items | Where-Object state -eq 'executing')
if ($active.Count -ne 1) { throw 'Master program must have exactly one executing item.' }
& (Join-Path $PSScriptRoot 'refresh-master-plan.ps1') -Check
Write-Output "Master program verified: $(@($program.items).Count) items."
