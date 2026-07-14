$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$directory = Join-Path $root 'docs\canonical-system'
$registryPath = Join-Path $directory 'system-registry.json'
$required = @('README.md', 'DEPENDENCY_MAP.md', 'PROOF_MATRIX.md', 'UNRESOLVED_GAPS.md', 'SOURCE_AUDIT.md', 'MACRO_GAP_CLOSURE_AUDIT.md', 'system-registry.json')
foreach ($name in $required) {
    if (!(Test-Path -LiteralPath (Join-Path $directory $name))) {
        throw "Canonical system record is missing: $name"
    }
}

$registry = Get-Content -LiteralPath $registryPath -Raw | ConvertFrom-Json
if ($registry.schema_version -ne 1) { throw 'Unsupported canonical-system registry schema.' }
$systems = @($registry.systems)
if ($systems.Count -lt 10) { throw 'Canonical system registry is unexpectedly incomplete.' }
$ids = @($systems.id)
if (($ids | Group-Object | Where-Object Count -gt 1).Count -gt 0) { throw 'Canonical system registry has duplicate IDs.' }
$allowed = @($registry.status_values)
foreach ($system in $systems) {
    if ([string]::IsNullOrWhiteSpace($system.id) -or [string]::IsNullOrWhiteSpace($system.purpose)) {
        throw 'Every canonical system needs an ID and purpose.'
    }
    if ($allowed -notcontains $system.status) { throw "Canonical system $($system.id) has invalid status $($system.status)." }
    if ([string]::IsNullOrWhiteSpace($system.proof)) { throw "Canonical system $($system.id) has no proof requirement." }
    foreach ($dependency in @($system.depends_on)) {
        if ($ids -notcontains $dependency) { throw "Canonical system $($system.id) depends on missing system $dependency." }
    }
}

Write-Output "Canonical system registry verified: $($systems.Count) systems, $(@($systems | Where-Object status -eq 'gated').Count) gated."
