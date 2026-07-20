[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet(
        'verify-g1-c4-independent-platform-result.ps1',
        'test-g1-c4-independent-platform-result.ps1',
        'verify-g1-c4-closure-readiness.ps1',
        'verify-g1-c4-hierarchy-history-implementation.ps1'
    )]
    [string]$ScriptName,
    [string]$CheckpointPath,
    [string]$ProgramPath,
    [string]$ObservationPath,
    [string]$TempBase,
    [switch]$ValidateOnly
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$sourceCommit = '17f39f7018de8a02c8292bcde0fafa2bf58fc7d4'
$historicalReplayCommit = '51a67b222739f6ba0a51f151976ded4d52d76f55'
$historicalVerifyBlob = '21981e1ea9ea9c0117e322e0e0610e4d560edb3c'
$currentVerifyBlob = 'e6dad3bb7f651af745b06f0f95aa3b97d47efa5c'
$historicalCargoTomlBlob = 'c89e42751c45947cca539f2d02c2216e7b85cdda'
$currentCargoTomlBlob = '7b9d45f902b2b696d6e8fc7afecc1e8712d129a3'
$historicalCargoLockBlob = 'c25af5e0dffd87a8f2340b70ba4269a03945c3ee'
$currentCargoLockBlob = '2a36e57cfffce80167a1b71a0f975baed1c16da1'
$expectedObservationBlob = '02e962a7d4e9f83f506162cfb1b97e8717b18c10'
if ([string]::IsNullOrWhiteSpace($CheckpointPath)) {
    $CheckpointPath = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
}
if ([string]::IsNullOrWhiteSpace($ProgramPath)) {
    $ProgramPath = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
}
if ([string]::IsNullOrWhiteSpace($ObservationPath)) {
    $ObservationPath = Join-Path $root 'docs\canonical-system\G1_C4_LOCAL_PLATFORM_OBSERVATIONS.json'
}
foreach ($path in @($CheckpointPath, $ProgramPath, $ObservationPath)) {
    if (!(Test-Path -LiteralPath $path -PathType Leaf)) { throw "Adapter input is unavailable: $path" }
}

$bundledPython = Join-Path $env:USERPROFILE '.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe'
$python = if (Test-Path -LiteralPath $bundledPython -PathType Leaf) {
    $bundledPython
} elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
    'python3'
} else {
    throw 'Python runtime unavailable for strict C5 successor validation.'
}
& $python (Join-Path $PSScriptRoot 'verify-g1-c4-retained-successor-adapter.py') --checkpoint $CheckpointPath --program $ProgramPath --observation $ObservationPath
if ($LASTEXITCODE -ne 0) { throw 'Current C5 successor route validation failed.' }

$observationRelative = 'docs/canonical-system/G1_C4_LOCAL_PLATFORM_OBSERVATIONS.json'
$replayObservationBlob = (& git -C $root rev-parse "$historicalReplayCommit`:$observationRelative").Trim()
$headObservationBlob = (& git -C $root rev-parse "HEAD`:$observationRelative").Trim()
$workingObservationBlob = (& git -C $root hash-object -- $ObservationPath).Trim()
if ($LASTEXITCODE -ne 0 -or $replayObservationBlob -ne $expectedObservationBlob -or
    $headObservationBlob -ne $expectedObservationBlob -or $workingObservationBlob -ne $expectedObservationBlob) {
    throw 'Retained C4 observation is not the exact immutable canonical blob.'
}

