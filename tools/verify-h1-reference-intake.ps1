$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Add-Type -AssemblyName System.IO.Compression.FileSystem

$survival = Join-Path $root 'forge documents from gpt handover\MINDWARP_FORGE_CONTINUATION_SURVIVAL_PACK_2026-07-12.zip'
$emergency = Join-Path $root 'forge documents from gpt handover\MINDWARP_FORGE_ANDROID_BOOTSTRAP_EMERGENCY_PACK_2026-07-12.zip'
$expectedArchives = @{
  $survival = 'f0f01b7469226d3d5c77780c23e97a96342b517ece536bc0351e9486117b251b'
  $emergency = '36aab7f95731fcff0cb400c4cf807b5167227e2ef3942e247bcbd403a3689d50'
}
foreach ($path in $expectedArchives.Keys) {
  if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "H1 recovered archive is missing: $path" }
  $actual = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLowerInvariant()
  if ($actual -ne $expectedArchives[$path]) { throw "H1 recovered archive fixity drifted: $path" }
}

$prefix = 'MINDWARP_FORGE_CONTINUATION_SURVIVAL_PACK/07_LEGACY_REPORTS/'
$entries = [ordered]@{
  ($prefix + 'one_button_humanoid_blueprint.json') = @{ bytes = 5430; sha256 = '74b23331be5291bf399cd4d4b364059de7ab4d305569e19e7090470f73502491' }
  ($prefix + 'one_button_humanoid_vertical_slice_report.json') = @{ bytes = 95288; sha256 = '3027c877ff79007a90385b0ad3fdde0dcc59c42c91e0fdef9c90a005bcd21dff' }
  ($prefix + 'asset_category_contracts.json') = @{ bytes = 6205; sha256 = '1fe453c604ce855c2046707ae55fc71c782d2f5bf8b0dd70847aab4b6555eac0' }
}

function Get-EntryReceipt([string]$ArchivePath, [string]$EntryName) {
  $archive = [IO.Compression.ZipFile]::OpenRead($ArchivePath)
  try {
    $entry = $archive.GetEntry($EntryName)
    if ($null -eq $entry) { throw "H1 archive entry is missing: $EntryName" }
    $stream = $entry.Open()
    $sha = [Security.Cryptography.SHA256]::Create()
    try {
      $hash = ([BitConverter]::ToString($sha.ComputeHash($stream))).Replace('-', '').ToLowerInvariant()
      return [pscustomobject]@{ Bytes = $entry.Length; Sha256 = $hash }
    } finally { $sha.Dispose(); $stream.Dispose() }
  } finally { $archive.Dispose() }
}

foreach ($name in $entries.Keys) {
  $expected = $entries[$name]
  $primary = Get-EntryReceipt $survival $name
  $duplicate = Get-EntryReceipt $emergency $name
  if ($primary.Bytes -ne $expected.bytes -or $primary.Sha256 -ne $expected.sha256) { throw "H1 primary entry drifted: $name" }
  if ($duplicate.Bytes -ne $primary.Bytes -or $duplicate.Sha256 -ne $primary.Sha256) { throw "H1 emergency duplicate is no longer byte-identical: $name" }
}

$archive = [IO.Compression.ZipFile]::OpenRead($survival)
try {
  $assetEntries = @($archive.Entries | Where-Object FullName -Match '(?i)\.(obj|fbx|gltf|glb|blend|dae|stl|ply|png|jpe?g|webp|svg)$')
  if ($assetEntries.Count -ne 0) { throw 'H1 survival pack unexpectedly contains model or image assets requiring separate intake.' }
} finally { $archive.Dispose() }

$source = Get-Content -LiteralPath (Join-Path $root 'crates\reference-intake\src\lib.rs') -Raw
foreach ($required in @('duplicate reference evidence','reference provenance exceeds its authority','CanonicalBaseline','NumericAcceptanceThreshold','C:/absolute.json','https://example.invalid','minimal_suite_is_deterministic_diverse_and_authority_negative')) {
  if (!$source.Contains($required)) { throw "H1 intake implementation is missing: $required" }
}
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\reference-intake-contract.md') -Raw
$result = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\H1_REFERENCE_INTAKE_RESULT.md') -Raw
foreach ($required in @('no weights','byte-identical','evidence_only','no recovered mesh','0.803','2.26 ms','licensed external visual humanoid target')) {
  if (!$contract.ToLowerInvariant().Contains($required) -and !$result.ToLowerInvariant().Contains($required)) { throw "H1 intake record is missing: $required" }
}
Write-Output 'H1 reference intake verified: authoritative archive fixity, duplicate provenance, no recovered asset, minimal diverse suite, and authority-negative validation retained.'
