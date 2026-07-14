$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$verifier = Join-Path $PSScriptRoot 'verify-worker-batch-state.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-worker-state-$PID.json"
function Fixture([string]$state,[string]$previous,[string]$next='next') {
  [ordered]@{
    schema_version=2; canonical_role='sole-active-work-checkpoint'; batch_id='fixture'; master_program_item='F5'
    state=$state; previous_state=$previous; objective='fixture objective'; next_action=$next; substage_id='fixture-stage'
    consecutive_no_progress_limit=1; atlas_route=@{milestone='F5';systems=@('task-bootstrap')}
    risk_level='bounded-fixture'; research_gate='fixture'; authority_lane='no authority'; context_health='green'
    unresolved_risks=@('fixture risk'); evidence_requirements=@('fixture evidence'); verification_plan=@('fixture verification')
    resume_after='fixture resume'; evidence=@(); verification_receipts=@(); transition=''; metrics=$null
  }
}
try {
  $invalid = Fixture 'complete' 'executing'
  $invalid | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'False-terminal state was accepted.' }

  $checkpoint = Fixture 'checkpoint' 'executing' 'resume'
  $checkpoint.evidence = @('fixture')
  $checkpoint | ConvertTo-Json -Depth 8 | Set-Content $temp
  & $verifier -Path $temp | Out-Null

  $incomplete = Fixture 'complete' 'recorded'
  $incomplete.evidence = @('artifact')
  $incomplete | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Completion without receipts and metrics was accepted.' }

  $complete = Fixture 'complete' 'recorded'
  $complete.evidence = @('artifact'); $complete.verification_receipts = @('gate-pass'); $complete.transition = 'W1 -> W2'
  $complete.metrics = @{token_cost=@{status='unknown';value=$null};elapsed_time=@{status='measured';value='1m'}}
  $complete | ConvertTo-Json -Depth 8 | Set-Content $temp
  & $verifier -Path $temp | Out-Null

  $unsafe = $complete
  $unsafe.approved = $true
  $unsafe | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Authority-bearing worker completion was accepted.' }
  Write-Output 'Worker batch fixtures verified: canonical schema, resume, completion receipts, metric integrity, and authority rejection.'
} finally { Remove-Item -LiteralPath $temp -Force -ErrorAction SilentlyContinue }
