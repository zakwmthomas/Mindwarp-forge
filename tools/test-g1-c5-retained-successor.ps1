$ErrorActionPreference='Stop'
$bundled='C:\Users\zakwm\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python=if(Test-Path -LiteralPath $bundled -PathType Leaf){$bundled}else{'python'}
& $python (Join-Path $PSScriptRoot 'test-g1-c5-retained-successor.py')
if($LASTEXITCODE-ne0){throw 'C5 retained-successor hostile fixtures failed.'}
