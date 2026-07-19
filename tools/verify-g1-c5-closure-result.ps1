$ErrorActionPreference='Stop'
$root=Split-Path -Parent $PSScriptRoot
& (Join-Path $PSScriptRoot 'verify-g1-c5-retained-successor.ps1') -Root $root
if(!$?){throw 'C5 retained successor classification failed.'}
$bundled='C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python=if(Test-Path -LiteralPath $bundled -PathType Leaf){$bundled}else{'python'}
$approvedGh=Join-Path $env:USERPROFILE '.local\github-cli-2.96.0\bin\gh.exe'
$approvedGhSha256='cd79f16203f1fbe56937c4c96e2b6eadd10549418dcb241d91576ac77af0ac8b'
if(!(Test-Path -LiteralPath $approvedGh -PathType Leaf)-or(Get-FileHash $approvedGh -Algorithm SHA256).Hash.ToLowerInvariant()-ne$approvedGhSha256){throw 'Approved GitHub CLI identity is unavailable for C5 closure replay.'}
$priorPath=$env:Path
try{
 $env:Path=(Split-Path -Parent $approvedGh)+';'+$env:Path
 & $python (Join-Path $PSScriptRoot 'verify-g1-c5-independent-platform-result.py') --receipt (Join-Path $root 'docs\canonical-system\G1_C5_INDEPENDENT_PLATFORM_EXECUTION.json')
 if($LASTEXITCODE-ne0){throw 'C5 closure did not replay the hosted independent receipt.'}
}finally{$env:Path=$priorPath}
& $python (Join-Path $PSScriptRoot 'verify-g1-c5-closure-result.py') --root $root
if($LASTEXITCODE-ne0){throw 'C5 closure result cross-record verification failed.'}
