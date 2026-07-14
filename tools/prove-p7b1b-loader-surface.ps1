[CmdletBinding()]
param(
    [string]$OutputPath = 'evidence\p7b1b\loader-surface-proof.json',
    [switch]$SelfTest
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
$root = Split-Path -Parent $PSScriptRoot
$expectedOutput = Join-Path $root 'evidence\p7b1b\loader-surface-proof.json'
$candidates = @(
    [ordered]@{ name = 'dynamic'; path = 'target\p7b1b-startup-proof\dynamic\x86_64-pc-windows-msvc\debug\containment-denial-canary.exe'; sha256 = 'b1319077ce29984c50ea84d52f775bb7a0b0e868744c9a42e86d10d6167bcb66' },
    [ordered]@{ name = 'static'; path = 'target\p7b1b-startup-proof\static\x86_64-pc-windows-msvc\debug\containment-denial-canary.exe'; sha256 = '25109e5e9c4de262b4dc4020f2b75b6ba6515c3c6570fad575e131d7bf820856' }
)

function Fail([string]$Message) { throw "PE parser rejected input: $Message" }
function Assert-Range([byte[]]$Bytes, [long]$Offset, [long]$Length, [string]$What) {
    if ($Offset -lt 0 -or $Length -lt 0 -or $Offset -gt $Bytes.LongLength -or $Length -gt ($Bytes.LongLength - $Offset)) { Fail "$What is out of range" }
}
function U16([byte[]]$Bytes, [long]$Offset, [string]$What = 'u16') { Assert-Range $Bytes $Offset 2 $What; [BitConverter]::ToUInt16($Bytes, [int]$Offset) }
function U32([byte[]]$Bytes, [long]$Offset, [string]$What = 'u32') { Assert-Range $Bytes $Offset 4 $What; [BitConverter]::ToUInt32($Bytes, [int]$Offset) }
function U64([byte[]]$Bytes, [long]$Offset, [string]$What = 'u64') { Assert-Range $Bytes $Offset 8 $What; [BitConverter]::ToUInt64($Bytes, [int]$Offset) }
function Hex32([uint32]$Value) { '0x{0:x8}' -f $Value }
function Hash-Bytes([byte[]]$Bytes) { ([Security.Cryptography.SHA256]::Create().ComputeHash($Bytes) | ForEach-Object { $_.ToString('x2') }) -join '' }
function Slice([byte[]]$Bytes, [long]$Offset, [long]$Length, [string]$What) {
    Assert-Range $Bytes $Offset $Length $What
    $result = New-Object byte[] ([int]$Length)
    if ($Length -gt 0) { [Array]::Copy($Bytes, $Offset, $result, 0, $Length) }
    $result
}
function Ascii-Z([byte[]]$Bytes, [long]$Offset, [string]$What) {
    Assert-Range $Bytes $Offset 1 $What
    $end = $Offset
    while ($end -lt $Bytes.LongLength -and $Bytes[$end] -ne 0) {
        if ($end - $Offset -ge 4096) { Fail "$What exceeds string bound" }
        $value = $Bytes[$end]
        if ($value -lt 0x20 -or $value -gt 0x7e) { Fail "$What is not printable ASCII" }
        $end++
    }
    if ($end -ge $Bytes.LongLength) { Fail "$What is unterminated" }
    [Text.Encoding]::ASCII.GetString($Bytes, [int]$Offset, [int]($end - $Offset))
}
function Resolve-Rva([byte[]]$Bytes, [uint32]$Rva, [long]$Length, $Pe, [string]$What) {
    if ($Length -lt 0) { Fail "$What has a negative length" }
    if ([uint64]$Rva + [uint64]$Length -gt [uint64]$Pe.size_of_image) { Fail "$What exceeds SizeOfImage" }
    $matches = @()
    if ([uint64]$Rva + [uint64]$Length -le [uint64]$Pe.size_of_headers) { $matches += [long]$Rva }
    foreach ($section in $Pe.sections) {
        $span = [Math]::Max([uint64]$section.virtual_size, [uint64]$section.raw_size)
        if ([uint64]$Rva -ge [uint64]$section.virtual_address -and [uint64]$Rva + [uint64]$Length -le [uint64]$section.virtual_address + $span) {
            $delta = [uint64]$Rva - [uint64]$section.virtual_address
            if ($delta + [uint64]$Length -gt [uint64]$section.raw_size) { Fail "$What is virtual-only or raw-truncated" }
            $matches += [long]([uint64]$section.raw_pointer + $delta)
        }
    }
    if ($matches.Count -ne 1) { Fail "$What RVA maps ambiguously or not at all" }
    Assert-Range $Bytes $matches[0] $Length $What
    $matches[0]
}
function Read-Directories([byte[]]$Bytes, [long]$Optional, [uint16]$OptionalSize) {
    if ($OptionalSize -lt 112) { Fail 'optional header is too small for PE32+ directories' }
    $count = U32 $Bytes ($Optional + 108) 'NumberOfRvaAndSizes'
    if ($count -gt 16) { $count = 16 }
    $dirs = @()
    for ($i = 0; $i -lt 16; $i++) {
        if ($i -lt $count -and (112 + (($i + 1) * 8)) -le $OptionalSize) {
            $dirs += ,([ordered]@{ rva = U32 $Bytes ($Optional + 112 + $i * 8) "directory $i RVA"; size = U32 $Bytes ($Optional + 116 + $i * 8) "directory $i size" })
        } else { $dirs += ,([ordered]@{ rva = [uint32]0; size = [uint32]0 }) }
    }
    $dirs
}
function Read-Imports([byte[]]$Bytes, $Pe, [int]$Index, [bool]$Delay) {
    $dir = $Pe.directories[$Index]
    if ($dir.rva -eq 0 -and $dir.size -eq 0) { return @() }
    if ($dir.rva -eq 0 -or $dir.size -eq 0) { Fail "directory $Index has only one zero member" }
    $entrySize = if ($Delay) { 32 } else { 20 }
    if ($dir.size -lt $entrySize -or ($dir.size % $entrySize) -ne 0) { Fail "directory $Index has an invalid descriptor size" }
    $base = Resolve-Rva $Bytes $dir.rva $dir.size $Pe "directory $Index"
    $result = @(); $terminated = $false
    for ($i = 0; $i -lt [int]($dir.size / $entrySize); $i++) {
        $off = $base + $i * $entrySize
        $allZero = $true
        for ($j = 0; $j -lt $entrySize; $j += 4) { if ((U32 $Bytes ($off + $j) 'descriptor member') -ne 0) { $allZero = $false } }
        if ($allZero) { $terminated = $true; break }
        if ($Delay) {
            $attributes = U32 $Bytes $off 'delay attributes'
            if (($attributes -band 1) -ne 1 -or ($attributes -band 0xfffffffe) -ne 0) { Fail 'unknown delay-import pointer layout' }
            $nameRva = U32 $Bytes ($off + 4) 'delay DLL name'
            $lookupRva = U32 $Bytes ($off + 16) 'delay lookup table'
            if ($lookupRva -eq 0) { $lookupRva = U32 $Bytes ($off + 12) 'delay IAT' }
        } else {
            $lookupRva = U32 $Bytes $off 'import lookup table'
            $nameRva = U32 $Bytes ($off + 12) 'import DLL name'
            if ($lookupRva -eq 0) { $lookupRva = U32 $Bytes ($off + 16) 'import address table' }
        }
        if ($nameRva -eq 0 -or $lookupRva -eq 0) { Fail 'non-null import descriptor lacks a name or thunk table' }
        $dll = (Ascii-Z $Bytes (Resolve-Rva $Bytes $nameRva 1 $Pe 'import DLL name') 'import DLL name').ToLowerInvariant()
        $functions = @(); $thunkOff = Resolve-Rva $Bytes $lookupRva 8 $Pe 'import thunk table'; $thunkTerminated = $false
        for ($t = 0; $t -lt 65536; $t++) {
            Assert-Range $Bytes ($thunkOff + $t * 8) 8 'import thunk'
            $value = U64 $Bytes ($thunkOff + $t * 8) 'import thunk'
            if ($value -eq 0) { $thunkTerminated = $true; break }
            $ordinalFlag = [Convert]::ToUInt64('8000000000000000', 16)
            if (($value -band $ordinalFlag) -ne 0) { $functions += ('ordinal:{0}' -f ($value -band 0xffff)) }
            else {
                if ($value -gt [uint64][uint32]::MaxValue) { Fail 'import name RVA exceeds PE32+ RVA width' }
                $nameOff = Resolve-Rva $Bytes ([uint32]$value) 3 $Pe 'import-by-name'
                [void](U16 $Bytes $nameOff 'import hint')
                $functions += Ascii-Z $Bytes ($nameOff + 2) 'import function name'
            }
        }
        if (!$thunkTerminated) { Fail 'import thunk table lacks a terminator' }
        $result += ,([ordered]@{ dll = $dll; functions = @($functions | Sort-Object -Unique) })
    }
    if (!$terminated) { Fail "directory $Index lacks a null descriptor" }
    $byDll = @{}
    foreach ($item in $result) { if (!$byDll.ContainsKey($item.dll)) { $byDll[$item.dll] = @() }; $byDll[$item.dll] += @($item.functions) }
    $merged = @()
    foreach ($dllName in @($byDll.Keys | Sort-Object)) { $merged += ,([ordered]@{ dll = $dllName; functions = @($byDll[$dllName] | Sort-Object -Unique) }) }
    $merged
}
function Read-Resources([byte[]]$Bytes, $Pe) {
    $dir = $Pe.directories[2]
    if ($dir.rva -eq 0 -and $dir.size -eq 0) { return [ordered]@{ directory_sha256 = $null; manifests = @() } }
    if ($dir.rva -eq 0 -or $dir.size -lt 16) { Fail 'resource directory is malformed' }
    $base = Resolve-Rva $Bytes $dir.rva $dir.size $Pe 'resource directory'
    $blob = Slice $Bytes $base $dir.size 'resource directory'
    $manifests = @(); $visited = @{}
    function Walk([uint32]$Relative, [int]$Depth, [string[]]$Ids) {
        if ($Depth -gt 3 -or $Relative -gt $dir.size -or 16 -gt ($dir.size - $Relative)) { Fail 'resource tree is out of range or too deep' }
        $key = "$Depth/$Relative/$($Ids -join '/')"; if ($visited.ContainsKey($key)) { Fail 'resource tree cycle detected' }; $visited[$key] = $true
        $off = $base + $Relative
        $named = U16 $Bytes ($off + 12) 'resource named count'; $numeric = U16 $Bytes ($off + 14) 'resource id count'
        $count = [int]$named + [int]$numeric
        if ($count -gt 4096 -or 8 * $count -gt ($dir.size - $Relative - 16)) { Fail 'resource entries exceed directory bounds' }
        for ($i = 0; $i -lt $count; $i++) {
            $entry = $off + 16 + $i * 8; $name = U32 $Bytes $entry 'resource name'; $child = U32 $Bytes ($entry + 4) 'resource child'
            $id = if (($name -band 0x80000000) -ne 0) { 'name@0x{0:x8}' -f ($name -band 0x7fffffff) } else { [string]$name }
            $nextIds = @($Ids) + $id
            if (($child -band 0x80000000) -ne 0) { Walk ([uint32]($child -band 0x7fffffff)) ($Depth + 1) $nextIds }
            else {
                if ($Depth -ne 2 -or $child -gt $dir.size -or 16 -gt ($dir.size - $child)) { Fail 'resource data entry is misplaced or out of range' }
                $dataOff = $base + $child; $dataRva = U32 $Bytes $dataOff 'resource data RVA'; $dataSize = U32 $Bytes ($dataOff + 4) 'resource data size'
                $contentOff = Resolve-Rva $Bytes $dataRva $dataSize $Pe 'resource data'
                if ($nextIds[0] -eq '24') { $manifests += ,([ordered]@{ id = $nextIds[1]; language = $nextIds[2]; size = $dataSize; sha256 = Hash-Bytes (Slice $Bytes $contentOff $dataSize 'manifest data') }) }
            }
        }
    }
    Walk 0 0 @()
    [ordered]@{ directory_sha256 = Hash-Bytes $blob; manifests = @($manifests | Sort-Object id, language) }
}
function Parse-Pe([byte[]]$Bytes) {
    Assert-Range $Bytes 0 64 'DOS header'
    if ($Bytes[0] -ne 0x4d -or $Bytes[1] -ne 0x5a) { Fail 'DOS signature is not MZ' }
    $peOff = U32 $Bytes 0x3c 'e_lfanew'; Assert-Range $Bytes $peOff 24 'PE header'
    if ((U32 $Bytes $peOff 'PE signature') -ne 0x00004550) { Fail 'PE signature is invalid' }
    $machine = U16 $Bytes ($peOff + 4) 'machine'; if ($machine -ne 0x8664) { Fail 'machine is not x86-64' }
    $sectionCount = U16 $Bytes ($peOff + 6) 'section count'; if ($sectionCount -lt 1 -or $sectionCount -gt 96) { Fail 'section count is invalid' }
    $optionalSize = U16 $Bytes ($peOff + 20) 'optional header size'; $optional = $peOff + 24
    Assert-Range $Bytes $optional $optionalSize 'optional header'
    $magic = U16 $Bytes $optional 'optional magic'; if ($magic -ne 0x20b) { Fail 'image is not PE32+' }
    $sectionsOff = $optional + $optionalSize; Assert-Range $Bytes $sectionsOff (40 * $sectionCount) 'section table'
    $sizeImage = U32 $Bytes ($optional + 56) 'SizeOfImage'; $sizeHeaders = U32 $Bytes ($optional + 60) 'SizeOfHeaders'
    if ($sizeImage -eq 0 -or $sizeHeaders -eq 0 -or $sizeHeaders -gt $Bytes.Length) { Fail 'image/header size is invalid' }
    $sections = @(); $rawRanges = @()
    for ($i = 0; $i -lt $sectionCount; $i++) {
        $off = $sectionsOff + $i * 40; $nameBytes = Slice $Bytes $off 8 'section name'; $zero = [Array]::IndexOf($nameBytes, [byte]0); if ($zero -lt 0) { $zero = 8 }
        $name = [Text.Encoding]::ASCII.GetString($nameBytes, 0, $zero)
        $virtualSize = U32 $Bytes ($off + 8) 'section virtual size'; $virtualAddress = U32 $Bytes ($off + 12) 'section RVA'
        $rawSize = U32 $Bytes ($off + 16) 'section raw size'; $rawPointer = U32 $Bytes ($off + 20) 'section raw pointer'
        if ($rawSize -gt 0) {
            Assert-Range $Bytes $rawPointer $rawSize 'section raw data'
            foreach ($range in $rawRanges) { if ([uint64]$rawPointer -lt $range.end -and [uint64]$rawPointer + $rawSize -gt $range.start) { Fail 'section raw ranges overlap' } }
            $rawRanges += ,@{ start = [uint64]$rawPointer; end = [uint64]$rawPointer + $rawSize }
        }
        $sections += ,([ordered]@{ name = $name; virtual_address = Hex32 $virtualAddress; virtual_size = $virtualSize; raw_pointer = $rawPointer; raw_size = $rawSize; characteristics = Hex32 (U32 $Bytes ($off + 36) 'section characteristics') })
    }
    $pe = [ordered]@{ size_of_image = $sizeImage; size_of_headers = $sizeHeaders; sections = $sections; directories = Read-Directories $Bytes $optional $optionalSize }
    $imports = @(Read-Imports $Bytes $pe 1 $false); $delay = @(Read-Imports $Bytes $pe 13 $true)
    $tls = $pe.directories[9]; $callbacks = @()
    if ($tls.rva -ne 0 -or $tls.size -ne 0) {
        if ($tls.rva -eq 0 -or $tls.size -lt 40) { Fail 'TLS directory is malformed' }
        $tlsOff = Resolve-Rva $Bytes $tls.rva 40 $pe 'TLS directory'; $callbackVa = U64 $Bytes ($tlsOff + 24) 'TLS callback VA'
        if ($callbackVa -ne 0) {
            $imageBase = U64 $Bytes ($optional + 24) 'ImageBase'; if ($callbackVa -lt $imageBase -or $callbackVa - $imageBase -gt [uint32]::MaxValue) { Fail 'TLS callback array VA is invalid' }
            $callbackRva = [uint32]($callbackVa - $imageBase); $callbackOff = Resolve-Rva $Bytes $callbackRva 8 $pe 'TLS callback array'; $done = $false
            for ($i = 0; $i -lt 4096; $i++) { $va = U64 $Bytes ($callbackOff + $i * 8) 'TLS callback'; if ($va -eq 0) { $done = $true; break }; if ($va -lt $imageBase -or $va - $imageBase -ge $sizeImage) { Fail 'TLS callback target is outside image' }; $callbacks += ('0x{0:x16}' -f $va) }
            if (!$done) { Fail 'TLS callback array lacks a terminator' }
        }
    }
    $load = $pe.directories[10]; $loadResult = [ordered]@{ size = 0; declared_size = 0; sha256 = $null }
    if ($load.rva -ne 0 -or $load.size -ne 0) {
        if ($load.rva -eq 0 -or $load.size -lt 4) { Fail 'load-config directory is malformed' }
        $loadOff = Resolve-Rva $Bytes $load.rva $load.size $pe 'load-config directory'; $declared = U32 $Bytes $loadOff 'load-config declared size'
        if ($declared -lt 4 -or $declared -gt $load.size) { Fail 'load-config declared size is unsupported' }
        $loadResult = [ordered]@{ size = $load.size; declared_size = $declared; sha256 = Hash-Bytes (Slice $Bytes $loadOff $load.size 'load-config directory') }
    }
    [ordered]@{
        pe_magic = '0x020b'; machine = '0x8664'; entry_point_rva = Hex32 (U32 $Bytes ($optional + 16) 'entry point')
        size_of_image = $sizeImage; size_of_headers = $sizeHeaders; sections = $sections
        imports = $imports; delay_imports = $delay; tls = [ordered]@{ callback_count = $callbacks.Count; callback_vas = $callbacks }
        load_config = $loadResult; resources = Read-Resources $Bytes $pe
    }
}
function Assert-LeafNoReparse([string]$Path) {
    $full = [IO.Path]::GetFullPath($Path); $rootFull = [IO.Path]::GetFullPath($root).TrimEnd('\') + '\'
    if (!$full.StartsWith($rootFull, [StringComparison]::OrdinalIgnoreCase)) { throw "Candidate must be under repository root: $Path" }
    $cursor = Get-Item -LiteralPath $full -Force
    while ($null -ne $cursor -and $cursor.FullName.StartsWith($rootFull.TrimEnd('\'), [StringComparison]::OrdinalIgnoreCase)) {
        if (($cursor.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw "ReparsePoint rejected: $($cursor.FullName)" }
        if ($cursor -is [IO.FileInfo]) { $cursor = $cursor.Directory } else { $cursor = $cursor.Parent }
    }
    $full
}
function Run-SelfTest {
    $source = [IO.File]::ReadAllBytes((Join-Path $root $candidates[0].path)); [void](Parse-Pe $source)
    $cases = @()
    $cases += ,@{ name = 'truncation'; bytes = $source[0..127] }
    $peOff = [BitConverter]::ToUInt32($source, 0x3c); $optional = $peOff + 24
    $pe32 = [byte[]]$source.Clone(); [Array]::Copy([BitConverter]::GetBytes([uint16]0x10b), 0, $pe32, $optional, 2); $cases += ,@{ name = 'PE32'; bytes = $pe32 }
    $badPe = [byte[]]$source.Clone(); $badOffset = [Convert]::ToUInt32('fffffff0', 16); [Array]::Copy([BitConverter]::GetBytes($badOffset), 0, $badPe, 0x3c, 4); $cases += ,@{ name = 'e_lfanew'; bytes = $badPe }
    $sectionCount = [BitConverter]::ToUInt16($source, $peOff + 6); if ($sectionCount -gt 1) {
        $optionalSize = [BitConverter]::ToUInt16($source, $peOff + 20); $sectionTable = $optional + $optionalSize
        $overlap = [byte[]]$source.Clone(); [Array]::Copy($overlap, $sectionTable + 20, $overlap, $sectionTable + 60, 4); $cases += ,@{ name = 'overlap'; bytes = $overlap }
    }
    $badImport = [byte[]]$source.Clone(); $badRva = [Convert]::ToUInt32('fffffff0', 16); [Array]::Copy([BitConverter]::GetBytes($badRva), 0, $badImport, $optional + 120, 4); $cases += ,@{ name = 'import-rva'; bytes = $badImport }
    foreach ($case in $cases) { $rejected = $false; try { [void](Parse-Pe $case.bytes) } catch { $rejected = $true }; if (!$rejected) { throw "Hostile fixture was accepted: $($case.name)" } }
    Write-Output "P7b-1b loader-surface hostile fixtures rejected: $($cases.name -join ', ')."
}

if ($SelfTest) { Run-SelfTest; exit 0 }
$outputCandidate = if ([IO.Path]::IsPathRooted($OutputPath)) { $OutputPath } else { Join-Path (Get-Location) $OutputPath }
$resolvedOutput = [IO.Path]::GetFullPath($outputCandidate)
if (![string]::Equals($resolvedOutput, [IO.Path]::GetFullPath($expectedOutput), [StringComparison]::OrdinalIgnoreCase)) { throw "Output must be JSON under the exact evidence boundary: $expectedOutput" }
$outputParent = Split-Path -Parent $resolvedOutput
if (!(Test-Path -LiteralPath $outputParent -PathType Container)) { throw "Evidence directory is missing: $outputParent" }
if (((Get-Item -LiteralPath $outputParent -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'ReparsePoint rejected for evidence directory' }

$parsed = @()
foreach ($candidate in $candidates) {
    $full = Assert-LeafNoReparse (Join-Path $root $candidate.path); $bytes = [IO.File]::ReadAllBytes($full); $actual = Hash-Bytes $bytes
    if ($actual -ne $candidate.sha256) { throw "Candidate hash mismatch for $($candidate.name): $actual" }
    $parsed += ,([ordered]@{ name = $candidate.name; repository_path = $candidate.path.Replace('\','/'); sha256 = $actual; byte_length = $bytes.Length; image = Parse-Pe $bytes })
}
$toolHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $PSCommandPath).Hash.ToLowerInvariant()
$receipt = [ordered]@{
    schema = 1; proof = 'p7b1b-offline-pe-loader-surface'; status = 'completed_claim_limited'; parser_sha256 = $toolHash
    source_receipt = [ordered]@{ path = 'evidence/p7b1b/startup-compatibility-proof.json'; sha256 = '1123373704a528e86c81e3d32e16c1842d95ecd84002565e9b0fd1cb0b0e3585' }
    candidates = $parsed
    canary_executed = $false; profile_created = $false; registry_modified = $false; acl_modified = $false; capability_added = $false
    runtime_cause_proved = $false; denial_proved = $false
    conclusion = 'Static PE structure is bound; it does not identify the runtime DLL-init cause.'
}
$json = ($receipt | ConvertTo-Json -Depth 20) + "`n"; $utf8 = New-Object Text.UTF8Encoding($false); $newBytes = $utf8.GetBytes($json)
if (Test-Path -LiteralPath $resolvedOutput) {
    $existing = [IO.File]::ReadAllBytes($resolvedOutput)
    if ((Hash-Bytes $existing) -ne (Hash-Bytes $newBytes)) { throw 'Receipt overwrite rejected because existing bytes differ' }
    Write-Output "P7b-1b loader-surface receipt already matches: $resolvedOutput"
} else {
    [IO.File]::WriteAllBytes($resolvedOutput, $newBytes)
    Write-Output "P7b-1b loader-surface receipt written: $resolvedOutput"
}
