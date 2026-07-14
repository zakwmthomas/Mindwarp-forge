$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$selector = Join-Path $PSScriptRoot 'select-next-worker-item.ps1'
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$active = @($program.items | Where-Object status -eq 'active')
if ($active.Count -ne 1) { throw 'Selector fixture requires exactly one active master-program item.' }
$expected = $active[0].id
$selected = (& $selector | ConvertFrom-Json)
if ($selected.id -ne $expected) { throw "Selector chose $($selected.id), expected active $expected." }
$batchPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
$original = Get-Content $batchPath -Raw
try {
  $stale = $original | ConvertFrom-Json
  $stale.master_program_item = 'A1'
  $stale.state = 'executing'
  $stale | ConvertTo-Json | Set-Content $batchPath
  $selected = (& $selector | ConvertFrom-Json)
  if ($selected.id -ne $expected) { throw 'Stale batch instruction overrode the active master-program item.' }
  $stale.state = 'complete'
  $stale | ConvertTo-Json | Set-Content $batchPath
  $selected = (& $selector | ConvertFrom-Json)
  if ($selected.id -ne $expected) { throw 'Completed stale batch changed the active master-program selection.' }
} finally { Set-Content $batchPath $original -NoNewline }

$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-worker-selector-$PID"
try {
  New-Item -ItemType Directory -Force -Path (Join-Path $temp 'docs\canonical-system'),(Join-Path $temp 'context\active') | Out-Null
  @{
    schema_version = 1
    items = @(
      @{id='BASE';status='complete';gate='hard';depends_on=@()},
      @{id='WAIT';status='active';gate='hard';depends_on=@('BASE')},
      @{id='CHILD';status='planned';gate='hard';depends_on=@('WAIT')},
      @{id='GRANDCHILD';status='planned';gate='hard';depends_on=@('CHILD')},
      @{id='OWNER';status='active';gate='owner';depends_on=@('BASE')},
      @{id='UNREADY';status='planned';gate='hard';depends_on=@('MISSING')},
      @{id='SAFE';status='planned';gate='hard';depends_on=@('BASE')}
    )
  } | ConvertTo-Json -Depth 8 | Set-Content (Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json')
  @{state='complete';master_program_item='WAIT'} | ConvertTo-Json | Set-Content (Join-Path $temp 'context\active\WORKER_BATCH_STATE.json')
  $fallback = (& $selector -Root $temp -WaitingItem WAIT -OwnerWaitWakes 5 | ConvertFrom-Json)
  if ($fallback.id -ne 'SAFE') { throw "Five-wake fallback selected $($fallback.id), expected SAFE." }
  $rejected = $false
  try { & $selector -Root $temp -WaitingItem WAIT -OwnerWaitWakes 5 | Out-Null } catch { $rejected = $true }
  if ($rejected) { throw 'Five-wake fallback was not deterministic.' }
  $programPath = Join-Path $temp 'docs\canonical-system\MASTER_PROGRAM.json'
  $fixture = Get-Content $programPath -Raw | ConvertFrom-Json
  $fixture.items = @($fixture.items | Where-Object id -ne 'SAFE')
  $fixture | ConvertTo-Json -Depth 8 | Set-Content $programPath
  $noIndependent = $false
  try { & $selector -Root $temp -WaitingItem WAIT -OwnerWaitWakes 5 | Out-Null } catch { $noIndependent = $_.Exception.Message -match 'No dependency-ready independent worker item' }
  if (!$noIndependent) { throw 'Five-wake fallback accepted a gated, descendant, or dependency-incomplete item.' }
} finally { if (Test-Path $temp) { Remove-Item -LiteralPath $temp -Recurse -Force } }
Write-Output "Worker selector fixtures verified: active $expected selection, stale-batch rejection, and dependency-safe five-wake fallback."
