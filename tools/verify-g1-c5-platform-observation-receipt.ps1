param(
  [string]$ReceiptPath,
  [Parameter(Mandatory=$true)][string]$SemanticSha256,
  [Parameter(Mandatory=$true)][string]$BoundedSourceManifestSha256,
  [Parameter(Mandatory=$true)][string]$Rustc,
  [Parameter(Mandatory=$true)][string]$Cargo,
  [Parameter(Mandatory=$true)][string]$NativeCommand,
  [Parameter(Mandatory=$true)][string]$I686Command,
  [Parameter(Mandatory=$true)][string]$AndroidCommand,
  [Parameter(Mandatory=$true)][string]$NativeStdout,
  [Parameter(Mandatory=$true)][string]$NativeStderr,
  [Parameter(Mandatory=$true)][string]$I686Stdout,
  [Parameter(Mandatory=$true)][string]$I686Stderr,
  [Parameter(Mandatory=$true)][string]$AndroidStdout,
  [Parameter(Mandatory=$true)][string]$AndroidStderr,
  [Parameter(Mandatory=$true)][string]$NativeExecutable,
  [Parameter(Mandatory=$true)][string]$I686Executable
)
$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
if([string]::IsNullOrWhiteSpace($ReceiptPath)){$ReceiptPath=Join-Path $root 'docs\canonical-system\G1_C5_LOCAL_PLATFORM_OBSERVATIONS.json'}
$fixtureRoot=Join-Path $root 'tools\fixtures\c5-significance-scheduler-receipt'
$manifestPath=Join-Path $fixtureRoot 'Cargo.toml'
if(!(Test-Path -LiteralPath $fixtureRoot -PathType Container)-or!(Test-Path -LiteralPath $manifestPath -PathType Leaf)){throw 'C5 semantic receipt fixture is missing.'}
$expectedNativeCommand="run --quiet --locked --offline --manifest-path `"$manifestPath`""
$expectedI686Command="run --quiet --locked --offline --target i686-pc-windows-msvc --manifest-path `"$manifestPath`""
$expectedAndroidCommand="check --quiet --locked --offline --target aarch64-linux-android --manifest-path `"$manifestPath`""
if($NativeCommand-ne$expectedNativeCommand-or$I686Command-ne$expectedI686Command-or$AndroidCommand-ne$expectedAndroidCommand){throw 'C5 platform observation command binding changed.'}
function Assert-Properties($Value,[string[]]$Expected,[string]$Label){$actual=@($Value.PSObject.Properties.Name|Sort-Object);$wanted=@($Expected|Sort-Object);if(($actual-join "`n")-ne($wanted-join "`n")){throw "C5 $Label properties changed."}}
function Assert-JsonInteger($Value,[string]$Label){if($Value-is[bool]-or-($Value-isnot[int]-and$Value-isnot[long])){throw "C5 $Label must be an integer."}}
function Assert-JsonBoolean($Value,[string]$Label){if($Value-isnot[bool]){throw "C5 $Label must be a boolean."}}
if(!(Test-Path -LiteralPath $ReceiptPath -PathType Leaf)){throw 'C5 retained platform observation receipt is missing.'}
$r=Get-Content -Raw $ReceiptPath|ConvertFrom-Json
Assert-Properties $r @('schema_version','receipt_id','semantic_receipt_sha256','source_commit','tracked_tree_manifest_sha256','bounded_source_manifest_sha256','rustc','cargo','observations','independent_second_platform_execution','promotion_authority','c6_authority') 'receipt root'
Assert-JsonInteger $r.schema_version 'schema_version'
Assert-JsonBoolean $r.independent_second_platform_execution 'independent_second_platform_execution'
Assert-JsonBoolean $r.promotion_authority 'promotion_authority'
Assert-JsonBoolean $r.c6_authority 'c6_authority'
if($r.schema_version-ne1-or$r.receipt_id-ne'G1-C5-LOCAL-PLATFORM-OBSERVATIONS'-or$r.semantic_receipt_sha256-ne$SemanticSha256-or$r.bounded_source_manifest_sha256-ne$BoundedSourceManifestSha256-or$r.independent_second_platform_execution-ne$false-or$r.promotion_authority-ne$false-or$r.c6_authority-ne$false){throw 'C5 platform receipt root binding changed.'}
if($r.source_commit-notmatch'^[0-9a-f]{40}$'-or$r.tracked_tree_manifest_sha256-notmatch'^[0-9a-f]{64}$'){throw 'C5 platform source provenance is malformed.'}
git cat-file -e "$($r.source_commit)^{commit}"
if($LASTEXITCODE-ne0){throw 'C5 platform source commit is unavailable.'}
$treeLines=@(git -c core.quotePath=true ls-tree -r --full-tree $r.source_commit)
if($LASTEXITCODE-ne0-or!$treeLines.Count){throw 'C5 tracked-tree manifest is unavailable.'}
# Git tree rows are canonically joined with LF and deliberately have no terminal newline.
$tree=$treeLines-join"`n"
$sha=[Security.Cryptography.SHA256]::Create()
try{$actual=([BitConverter]::ToString($sha.ComputeHash([Text.Encoding]::UTF8.GetBytes($tree))).Replace('-','').ToLowerInvariant())}finally{$sha.Dispose()}
if($actual-ne$r.tracked_tree_manifest_sha256){throw 'C5 tracked-tree manifest binding changed.'}
git merge-base --is-ancestor $r.source_commit HEAD
if($LASTEXITCODE-ne0){throw 'C5 platform source commit is not an ancestor of the verification commit.'}
if($r.rustc-ne$Rustc-or$r.cargo-ne$Cargo-or@($r.observations).Count-ne3){throw 'C5 platform toolchain or observation count changed.'}
$native=$r.observations[0];$i686=$r.observations[1];$android=$r.observations[2]
foreach($entry in @($native,$i686,$android)){Assert-Properties $entry @('classification','target','os','architecture','pointer_width','endian','runner','command','exit_code','process_ids','stdout_sha256','stderr_sha256','executable') 'observation'}
Assert-Properties $native.executable @('present','sha256') 'native executable'
Assert-Properties $i686.executable @('present','sha256') 'i686 executable'
Assert-Properties $android.executable @('present','reason') 'Android executable'
foreach($entry in @($native,$i686,$android)){Assert-JsonInteger $entry.pointer_width 'pointer_width';Assert-JsonInteger $entry.exit_code 'exit_code';foreach($processId in @($entry.process_ids)){Assert-JsonInteger $processId 'process_id'}}
Assert-JsonBoolean $native.executable.present 'native executable present'
Assert-JsonBoolean $i686.executable.present 'i686 executable present'
Assert-JsonBoolean $android.executable.present 'Android executable present'
# Local MSVC PE hashes identify the actual observed artifacts. Clean builds in distinct target
# directories need not be byte-identical, so fresh hashes are shape-checked rather than falsely
# asserted equal to the retained hashes; semantic replay carries the portable claim.
if($NativeExecutable-notmatch'^[0-9a-f]{64}$'-or$I686Executable-notmatch'^[0-9a-f]{64}$'){throw 'C5 current platform executable identity is malformed.'}
if($native.classification-ne'native_same_host_execution'-or$native.target-ne'x86_64-pc-windows-msvc'-or$native.os-ne'windows'-or$native.architecture-ne'x86_64'-or$native.pointer_width-ne64-or$native.endian-ne'little'-or$native.runner-ne'cargo-native'-or$native.command-ne$NativeCommand-or$native.exit_code-ne0-or@($native.process_ids|Where-Object{$_-isnot[int]-or$_-le0}).Count-ne0-or@($native.process_ids|Select-Object -Unique).Count-ne2-or@($native.stdout_sha256).Count-ne2-or@($native.stdout_sha256|Where-Object{$_-ne$NativeStdout}).Count-ne0-or@($native.stderr_sha256).Count-ne2-or@($native.stderr_sha256|Where-Object{$_-ne$NativeStderr}).Count-ne0-or!$native.executable.present-or$native.executable.sha256-notmatch'^[0-9a-f]{64}$'){throw 'C5 native platform observation is invalid.'}
if($i686.classification-ne'same_host_second_architecture'-or$i686.target-ne'i686-pc-windows-msvc'-or$i686.os-ne'windows'-or$i686.architecture-ne'i686'-or$i686.pointer_width-ne32-or$i686.endian-ne'little'-or$i686.runner-ne'cargo-i686'-or$i686.command-ne$I686Command-or$i686.exit_code-ne0-or@($i686.process_ids|Where-Object{$_-isnot[int]-or$_-le0}).Count-ne0-or@($i686.process_ids).Count-ne1-or@($i686.stdout_sha256).Count-ne1-or$i686.stdout_sha256[0]-ne$I686Stdout-or@($i686.stderr_sha256).Count-ne1-or$i686.stderr_sha256[0]-ne$I686Stderr-or!$i686.executable.present-or$i686.executable.sha256-notmatch'^[0-9a-f]{64}$'){throw 'C5 i686 observation is invalid.'}
if($android.classification-ne'compile_only'-or$android.target-ne'aarch64-linux-android'-or$android.os-ne'android'-or$android.architecture-ne'aarch64'-or$android.pointer_width-ne64-or$android.endian-ne'little'-or$android.runner-ne'cargo-check'-or$android.command-ne$AndroidCommand-or$android.exit_code-ne0-or@($android.process_ids|Where-Object{$_-isnot[int]-or$_-le0}).Count-ne0-or@($android.process_ids).Count-ne1-or@($android.stdout_sha256).Count-ne1-or$android.stdout_sha256[0]-ne$AndroidStdout-or@($android.stderr_sha256).Count-ne1-or$android.stderr_sha256[0]-ne$AndroidStderr-or$android.executable.present-or$android.executable.reason-ne'cargo check produces metadata only; no execution occurred'){throw 'C5 Android compile-only observation is invalid.'}
Write-Output "C5 retained platform observations verified: commit $($r.source_commit), semantic $SemanticSha256, no independent execution and no C6 authority."
