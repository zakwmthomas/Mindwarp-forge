$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$verifier = Join-Path $PSScriptRoot 'verify-modularity.ps1'
& $verifier -Root $root
if (!$?) { throw 'Live modularity verification failed.' }

function Invoke-FixtureVerifier([string]$fixtureRoot, [string]$policyPath) {
  $oldPreference = $ErrorActionPreference
  $ErrorActionPreference = 'Continue'
  try {
    $output = & powershell -NoProfile -ExecutionPolicy Bypass -File $verifier -Root $fixtureRoot -BoundaryPath $policyPath 2>&1
    return [pscustomobject]@{ ExitCode = $LASTEXITCODE; Text = ($output -join "`n") }
  } finally {
    $ErrorActionPreference = $oldPreference
  }
}

$fixture = Join-Path ([IO.Path]::GetTempPath()) "mindwarp-forge-modularity-$PID"
if (Test-Path $fixture) { Remove-Item -LiteralPath $fixture -Recurse -Force }
New-Item -ItemType Directory -Path (Join-Path $fixture 'a\src'),(Join-Path $fixture 'b\src') -Force | Out-Null
try {
  Set-Content (Join-Path $fixture 'a\src\lib.rs') 'use tauri::Manager;'
  Set-Content (Join-Path $fixture 'b\src\lib.rs') 'use std::process::Command;'
  $policy = [ordered]@{
    schema_version = 1
    verify_cargo_dependencies = $false
    modules = @(
      [ordered]@{ id='module-a'; root='a'; dependencies=@(); source_extensions=@('.rs'); forbidden_imports=@([ordered]@{pattern='\btauri::';reason='desktop capability leak'}) },
      [ordered]@{ id='module-b'; root='b'; dependencies=@(); source_extensions=@('.rs'); forbidden_imports=@([ordered]@{pattern='\bstd::process::';reason='process capability leak'}) }
    )
  }
  $policyPath = Join-Path $fixture 'boundaries.json'; $policy | ConvertTo-Json -Depth 8 | Set-Content $policyPath
  $result = Invoke-FixtureVerifier $fixture $policyPath
  if ($result.ExitCode -eq 0) { throw 'Forbidden-import fixture unexpectedly passed.' }
  $text = $result.Text
  if ($text -notmatch 'module-a: forbidden\s+import' -or $text -notmatch 'module-b: forbidden\s+import') { throw 'Failure isolation did not retain both module violations.' }

  $policy.modules[0].dependencies = @('module-b'); $policy.modules[1].dependencies = @('module-a')
  Set-Content (Join-Path $fixture 'a\src\lib.rs') 'pub fn safe() {}'; Set-Content (Join-Path $fixture 'b\src\lib.rs') 'pub fn safe() {}'
  $policy | ConvertTo-Json -Depth 8 | Set-Content $policyPath
  $result = Invoke-FixtureVerifier $fixture $policyPath
  if ($result.ExitCode -eq 0) { throw 'Dependency-cycle fixture unexpectedly passed.' }
  if ($result.Text -notmatch 'Dependency cycle:') { throw 'Dependency cycle was not reported.' }
} finally {
  if (Test-Path $fixture) { Remove-Item -LiteralPath $fixture -Recurse -Force }
}
Write-Output 'Modularity fixtures verified: forbidden imports, cycle rejection, and isolated multi-module reporting.'
