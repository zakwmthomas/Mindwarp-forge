param(
  [string]$Query = '',
  [ValidateSet('any','idea','plan','decision','task','research','correction','philosophy','requirement','constraint','preference','risk','question','observation','context')]
  [string]$Type = 'any',
  [ValidateSet('any','captured_user','assistant','direct_project_user','imported_content','external_tool','system','legacy_unknown')]
  [string]$Actor = 'any',
  [ValidateRange(1,100)]
  [int]$Limit = 20
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$cataloguePath = Join-Path $root '.local\forge-bootstrap\KNOWLEDGE_CATALOG.json'
if (!(Test-Path -LiteralPath $cataloguePath)) {
  throw 'Typed knowledge catalogue is unavailable. Run tools/ensure-context-current.ps1 with the current Forge build.'
}
$catalogue = Get-Content -LiteralPath $cataloguePath -Raw | ConvertFrom-Json
if ($catalogue.schema_version -ne 2 -or $catalogue.classifier_version -lt 2) {
  throw 'Typed knowledge catalogue is stale or unsupported.'
}
$entries = @($catalogue.entries)
if ($Type -ne 'any') { $entries = @($entries | Where-Object record_type -eq $Type) }
if ($Actor -ne 'any') { $entries = @($entries | Where-Object source_actor -eq $Actor) }
if (![string]::IsNullOrWhiteSpace($Query)) {
  $escaped = [regex]::Escape($Query)
  $entries = @($entries | Where-Object { $_.title -match $escaped -or $_.summary -match $escaped })
}
$entries |
  Sort-Object classifier_confidence -Descending |
  Select-Object -First $Limit record_type,source_actor,classifier_confidence,title,summary,source_evidence_ids |
  Format-Table -Wrap -AutoSize

