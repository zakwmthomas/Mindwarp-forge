$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
$crate=Join-Path $root 'crates\body-plan-structure'
$sourcePath=Join-Path $crate 'src\lib.rs'
$testPath=Join-Path $crate 'tests\body_plan_v1.rs'
$macroPath=Join-Path $root 'crates\macro-lineage-binding\src\lib.rs'
$resultPath=Join-Path $root 'docs\canonical-system\G1_C6_BODY_PLAN_STRUCTURE_IMPLEMENTATION_RESULT.md'
foreach($path in @($sourcePath,$testPath,$macroPath,$resultPath)){if(!(Test-Path -LiteralPath $path -PathType Leaf)){throw "C6 body-plan evidence missing: $path"}}

$metadata=(& cargo metadata --locked --no-deps --format-version 1|ConvertFrom-Json)
if($LASTEXITCODE-ne0){throw 'C6 body-plan Cargo metadata failed.'}
$package=@($metadata.packages|Where-Object name -eq 'body-plan-structure')
if($package.Count-ne1){throw 'C6 body-plan package is not unique.'}
$dependencies=@($package[0].dependencies|ForEach-Object name|Sort-Object)
if(Compare-Object $dependencies @('serde','serde_json','sha2')){throw 'C6 body-plan dependency allowlist drifted.'}
$consumers=@($metadata.packages|Where-Object{@($_.dependencies|Where-Object name -eq 'body-plan-structure').Count}|ForEach-Object name)
if($consumers.Count-ne1-or$consumers[0]-ne'macro-lineage-binding'){throw "C6 body-plan consumer set drifted: $($consumers-join',')"}

$source=Get-Content -Raw -LiteralPath $sourcePath
$normalized=$source-replace'\s+',''
$tests=Get-Content -Raw -LiteralPath $testPath
$macro=Get-Content -Raw -LiteralPath $macroPath
$result=Get-Content -Raw -LiteralPath $resultPath
foreach($token in @('BodyPlanFamily','StructuralExpression','IndeterminateBudget','reference_fixtures','reference_proof_receipt','mindwarp.body-plan-family.v1','mindwarp.body-plan-expression.v1','mindwarp.body-plan-reference-receipt.v1')){if(!$source.Contains($token)){throw "C6 body-plan source missing: $token"}}
if([regex]::Matches($tests,'(?m)^\s*#\[test\]\s*$').Count-ne17-or!$macro.Contains('fn candidate_accepts_only_exact_validated_family_fingerprint')){throw 'C6 body-plan matrix is not exact 17 plus one consumer group.'}
foreach($token in @('8b514e7a585efdd41f76479a0869e7907746c2b9f27b6cdcb7ef15df077549f6','fdbdc35205fb0c955c7e436ab53bba60d0bcb61bbf1ca95d4ec7fb3c528ae529','legal_non_tree_family','forbidden capability surface')){if(!$tests.Contains($token)){throw "C6 body-plan test evidence missing: $token"}}
foreach($token in @('std::fs','std::net','std::process','std::time','forge_kernel','tauri','reqwest','ureq','hyper','tokio::net','getrandom','rand::','fastrand::')){if($normalized.Contains($token)){throw "C6 body-plan capability surface crossed: $token"}}
foreach($token in @('usestdas','use::stdas','externcratestdas')){if($normalized.Contains($token)){throw "C6 body-plan std alias capability surface crossed: $token"}}
if($normalized-match'std::\{[^}]*(?:fs|net|process|time)'){throw 'C6 body-plan grouped std capability surface crossed.'}
foreach($token in @('17 body-plan test groups plus one macro-lineage consumer group','structural variation seam','i686-pc-windows-msvc','aarch64-linux-android','dimorphism applicability','one direct consumer')){if(!$result.Contains($token)){throw "C6 body-plan result missing: $token"}}

& cargo test -p body-plan-structure --locked
if($LASTEXITCODE-ne0){throw 'C6 body-plan native tests failed.'}
& cargo test -p macro-lineage-binding --locked
if($LASTEXITCODE-ne0){throw 'C6 macro-lineage regression failed.'}
& cargo clippy -p body-plan-structure -p macro-lineage-binding --all-targets --locked -- -D warnings
if($LASTEXITCODE-ne0){throw 'C6 body-plan strict Clippy failed.'}
& cargo test -p body-plan-structure -p macro-lineage-binding --locked --target i686-pc-windows-msvc
if($LASTEXITCODE-ne0){throw 'C6 body-plan i686 execution failed.'}
& cargo check -p body-plan-structure -p macro-lineage-binding --locked --target aarch64-linux-android
if($LASTEXITCODE-ne0){throw 'C6 body-plan Android ARM64 compile-only check failed.'}
Write-Output 'G1 C6 body-plan implementation verified: exact 17+1 matrix, canonical identities, structural variation, bounded validation, one consumer, native/i686 execution and Android compile-only passed.'
