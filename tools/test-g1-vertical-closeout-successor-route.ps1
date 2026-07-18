$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$temp = Join-Path ([IO.Path]::GetTempPath()) ('forge-gp4-closeout-route-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $temp | Out-Null
try {
  $run = 'run-11111111111111111111111111111111'
  $program = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw | ConvertFrom-Json
  $gp4 = @($program.items | Where-Object id -eq 'GP4')[0]
  $closeout = @($program.items | Where-Object id -eq 'G1-VERTICAL-CLOSEOUT')[0]
  $gp4.state = 'verified'; $gp4.status = 'complete'; $gp4.gate = 'recorded'; $gp4.proof = "Registered complete gate $run passed."
  $closeout.state = 'executing'; $closeout.status = 'active'
  $checkpoint = Get-Content -LiteralPath (Join-Path $root 'context\active\WORKER_BATCH_STATE.json') -Raw | ConvertFrom-Json
  $checkpoint.batch_id = 'G1-VERTICAL-CLOSEOUT-V1'; $checkpoint.master_program_item = 'G1-VERTICAL-CLOSEOUT'; $checkpoint.substage_id = 'g1-vertical-closeout-recorded'
  $checkpoint.authority_lane = 'Owner-authorized bounded G1 vertical closeout evidence receipt only; broad_g1=false; runtime_selected=false; runtime_containment_pending=true; evidence_only=true; promotion_authority=false. No runtime, broad C4, C3B, Companion, Greenfield, procedural generation, persistence expansion, filesystem, network, process or Kernel mutation.'
  $checkpoint.verification_receipts = @("registered-full-gate:${run}:passed")
  $programPath = Join-Path $temp 'program.json'; $checkpointPath = Join-Path $temp 'checkpoint.json'; $resultPath = Join-Path $temp 'result.md'
  function Save-Fixture { $program | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $programPath -Encoding utf8; $checkpoint | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $checkpointPath -Encoding utf8; Set-Content -LiteralPath $resultPath -Value "GP4 registered proof $run" -Encoding utf8 }
  function Assert-Routes-Pass { Save-Fixture; if ((& (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -ne $true) { throw 'Valid closeout route was rejected.' }; & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ResultPath $resultPath -RouteOnly | Out-Null; & (Join-Path $root 'tools\verify-g1-gp3-encounter-grammar-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath | Out-Null }
  Assert-Routes-Pass
  foreach ($field in @('batch_id','master_program_item','substage_id','authority_lane')) {
    $copy = $checkpoint.$field; $checkpoint.$field = 'forged'; if ((& (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -eq $true) { throw "Forged closeout $field was admitted." }; $checkpoint.$field = $copy
  }
  $baseAuthority=$checkpoint.authority_lane
  foreach($token in @('broad_g1=false','runtime_selected=false','runtime_containment_pending=true','evidence_only=true','promotion_authority=false','No runtime','broad C4','C3B','Companion','Greenfield','procedural generation','persistence expansion','filesystem','network','process','Kernel mutation')) { $checkpoint.authority_lane=$baseAuthority.Replace($token,''); if ((& (Join-Path $root 'tools\test-c3-federated-interruption.ps1') -Checkpoint $checkpoint) -eq $true) { throw "Closeout authority omission was admitted: $token" } }
  $checkpoint.authority_lane=$baseAuthority
  $baseProof=$gp4.proof; $gp4.proof='no registered run'; Save-Fixture; try { & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ResultPath $resultPath -RouteOnly | Out-Null; throw 'Missing run proof was admitted.' } catch { if ($_.Exception.Message -eq 'Missing run proof was admitted.') { throw } }; $gp4.proof=$baseProof
  $baseStatus=$gp4.status; $gp4.status='active'; Save-Fixture; try { & (Join-Path $root 'tools\verify-g1-gp3-encounter-grammar-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath | Out-Null; throw 'Forged GP4 status was admitted.' } catch { if ($_.Exception.Message -eq 'Forged GP4 status was admitted.') { throw } }; $gp4.status=$baseStatus
  $baseDeps=@($closeout.depends_on); $closeout.depends_on=@($baseDeps | Where-Object { $_ -ne 'GP4' }); Save-Fixture; try { & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ResultPath $resultPath -RouteOnly | Out-Null; throw 'Missing closeout dependency was admitted.' } catch { if ($_.Exception.Message -eq 'Missing closeout dependency was admitted.') { throw } }; $closeout.depends_on=$baseDeps
  $checkpoint.verification_receipts=@(); Save-Fixture; try { & (Join-Path $root 'tools\verify-g1-gp4-signal-anchor-readiness.ps1') -ProgramPath $programPath -CheckpointPath $checkpointPath -ResultPath $resultPath -RouteOnly | Out-Null; throw 'Missing successful receipt was admitted.' } catch { if ($_.Exception.Message -eq 'Missing successful receipt was admitted.') { throw } }
  Write-Output 'G1 vertical closeout successor route verified with forged route, master, status, dependency and receipt rejection.'
}
finally { Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue }
