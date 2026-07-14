param([string]$Root)
$ErrorActionPreference = 'Stop'
$root = if ($Root) { $Root } else { Split-Path -Parent $PSScriptRoot }
. (Join-Path $PSScriptRoot 'canonical-text-hash.ps1')
$briefPath = Join-Path $root 'governance\WORKER_FEEDBACK_BRIEF.md'
if (!(Test-Path -LiteralPath $briefPath)) { throw 'Worker feedback brief missing.' }
$brief = Get-Content -LiteralPath $briefPath -Raw
$rows = [regex]::Matches($brief, '\| `(?<path>[^`]+)` \| `(?<hash>[0-9a-f]{64})` \|')
if ($rows.Count -eq 0) { throw 'Worker feedback brief has no source-fixity records.' }
foreach ($row in $rows) {
  $relative = $row.Groups['path'].Value
  $source = Join-Path $root $relative
  if (!(Test-Path -LiteralPath $source)) { throw "Worker feedback source missing: $relative" }
  $actual = Get-CanonicalTextSha256 $source
  if ($actual -ne $row.Groups['hash'].Value) { throw "Stale worker feedback source hash: $relative" }
}
Write-Output "Worker feedback freshness verified: $($rows.Count) source hashes."
