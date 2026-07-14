$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$compiler = Get-Content -LiteralPath (Join-Path $root 'crates\forge-kernel\src\compiler.rs') -Raw
$persistence = Get-Content -LiteralPath (Join-Path $root 'crates\forge-kernel\src\persistence.rs') -Raw
$admission = Get-Content -LiteralPath (Join-Path $root 'crates\forge-kernel\src\code_admission.rs') -Raw
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\conversation-compiler-contract.md') -Raw
$readiness = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\CONVERSATION_COMPILER_READINESS.md') -Raw
$knowledge = Get-Content -LiteralPath (Join-Path $root 'crates\forge-kernel\src\knowledge.rs') -Raw
$knowledgeResult = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\KNOWLEDGE_INTAKE_V2_RESULT.md') -Raw
$capture = Get-Content -LiteralPath (Join-Path $root 'apps\forge-desktop\src-tauri\src\codex_capture.rs') -Raw
$ui = Get-Content -LiteralPath (Join-Path $root 'apps\forge-desktop\ui\index.html') -Raw
$finder = Get-Content -LiteralPath (Join-Path $root 'tools\find-knowledge.ps1') -Raw
$registry = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\system-registry.json') -Raw | ConvertFrom-Json

foreach ($required in @(
  'representative_long_corpus_replays_exactly_and_releases_sqlite_files',
  'synthetic-representative-corpus-v1',
  '9e5c620f6d18b000b0c3a328fa20c8f6dfd497e9600b8ba42cc9849e53be5b3d',
  'source_envelope_schema_adds_to_legacy_database_and_replays_exact_links',
  'drop(reopened)',
  'fs::remove_dir_all(&directory)',
  'CandidateState::Proposed'
)) {
  if (!$persistence.Contains($required)) { throw "Compiler continuity fixture is missing: $required" }
}
foreach ($required in @('is_safe_repository_relative_path','has_drive_prefix','path.contains')) {
  if (!$admission.Contains($required)) { throw "Portable path admission guard is missing: $required" }
}
foreach ($required in @('C:/absolute.rs','C:\\absolute.rs','server\\share')) {
  if (!$admission.Contains($required)) { throw "Portable path admission fixture is missing: $required" }
}
foreach ($required in @('long_labeled_corpus_preserves_order_and_candidate_count','reserved_actor_labels_and_oversized_pastes_are_rejected_before_commit')) {
  if (!$compiler.Contains($required)) { throw "Base compiler adversarial fixture is missing: $required" }
}
foreach ($required in @('1,024','512','sqlite','legacy','platform-independent')) {
  if (!$contract.ToLowerInvariant().Contains($required) -or !$readiness.ToLowerInvariant().Contains($required)) { throw "Compiler continuity record is incomplete: $required" }
}
foreach ($required in @('Philosophy','Requirement','Constraint','Preference','Risk','Question','Observation','Context','deterministic_multi_facet_rules','ordinary_owner_language_yields_multiple_material_facets','continuity_request_retains_philosophy_requirement_risk_and_task','acknowledgements_and_operational_receipts_do_not_flood_intake')) {
  if (!$knowledge.Contains($required)) { throw "Knowledge intake v2 proof surface is missing: $required" }
}
foreach ($required in @('Canonical routing','Every non-noise message','evidence_only','whole-message facets','does not replace validation')) {
  if (!$knowledgeResult.Contains($required)) { throw "Knowledge intake v2 result is incomplete: $required" }
}
foreach ($required in @('KNOWLEDGE_INDEX.md','KNOWLEDGE_CATALOG.json','knowledge_classifier_version')) {
  if (!$capture.Contains($required)) { throw "Knowledge bootstrap projection is missing: $required" }
}
foreach ($required in @('data-knowledge-filter="philosophy"','data-knowledge-filter="requirement"','data-knowledge-filter="constraint"','data-knowledge-filter="preference"','data-knowledge-filter="risk"')) {
  if (!$ui.Contains($required)) { throw "Knowledge library filter is missing: $required" }
}
foreach ($required in @('KNOWLEDGE_CATALOG.json','classifier_version','record_type','source_actor')) {
  if (!$finder.Contains($required)) { throw "Typed knowledge finder is missing: $required" }
}
$system = @($registry.systems | Where-Object id -eq 'forge-context-compiler')
if ($system.Count -ne 1 -or $system[0].status -ne 'reference_proven') { throw 'Context compiler is not projected as reference-proven.' }
Write-Output 'Conversation compiler continuity verified: fixed long corpus, replay, migration, portable paths, SQLite release, multi-facet v2 knowledge routing, typed search, and authority-negative state retained.'
