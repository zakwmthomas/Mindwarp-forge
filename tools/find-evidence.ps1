param([Parameter(Mandatory=$true)][string]$Query, [string]$Session, [ValidateSet('user','assistant','any')][string]$Role = 'any')
$root = Split-Path -Parent $PSScriptRoot
$cataloguePath = Join-Path $root '.local\forge-bootstrap\EVIDENCE_CATALOG.json'
if (Test-Path -LiteralPath $cataloguePath) {
    $catalogue = Get-Content $cataloguePath -Raw | ConvertFrom-Json
    $entries = @($catalogue.entries) | Where-Object { $_.search_hint -match [regex]::Escape($Query) }
    if ($Session) { $entries = @($entries | Where-Object session_id -eq $Session) }
    if ($Role -ne 'any') { $entries = @($entries | Where-Object role -eq $Role) }
    $entries | Select-Object -First 20 session_id,message_index,timestamp,role,evidence_id,path,search_hint | Format-Table -Wrap -AutoSize
    exit
}
$files = Get-ChildItem (Join-Path $root '.local\forge-bootstrap\sessions') -Filter '*.md' -File
if ($Session) { $files = @($files | Where-Object BaseName -eq $Session) }
$files | Select-String -Pattern $Query -SimpleMatch | Select-Object -First 20 Path,LineNumber,Line | Format-Table -Wrap -AutoSize
