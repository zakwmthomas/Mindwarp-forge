$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$registryPath = Join-Path $root 'governance\record-role-registry.json'
$registry = Get-Content $registryPath -Raw | ConvertFrom-Json
if ($registry.schema_version -ne 1 -or @($registry.rules).Count -eq 0) {
  throw 'Record-role registry is invalid.'
}

$duplicatePatterns = @($registry.rules | Group-Object pattern | Where-Object Count -gt 1)
if ($duplicatePatterns.Count -gt 0) {
  throw "Record-role registry has duplicate patterns: $($duplicatePatterns.Name -join ', ')"
}

function Convert-GlobToRegex([string]$pattern) {
  $escaped = [regex]::Escape($pattern.Replace('\', '/'))
  $escaped = $escaped.Replace('\*\*', '.*').Replace('\*', '[^/]*').Replace('\?', '[^/]')
  return '^' + $escaped + '$'
}

$compiled = @($registry.rules | ForEach-Object {
  [pscustomobject]@{ Pattern = $_.pattern; Role = $_.role; Regex = Convert-GlobToRegex $_.pattern }
})
$ignored = @($registry.ignored_roots)
$unclassified = [System.Collections.Generic.List[string]]::new()
$classified = 0

Get-ChildItem $root -Recurse -Force -File | ForEach-Object {
  $relative = $_.FullName.Substring($root.Length).TrimStart('\', '/').Replace('\', '/')
  $first = $relative.Split('/')[0]
  if ($ignored -contains $first -or $relative -match '(^|/)node_modules/|(^|/)dist/') { return }
  $match = $compiled | Where-Object { $relative -match $_.Regex } | Select-Object -First 1
  if ($null -eq $match) { $unclassified.Add($relative) } else { $classified++ }
}

if ($unclassified.Count -gt 0) {
  throw "Unclassified durable Forge paths:`n$($unclassified -join "`n")"
}

foreach ($requiredRole in @('canonical_checkpoint','canonical_execution_registry','generated_projection','immutable_evidence','canonical_contract','implementation_and_tests','verification_and_operations')) {
  if (@($registry.rules | Where-Object role -eq $requiredRole).Count -eq 0) {
    throw "Record-role registry lacks required role: $requiredRole"
  }
}

Write-Output "Record roles verified: $classified durable files classified by $(@($registry.rules).Count) ordered rules."
