$ErrorActionPreference='Stop'
function Test-C5Observation($o,$expectedSemantic,$expectedSource,$expectedTarget,$expectedRunner){
  if([int]$o.process_count-lt2){throw 'portability.single-process'}
  if(@($o.stdout_sha256|Select-Object -Unique).Count-ne1-or$o.semantic_sha256-ne$expectedSemantic){throw 'portability.stdout-mismatch'}
  if($o.source_manifest_sha256-ne$expectedSource){throw 'portability.source-mismatch'}
  if($o.target-ne$expectedTarget){throw 'portability.target-drift'}
  if($o.runner-ne$expectedRunner){throw 'portability.runner-drift'}
  if($o.mode-eq'compile_only'){return 'compile_only'}
  if($o.mode-ne'execution'){throw 'invalid mode'}
  if($o.host_id-eq$o.reference_host_id){
    if($o.target-eq'x86_64-pc-windows-msvc'){return 'native_same_host_execution'}
    if($o.target-eq'i686-pc-windows-msvc'){return 'same_host_second_architecture'}
    throw 'portability.same-host-target-unsupported'
  }
  if($o.os_family-eq$o.reference_os_family-and$o.architecture-eq$o.reference_architecture){return 'same_platform_remote'}
  'independent_platform_execution'
}
$base=[pscustomobject]@{process_count=2;stdout_sha256=@('a','a');semantic_sha256='s';source_manifest_sha256='m';target='x86_64-pc-windows-msvc';runner='cargo-native';mode='execution';host_id='host-a';reference_host_id='host-a';os_family='windows';reference_os_family='windows';architecture='x86_64';reference_architecture='x86_64'}
function Copy-O($o){$o|ConvertTo-Json -Depth 5|ConvertFrom-Json}
if((Test-C5Observation $base s m 'x86_64-pc-windows-msvc' 'cargo-native')-ne'native_same_host_execution'){throw 'portability.native-misclassified'}
$x=Copy-O $base;$x.target='i686-pc-windows-msvc';$x.runner='cargo-i686';$x.architecture='i686';if((Test-C5Observation $x s m 'i686-pc-windows-msvc' 'cargo-i686')-ne'same_host_second_architecture'){throw 'portability.i686-misclassified'}
$x=Copy-O $base;$x.target='aarch64-linux-android';$x.runner='cargo-check';$x.mode='compile_only';$x.architecture='aarch64';$x.os_family='android';if((Test-C5Observation $x s m 'aarch64-linux-android' 'cargo-check')-ne'compile_only'){throw 'portability.compile-as-execution'}
$x=Copy-O $base;$x.process_count=1;try{Test-C5Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.single-process admitted'}catch{if($_.Exception.Message-ne'portability.single-process'){throw}}
$x=Copy-O $base;$x.stdout_sha256=@('a','b');try{Test-C5Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.stdout-mismatch admitted'}catch{if($_.Exception.Message-ne'portability.stdout-mismatch'){throw}}
$x=Copy-O $base;$x.source_manifest_sha256='x';try{Test-C5Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.source-mismatch admitted'}catch{if($_.Exception.Message-ne'portability.source-mismatch'){throw}}
$x=Copy-O $base;$x.host_id='host-b';if((Test-C5Observation $x s m $base.target $base.runner)-ne'same_platform_remote'){throw 'portability.same-platform-remote'}
$x=Copy-O $base;$x.target='i686-pc-windows-msvc';try{Test-C5Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.target-drift admitted'}catch{if($_.Exception.Message-ne'portability.target-drift'){throw}}
$x=Copy-O $base;$x.runner='other';try{Test-C5Observation $x s m $base.target $base.runner|Out-Null;throw 'portability.runner-drift admitted'}catch{if($_.Exception.Message-ne'portability.runner-drift'){throw}}
$x=Copy-O $base;$x.target='thumbv7em-none-eabi';try{Test-C5Observation $x s m $x.target $base.runner|Out-Null;throw 'portability.same-host-target-unsupported admitted'}catch{if($_.Exception.Message-ne'portability.same-host-target-unsupported'){throw}}
Write-Output 'C5 portability classifier verified: exact native x64, same-host i686, and Android compile-only classifications plus seven fail-closed hostile observations.'
