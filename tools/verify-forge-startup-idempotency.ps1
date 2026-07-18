$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$desktopSourcePath = Join-Path $root 'apps\forge-desktop\src-tauri\src\main.rs'
$contextGatePath = Join-Path $root 'tools\ensure-context-current.ps1'
$desktopSource = Get-Content -LiteralPath $desktopSourcePath -Raw
$contextGate = Get-Content -LiteralPath $contextGatePath -Raw

foreach ($forbidden in @(
  'launch_codex_workspace',
  '.arg("app").arg(project_root)',
  '.join("OpenAI").join("Codex")',
  'get.microsoft.com/installer/download',
  'ChatGPT Installer'
)) {
  if ($desktopSource.Contains($forbidden)) {
    throw "Forge desktop startup retains forbidden assistant-launch or installer behavior: $forbidden"
  }
}

foreach ($forbidden in @(
  'codex.exe',
  'ChatGPT.exe',
  'get.microsoft.com/installer/download',
  '9PLM9XGG6VKS'
)) {
  if ($contextGate.Contains($forbidden)) {
    throw "Forge context gate retains forbidden assistant-launch or installer behavior: $forbidden"
  }
}

foreach ($required in @(
  'verify-forge-startup-idempotency.ps1',
  '$desktopStartupSource',
  'Forge Desktop is older than its startup source',
  'Start-Process -FilePath $app'
)) {
  if (!$contextGate.Contains($required)) {
    throw "Forge context gate is missing a startup idempotency shield: $required"
  }
}

Write-Output 'Forge startup idempotency verified: Forge may launch only Forge, rejects stale startup binaries, and does not launch or install an assistant client.'
