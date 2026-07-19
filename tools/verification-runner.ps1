$ErrorActionPreference = 'Stop'

function Invoke-ForgeVerifier {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)][string]$ScriptRoot,
        [Parameter(Mandatory = $true)][string]$ScriptName
    )

    $scriptPath = Join-Path $ScriptRoot $ScriptName
    if (!(Test-Path -LiteralPath $scriptPath -PathType Leaf)) {
        throw "Forge verifier is unavailable: $ScriptName"
    }

    $retainedC4Verifiers = @(
        'verify-g1-c4-independent-platform-result.ps1',
        'test-g1-c4-independent-platform-result.ps1',
        'verify-g1-c4-closure-readiness.ps1',
        'verify-g1-c4-hierarchy-history-implementation.ps1'
    )
    if ($ScriptName -in $retainedC4Verifiers) {
        $canonicalTools = [IO.Path]::GetFullPath($PSScriptRoot)
        if ([IO.Path]::GetFullPath($ScriptRoot) -ne $canonicalTools) {
            throw "Retained C4 verifier requires the canonical tools root: $ScriptName"
        }
        & (Join-Path $canonicalTools 'invoke-g1-c4-retained-successor-adapter.ps1') -ScriptName $ScriptName
    } else {
        & $scriptPath
    }
    $succeeded = $?
    $exitCode = $LASTEXITCODE
    if (!$succeeded) {
        if ($null -eq $exitCode -or $exitCode -eq 0) { $exitCode = 1 }
        throw "Forge verifier failed: $ScriptName (exit $exitCode)"
    }
}
