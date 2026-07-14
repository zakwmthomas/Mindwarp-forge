param([string]$Path)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$path = if ($Path) { $Path } else { Join-Path $root 'context\active\WORKER_BATCH_STATE.json' }
$state = Get-Content -LiteralPath $path -Raw | ConvertFrom-Json

$context = $state.stage_context
if ($null -eq $context -or $context.stage_id -ne $state.substage_id) {
  throw 'Stage context is missing or stale relative to the active substage.'
}
foreach ($field in @('macro_sources','macro_findings','micro_sources','micro_findings')) {
  if (@($context.$field).Count -eq 0 -or @($context.$field | Where-Object { [string]::IsNullOrWhiteSpace([string]$_) }).Count -gt 0) {
    throw "Stage context lacks a bounded two-scale record: $field"
  }
}

$visual = $state.visual_quality_gate
if ($null -eq $visual -or $null -eq $visual.asset_use_intent -or [string]::IsNullOrWhiteSpace($visual.rationale)) {
  throw 'Visual-quality intent and rationale are required for every material substage.'
}
if (!$visual.asset_use_intent) {
  if ($visual.status -ne 'not_applicable' -or @($visual.receipts).Count -ne 0) {
    throw 'A no-asset stage must be explicitly not_applicable and cannot carry visual receipts.'
  }
} else {
  if ($visual.status -eq 'required_pending') {
    if (!$visual.dependent_implementation_blocked) {
      throw 'A pending visual gate must explicitly block dependent implementation.'
    }
    foreach ($receipt in @($visual.receipts)) {
      if ($receipt.disposition -notin @('owner_check_required','rejected')) {
        throw 'A pending visual gate can retain only rejected or owner-check receipts.'
      }
    }
  } elseif ($visual.status -ne 'passed' -or @($visual.receipts | Where-Object disposition -eq 'verified_fit').Count -eq 0) {
    throw 'Visual asset use requires a pending block or at least one passed pixel-level fitness receipt.'
  }
  foreach ($receipt in @($visual.receipts | Where-Object disposition -eq 'verified_fit')) {
    if ($receipt.content_sha256 -notmatch '^[0-9a-f]{64}$' -or
        [string]::IsNullOrWhiteSpace($receipt.provenance) -or
        [string]::IsNullOrWhiteSpace($receipt.intended_comparison) -or
        [string]::IsNullOrWhiteSpace($receipt.accuracy_limits) -or
        !$receipt.pixel_inspection_performed -or
        @($receipt.rendered_views_inspected).Count -eq 0 -or
        $receipt.disposition -ne 'verified_fit') {
      throw 'Visual receipt lacks provenance, actual-pixel inspection, accuracy limits, useful views, or verified fitness.'
    }
    if ($receipt.human_subject -and (
        $receipt.anatomical_credibility -ne 'verified' -or
        $receipt.comparison_completeness -ne 'verified' -or
        $receipt.pose_view_lighting_fitness -ne 'verified')) {
      throw 'Human visual reference lacks verified anatomy, completeness, or comparison conditions.'
    }
  }
}

$ladder = $state.simulation_ladder
$tiers = @('static_reasoning','typed_model','in_memory_fixture','disposable_simulation','bounded_integrated_pc','external_execution')
$tierIndex = [array]::IndexOf($tiers, [string]$ladder.cheapest_sufficient_tier)
if ($null -eq $ladder -or $tierIndex -lt 0 -or [string]::IsNullOrWhiteSpace($ladder.final_integration_gate)) {
  throw 'Simulation ladder lacks a valid cheapest tier or retained final integration gate.'
}
foreach ($result in @($ladder.tiers_completed)) {
  if ($tiers -notcontains $result.tier -or [string]::IsNullOrWhiteSpace($result.result) -or [string]::IsNullOrWhiteSpace($result.evidence)) {
    throw 'Simulation ladder contains an incomplete lower-tier result.'
  }
}
if ($tierIndex -ge 1 -and @($ladder.tiers_completed).Count -eq 0) {
  throw 'A non-static proof tier requires retained cheaper-tier evidence.'
}
$expensive = $tierIndex -ge 4
if ([bool]$ladder.expensive_execution_planned -ne $expensive) {
  throw 'Expensive-execution declaration does not match the selected tier.'
}
if ($expensive) {
  foreach ($field in @('unresolved_risk','expected_information_gain','estimated_cost','regression_guard','stop_condition')) {
    if ([string]::IsNullOrWhiteSpace($ladder.$field)) { throw "Expensive escalation lacks: $field" }
  }
  if (@($ladder.tiers_completed).Count -eq 0) { throw 'Expensive escalation lacks retained lower-tier results.' }
}
Write-Output "Stage quality verified: context matches $($state.substage_id); visual gate $($visual.status); cheapest proof tier $($ladder.cheapest_sufficient_tier)."
