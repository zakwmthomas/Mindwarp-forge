$ErrorActionPreference='Stop'
function Test-C4Observation($o,$expectedSemantic,$expectedSource,$expectedTarget,$expectedRunner){
  if([int]$o.process_count-lt2){throw 'portability.single-process'}
  if(@($o.stdout_sha256|Select-Object -Unique).Count-ne1-or$o.semantic_sha256-ne$expectedSemantic){throw 'portability.stdout-mismatch'}
  if($o.source_manifest_sha256-ne$expectedSource){throw 'portability.source-mismatch'}
  if($o.target-ne$expectedTarget){throw 'portability.target-drift'}
  if($o.runner-ne$expectedRunner){throw 'portability.runner-drift'}
  if($o.mode-eq'compile_only'){return 'compile_only'}
  if($o.mode-ne'execution'){throw 'invalid mode'}
  if($o.host_id-eq$o.reference_host_id){return 'same_host_second_architecture'}
  if($o.os_family-eq$o.reference_os_family-and$o.architecture-eq$o.reference_architecture){return 'same_platform_remote'}
  'independent_platform_execution'
}
$base=[pscustomobject]@{process_count=2;stdout_sha256=@('a','a');semantic_sha256='s';source_manifest_sha256='m';target='x86_64-pc-windows-msvc';runner='cargo-native';mode='execution';host_id='host-a';reference_host_id='host-a';os_family='windows';reference_os_family='windows';architecture='x86_64';reference_architecture='x86_64'}
function Copy-O($o){$o|ConvertTo-Json -Depth 5|ConvertFrom-Json}
$x=Copy-O $base;$x.process_count=1;try{Test-C4Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.single-process admitted'}catch{if($_.Exception.Message-ne'portability.single-process'){throw}}
$x=Copy-O $base;$x.stdout_sha256=@('a','b');try{Test-C4Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.stdout-mismatch admitted'}catch{if($_.Exception.Message-ne'portability.stdout-mismatch'){throw}}
$x=Copy-O $base;$x.source_manifest_sha256='x';try{Test-C4Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.source-mismatch admitted'}catch{if($_.Exception.Message-ne'portability.source-mismatch'){throw}}
$x=Copy-O $base;$x.mode='compile_only';if((Test-C4Observation $x s m $base.target $base.runner)-ne'compile_only'){throw 'portability.compile-as-execution'}
if((Test-C4Observation $base s m $base.target $base.runner)-eq'independent_platform_execution'){throw 'portability.same-host-as-independent'}
$x=Copy-O $base;$x.host_id='host-b';if((Test-C4Observation $x s m $base.target $base.runner)-ne'same_platform_remote'){throw 'portability.same-platform-remote'}
$x=Copy-O $base;$x.target='i686-pc-windows-msvc';try{Test-C4Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.target-drift admitted'}catch{if($_.Exception.Message-ne'portability.target-drift'){throw}}
$x=Copy-O $base;$x.runner='other';try{Test-C4Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.runner-drift admitted'}catch{if($_.Exception.Message-ne'portability.runner-drift'){throw}}
Write-Output 'C4 portability classifier verified: eight hostile observations fail closed or receive non-independent typed classifications.'