& git -C $root merge-base --is-ancestor $sourceCommit HEAD
if ($LASTEXITCODE -ne 0) { throw 'Retained C4 source is not an ancestor of the current repository state.' }
& git -C $root merge-base --is-ancestor $sourceCommit $historicalReplayCommit
if ($LASTEXITCODE -ne 0) { throw 'Historical C4 replay snapshot does not descend from the retained C4 source.' }
& git -C $root merge-base --is-ancestor $historicalReplayCommit HEAD
if ($LASTEXITCODE -ne 0) { throw 'Historical C4 replay snapshot is not an ancestor of the current repository state.' }
$boundedText = & git -C $root show "$sourceCommit`:tools/fixtures/c4-hierarchy-history-receipt/bounded-paths.txt"
if ($LASTEXITCODE -ne 0) { throw 'Historical C4 bounded path list is unavailable.' }
$bounded = @(($boundedText -split "`r?`n") | Where-Object { ![string]::IsNullOrWhiteSpace($_) })
if ($bounded.Count -ne 27 -or @($bounded | Select-Object -Unique).Count -ne 27 -or
    $bounded -notcontains 'tools/fixtures/c4-hierarchy-history-receipt/bounded-paths.txt') {
    throw 'Historical C4 bounded path list is not exact, unique and self-binding.'
}
foreach ($relative in $bounded) {
    if ($relative -match '\\' -or $relative.StartsWith('/') -or
        @(($relative -split '/') | Where-Object { $_ -in @('', '.', '..') }).Count -ne 0) {
        throw "Historical C4 bounded path is noncanonical: $relative"
    }
    $full = Join-Path $root $relative
    if (!(Test-Path -LiteralPath $full -PathType Leaf) -or
        (Get-Item -LiteralPath $full -Force).Attributes.HasFlag([IO.FileAttributes]::ReparsePoint)) {
        throw "Current C4 bounded path is missing or redirected: $relative"
    }
    $historicalBlob = (& git -C $root rev-parse "$sourceCommit`:$relative").Trim()
    $replayBlob = (& git -C $root rev-parse "$historicalReplayCommit`:$relative").Trim()
    $headBlob = (& git -C $root rev-parse "HEAD`:$relative").Trim()
    $workingBlob = (& git -C $root hash-object -- $full).Trim()
    if ($LASTEXITCODE -ne 0 -or $historicalBlob -notmatch '^[0-9a-f]{40}$' -or
        $replayBlob -notmatch '^[0-9a-f]{40}$' -or
        $headBlob -notmatch '^[0-9a-f]{40}$' -or $workingBlob -notmatch '^[0-9a-f]{40}$') {
        throw "C4 bounded blob identity is unavailable: $relative"
    }
    if ($replayBlob -ne $historicalBlob) {
        throw "Historical closure replay changed the retained C4 bounded proof surface: $relative"
    }
    if ($workingBlob -ne $headBlob) { throw "C4 bounded path has uncommitted drift: $relative" }
    if ($relative -eq 'tools/verify.ps1') {
        if ($historicalBlob -ne $historicalVerifyBlob -or $headBlob -ne $currentVerifyBlob) {
            throw 'Outer verify.ps1 is not the one exact classified C5 closure-orchestration transition.'
        }
    } elseif ($relative -eq 'Cargo.toml') {
        if ($historicalBlob -ne $historicalCargoTomlBlob -or $headBlob -ne $currentCargoTomlBlob) {
            throw 'Workspace manifest is not the exact classified bounded C6 registration transition.'
        }
    } elseif ($relative -eq 'Cargo.lock') {
        if ($historicalBlob -ne $historicalCargoLockBlob -or $headBlob -ne $currentCargoLockBlob) {
            throw 'Workspace lockfile is not the exact classified bounded C6 registration transition.'
        }
    } elseif ($historicalBlob -ne $headBlob) {
        throw "Retained C4 bounded path drifted outside the classified outer orchestration delta: $relative"
    }
}

if ($ValidateOnly) {
    Write-Output 'C4 successor adapter validated: exact current C5 route plus historical C4 identity, two body-plan workspace registrations and one closure-orchestration delta.'
    return
}

