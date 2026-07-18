$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
  & cargo test -p forge-kernel --lib federation --quiet
  if ($LASTEXITCODE -ne 0) { throw 'Federation and storage tests failed.' }
  & cargo build -p forge-kernel --bins --quiet
  if ($LASTEXITCODE -ne 0) { throw 'Federation/query/storage CLIs did not build.' }

  $records = @(
    'governance/federation/projects/mindwarp-forge.json',
    'governance/federation/projects/greenfield.json',
    'governance/federation/workstreams/greenfield-release-readiness.json',
    'governance/federation/workstreams/forge-filing-improvements.json',
    'governance/federation/workstreams/forge-live-mainline.json',
    'governance/federation/session-routes/019f6cc7-f99f-7781-81f9-88ef1d4b5121.json',
    'governance/federation/links/greenfield-forge-reuse.json'
  )
  foreach ($record in $records) {
    if (!(Test-Path -LiteralPath $record)) { throw "Missing federation record: $record" }
    $null = Get-Content -LiteralPath $record -Raw | ConvertFrom-Json
  }

  $catalog = Get-Item -LiteralPath '.local/forge-bootstrap/KNOWLEDGE_CATALOG.json'
  $binding = Get-Item -LiteralPath '.local/forge-workspace-binding.json'
  if ($catalog.Length -gt 65536) { throw 'Knowledge catalogue is not a compact SQLite pointer.' }
  if ($binding.Length -gt 1048576) { throw 'Managed workspace binding exceeds 1 MiB.' }
  $catalogue = Get-Content -LiteralPath $catalog.FullName -Raw | ConvertFrom-Json
  if ($catalogue.storage -ne 'sqlite_fts5' -or @($catalogue.entries).Count -ne 0) {
    throw 'Knowledge catalogue does not route to SQLite FTS5.'
  }

  $database = Join-Path ([Environment]::GetFolderPath('ApplicationData')) 'com.mindwarp.forge\forge.sqlite3'
  if (Test-Path -LiteralPath $database) {
    $project = 'project-56b2c4780da98bfbaf216613c281ec28e2eb61a2a22733dee34c0efcba357349'
    $timer = [Diagnostics.Stopwatch]::StartNew()
    $result = & '.\target\debug\forge-query.exe' Android --database $database --project $project --limit 3
    $timer.Stop()
    if ($LASTEXITCODE -ne 0) { throw 'Greenfield indexed query failed.' }
    $hits = ($result -join "`n") | ConvertFrom-Json
    if (@($hits).Count -eq 0 -or @($hits)[0].source_session_id -ne '019f6cc7-f99f-7781-81f9-88ef1d4b5121') {
      throw 'Greenfield session is not routed through its explicit project index.'
    }
    if ($timer.ElapsedMilliseconds -gt 1000) { throw "Indexed query exceeded 1 second: $($timer.ElapsedMilliseconds) ms" }
  }

  $plan = (& '.\target\debug\forge-storage.exe' cache-plan $root) -join "`n" | ConvertFrom-Json
  if (!$plan.approval_required -or $plan.executed) { throw 'Cache plan crossed the preview-only boundary.' }
  if (!(Test-Path -LiteralPath '.local/forge-bootstrap/sessions/019f6cc7-f99f-7781-81f9-88ef1d4b5121.md')) {
    throw 'Exact Greenfield conversation projection is missing.'
  }
  Write-Output "Federated continuity verified: explicit Greenfield routing, indexed retrieval, compact projections, lossless archive proof, and preview-only cleanup."
} finally {
  Pop-Location
}
