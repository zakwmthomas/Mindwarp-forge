$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$verifier = Join-Path $PSScriptRoot 'verify-stage-quality-gates.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) "forge-stage-quality-$PID.json"

function Valid-State {
  [ordered]@{
    substage_id='fixture-stage'
    stage_context=@{stage_id='fixture-stage';macro_sources=@('master');macro_findings=@('route');micro_sources=@('contract');micro_findings=@('invariant')}
    visual_quality_gate=@{asset_use_intent=$false;status='not_applicable';rationale='No visual asset in fixture';receipts=@()}
    simulation_ladder=@{cheapest_sufficient_tier='typed_model';tiers_completed=@(@{tier='static_reasoning';result='pass';evidence='fixture logic'});expensive_execution_planned=$false;unresolved_risk='';expected_information_gain='';estimated_cost='';regression_guard='';stop_condition='';final_integration_gate='fixture full gate'}
  }
}
function Must-Fail($state,[string]$message) {
  $state | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $temp
  $failed=$false
  try { & $verifier -Path $temp | Out-Null } catch { $failed=$true }
  if (!$failed) { throw $message }
}
try {
  $valid=Valid-State; $valid | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $temp; & $verifier -Path $temp | Out-Null
  $stale=Valid-State; $stale.stage_context.stage_id='old-stage'; Must-Fail $stale 'Stale stage context was accepted.'
  $metadata=Valid-State; $metadata.visual_quality_gate=@{asset_use_intent=$true;status='passed';rationale='fixture';receipts=@(@{content_sha256=('a'*64);provenance='fixture';intended_comparison='human';accuracy_limits='none';pixel_inspection_performed=$false;rendered_views_inspected=@();disposition='verified_fit';human_subject=$true;anatomical_credibility='verified';comparison_completeness='verified';pose_view_lighting_fitness='verified'})}; Must-Fail $metadata 'Metadata-only visual admission was accepted.'
  $costly=Valid-State; $costly.simulation_ladder.cheapest_sufficient_tier='bounded_integrated_pc'; $costly.simulation_ladder.expensive_execution_planned=$true; Must-Fail $costly 'Unjustified expensive execution was accepted.'
  Write-Output 'Stage-quality fixtures verified: stale context, metadata-only visuals, and unjustified expensive execution fail closed.'
} finally { Remove-Item -LiteralPath $temp -Force -ErrorAction SilentlyContinue }