$tempBasePath = if ([string]::IsNullOrWhiteSpace($TempBase)) { [IO.Path]::GetTempPath() } else { $TempBase }
if (!(Test-Path -LiteralPath $tempBasePath -PathType Container) -or
    (Get-Item -LiteralPath $tempBasePath -Force).Attributes.HasFlag([IO.FileAttributes]::ReparsePoint)) {
    throw 'Historical C4 adapter temp base is missing or redirected.'
}
$tempBase = [IO.Path]::GetFullPath($tempBasePath).TrimEnd('\') + '\'
$tempRoot = Join-Path $tempBase ('forge-c4-retained-' + [guid]::NewGuid().ToString('N'))
$clone = Join-Path $tempRoot 'source'
$priorPath = $env:Path
try {
    New-Item -ItemType Directory -Path $tempRoot | Out-Null
    if ((Get-Item -LiteralPath $tempRoot -Force).Attributes.HasFlag([IO.FileAttributes]::ReparsePoint)) {
        throw 'Historical C4 adapter temp root is redirected.'
    }
    & git -c core.hooksPath=NUL clone --quiet --shared --no-checkout $root $clone
    if ($LASTEXITCODE -ne 0) { throw 'Disposable historical C4 clone failed.' }
    & git -c core.hooksPath=NUL -c core.autocrlf=false -c core.eol=lf -C $clone checkout --quiet --detach $historicalReplayCommit
    if ($LASTEXITCODE -ne 0 -or (& git -C $clone rev-parse HEAD).Trim() -ne $historicalReplayCommit) {
        throw 'Disposable historical C4 checkout is not exact.'
    }
    if (@(& git -C $clone status --porcelain).Count -ne 0) { throw 'Historical C4 checkout is dirty before replay.' }

    $approvedGh = Join-Path $env:USERPROFILE '.local\github-cli-2.96.0\bin\gh.exe'
    $approvedGhSha256 = 'cd79f16203f1fbe56937c4c96e2b6eadd10549418dcb241d91576ac77af0ac8b'
    if (!(Test-Path -LiteralPath $approvedGh -PathType Leaf) -or
        (Get-FileHash -LiteralPath $approvedGh -Algorithm SHA256).Hash.ToLowerInvariant() -ne $approvedGhSha256) {
        throw 'Approved GitHub CLI identity is unavailable for retained C4 replay.'
    }
    $env:Path = (Split-Path -Parent $approvedGh) + ';' + $env:Path
    $resolvedGh = (Get-Command gh -CommandType Application -ErrorAction Stop).Source
    if (![IO.Path]::GetFullPath($resolvedGh).Equals([IO.Path]::GetFullPath($approvedGh), [StringComparison]::OrdinalIgnoreCase)) {
        throw 'GitHub CLI resolution escaped the approved executable identity.'
    }
    $historicalScript = Join-Path (Join-Path $clone 'tools') $ScriptName
    if ($ScriptName -eq 'verify-g1-c4-hierarchy-history-implementation.ps1') {
        $freshObservation = Join-Path $tempRoot 'fresh-local-platform-observation.json'
        Push-Location $clone
        try {
            & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $historicalScript -ObservationReceiptPath $freshObservation
        } finally {
            Pop-Location
        }
        if (!(Test-Path -LiteralPath $freshObservation -PathType Leaf)) {
            throw 'Disposable historical C4 hierarchy verification did not produce a fresh local observation.'
        }
    } else {
        & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $historicalScript
    }
    if ($LASTEXITCODE -ne 0) { throw "Historical retained C4 verifier failed: $ScriptName" }
    if (@(& git -C $clone status --porcelain).Count -ne 0) { throw 'Historical C4 checkout changed during replay.' }
    $replayKind = if ($ScriptName -eq 'verify-g1-c4-hierarchy-history-implementation.ps1') {
        'full verification with a disposable fresh observation bound to the closure snapshot executing the unchanged retained bounded surface'
    } else {
        'exact retained replay'
    }
    Write-Output "C4 retained successor adapter passed: current exact C5 route plus historical closure snapshot $historicalReplayCommit $replayKind for $ScriptName against retained source $sourceCommit."
} finally {
    $env:Path = $priorPath
    if (Test-Path -LiteralPath $tempRoot) {
        $resolved = [IO.Path]::GetFullPath($tempRoot)
        if (!$resolved.StartsWith($tempBase, [StringComparison]::OrdinalIgnoreCase) -or $resolved -eq $tempBase.TrimEnd('\')) {
            throw 'Refusing unsafe historical C4 cleanup target.'
        }
        if ((Get-Item -LiteralPath $resolved -Force).Attributes.HasFlag([IO.FileAttributes]::ReparsePoint)) {
            throw 'Refusing redirected historical C4 cleanup target.'
        }
        Remove-Item -LiteralPath $resolved -Recurse -Force -ErrorAction Stop
    }
}
