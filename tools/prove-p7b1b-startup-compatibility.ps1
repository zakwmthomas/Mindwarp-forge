param(
    [string]$OutputPath = 'evidence\p7b1b\startup-compatibility-proof.json'
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$vcvars = 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat'
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
$rustc = Join-Path $env:USERPROFILE '.cargo\bin\rustc.exe'
$target = 'x86_64-pc-windows-msvc'
$dynamicTarget = Join-Path $root 'target\p7b1b-startup-proof\dynamic'
$staticTarget = Join-Path $root 'target\p7b1b-startup-proof\static'
$normalCanary = Join-Path $root 'target\debug\containment-denial-canary.exe'
$normalRunner = Join-Path $root 'target\debug\containment-canary-runner.exe'
$output = if ([IO.Path]::IsPathRooted($OutputPath)) {
    [IO.Path]::GetFullPath($OutputPath)
} else {
    [IO.Path]::GetFullPath((Join-Path $root $OutputPath))
}
$evidenceRoot = [IO.Path]::GetFullPath((Join-Path $root 'evidence\p7b1b'))
$evidencePrefix = $evidenceRoot + [IO.Path]::DirectorySeparatorChar

if (!$output.StartsWith($evidencePrefix, [StringComparison]::OrdinalIgnoreCase) -or
    [IO.Path]::GetExtension($output) -ne '.json') {
    throw 'Startup-compatibility proof must be JSON under evidence\p7b1b.'
}
foreach ($path in @($vcvars, $cargo, $rustc, $normalCanary, $normalRunner)) {
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Required local build input missing: $path"
    }
}
if (Test-Path -LiteralPath $output) {
    $outputItem = Get-Item -LiteralPath $output -Force
    if (($outputItem.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
        throw 'Startup-compatibility proof output cannot be a reparse point.'
    }
}
$cursor = Split-Path -Parent $output
while ($cursor.StartsWith($evidenceRoot, [StringComparison]::OrdinalIgnoreCase)) {
    if (Test-Path -LiteralPath $cursor) {
        $item = Get-Item -LiteralPath $cursor -Force
        if (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "Startup-compatibility proof ancestor is a reparse point: $cursor"
        }
    }
    if ($cursor.Equals($evidenceRoot, [StringComparison]::OrdinalIgnoreCase)) { break }
    $cursor = Split-Path -Parent $cursor
}

function Get-Sha256([string]$Path) {
    (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Get-PeEvidence([string]$Path) {
    $bytes = [IO.File]::ReadAllBytes($Path)
    if ($bytes.Length -lt 0x100 -or
        $bytes[0] -ne [byte][char]'M' -or
        $bytes[1] -ne [byte][char]'Z') {
        throw "Not a bounded PE image: $Path"
    }
    $pe = [BitConverter]::ToInt32($bytes, 0x3c)
    if ($pe -lt 0 -or $pe + 96 -ge $bytes.Length -or
        [Text.Encoding]::ASCII.GetString($bytes, $pe, 4) -ne "PE`0`0") {
        throw "Invalid PE header: $Path"
    }
    $optional = $pe + 24
    $magic = [BitConverter]::ToUInt16($bytes, $optional)
    $dll = [BitConverter]::ToUInt16($bytes, $optional + 70)
    [ordered]@{
        sha256 = Get-Sha256 $Path
        bytes = $bytes.Length
        pe_magic = ('0x{0:x}' -f $magic)
        dll_characteristics = ('0x{0:x}' -f $dll)
        appcontainer = (($dll -band 0x1000) -ne 0)
        high_entropy_aslr = (($dll -band 0x20) -ne 0)
        dynamic_base = (($dll -band 0x40) -ne 0)
        nx_compatible = (($dll -band 0x100) -ne 0)
    }
}

function Get-Imports([string]$Path) {
    $raw = & dumpbin.exe /dependents $Path
    if ($LASTEXITCODE -ne 0) {
        throw "dumpbin dependency inspection failed: $Path"
    }
    @($raw | ForEach-Object {
        if ($_ -match '^\s+([A-Za-z0-9._-]+\.dll)\s*$') {
            $matches[1].ToLowerInvariant()
        }
    } | Sort-Object -Unique)
}

function Assert-PePolicy($Evidence, [string]$Kind) {
    if ($Evidence.pe_magic -ne '0x20b' -or
        !$Evidence.appcontainer -or
        !$Evidence.high_entropy_aslr -or
        !$Evidence.dynamic_base -or
        !$Evidence.nx_compatible) {
        throw "$Kind candidate lost a required PE security property."
    }
}

$beforeCanary = Get-Sha256 $normalCanary
$beforeRunner = Get-Sha256 $normalRunner
$environment = cmd.exe /c "call `"$vcvars`" >nul && set"
foreach ($line in $environment) {
    if ($line -match '^(.*?)=(.*)$') {
        Set-Item -Path "env:$($matches[1])" -Value $matches[2]
    }
}
$env:VSLANG = '1033'
$dumpbin = (Get-Command dumpbin.exe -ErrorAction Stop).Source

$env:CARGO_TARGET_DIR = $dynamicTarget
$env:RUSTFLAGS = '-D warnings -C target-feature=-crt-static'
& $cargo build --locked --offline --target $target -p containment-denial-canary
if ($LASTEXITCODE -ne 0) { throw 'Isolated dynamic-CRT canary build failed.' }

$env:CARGO_TARGET_DIR = $staticTarget
$env:RUSTFLAGS = '-D warnings -C target-feature=+crt-static'
& $cargo build --locked --offline --target $target -p containment-denial-canary
if ($LASTEXITCODE -ne 0) { throw 'Isolated static-CRT canary build failed.' }

$dynamicExe = Join-Path $dynamicTarget "$target\debug\containment-denial-canary.exe"
$staticExe = Join-Path $staticTarget "$target\debug\containment-denial-canary.exe"
$dynamic = Get-PeEvidence $dynamicExe
$static = Get-PeEvidence $staticExe
$dynamic.imports = @(Get-Imports $dynamicExe)
$static.imports = @(Get-Imports $staticExe)
Assert-PePolicy $dynamic 'Dynamic'
Assert-PePolicy $static 'Static'

$requiredSystemImports = @('advapi32.dll', 'kernel32.dll', 'ntdll.dll', 'ws2_32.dll')
foreach ($required in $requiredSystemImports) {
    if ($static.imports -notcontains $required) {
        throw "Static candidate lost required system import: $required"
    }
}
if ($dynamic.imports -notcontains 'vcruntime140.dll' -or
    @($dynamic.imports | Where-Object { $_ -like 'api-ms-win-crt-*' }).Count -eq 0) {
    throw 'Dynamic baseline did not expose the expected CRT import surface.'
}
if ($static.imports -contains 'vcruntime140.dll' -or
    @($static.imports | Where-Object { $_ -like 'api-ms-win-crt-*' }).Count -ne 0) {
    throw 'Static candidate retains a forbidden dynamic CRT import.'
}
foreach ($import in $static.imports) {
    if ($dynamic.imports -notcontains $import) {
        throw "Static candidate broadened the direct import surface: $import"
    }
}

$afterCanary = Get-Sha256 $normalCanary
$afterRunner = Get-Sha256 $normalRunner
if ($beforeCanary -ne $afterCanary -or $beforeRunner -ne $afterRunner) {
    throw 'Isolated proof changed a normal workspace binary.'
}

$rustVersion = (& $rustc -Vv) -join "`n"
$receipt = [ordered]@{
    schema = 1
    status = 'prototype_tested_not_executed'
    target = $target
    rustc = $rustVersion
    proof_tool_sha256 = Get-Sha256 $PSCommandPath
    dumpbin_sha256 = Get-Sha256 $dumpbin
    cargo_lock_sha256 = Get-Sha256 (Join-Path $root 'Cargo.lock')
    canary_source_sha256 = Get-Sha256 (Join-Path $root 'crates\containment-denial-canary\src\main.rs')
    canary_build_script_sha256 = Get-Sha256 (Join-Path $root 'crates\containment-denial-canary\build.rs')
    dynamic = $dynamic
    static = $static
    dynamic_rustflags = '-D warnings -C target-feature=-crt-static'
    static_rustflags = '-D warnings -C target-feature=+crt-static'
    normal_workspace = [ordered]@{
        canary_sha256_before = $beforeCanary
        canary_sha256_after = $afterCanary
        runner_sha256_before = $beforeRunner
        runner_sha256_after = $afterRunner
        unchanged = $true
    }
    canary_executed = $false
    lpac_profile_created = $false
    capability_added = $false
    lpac_compatibility_proved = $false
}
$json = $receipt | ConvertTo-Json -Depth 8
if ([Text.Encoding]::UTF8.GetByteCount($json) -gt 8192) {
    throw 'Startup-compatibility proof exceeds the fixed receipt bound.'
}
if (Test-Path -LiteralPath $output) {
    $existing = Get-Content -LiteralPath $output -Raw
    if ($existing.TrimEnd() -ne $json.TrimEnd()) {
        throw 'Existing startup-compatibility proof conflicts with fresh evidence.'
    }
} else {
    [IO.Directory]::CreateDirectory((Split-Path -Parent $output)) | Out-Null
    [IO.File]::WriteAllText($output, "$json`n", [Text.UTF8Encoding]::new($false))
}

Write-Output $json
