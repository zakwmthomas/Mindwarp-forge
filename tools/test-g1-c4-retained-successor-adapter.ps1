$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$adapter = Join-Path $PSScriptRoot 'invoke-g1-c4-retained-successor-adapter.ps1'
$checkpointSource = Join-Path $root 'context\active\WORKER_BATCH_STATE.json'
$programSource = Join-Path $root 'docs\canonical-system\MASTER_PROGRAM.json'
$observationSource = Join-Path $root 'docs\canonical-system\G1_C4_LOCAL_PLATFORM_OBSERVATIONS.json'
$temporary = Join-Path ([IO.Path]::GetTempPath()) ('forge-c4-successor-adapter-test-' + [guid]::NewGuid().ToString('N'))
$adapterScratch = Join-Path ([IO.Path]::GetTempPath()) ('fca-' + [guid]::NewGuid().ToString('N').Substring(0, 8))

function Invoke-Adapter([string]$Checkpoint, [string]$Program, [string]$ScriptName = 'verify-g1-c4-independent-platform-result.ps1') {
    $saved = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $adapter -ScriptName $ScriptName -CheckpointPath $Checkpoint -ProgramPath $Program -ObservationPath $script:observationPath -ValidateOnly 2>&1 | Out-Null
    $exit = $LASTEXITCODE
    $ErrorActionPreference = $saved
    return $exit
}

