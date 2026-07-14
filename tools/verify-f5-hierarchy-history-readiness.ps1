$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$paths=@{gate='docs\canonical-system\HIERARCHY_HISTORY_DESIGN_GATE.md';audit='docs\canonical-system\HIERARCHY_HISTORY_SYSTEM_ALIGNMENT_AUDIT.md';contract='contracts\hierarchy-history-contract.md';hierarchy='crates\hierarchy-history\src\hierarchy.rs';history='crates\hierarchy-history\src\history.rs';proof='crates\hierarchy-history\src\proof.rs'}
foreach($name in $paths.Keys){$full=Join-Path $root $paths[$name];if(!(Test-Path $full)){throw "Hierarchy/history artifact missing: $($paths[$name])"};Set-Variable -Name $name -Value (Get-Content $full -Raw)}
foreach($required in @('output-affecting','explicit tuple','stable command/idempotency ID','Unsupported cross-target operations fail visibly','never delete source deltas','Old and new generator baselines may coexist','protected-Kernel')){if(!$gate.Contains($required)){throw "Hierarchy/history gate missing: $required"}}
foreach($required in @('Conditional-approval critical revalidation','No Man''s Sky','Factorio','Orleans','Git pack','freezing quality','capability-free')){if(!$audit.Contains($required)){throw "Hierarchy/history alignment audit missing: $required"}}
foreach($required in @('capability-free','BaselineManifest','ChildCursor','DeltaEnvelope','Snapshots are additive','cross-target work','prototype_tested')){if(!$contract.Contains($required)){throw "Hierarchy/history contract missing: $required"}}
foreach($required in @('MAX_CHILD_WINDOW','StaleCursor','work_budget','DescriptorOrigin')){if(!$hierarchy.Contains($required)){throw "Hierarchy fixture missing: $required"}}
foreach($required in @('CommandConflict','ForkConflict','UnsupportedCrossTarget','SnapshotMismatch','MigrationReceipt')){if(!$history.Contains($required)){throw "History fixture missing: $required"}}
foreach($required in @('measured_window_sizes','capabilities: Vec::new','proof_is_bounded_and_authority_negative','baseline_coexistence_allows_quality_improvement')){if(!$proof.Contains($required)){throw "Proof fixture missing: $required"}}
$program=Get-Content (Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json') -Raw|ConvertFrom-Json
$active=@($program.items|Where-Object status -eq 'active')
$f5=@($program.items|Where-Object id -eq 'F5')[0]
if($active.Count-ne 1-or($active[0].id-ne'F5'-and!($f5.status-eq'complete'-and$active[0].milestone-in@('G1','R1')))){throw 'Hierarchy/history reference is not retained through the F5 or later route.'}
$kernel=Get-Content (Join-Path $root 'crates\forge-kernel\src\lib.rs') -Raw
if($kernel.Contains('HierarchyDescriptor') -or $kernel.Contains('DeltaEnvelope')){throw 'Hierarchy/history reference leaked into protected Kernel.'}
Write-Output 'F5 hierarchy/history reference verified: critical revalidation, capability-free boundaries, strict descriptors, bounded paging, per-target replay/conflicts, migration, snapshot distrust, scale evidence, and authority-negative proof retained.'
