param([string]$RegistryPath,[string]$DesignPath)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$fixture=Join-Path $root 'tools\fixtures\gp4-signal-anchor-readiness\main.rs'
$registryPath=if($RegistryPath){$RegistryPath}else{Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_FIXED_REGISTRY.md'}
$designPath=if($DesignPath){$DesignPath}else{Join-Path $root 'docs\canonical-system\G1_GP4_SIGNAL_ANCHOR_DESIGN.md'}
if(!(Test-Path -LiteralPath $fixture)){throw 'GP4 computational fixture missing.'}
$temporary=Join-Path ([IO.Path]::GetTempPath()) ('forge-gp4-readiness-'+[guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path (Join-Path $temporary 'src') -Force|Out-Null
try {
  $escaped=$root.Replace('\','/').Replace(' ',' ')
  @"
[package]
name="gp4-readiness-proof"
version="0.1.0"
edition="2024"
[dependencies]
addressable-world-binding={path="$escaped/crates/addressable-world-binding"}
derived-world-rules={path="$escaped/crates/derived-world-rules"}
field-basis={path="$escaped/crates/field-basis"}
mindwarp-gameplay-foundation={path="$escaped/crates/mindwarp-gameplay-foundation"}
mindwarp-vertical-persistence={path="$escaped/crates/mindwarp-vertical-persistence"}
universe-identity={path="$escaped/crates/universe-identity"}
serde={version="1",features=["derive"]}
serde_json="1"
sha2="0.10"
"@|Set-Content -LiteralPath (Join-Path $temporary 'Cargo.toml') -Encoding utf8
  Copy-Item -LiteralPath $fixture -Destination (Join-Path $temporary 'src\main.rs')
  $oldRoot=$env:FORGE_ROOT;$oldTarget=$env:CARGO_TARGET_DIR
  $env:FORGE_ROOT=$root.Replace('\','/');$env:CARGO_TARGET_DIR=Join-Path $temporary 'target'
  try{$output=& cargo run --quiet --offline --manifest-path (Join-Path $temporary 'Cargo.toml');if($LASTEXITCODE -ne 0){throw 'GP4 computational fixture failed.'}}
  finally{$env:FORGE_ROOT=$oldRoot;$env:CARGO_TARGET_DIR=$oldTarget}
  $proof=($output|Select-Object -Last 1)|ConvertFrom-Json
  $registry=Get-Content -LiteralPath $registryPath -Raw
  $design=Get-Content -LiteralPath $designPath -Raw

  $labels=@{
    input_sha='canonical C3A input SHA-256';packet_id='C3A packet ID';packet_sha='canonical C3A packet SHA-256';identity='vertical identity fingerprint';baseline_sha='C4V baseline bytes SHA-256';reducer='C4V GP1 reducer fingerprint';codec='C4V codec-v1 fingerprint';grammar='GP3 grammar digest';situation='GP3 S4 situation digest';session='GP3 S4 session digest';gp2_registry='rule-registry digest';gp2_session='session-record digest';gp2_terminal='terminal shadow-state digest';gp2_rule='rule ID';gp2_decision='opened decision'
  }
  foreach($key in $labels.Keys){$actual=[string]$proof.values.$key;$pattern='(?m)^\| '+[regex]::Escape($labels[$key])+' \| `'+[regex]::Escape($actual)+'` \|$';if($registry -notmatch $pattern){throw "Computed GP4 value differs from registry: $key"}}
  foreach($key in @('upstream_threat','approach_ref','threat_ref','final_head')){$actual=[string]$proof.values.$key;if(([regex]::Matches($registry,[regex]::Escape($actual))).Count -lt 1){throw "Computed GP4 derived value absent: $key"}}

  $commandRows=@($proof.commands);$commandMatches=[regex]::Matches($registry,'(?m)^\| (?<sequence>[1-4]) \| (?<revision>[0-3]) \| `(?<parent>none|[0-9a-f]{64})` \| `(?<label>[^`]+)` \| `(?<actions>[^`]+)` \| `(?<id>[0-9a-f]{64})` \| (?<observation>[^|]+) \|$')
  if($commandRows.Count-ne4-or$commandMatches.Count-ne4){throw 'Expected four complete command rows.'}
  foreach($command in $commandRows){$match=@($commandMatches|Where-Object{[int]$_.Groups['sequence'].Value-eq[int]$command.sequence});if($match.Count-ne1){throw "Command row missing or duplicated: $($command.sequence)"};$m=$match[0]
    $parent=if($null-eq$command.parent){'none'}else{[string]$command.parent}
    if([int]$m.Groups['revision'].Value-ne[int]$command.revision-or$m.Groups['parent'].Value-ne$parent-or$m.Groups['label'].Value-ne[string]$command.label-or$m.Groups['actions'].Value-ne[string]$command.action_mapping-or$m.Groups['id'].Value-ne[string]$command.command_id){throw "Typed command row drift: $($command.sequence)"}
  }

  $slotMatches=[regex]::Matches($registry,'(?m)^\| `(?<id>[^`]+)` \| (?<sources>.+?) \| `(?<digest>[0-9a-f]{64})` \| (?<text>[^|]+) \| (?<cue>[^|]+) \|$')
  $slots=@($slotMatches|Where-Object{$_.Groups['id'].Value -notlike 'hard.*' -and $_.Groups['id'].Value -notlike 'compare.*'})
  if($slots.Count -ne 25){throw "Expected 25 full semantic slot rows; found $($slots.Count)."}
  $seen=@{};$resolved=@($proof.resolved_source_ids);$expectedSlots=@($proof.semantic_rows)
  foreach($match in $slots){$id=$match.Groups['id'].Value;if($seen.ContainsKey($id)){throw "Duplicate semantic slot: $id"};$seen[$id]=$true
    $ids=@($match.Groups['sources'].Value -split '; '|ForEach-Object{$_.Trim().Trim('`')})
    foreach($source in $ids){if($resolved -notcontains $source){throw "Unresolved semantic source: $id -> $source"}}
    $expected=@($expectedSlots|Where-Object slot_id -eq $id);if($expected.Count-ne1){throw "Fixture semantic row missing: $id"};$expected=$expected[0]
    if((Compare-Object $ids @($expected.source_ids) -SyncWindow 0)-or$match.Groups['digest'].Value-ne[string]$expected.source_id_list_digest-or$match.Groups['text'].Value.Trim()-ne[string]$expected.text_equivalent-or$match.Groups['cue'].Value.Trim()-ne[string]$expected.non_color_cue-or[string]$expected.reduced_motion_equivalent-ne[string]$expected.text_equivalent-or[string]$expected.screen_reader_label-ne("$id`: "+[string]$expected.text_equivalent)){throw "Typed semantic row drift: $id"}
  }

  $requirementMatches=[regex]::Matches($registry,'(?m)^\| `(?<id>(?:hard|compare)\.[^`]+)` \| (?<class>Hard|Compare) \| (?<question>[^|]+) \| (?<evidence>[^|]+) \| (?<method>[^|]+) \| (?<target>[^|]+) \|$')
  if($requirementMatches.Count -ne 29){throw "Expected 29 full adapter rows; found $($requirementMatches.Count)."}
  $reqSeen=@{};$hard=0;$compare=0
  $expectedRequirements=@($proof.requirement_rows)
  foreach($match in $requirementMatches){$id=$match.Groups['id'].Value;if($reqSeen.ContainsKey($id)){throw "Duplicate adapter row: $id"};$reqSeen[$id]=$true
    if($match.Groups['class'].Value -eq 'Hard'){$hard++;if($id -notlike 'hard.*'){throw "Adapter class prefix drift: $id"}}else{$compare++;if($id -notlike 'compare.*'){throw "Adapter class prefix drift: $id"}}
    $expected=@($expectedRequirements|Where-Object requirement_id -eq $id);if($expected.Count-ne1){throw "Fixture adapter row missing: $id"};$expected=$expected[0]
    if($match.Groups['class'].Value-ne[string]$expected.class-or[string]$expected.status-ne'Unmeasured'-or$match.Groups['question'].Value.Trim()-ne[string]$expected.question-or$match.Groups['evidence'].Value.Trim()-ne[string]$expected.required_evidence-or$match.Groups['method'].Value.Trim()-ne[string]$expected.method-or$match.Groups['target'].Value.Trim()-ne[string]$expected.target){throw "Typed adapter row drift: $id"}
    if(!$design.Contains("``$id``")){throw "Adapter row absent from typed design: $id"}
  }
  if($expectedRequirements.Count-ne29){throw 'Fixture does not define exactly 29 adapter rows.'}
  if($hard-ne16-or$compare-ne13){throw "Adapter class count drift: hard=$hard compare=$compare"}
  $designRequirementIds=@([regex]::Matches($design,'(?m)^- `(?<id>(?:hard|compare)\.[^`]+)`$')|ForEach-Object{$_.Groups['id'].Value})
  if($designRequirementIds.Count-ne29-or(Compare-Object @($reqSeen.Keys|Sort-Object) @($designRequirementIds|Sort-Object))){throw 'Typed design and fixed requirement ID sets differ.'}

  $schemaMatches=[regex]::Matches($design,'(?m)^\d+\. `(?<field>[a-z0-9_]+): (?<type>[^`]+)` — (?<clause>.+?)[;\.]$')
  $expectedSchema=@($proof.schema_rows)
  if($schemaMatches.Count-ne21-or$expectedSchema.Count-ne21){throw 'SignalAnchorBundleV1 must have 21 typed schema declarations.'}
  for($index=0;$index-lt21;$index++){$actual=$schemaMatches[$index];$expected=$expectedSchema[$index];if($actual.Groups['field'].Value-ne[string]$expected.field-or$actual.Groups['type'].Value-ne[string]$expected.rust_type-or$actual.Groups['clause'].Value-ne[string]$expected.clause){throw "SignalAnchorBundleV1 schema drift: $($expected.field)"}}
  $expectedSemantic=@('schema_version','run_id','session_id','phase','preparation','predecessor_outcome_id','session_state','ledger_before','ledger_after','failure','recoveries_used','stable_stop','trace')
  if((Compare-Object @($proof.semantic_fields) $expectedSemantic -SyncWindow 0)){throw 'Computed semantic projection does not cover exactly 13 non-world fields.'}
  function Read-NumberedIds([string]$text,[string]$start,[string]$end){$startAt=$text.IndexOf($start);$endAt=$text.IndexOf($end,$startAt+$start.Length);if($startAt-lt0-or$endAt-lt0){throw "GP2 numbered section missing: $start"};$matches=@([regex]::Matches($text.Substring($startAt,$endAt-$startAt),'(?m)^(?<ordinal>\d+)\. `(?<id>[^`]+)`$'));for($i=0;$i-lt$matches.Count;$i++){if([int]$matches[$i].Groups['ordinal'].Value-ne$i+1){throw 'GP2 numbered section ordinal drifted.'}};@($matches|ForEach-Object{$_.Groups['id'].Value})}
  $registryEmitted=Read-NumberedIds $registry 'Expected emitted record IDs, in order:' 'Expected GP2 receipt authority is real and exact:'
  $registryTransitions=Read-NumberedIds $registry 'Expected GP2 world-transition IDs, in order:' 'The selected threat mutation `work-area.state.safe`'
  $computedEmitted=@($proof.emitted);$computedTransitions=@($proof.transitions)
  $emissionOrderDrift=@(0..5|Where-Object{[string]$computedEmitted[$_]-ne[string]$registryEmitted[$_]}).Count
  $transitionOrderDrift=@(0..5|Where-Object{[string]$computedTransitions[$_]-ne[string]$registryTransitions[$_]}).Count
  if($computedEmitted.Count-ne6-or$computedTransitions.Count-ne6-or$emissionOrderDrift-or$transitionOrderDrift){throw 'Computed GP2 output order or cardinality drifted.'}
  if($computedEmitted-contains'work-area.state.safe'-or$computedTransitions-contains'work-area.state.safe'-or$registryEmitted-contains'work-area.state.safe'-or$registryTransitions-contains'work-area.state.safe'){throw 'Threat mutation leaked into GP2 outputs.'}
  Write-Output 'GP4 computational readiness verified: live public APIs reproduce dependency digests, command chain, shadow semantics, GP2 outputs, 25 resolved slots, 29 typed rows and 21 ordered bundle fields.'
} finally {if(Test-Path -LiteralPath $temporary){Remove-Item -LiteralPath $temporary -Recurse -Force}}
