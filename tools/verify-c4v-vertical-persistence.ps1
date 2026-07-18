$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$readiness = Join-Path $root 'docs\canonical-system\G1_C4V_VERTICAL_PERSISTENCE_READINESS.md'
$design = Join-Path $root 'docs\canonical-system\G1_C4V_VERTICAL_PERSISTENCE_DESIGN.md'
$result = Join-Path $root 'docs\canonical-system\G1_C4V_VERTICAL_PERSISTENCE_RESULT.md'
foreach ($path in @($readiness, $design, $result)) {
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "C4V record is missing: $path" }
}
$joined = (Get-Content -LiteralPath $readiness -Raw) + "`n" +
    (Get-Content -LiteralPath $design -Raw) + "`n" +
    (Get-Content -LiteralPath $result -Raw)
foreach ($token in @(
    'VerticalIdentityV1', 'VerticalCommandBatchV1', 'stable stop',
    'retry', 'stale', 'gap', 'fork', 'C3A', 'S1', 'S5',
    'length-framed', 'V1-to-V2', 'byte-identical',
    '4 MiB log', 'world-history-ledger', 'Nothing broader is locked in'
)) {
    if (!$joined.Contains($token)) { throw "C4V evidence is missing: $token" }
}
foreach ($forbidden in @(
    'production storage is implemented', 'runtime persistence is implemented',
    'GP4 is complete', 'C3B authority is granted', 'Greenfield integration is implemented'
)) {
    if ($joined.Contains($forbidden)) { throw "C4V evidence crosses authority: $forbidden" }
}
Push-Location $root
try {
    cargo test -p mindwarp-vertical-persistence
    if ($LASTEXITCODE -ne 0) { throw 'C4V focused tests failed.' }
} finally { Pop-Location }
Write-Output 'G1 C4V vertical persistence verified: seven adversarial tests prove exact C2/C3A/GP1 identity, atomic stable-stop append/retry, semantic restart, framed migration/rollback and read-only receipts.'
