param(
  [Parameter(Mandatory=$true)][ValidateNotNullOrEmpty()][string]$Query,
  [string]$Project = 'any',
  [string]$Workstream = 'any',
  [string]$Entity = 'any',
  [ValidateSet('any','idea','plan','decision','task','research','correction','philosophy','requirement','constraint','preference','risk','question','observation','context')]
  [string]$Type = 'any',
  [string]$System = 'any',
  [ValidateSet('any','captured_user','assistant','direct_project_user','imported_content','external_tool','system','legacy_unknown')]
  [string]$Actor = 'any',
  [ValidateSet('any','detected','triaged','awaiting_owner','approved','promoted','superseded','rejected','archived')]
  [string]$State = 'any',
  [ValidateRange(1,500)][int]$Limit = 20,
  [string]$Database = ''
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
if ([string]::IsNullOrWhiteSpace($Database)) {
  $Database = Join-Path ([Environment]::GetFolderPath('ApplicationData')) 'com.mindwarp.forge\forge.sqlite3'
}
if (!(Test-Path -LiteralPath $Database)) {
  throw "Forge knowledge database was not found: $Database"
}
$arguments = @($Query, '--database', $Database, '--limit', [string]$Limit)
if ($Project -ne 'any') { $arguments += @('--project', $Project) }
if ($Workstream -ne 'any') { $arguments += @('--workstream', $Workstream) }
if ($Entity -ne 'any') { $arguments += @('--entity', $Entity) }
if ($Type -ne 'any') { $arguments += @('--type', $Type) }
if ($System -ne 'any') { $arguments += @('--system', $System) }
if ($Actor -ne 'any') { $arguments += @('--actor', $Actor) }
if ($State -ne 'any') { $arguments += @('--state', $State) }
$binary = Join-Path $root 'target\debug\forge-query.exe'
if (Test-Path -LiteralPath $binary) {
  $json = & $binary @arguments
} else {
  $json = & cargo run --quiet -p forge-kernel --bin forge-query -- @arguments
}
if ($LASTEXITCODE -ne 0) { throw 'Indexed knowledge query failed.' }
$records = ($json -join "`n") | ConvertFrom-Json
@($records) |
  Select-Object project_refs,workstream_refs,entity_refs,system_refs,lifecycle_state,source_actor,title,summary,source_session_id |
  Format-Table -Wrap -AutoSize
