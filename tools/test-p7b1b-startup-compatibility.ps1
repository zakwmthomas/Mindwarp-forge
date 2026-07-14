$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$toolPath = Join-Path $root 'tools\prove-p7b1b-startup-compatibility.ps1'
$receiptPath = Join-Path $root 'evidence\p7b1b\startup-compatibility-proof.json'

foreach ($path in @($toolPath, $receiptPath)) {
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "Startup proof artifact missing: $path" }
}

$source = Get-Content -LiteralPath $toolPath -Raw
foreach ($required in @('--locked', '--offline', '-crt-static', '+crt-static',
        'p7b1b-startup-proof\dynamic', 'p7b1b-startup-proof\static', 'dumpbin.exe',
        'prototype_tested_not_executed', 'canary_executed = $false',
        'lpac_compatibility_proved = $false', 'ReparsePoint', 'beforeCanary', 'afterCanary')) {
    if (!$source.Contains($required)) { throw "Startup proof tool is missing shield: $required" }
}
foreach ($forbidden in @('run-once', '--owner-authorized-single-run', 'Start-Process',
        'Invoke-Expression', 'CreateAppContainerProfile', 'ResumeThread', 'CheckNetIsolation')) {
    if ($source.Contains($forbidden)) { throw "Startup proof tool crosses the no-execution boundary: $forbidden" }
}

$receipt = Get-Content -LiteralPath $receiptPath -Raw | ConvertFrom-Json
if ($receipt.schema -ne 1 -or $receipt.status -ne 'prototype_tested_not_executed') {
    throw 'Startup proof receipt has the wrong schema or claim boundary.'
}
foreach ($field in @('canary_executed', 'lpac_profile_created', 'capability_added', 'lpac_compatibility_proved')) {
    if ($receipt.$field -ne $false) { throw "Startup proof receipt overclaims or crossed authority: $field" }
}
foreach ($variant in @($receipt.dynamic, $receipt.static)) {
    if ($variant.pe_magic -ne '0x20b' -or !$variant.appcontainer -or
        !$variant.high_entropy_aslr -or !$variant.dynamic_base -or !$variant.nx_compatible) {
        throw 'A startup proof variant lost required PE security properties.'
    }
}
if ($receipt.dynamic.imports -notcontains 'vcruntime140.dll' -or
    @($receipt.dynamic.imports | Where-Object { $_ -like 'api-ms-win-crt-*' }).Count -eq 0) {
    throw 'Dynamic baseline lacks its expected CRT comparison surface.'
}
if ($receipt.static.imports -contains 'vcruntime140.dll' -or
    @($receipt.static.imports | Where-Object { $_ -like 'api-ms-win-crt-*' }).Count -ne 0) {
    throw 'Static candidate retains a dynamic CRT dependency.'
}
foreach ($import in $receipt.static.imports) {
    if ($receipt.dynamic.imports -notcontains $import) { throw "Static import surface broadened: $import" }
}
if (!$receipt.normal_workspace.unchanged -or
    $receipt.normal_workspace.canary_sha256_before -ne $receipt.normal_workspace.canary_sha256_after -or
    $receipt.normal_workspace.runner_sha256_before -ne $receipt.normal_workspace.runner_sha256_after) {
    throw 'Startup proof did not preserve normal workspace binaries.'
}

$escapedPath = Join-Path $root 'evidence\p7b1b-escape.json'
if (Test-Path -LiteralPath $escapedPath) { throw 'Negative-fixture output unexpectedly exists before the test.' }
$priorErrorPreference = $ErrorActionPreference
$ErrorActionPreference = 'Continue'
$output = & powershell -NoProfile -ExecutionPolicy Bypass -File $toolPath -OutputPath $escapedPath 2>&1
$childExit = $LASTEXITCODE
$ErrorActionPreference = $priorErrorPreference
if ($childExit -eq 0 -or ($output -join "`n") -notmatch 'must be JSON under') {
    throw 'Startup proof tool did not fail closed on a prefix-confusion output path.'
}
if (Test-Path -LiteralPath $escapedPath) { throw 'Rejected startup proof escaped its evidence boundary.' }

Write-Output 'P7b-1b startup compatibility verified: isolated offline proof is non-executing, bounded, import-reducing, security-flag preserving, workspace-preserving, and claim-limited.'
