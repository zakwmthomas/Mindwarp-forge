$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$verifier = Join-Path $PSScriptRoot 'verify-worker-batch-state.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-worker-state-$PID.json"
. (Join-Path $PSScriptRoot 'worker-handoff-integrity.ps1')
function Set-HandoffReceipts($fixture,[string]$disposition='revised') {
  $receipts = [ordered]@{}
  foreach ($section in @(Get-WorkerHandoffSectionNames)) {
    $receipts[$section] = [ordered]@{
      stage_id=$fixture.substage_id
      content_sha256=(Get-WorkerHandoffSectionHash -State $fixture -Section $section)
      review_disposition=$disposition
      review_note='fixture section reviewed against the active substage'
    }
  }
  $fixture.handoff_section_receipts = $receipts
}
function Fixture([string]$state,[string]$previous,[string]$next='next') {
  $fixture = [pscustomobject][ordered]@{
    schema_version=3; canonical_role='sole-active-work-checkpoint'; batch_id='fixture'; master_program_item='F5'
    state=$state; previous_state=$previous; objective='fixture objective'; next_action=$next; substage_id='fixture-stage'
    handoff_section_receipts=$null
    consecutive_no_progress_limit=1; atlas_route=@{milestone='F5';systems=@('task-bootstrap')}
    risk_level='bounded-fixture'; research_gate='fixture'; authority_lane='no authority'; context_health='green'
    stage_context=@{stage_id='fixture-stage';macro_sources=@('master');macro_findings=@('route');micro_sources=@('contract');micro_findings=@('invariant')}
    visual_quality_gate=@{asset_use_intent=$false;status='not_applicable';rationale='No visual asset in fixture';receipts=@()}
    simulation_ladder=@{cheapest_sufficient_tier='typed_model';tiers_completed=@(@{tier='static_reasoning';result='pass';evidence='fixture logic'});expensive_execution_planned=$false;unresolved_risk='';expected_information_gain='';estimated_cost='';regression_guard='';stop_condition='';final_integration_gate='fixture full gate'}
    unresolved_risks=@('fixture risk'); evidence_requirements=@('fixture evidence'); verification_plan=@('fixture verification')
    exit_criteria=@(@{id='criterion-1';label='Fixture criterion';status='planned';evidence_ids=@()})
    resume_after='fixture resume'; evidence=@(); verification_receipts=@(); transition=''; metrics=$null
  }
  Set-HandoffReceipts $fixture
  return $fixture
}
try {
  $invalid = Fixture 'complete' 'executing'
  $invalid | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'False-terminal state was accepted.' }

  $checkpoint = Fixture 'checkpoint' 'executing' 'resume'
  $checkpoint.evidence = @('fixture')
  Set-HandoffReceipts $checkpoint
  $checkpoint | ConvertTo-Json -Depth 8 | Set-Content $temp
  & $verifier -Path $temp | Out-Null

  $partialTransition = Fixture 'checkpoint' 'executing' 'new-stage continuation'
  $partialTransition.substage_id = 'new-stage'
  $partialTransition.stage_context.stage_id = 'new-stage'
  $partialTransition.handoff_section_receipts.objective.stage_id = 'new-stage'
  $partialTransition.handoff_section_receipts.next_action.stage_id = 'new-stage'
  $partialTransition.handoff_section_receipts.stage_context.stage_id = 'new-stage'
  $partialTransition | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Partial stage transition with stale handoff sections was accepted.' }

  $postReviewEdit = Fixture 'checkpoint' 'executing'
  $postReviewEdit.next_action = 'changed after review'
  $postReviewEdit | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Post-review handoff content drift was accepted.' }

  $missingReceipt = Fixture 'checkpoint' 'executing'
  $missingReceipt.handoff_section_receipts.Remove('resume_after')
  $missingReceipt | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Missing handoff receipt was accepted.' }

  $unknownReceipt = Fixture 'checkpoint' 'executing'
  $unknownReceipt.handoff_section_receipts['invented_section'] = $unknownReceipt.handoff_section_receipts.objective
  $unknownReceipt | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Unknown handoff receipt was accepted.' }

  $oldSchema = Fixture 'checkpoint' 'executing'
  $oldSchema.schema_version = 2
  $oldSchema | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Superseded worker batch schema was accepted.' }

  $badCriterion = Fixture 'checkpoint' 'executing'
  $badCriterion.exit_criteria[0].status = 'invented'
  Set-HandoffReceipts $badCriterion
  $badCriterion | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Invalid structured exit-criterion status was accepted.' }

  $unevidencedCriterion = Fixture 'checkpoint' 'executing'
  $unevidencedCriterion.exit_criteria[0].status = 'verified'
  Set-HandoffReceipts $unevidencedCriterion
  $unevidencedCriterion | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Verified structured exit criterion without evidence was accepted.' }

  $incomplete = Fixture 'complete' 'recorded'
  $incomplete.evidence = @('artifact')
  $incomplete | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Completion without receipts and metrics was accepted.' }

  $complete = Fixture 'complete' 'recorded'
  $complete.evidence = @('artifact'); $complete.verification_receipts = @('gate-pass'); $complete.transition = 'W1 -> W2'
  $complete.metrics = @{token_cost=@{status='unknown';value=$null};elapsed_time=@{status='measured';value='1m'}}
  $complete.exit_criteria[0].status = 'verified'; $complete.exit_criteria[0].evidence_ids = @('gate-pass')
  Set-HandoffReceipts $complete
  $complete | ConvertTo-Json -Depth 8 | Set-Content $temp
  & $verifier -Path $temp | Out-Null

  $unsafe = $complete
  $unsafe | Add-Member -NotePropertyName approved -NotePropertyValue $true
  $unsafe | ConvertTo-Json -Depth 8 | Set-Content $temp
  $failed = $false
  try { & $verifier -Path $temp | Out-Null } catch { $failed = $true }
  if (!$failed) { throw 'Authority-bearing worker completion was accepted.' }
  Write-Output 'Worker batch fixtures verified: schema-3 integrity receipts, structured exit criteria, shared section manifest, partial transitions, post-review drift, missing/unknown receipts, stage-quality, completion, metrics, and authority rejection.'
} finally { Remove-Item -LiteralPath $temp -Force -ErrorAction SilentlyContinue }
