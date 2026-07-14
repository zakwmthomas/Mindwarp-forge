$ErrorActionPreference = 'Stop'
$tool = Join-Path $PSScriptRoot 'forge-chat-visual.ps1'
$temp = Join-Path ([System.IO.Path]::GetTempPath()) ("forge-chat-visual-" + [guid]::NewGuid().ToString('N'))
[System.IO.Directory]::CreateDirectory($temp) | Out-Null
Add-Type -AssemblyName System.Drawing

function LowerHash([string]$Path) {
    return (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Write-CaptureManifest([string]$Path, [int]$ProcessId = 42, [string]$Handle = '9001') {
    $image = [System.Drawing.Image]::FromFile($Path)
    try {
        $manifest = [ordered]@{
            schema_version = 1
            artifact_kind = 'forge_window_capture'
            capture_method = 'test-fixture'
            chat_delivery_performed = $false
            window = [ordered]@{
                process_id = $ProcessId
                handle = $Handle
                title = 'Mind Warp Forge'
                left = 10
                top = 20
                width = $image.Width
                height = $image.Height
            }
            image = [ordered]@{
                file = [System.IO.Path]::GetFileName($Path)
                width = $image.Width
                height = $image.Height
                sha256 = LowerHash $Path
            }
        }
        $json = $manifest | ConvertTo-Json -Depth 8
        [System.IO.File]::WriteAllText("$Path.manifest.json", ($json + [Environment]::NewLine), (New-Object System.Text.UTF8Encoding($false)))
    }
    finally {
        $image.Dispose()
    }
}

function Expect-Rejection([scriptblock]$Action, [string]$Message) {
    $rejected = $false
    try { & $Action | Out-Null } catch { $rejected = $true }
    if (-not $rejected) { throw $Message }
}

try {
    $referencePath = Join-Path $temp 'reference.png'
    $alteredPath = Join-Path $temp 'altered.png'
    foreach ($fixture in @(
        @{ Path = $referencePath; Color = [System.Drawing.Color]::LimeGreen },
        @{ Path = $alteredPath; Color = [System.Drawing.Color]::OrangeRed }
    )) {
        $bitmap = New-Object System.Drawing.Bitmap 320, 180
        $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
        $brush = New-Object System.Drawing.SolidBrush $fixture.Color
        try {
            $graphics.FillRectangle($brush, 0, 0, 320, 180)
            $bitmap.Save($fixture.Path, [System.Drawing.Imaging.ImageFormat]::Png)
        }
        finally {
            $brush.Dispose()
            $graphics.Dispose()
            $bitmap.Dispose()
        }
        Write-CaptureManifest $fixture.Path
    }

    $comparisonPath = Join-Path $temp 'comparison.png'
    & $tool -ReferencePath $referencePath -AlteredPath $alteredPath -ComparisonPath $comparisonPath | Out-Null
    if (-not (Test-Path -LiteralPath $comparisonPath)) { throw 'Comparison image was not created.' }
    if (-not (Test-Path -LiteralPath "$comparisonPath.manifest.json")) { throw 'Comparison provenance manifest was not created.' }

    $comparison = [System.Drawing.Image]::FromFile($comparisonPath)
    try {
        if ($comparison.Width -ne 648 -or $comparison.Height -ne 244) {
            throw "Unexpected comparison dimensions: $($comparison.Width)x$($comparison.Height)."
        }
    }
    finally {
        $comparison.Dispose()
    }

    $manifest = Get-Content -LiteralPath "$comparisonPath.manifest.json" -Raw | ConvertFrom-Json
    if ($manifest.schema_version -ne 1 -or $manifest.artifact_kind -ne 'forge_visual_comparison') { throw 'Comparison manifest identity is invalid.' }
    if ($manifest.chat_delivery_performed -ne $false) { throw 'Local comparison falsely claims chat delivery.' }
    if ($manifest.window_identity.process_id -ne 42 -or $manifest.window_identity.handle -ne '9001' -or $manifest.window_identity.title -ne 'Mind Warp Forge') { throw 'Comparison manifest lost exact window identity.' }
    if ($manifest.reference.sha256 -ne (LowerHash $referencePath) -or $manifest.altered.sha256 -ne (LowerHash $alteredPath)) { throw 'Comparison manifest source hash is invalid.' }
    if ($manifest.comparison.sha256 -ne (LowerHash $comparisonPath) -or $manifest.comparison.width -ne 648 -or $manifest.comparison.height -ne 244) { throw 'Comparison manifest output proof is invalid.' }

    $identicalPath = Join-Path $temp 'identical.png'
    Copy-Item -LiteralPath $referencePath -Destination $identicalPath
    Write-CaptureManifest $identicalPath
    Expect-Rejection { & $tool -ReferencePath $referencePath -AlteredPath $identicalPath -ComparisonPath (Join-Path $temp 'identical-comparison.png') } 'Identical comparison inputs were accepted.'

    $missingManifestPath = Join-Path $temp 'missing-manifest.png'
    Copy-Item -LiteralPath $alteredPath -Destination $missingManifestPath
    Expect-Rejection { & $tool -ReferencePath $referencePath -AlteredPath $missingManifestPath -ComparisonPath (Join-Path $temp 'missing-manifest-comparison.png') } 'Comparison input without provenance was accepted.'

    Write-CaptureManifest $alteredPath 42 'DIFFERENT-HWND'
    Expect-Rejection { & $tool -ReferencePath $referencePath -AlteredPath $alteredPath -ComparisonPath (Join-Path $temp 'wrong-window-comparison.png') } 'Different Forge window identities were accepted.'
    Write-CaptureManifest $alteredPath

    $badDimensionsPath = Join-Path $temp 'bad-dimensions.png'
    Copy-Item -LiteralPath $alteredPath -Destination $badDimensionsPath
    Write-CaptureManifest $badDimensionsPath
    $badDimensionsManifest = Get-Content -LiteralPath "$badDimensionsPath.manifest.json" -Raw | ConvertFrom-Json
    $badDimensionsManifest.image.width = 999
    $badDimensionsManifest | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath "$badDimensionsPath.manifest.json" -Encoding UTF8
    Expect-Rejection { & $tool -ReferencePath $referencePath -AlteredPath $badDimensionsPath -ComparisonPath (Join-Path $temp 'bad-dimensions-comparison.png') } 'False capture dimensions were accepted.'

    Expect-Rejection { & $tool -ReferencePath $referencePath -AlteredPath $alteredPath -ComparisonPath $referencePath } 'Comparison was allowed to overwrite a source capture.'

    Expect-Rejection { & $tool -ProcessId 2147483647 -OutputPath (Join-Path $temp 'missing-pid.png') } 'Missing explicit PID was accepted.'
    Expect-Rejection { & $tool -WindowTitle ("Missing Forge " + [guid]::NewGuid().ToString('N')) -OutputPath (Join-Path $temp 'missing-title.png') } 'Missing exact title was accepted.'

    foreach ($required in @('PrintWindow','PW_RENDERFULLCONTENT','IsIconic','IsWindowVisible','GetSystemMetrics','OrdinalIgnoreCase','ambiguous across PIDs','chat_delivery_performed','forge_window_capture')) {
        if (-not (Select-String -LiteralPath $tool -SimpleMatch $required -Quiet)) {
            throw "Forge chat visual tool lacks required hardening behavior: $required"
        }
    }
    if (Select-String -LiteralPath $tool -Pattern '\$graphics\.CopyFromScreen\s*\(' -Quiet) {
        throw 'Forge chat visual tool still screen-scrapes potentially occluded pixels.'
    }

    Write-Output 'Forge chat visual verified: exact window selection, non-occluded capture, provenance manifests, distinct inputs, and no chat-delivery claim.'
}
finally {
    Remove-Item -LiteralPath $temp -Recurse -Force -ErrorAction SilentlyContinue
}
