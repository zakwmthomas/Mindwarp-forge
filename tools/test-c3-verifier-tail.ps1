$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'verification-runner.ps1')
$source = Get-Content -LiteralPath (Join-Path $PSScriptRoot 'verify.ps1') -Raw
$names = @([regex]::Matches($source, "Invoke-ForgeVerifier -ScriptRoot \`$PSScriptRoot -ScriptName '([^']+)'") | ForEach-Object { $_.Groups[1].Value })
$start = [array]::IndexOf($names, 'verify-g1-c3-post-optical-lineage-reassessment.ps1')
$end = [array]::IndexOf($names, 'verify-g1-c3-cross-boundary-ecotone-oracle.ps1')
if ($start -lt 0 -or $end -lt $start) { throw 'C3 verifier tail is missing or out of order.' }
foreach ($name in $names[$start..$end]) {
    Write-Output "C3 tail: $name"
    Invoke-ForgeVerifier -ScriptRoot $PSScriptRoot -ScriptName $name
}
Write-Output "C3 verifier tail passed: $($end - $start + 1) ordered checks."
