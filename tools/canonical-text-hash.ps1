function Get-CanonicalTextSha256([string]$LiteralPath) {
  $utf8 = [Text.UTF8Encoding]::new($false, $true)
  $text = [IO.File]::ReadAllText($LiteralPath, $utf8)
  $normalized = $text.Replace("`r`n", "`n").Replace("`r", "`n")
  $bytes = [Text.UTF8Encoding]::new($false).GetBytes($normalized)
  $sha = [Security.Cryptography.SHA256]::Create()
  try {
    return ([BitConverter]::ToString($sha.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant()
  } finally {
    $sha.Dispose()
  }
}
