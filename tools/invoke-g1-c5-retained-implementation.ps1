[CmdletBinding()]
param([string]$TempBase)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourceCommit = '9e48dd117c2b22b62bd31dba15c10c3a9bf4b100'

& (Join-Path $PSScriptRoot 'verify-g1-c5-retained-successor.ps1') -Root $root
if (!$?) { throw 'Current C5 retained-successor classification failed.' }

$tempBasePath = if ([string]::IsNullOrWhiteSpace($TempBase)) { [IO.Path]::GetTempPath() } else { $TempBase }
if (!(Test-Path -LiteralPath $tempBasePath -PathType Container) -or
    (Get-Item -LiteralPath $tempBasePath -Force).Attributes.HasFlag([IO.FileAttributes]::ReparsePoint)) {
    throw 'C5 retained replay temp base is missing or redirected.'
}
$base = [IO.Path]::GetFullPath($tempBasePath).TrimEnd('\') + '\'
$tempRoot = Join-Path $base ('forge-c5-retained-' + [guid]::NewGuid().ToString('N'))
$clone = Join-Path $tempRoot 'source'
try {
    New-Item -ItemType Directory -Path $tempRoot | Out-Null
    & git -c core.hooksPath=NUL clone --quiet --shared --no-checkout $root $clone
    if ($LASTEXITCODE -ne 0) { throw 'Disposable retained C5 clone failed.' }
    & git -c core.hooksPath=NUL -c core.autocrlf=false -c core.eol=lf -C $clone checkout --quiet --detach $sourceCommit
    if ($LASTEXITCODE -ne 0 -or (& git -C $clone rev-parse HEAD).Trim() -ne $sourceCommit) {
        throw 'Disposable retained C5 checkout is not exact.'
    }
    if (@(& git -C $clone status --porcelain).Count -ne 0) { throw 'Retained C5 checkout is dirty before replay.' }
    & powershell.exe -NoProfile -ExecutionPolicy Bypass -File (Join-Path $clone 'tools\verify-g1-c5-closure-readiness.ps1')
    if ($LASTEXITCODE -ne 0) { throw 'Exact retained C5 closure-readiness replay failed.' }
    & powershell.exe -NoProfile -ExecutionPolicy Bypass -File (Join-Path $clone 'tools\verify-g1-c5-significance-scheduler-implementation.ps1')
    if ($LASTEXITCODE -ne 0) { throw 'Exact retained C5 implementation and portability replay failed.' }
    if (@(& git -C $clone status --porcelain).Count -ne 0) { throw 'Retained C5 checkout changed during replay.' }
    Write-Output "C5 retained implementation verified: exact source $sourceCommit replayed after strict current evidence-successor classification."
} finally {
    if (Test-Path -LiteralPath $tempRoot) {
        $resolved = [IO.Path]::GetFullPath($tempRoot)
        if (!$resolved.StartsWith($base, [StringComparison]::OrdinalIgnoreCase) -or $resolved -eq $base.TrimEnd('\')) {
            throw 'Refusing unsafe retained C5 cleanup target.'
        }
        if ((Get-Item -LiteralPath $resolved -Force).Attributes.HasFlag([IO.FileAttributes]::ReparsePoint)) {
            throw 'Refusing redirected retained C5 cleanup target.'
        }
        Remove-Item -LiteralPath $resolved -Recurse -Force -ErrorAction Stop
    }
}
