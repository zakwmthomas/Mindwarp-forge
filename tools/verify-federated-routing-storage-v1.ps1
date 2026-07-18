$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
$temporary = Join-Path ([System.IO.Path]::GetTempPath()) ("forge-federated-v1-" + [guid]::NewGuid().ToString('N'))
$database = Join-Path $temporary 'fixture.sqlite3'
$resultPath = Join-Path $root 'docs\canonical-system\G1_FEDERATED_PROJECT_ROUTING_AND_STORAGE_V1_RESULT.md'
if (-not (Test-Path -LiteralPath $resultPath)) { throw 'Federated routing and storage result is missing.' }
$result = Get-Content -LiteralPath $resultPath -Raw
foreach ($required in @(
  'implemented and verified',
  '102 passed',
  'preview-only, approval-required',
  'generic live backfill command has been removed',
  'owner subsequently accepted that independently reproduced receipt',
  'Acceptance requires no live rewrite',
  'receiver-before-face ordering'
)) {
  if ($result -notlike "*$required*") { throw "Federated routing and storage result drift: $required" }
}

New-Item -ItemType Directory -Path (Join-Path $temporary 'target\debug') -Force | Out-Null
try {
  & git init --quiet $temporary
  if ($LASTEXITCODE -ne 0) { throw 'Disposable Git fixture initialization failed.' }
  Set-Content -LiteralPath (Join-Path $temporary 'source.txt') -Value 'managed source' -NoNewline
  Set-Content -LiteralPath (Join-Path $temporary 'target\debug\cache.bin') -Value 'rebuildable cache' -NoNewline

  Push-Location $root
  try {
    & $cargo build -p forge-kernel --bin forge-federate --bin forge-query --bin forge-storage
    if ($LASTEXITCODE -ne 0) { throw 'Federated CLI build failed.' }
    $federate = Join-Path $root 'target\debug\forge-federate.exe'
    $query = Join-Path $root 'target\debug\forge-query.exe'
    $storage = Join-Path $root 'target\debug\forge-storage.exe'

    & $federate backfill-knowledge-disposable $database --disposable-fixture | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Disposable knowledge generation failed.' }
    $savedErrorAction = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    & $federate backfill-knowledge $database - 2>$null | Out-Null
    $genericBackfillExit = $LASTEXITCODE
    $ErrorActionPreference = $savedErrorAction
    if ($genericBackfillExit -eq 0) { throw 'Generic live knowledge backfill verb must remain unavailable.' }

    foreach ($record in @(
      'governance\federation\projects\mindwarp-forge.json',
      'governance\federation\projects\greenfield.json'
    )) {
      & $federate register-project $database (Join-Path $root $record) | Out-Null
      if ($LASTEXITCODE -ne 0) { throw "Project registration failed: $record" }
    }
    foreach ($record in @(
      'governance\federation\workstreams\forge-filing-improvements.json',
      'governance\federation\workstreams\forge-live-mainline.json',
      'governance\federation\workstreams\greenfield-release-readiness.json'
    )) {
      & $federate register-workstream $database (Join-Path $root $record) | Out-Null
      if ($LASTEXITCODE -ne 0) { throw "Workstream registration failed: $record" }
    }
    & $federate route-session $database (Join-Path $root 'governance\federation\session-routes\019f6cc7-f99f-7781-81f9-88ef1d4b5121.json') | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Session route registration failed.' }
    & $federate link-projects $database (Join-Path $root 'governance\federation\links\greenfield-forge-reuse.json') | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Cross-project evidence link registration failed.' }

    $inventory = (& $storage managed-inventory $temporary | Out-String | ConvertFrom-Json)
    if ($inventory.repository_kind -ne 'git') { throw 'Managed inventory did not bind the Git root.' }
    if (@($inventory.files.relative_path) -contains 'target/debug/cache.bin') { throw 'Managed inventory admitted a cache file.' }
    $plan = (& $storage cache-plan $temporary | Out-String | ConvertFrom-Json)
    if (!$plan.approval_required -or $plan.executed -or [string]::IsNullOrWhiteSpace($plan.plan_hash)) {
      throw 'Cleanup preview lost its approval-negative plan binding.'
    }

    & $storage compact-bootstrap $temporary $database | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Compact bootstrap projection failed.' }
    $catalog = Get-Content -Raw -LiteralPath (Join-Path $temporary '.local\forge-bootstrap\KNOWLEDGE_CATALOG.json') | ConvertFrom-Json
    if ($catalog.storage -ne 'sqlite_fts5' -or @($catalog.entries).Count -ne 0) {
      throw 'Bootstrap catalogue is not a compact SQLite pointer.'
    }
    $hits = @((& $query absentterm --database $database --limit 1 | Out-String | ConvertFrom-Json))
    if ($LASTEXITCODE -ne 0) { throw 'Read-only indexed query failed.' }
    if ($hits.Count -ne 0) { throw 'Empty disposable knowledge generation returned a hit.' }
  } finally {
    Pop-Location
  }
} finally {
  if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse -Force }
}

Write-Host 'Federated routing and storage V1 verification passed.'