try {
    New-Item -ItemType Directory -Path $temporary | Out-Null
    New-Item -ItemType Directory -Path $adapterScratch | Out-Null
    $checkpointPath = Join-Path $temporary 'checkpoint.json'
    $programPath = Join-Path $temporary 'program.json'
    $script:observationPath = Join-Path $temporary 'observation.json'
    Copy-Item -LiteralPath $observationSource -Destination $script:observationPath
    $checkpoint = Get-Content -LiteralPath $checkpointSource -Raw | ConvertFrom-Json
    $program = Get-Content -LiteralPath $programSource -Raw | ConvertFrom-Json
    function Save-Fixture {
        $checkpoint | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $checkpointPath -Encoding utf8
        $program | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $programPath -Encoding utf8
    }
    function Assert-Rejected([string]$Label, [scriptblock]$Mutate, [scriptblock]$Restore) {
        & $Mutate
        Save-Fixture
        if ((Invoke-Adapter $checkpointPath $programPath) -eq 0) { throw "$Label was admitted." }
        & $Restore
    }

    Save-Fixture
    if ((Invoke-Adapter $checkpointPath $programPath) -ne 0) { throw 'Exact current C5 successor route was rejected.' }

    foreach ($field in @('batch_id', 'master_program_item', 'state', 'substage_id', 'authority_lane')) {
        $saved = $checkpoint.$field
        Assert-Rejected "Forged checkpoint $field" { $checkpoint.$field = 'forged' } { $checkpoint.$field = $saved }
    }
    $savedSubstage = $checkpoint.substage_id
    Assert-Rejected 'Legacy C5 substage' { $checkpoint.substage_id = 'c5-reconciliation-readiness' } { $checkpoint.substage_id = $savedSubstage }
    $savedAuthority = $checkpoint.authority_lane
    Assert-Rejected 'Authority suffix' { $checkpoint.authority_lane = $savedAuthority + ' Extra authority.' } { $checkpoint.authority_lane = $savedAuthority }

    $savedReceipts = @($checkpoint.verification_receipts)
    Assert-Rejected 'Missing ProofReceipt review' {
        $checkpoint.verification_receipts = @($savedReceipts | Where-Object { $_ -ne 'independent-review:c5-proofreceipt-integration:accepted' })
    } { $checkpoint.verification_receipts = $savedReceipts }
    Assert-Rejected 'Missing implementation owner release' {
        $checkpoint.verification_receipts = @($savedReceipts | Where-Object { $_ -ne 'owner-authorization:c5-frozen-implementation-candidate:released' })
    } { $checkpoint.verification_receipts = $savedReceipts }
    Assert-Rejected 'Missing predecessor full gate' {
        $checkpoint.verification_receipts = @($savedReceipts | Where-Object { $_ -ne 'registered-full-gate:run-71ef6dfd6e2945ab9745c85f3dcf4d6b:passed' })
    } { $checkpoint.verification_receipts = $savedReceipts }
    Assert-Rejected 'Duplicate retained receipt' {
        $checkpoint.verification_receipts = @($savedReceipts) + $savedReceipts[0]
    } { $checkpoint.verification_receipts = $savedReceipts }

    $c4 = @($program.items | Where-Object id -eq 'C4')[0]
    $c5 = @($program.items | Where-Object id -eq 'C5')[0]
    $c6 = @($program.items | Where-Object id -eq 'C6')[0]
    $savedC4Dependencies = @($c4.depends_on)
    Assert-Rejected 'Reordered C4 dependencies' { $c4.depends_on = @('C3A', 'C2') } { $c4.depends_on = $savedC4Dependencies }
    $savedC5Dependencies = @($c5.depends_on)
    Assert-Rejected 'Expanded C5 dependencies' { $c5.depends_on = @('C4', 'C3B') } { $c5.depends_on = $savedC5Dependencies }
    $savedC5State = $c5.state; $savedC5Status = $c5.status
    Assert-Rejected 'Extra active item' { $c5.state = 'executing'; $c5.status = 'active' } { $c5.state = $savedC5State; $c5.status = $savedC5Status }
    $savedC6Gate = $c6.gate
    Assert-Rejected 'C6 implementation gate downgraded' { $c6.gate = 'design' } { $c6.gate = $savedC6Gate }

    Save-Fixture
    $duplicateProgram = Get-Content -LiteralPath $programPath -Raw | ConvertFrom-Json
    $duplicateProgram.items = @($duplicateProgram.items) + @($duplicateProgram.items | Where-Object id -eq 'C5')[0]
    $duplicateProgram | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $programPath -Encoding utf8
    if ((Invoke-Adapter $checkpointPath $programPath) -eq 0) { throw 'Duplicate C5 program item was admitted.' }

    Save-Fixture
    $raw = Get-Content -LiteralPath $checkpointPath -Raw
    $duplicateKey = [regex]::Replace($raw, '"batch_id"\s*:', '"batch_id":"forged","batch_id":', 1)
    Set-Content -LiteralPath $checkpointPath -Value $duplicateKey -Encoding utf8
    if ((Invoke-Adapter $checkpointPath $programPath) -eq 0) { throw 'Duplicate checkpoint key was admitted.' }

    Save-Fixture
    $typed = Get-Content -LiteralPath $checkpointPath -Raw | ConvertFrom-Json
    $typed.state = $true
    $typed | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $checkpointPath -Encoding utf8
    if ((Invoke-Adapter $checkpointPath $programPath) -eq 0) { throw 'Boolean checkpoint state was admitted.' }

    Save-Fixture
    $observation = Get-Content -LiteralPath $observationSource -Raw | ConvertFrom-Json
    $observation.promotion_authority = $true
    $observation | ConvertTo-Json -Depth 20 | Set-Content -LiteralPath $script:observationPath -Encoding utf8
    if ((Invoke-Adapter $checkpointPath $programPath) -eq 0) { throw 'C4 observation promotion authority was admitted.' }
    Copy-Item -LiteralPath $observationSource -Destination $script:observationPath -Force
    $observation = Get-Content -LiteralPath $observationSource -Raw | ConvertFrom-Json
    $observation.observations[0].classification = 'forged'
    $observation | ConvertTo-Json -Depth 20 | Set-Content -LiteralPath $script:observationPath -Encoding utf8
    if ((Invoke-Adapter $checkpointPath $programPath) -eq 0) { throw 'Nested C4 observation drift was admitted.' }
    Copy-Item -LiteralPath $observationSource -Destination $script:observationPath -Force
    $observationRaw = Get-Content -LiteralPath $script:observationPath -Raw
    $duplicateObservation = [regex]::Replace($observationRaw, '"schema_version"\s*:', '"schema_version":2,"schema_version":', 1)
    Set-Content -LiteralPath $script:observationPath -Value $duplicateObservation -Encoding utf8
    if ((Invoke-Adapter $checkpointPath $programPath) -eq 0) { throw 'Duplicate C4 observation key was admitted.' }
    Copy-Item -LiteralPath $observationSource -Destination $script:observationPath -Force

    Save-Fixture
    if ((Invoke-Adapter $checkpointPath $programPath 'forged.ps1') -eq 0) { throw 'Unknown retained verifier name was admitted.' }

    . (Join-Path $PSScriptRoot 'verification-runner.ps1')
    $fakeRoot = Join-Path $temporary 'fake-tools'
    New-Item -ItemType Directory -Path $fakeRoot | Out-Null
    Set-Content -LiteralPath (Join-Path $fakeRoot 'verify-g1-c4-independent-platform-result.ps1') -Value 'exit 0' -Encoding utf8
    try {
        Invoke-ForgeVerifier -ScriptRoot $fakeRoot -ScriptName 'verify-g1-c4-independent-platform-result.ps1'
        throw 'Noncanonical retained verifier root was admitted.'
    } catch {
        if ($_.Exception.Message -eq 'Noncanonical retained verifier root was admitted.') { throw }
    }

    $before = @(Get-ChildItem -Path $adapterScratch -Directory -Filter 'forge-c4-retained-*' -ErrorAction SilentlyContinue |
        ForEach-Object { $_.FullName })
    & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $adapter -ScriptName 'verify-g1-c4-independent-platform-result.ps1' -TempBase $adapterScratch
    if ($LASTEXITCODE -ne 0) { throw 'Historical retained C4 replay failed through the adapter.' }
    $after = @(Get-ChildItem -Path $adapterScratch -Directory -Filter 'forge-c4-retained-*' -ErrorAction SilentlyContinue |
        ForEach-Object { $_.FullName })
    if (@(Compare-Object @($before) @($after)).Count -ne 0) { throw 'Historical retained C4 adapter left disposable clone residue.' }
} finally {
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse -Force }
    if (Test-Path -LiteralPath $adapterScratch) { Remove-Item -LiteralPath $adapterScratch -Recurse -Force }
}

Write-Output 'C4 retained successor adapter verified: exact route/types/authority/dependencies/receipts, historical replay and cleanup fail closed.'
