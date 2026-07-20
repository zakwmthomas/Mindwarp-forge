$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
. (Join-Path $PSScriptRoot 'g1-c6-successor-route.ps1')
$checkpoint=Get-Content -Raw (Join-Path $root 'context\active\WORKER_BATCH_STATE.json')|ConvertFrom-Json
if(!(Test-G1C6OrganismSubjectIdentityImplementationRoute -Checkpoint $checkpoint)-and!(Test-G1C6EcologicalNicheSemanticsSchemaGapRoute -Checkpoint $checkpoint)){throw 'C6 organism-subject identity implementation or exact recorded successor route is not exact.'}
foreach($receipt in @('receipt:G1-C6-BODY-PLAN-STRUCTURE-V1:recorded','owner-authorization:c6-organism-subject-identity-v1:released')){if(@($checkpoint.verification_receipts)-notcontains$receipt){throw "C6 identity implementation receipt missing: $receipt"}}

$sourcePath=Join-Path $root 'crates\organism-subject-identity\src\lib.rs'
$realTestPath=Join-Path $root 'crates\organism-subject-identity\tests\real_subject_replay.rs'
$redTestPath=Join-Path $root 'crates\organism-subject-identity\tests\red_contract.rs'
$consumerPath=Join-Path $root 'crates\person-form-eligibility\src\lib.rs'
$consumerTestPath=Join-Path $root 'crates\person-form-eligibility\tests\identity_bound_subject.rs'
$resultPath=Join-Path $root 'docs\canonical-system\G1_C6_ORGANISM_SUBJECT_IDENTITY_IMPLEMENTATION_RESULT.md'
foreach($path in @($sourcePath,$realTestPath,$redTestPath,$consumerPath,$consumerTestPath,$resultPath)){if(!(Test-Path -LiteralPath $path -PathType Leaf)){throw "C6 identity implementation evidence missing: $path"}}

$metadata=& cargo metadata --locked --no-deps --format-version 1|ConvertFrom-Json
if($LASTEXITCODE-ne0){throw 'C6 identity Cargo metadata failed.'}
$package=@($metadata.packages|Where-Object name -eq 'organism-subject-identity')
if($package.Count-ne1){throw 'C6 identity package is not unique.'}
$production=@($package[0].dependencies|Where-Object kind -ne 'dev'|ForEach-Object name|Sort-Object)
$expected=@('body-plan-structure','derived-world-rules','entity-lifecycle','entity-lifecycle-history-binding','hierarchy-history','macro-lineage-binding','niche-graph-binding','serde','serde_json','sha2')
if(Compare-Object $production $expected){throw "C6 identity production dependency allowlist drifted: $($production-join',')"}
$consumers=@($metadata.packages|Where-Object{@($_.dependencies|Where-Object name -eq 'organism-subject-identity').Count}|ForEach-Object name|Sort-Object)
if(Compare-Object $consumers @('person-form-eligibility')){throw "C6 identity consumer set drifted: $($consumers-join',')"}

$source=Get-Content -Raw $sourcePath
$realTests=Get-Content -Raw $realTestPath
$redTests=Get-Content -Raw $redTestPath
$consumer=Get-Content -Raw $consumerPath
$consumerTests=Get-Content -Raw $consumerTestPath
$result=Get-Content -Raw $resultPath
$matrixText=$source+"`n"+$realTests+"`n"+$redTests+"`n"+$consumer+"`n"+$consumerTests
$groups=@([regex]::Matches($matrixText,'(?m)^\s*fn c6_(\d{2})_')|ForEach-Object{$_.Groups[1].Value})
if($groups.Count-ne33-or(@($groups|Sort-Object -Unique).Count)-ne33-or(@($groups|Sort-Object)-join',')-ne((1..33|ForEach-Object{$_.ToString('00')})-join',')){throw "C6 identity focused matrix is not exact 01..33: $($groups-join',')"}
foreach($token in @('recover_known_good_prefix','accepted_records != encoded_deltas.len()','first_failure.is_some()','validate_state(&initial)','cross-kind identity collapse','build_subject_bundle','build_reference_receipt','capabilities: []','biological_membership: false')){if(!$source.Contains($token)){throw "C6 identity source invariant missing: $token"}}
foreach($token in @('reference_fixtures','build_macro_lineage_candidate','build_lineage_subject_ref','bind_lifecycle_history_subject','build_subject_bundle','corrupt.last_mut')){if(!$realTests.Contains($token)){throw "C6 identity real replay fixture missing: $token"}}
foreach($token in @('evaluate_identity_bound_person_form_prerequisites','groundings.len() != MAX_PERSON_FORM_GROUNDINGS','AssessedLineageMismatch','BodyPlanMismatch')){if(!$consumer.Contains($token)){throw "C6 identity consumer invariant missing: $token"}}
foreach($token in @('evaluate_identity_bound_person_form_prerequisites(','build_subject_bundle','bind_lifecycle_history_subject','BoundSubjectError::IndeterminateBudget')){if(!$consumerTests.Contains($token)){throw "C6 identity real consumer fixture missing: $token"}}
foreach($token in @('exact 33-group implementation matrix','one production consumer','i686-pc-windows-msvc','aarch64-linux-android','24 examinations','run-500f816d66e94a359e9cf8617982bf49','dimorphism applicability')){if(!$result.Contains($token)){throw "C6 identity result missing: $token"}}

$normalized=$source-replace'\s+',''
foreach($token in @('std::fs','std::net','std::process','std::time','forge_kernel','tauri','reqwest','ureq','hyper','tokio::net','getrandom','rand::','fastrand::')){if($normalized.Contains($token)){throw "C6 identity capability surface crossed: $token"}}
foreach($field in @('species_members','population_members','population_count','ancestry_edges','reproductive_role','sex_identity','dimorphism_profile','runtime_handle')){if($source-match("pub\s+"+[regex]::Escape($field)+"\s*:")){throw "C6 identity forbidden semantic field crossed: $field"}}

& cargo test -p organism-subject-identity -p person-form-eligibility --all-targets --locked
if($LASTEXITCODE-ne0){throw 'C6 identity native focused tests failed.'}
& cargo test -p body-plan-structure -p macro-lineage-binding -p entity-lifecycle -p entity-lifecycle-history-binding -p hierarchy-history --locked
if($LASTEXITCODE-ne0){throw 'C6 identity retained upstream regressions failed.'}
& cargo clippy -p organism-subject-identity -p person-form-eligibility --all-targets --locked -- -D warnings
if($LASTEXITCODE-ne0){throw 'C6 identity strict Clippy failed.'}
& cargo test -p organism-subject-identity -p person-form-eligibility --all-targets --locked --target i686-pc-windows-msvc
if($LASTEXITCODE-ne0){throw 'C6 identity i686 execution failed.'}
& cargo check -p organism-subject-identity -p person-form-eligibility --locked --target aarch64-linux-android
if($LASTEXITCODE-ne0){throw 'C6 identity Android ARM64 compile-only check failed.'}
Write-Output 'G1 C6 organism-subject identity implementation verified: exact 33-group matrix, real upstream/C4 replay, strict codecs, one consumer, capability boundary, native/i686 execution and Android compile-only pass.'
