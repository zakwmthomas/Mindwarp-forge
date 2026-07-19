$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$temp = Join-Path ([IO.Path]::GetTempPath()) ('forge-c4-successor-route-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temp | Out-Null
try {
  $program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
  $checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
  $closeout = @($program.items | Where-Object id -eq 'G1-VERTICAL-CLOSEOUT')[0]
  $c4 = @($program.items | Where-Object id -eq 'C4')[0]
  $closeout.state = 'verified'; $closeout.status = 'complete'
  $c4.state = 'executing'; $c4.status = 'active'
  $checkpoint.batch_id = 'G1-C4-HIERARCHY-HISTORY-CLOSURE-V1'
  $checkpoint.master_program_item = 'C4'
  $checkpoint.state = 'executing'
  $checkpoint.substage_id = 'c4-reconciliation-readiness'
  $checkpoint.authority_lane = 'Owner-authorized broad C4 hierarchy/history reconciliation and capability-free closure proof only. Exact dependencies C2 and C3A. No C3B, C5, C6, C7, broad G1 closure, runtime, storage engine, filesystem, network, multiplayer, cross-target transactions, Companion, Greenfield, visual assets or Kernel mutation.'
  $checkpoint.verification_receipts = @('registered-full-gate:run-7e5c44dc8f48424a8cec42da756e3127:passed','receipt:G1-VERTICAL-CLOSEOUT:recorded')
  $programPath = Join-Path $temp 'program.json'
  $checkpointPath = Join-Path $temp 'checkpoint.json'
  $resultPath = Join-Path $temp 'result.md'
  function Save-Fixture {
    $program | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $programPath -Encoding utf8
    $checkpoint | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $checkpointPath -Encoding utf8
    Set-Content -LiteralPath $resultPath -Value 'GP4 registered proof run-7e5c44dc8f48424a8cec42da756e3127' -Encoding utf8
  }
  function Assert-Pass {
    Save-Fixture
    if ((& (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -ne $true) { throw 'Valid C4 successor route was rejected.' }
    & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ResultPath $resultPath -RouteOnly | Out-Null
    & (Join-Path $root 'tools\verify-g1-gp3-encounter-grammar-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath | Out-Null
    & (Join-Path $root 'tools\verify-g1-c4-closure-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -RouteOnly | Out-Null
  }
  Assert-Pass
  $checkpoint.substage_id = 'c4-independent-platform-gate'
  Assert-Pass
  $checkpoint.substage_id = 'c4-reconciliation-readiness'
  foreach ($field in @('batch_id','master_program_item','substage_id','authority_lane')) {
    $saved = $checkpoint.$field; $checkpoint.$field = 'forged'
    if ((& (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -eq $true) { throw "Forged C4 $field was admitted." }
    $checkpoint.$field = $saved
  }
  $baseAuthority = $checkpoint.authority_lane
  foreach ($token in @('Exact dependencies C2 and C3A','No C3B','C5','C6','C7','broad G1 closure','runtime','storage engine','filesystem','network','multiplayer','cross-target transactions','Companion','Greenfield','visual assets','Kernel mutation')) {
    $checkpoint.authority_lane = $baseAuthority.Replace($token,'')
    if ((& (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -eq $true) { throw "C4 authority omission was admitted: $token" }
  }
  $checkpoint.authority_lane = $baseAuthority
  $savedDeps = @($c4.depends_on); $c4.depends_on = @('C3A'); Save-Fixture
  try { & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ResultPath $resultPath -RouteOnly | Out-Null; throw 'Missing C2 dependency was admitted.' } catch { if ($_.Exception.Message -eq 'Missing C2 dependency was admitted.') { throw } }
  $c4.depends_on = $savedDeps
  foreach($forgedDeps in @(@('C2','C3A','C3B'),@('C3A','C2'))){$c4.depends_on=$forgedDeps;Save-Fixture;try{& (Join-Path $root 'tools\verify-g1-c4-closure-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -RouteOnly|Out-Null;throw 'Forged C4 dependency set was admitted.'}catch{if($_.Exception.Message-eq'Forged C4 dependency set was admitted.'){throw}}};$c4.depends_on=$savedDeps
  foreach($id in @('C2','C3A','C4V','G1-VERTICAL-CLOSEOUT')){$item=@($program.items|Where-Object id -eq $id)[0];$saved=$item.proof;$item.proof='forged';Save-Fixture;try{& (Join-Path $root 'tools\verify-g1-c4-closure-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -RouteOnly|Out-Null;throw "Forged $id prerequisite was admitted."}catch{if($_.Exception.Message-eq"Forged $id prerequisite was admitted."){throw}};$item.proof=$saved}
  $c5=@($program.items|Where-Object id -eq 'C5')[0];$savedC5State=$c5.state;$savedC5Status=$c5.status;$c5.state='executing';$c5.status='active';Save-Fixture;try{& (Join-Path $root 'tools\verify-g1-c4-closure-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -RouteOnly|Out-Null;throw 'Extra active C5 was admitted.'}catch{if($_.Exception.Message-eq'Extra active C5 was admitted.'){throw}};$c5.state=$savedC5State;$c5.status=$savedC5Status
  $savedCloseout = $closeout.status; $closeout.status = 'active'; Save-Fixture
  try { & (Join-Path $root 'tools\verify-g1-gp3-encounter-grammar-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath | Out-Null; throw 'Unclosed vertical receipt was admitted.' } catch { if ($_.Exception.Message -eq 'Unclosed vertical receipt was admitted.') { throw } }
  $closeout.status = $savedCloseout
  $checkpoint.verification_receipts = @(); Save-Fixture
  try { & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ResultPath $resultPath -RouteOnly | Out-Null; throw 'Missing GP4 run receipt was admitted.' } catch { if ($_.Exception.Message -eq 'Missing GP4 run receipt was admitted.') { throw } }
  Write-Output 'G1 C4 successor route verified with forged route, authority, dependency, predecessor-state and receipt rejection.'
}
finally { Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue }
