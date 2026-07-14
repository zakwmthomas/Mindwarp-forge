param(
  [string]$Root,
  [string]$WaitingItem,
  [int]$OwnerWaitWakes = 0
)
$ErrorActionPreference = 'Stop'
$root = if ([string]::IsNullOrWhiteSpace($Root)) { Split-Path -Parent $PSScriptRoot } else { $Root }
$program = Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
$batch = Get-Content (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
$items = @{}; foreach ($item in $program.items) { $items[$item.id] = $item }

if ($OwnerWaitWakes -ge 5) {
  if ([string]::IsNullOrWhiteSpace($WaitingItem)) { throw 'Five-wake fallback requires WaitingItem.' }
  if (!$items.ContainsKey($WaitingItem)) { throw "Waiting item is absent from master program: $WaitingItem" }

  function Test-DescendsFromWaiting([object]$item, [hashtable]$visited) {
    foreach ($dependency in @($item.depends_on)) {
      if ($dependency -eq $WaitingItem) { return $true }
      if (!$visited.ContainsKey($dependency) -and $items.ContainsKey($dependency)) {
        $visited[$dependency] = $true
        if (Test-DescendsFromWaiting $items[$dependency] $visited) { return $true }
      }
    }
    return $false
  }

  $independent = @($program.items | Where-Object {
    $_.id -ne $WaitingItem -and
    $_.gate -eq 'hard' -and
    $_.status -in @('active','planned','partial','ready','dependency_ready') -and
    @($_.depends_on | Where-Object { !$items.ContainsKey($_) -or $items[$_].status -ne 'complete' }).Count -eq 0 -and
    !(Test-DescendsFromWaiting $_ @{})
  } | Sort-Object id)
  if ($independent.Count -eq 0) { throw 'No dependency-ready independent worker item.' }
  $independent[0] | ConvertTo-Json -Compress
  exit
}

if ($batch.state -notin @('complete','blocked') -and $items.ContainsKey($batch.master_program_item) -and $items[$batch.master_program_item].status -eq 'active') { $items[$batch.master_program_item] | ConvertTo-Json -Compress; exit }
$active = @($program.items | Where-Object status -eq 'active')
if ($active.Count -eq 1) { $active[0] | ConvertTo-Json -Compress; exit }
$ready = @($program.items | Where-Object {
  $_.status -in @('active','planned','partial') -and @($_.depends_on | Where-Object { $items[$_].status -ne 'complete' }).Count -eq 0
} | Sort-Object id)
if ($ready.Count -eq 0) { throw 'No dependency-ready worker item.' }
$ready[0] | ConvertTo-Json -Compress
