$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
  cargo fmt --check -p mindwarp-signal-anchor-vertical
  if ($LASTEXITCODE -ne 0) { throw 'GP4 formatting verification failed.' }
  cargo clippy -p mindwarp-signal-anchor-vertical --tests --no-deps -- -D warnings
  if ($LASTEXITCODE -ne 0) { throw 'GP4 focused clippy verification failed.' }
  cargo test -p mindwarp-signal-anchor-vertical
  if ($LASTEXITCODE -ne 0) { throw 'GP4 focused test verification failed.' }
  cargo test -p mindwarp-gameplay-foundation
  if ($LASTEXITCODE -ne 0) { throw 'GP4 retained gameplay-foundation verification failed.' }
  cargo test -p mindwarp-vertical-persistence
  if ($LASTEXITCODE -ne 0) { throw 'GP4 retained C4V verification failed.' }
  & (Join-Path $PSScriptRoot 'verify-g1-gp4-signal-anchor-readiness.ps1')
  if (!$?) { throw 'GP4 retained readiness verification failed.' }
  & (Join-Path $PSScriptRoot 'test-g1-vertical-closeout-successor-route.ps1')
  if (!$?) { throw 'GP4 bounded closeout successor route verification failed.' }
  Write-Output 'GP4 Signal Anchor vertical verification passed.'
}
finally { Pop-Location }
