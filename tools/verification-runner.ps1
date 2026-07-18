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

    & $scriptPath
    $succeeded = $?
    $exitCode = $LASTEXITCODE
    if (!$succeeded) {
        if ($null -eq $exitCode -or $exitCode -eq 0) { $exitCode = 1 }
        throw "Forge verifier failed: $ScriptName (exit $exitCode)"
    }
}
