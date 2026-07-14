$ErrorActionPreference = 'Stop'
$source = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'ensure-context-current.ps1') -Raw

$workspaceRelease = '$workspaceReleaseApp'
$workspaceDebug = '$workspaceDebugApp'
$legacyRelease = '$legacyReleaseApp'
$legacyDebug = '$legacyDebugApp'
$selection = '@($workspaceReleaseApp, $workspaceDebugApp, $legacyReleaseApp, $legacyDebugApp)'

foreach ($required in @(
  "'target\release\forge-desktop.exe'",
  "'target\debug\forge-desktop.exe'",
  "'apps\forge-desktop\src-tauri\target\release\forge-desktop.exe'",
  "'apps\forge-desktop\src-tauri\target\debug\forge-desktop.exe'",
  $selection,
  'Sort-Object LastWriteTimeUtc -Descending',
  'Select-Object -First 1 -ExpandProperty FullName'
)) {
  if (!$source.Contains($required)) { throw "Forge launch path resolution is missing: $required" }
}
if ($source.IndexOf($workspaceRelease) -gt $source.IndexOf($legacyRelease) -or
    $source.IndexOf($workspaceDebug) -gt $source.IndexOf($legacyDebug)) {
  throw 'Workspace Cargo outputs must be preferred over legacy member-local outputs.'
}
Write-Output 'Forge context launch paths verified: newest workspace or legacy Cargo output is selected deterministically.'
