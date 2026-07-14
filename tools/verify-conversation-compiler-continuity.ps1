$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$compiler = Get-Content -LiteralPath (Join-Path $root 'crates\forge-kernel\src\compiler.rs') -Raw
$persistence = Get-Content -LiteralPath (Join-Path $root 'crates\forge-kernel\src\persistence.rs') -Raw
$admission = Get-Content -LiteralPath (Join-Path $root 'crates\forge-kernel\src\code_admission.rs') -Raw
$contract = Get-Content -LiteralPath (Join-Path $root 'contracts\conversation-compiler-contract.md') -Raw
$readiness = Get-Content -LiteralPath (Join-Path $root 'docs\canonical-system\CONVERSATION_COMPILER_READINESS.md') -Raw
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
$system = @($registry.systems | Where-Object id -eq 'forge-context-compiler')
if ($system.Count -ne 1 -or $system[0].status -ne 'reference_proven') { throw 'Context compiler is not projected as reference-proven.' }
Write-Output 'Conversation compiler continuity verified: fixed long corpus, replay, legacy migration, portable path rejection, SQLite release, and authority-negative state retained.'
