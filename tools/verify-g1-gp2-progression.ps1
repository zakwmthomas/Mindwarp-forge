$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
& (Join-Path $root 'tools\verify-g1-gp2-progression-readiness.ps1')
Push-Location $root
try {
    cargo test -p mindwarp-gameplay-foundation --test gp2_progression
    if ($LASTEXITCODE -ne 0) { throw 'GP2 focused tests failed.' }
    cargo test -p mindwarp-gameplay-foundation --test gp1_base_loop
    if ($LASTEXITCODE -ne 0) { throw 'GP1 retained regression failed.' }
    cargo test -p mindwarp-gameplay-foundation --test gp0_contract
    if ($LASTEXITCODE -ne 0) { throw 'GP0 retained regression failed.' }
} finally { Pop-Location }
Write-Output 'G1 GP2 progression verified: 18 private explicit rules, typed nonfungible lanes, exact authority binding, hostile codec/history checks and rule-derived incomparable strategies pass.'
