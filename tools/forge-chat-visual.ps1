[CmdletBinding(DefaultParameterSetName = 'Capture')]
param(
    [Parameter(ParameterSetName = 'Capture')]
    [ValidateSet('Capture')]
    [string]$Mode = 'Capture',

    [Parameter(ParameterSetName = 'Capture')]
    [string]$WindowTitle = 'Mind Warp Forge',

    [Parameter(ParameterSetName = 'Capture')]
    [ValidateRange(1, 2147483647)]
    [int]$ProcessId,

    [Parameter(Mandatory, ParameterSetName = 'Capture')]
    [string]$OutputPath,

    [Parameter(Mandatory, ParameterSetName = 'Compare')]
    [string]$ReferencePath,

    [Parameter(Mandatory, ParameterSetName = 'Compare')]
    [string]$AlteredPath,

    [Parameter(Mandatory, ParameterSetName = 'Compare')]
    [Alias('ComparisonOutputPath')]
    [string]$ComparisonPath
)

$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

function Resolve-OutputPath {
    param([Parameter(Mandatory)][string]$Path)

    $absolute = [System.IO.Path]::GetFullPath($Path)
    $directory = [System.IO.Path]::GetDirectoryName($absolute)
    if (-not [string]::IsNullOrWhiteSpace($directory)) {
        [System.IO.Directory]::CreateDirectory($directory) | Out-Null
    }
    return $absolute
}

function Get-ManifestPath {
    param([Parameter(Mandatory)][string]$ImagePath)
    return "$ImagePath.manifest.json"
}

function Write-DeterministicJson {
    param(
        [Parameter(Mandatory)][string]$Path,
        [Parameter(Mandatory)]$Value
    )

    $json = $Value | ConvertTo-Json -Depth 8
    $encoding = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, ($json + [Environment]::NewLine), $encoding)
}

function Get-LowerSha256 {
    param([Parameter(Mandatory)][string]$Path)
    return (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Test-ImageHasVisualContent {
    param([Parameter(Mandatory)][System.Drawing.Bitmap]$Bitmap)

    $colors = New-Object 'System.Collections.Generic.HashSet[int]'
    $xStep = [Math]::Max(1, [int]($Bitmap.Width / 16))
    $yStep = [Math]::Max(1, [int]($Bitmap.Height / 16))
    for ($y = 0; $y -lt $Bitmap.Height; $y += $yStep) {
        for ($x = 0; $x -lt $Bitmap.Width; $x += $xStep) {
            [void]$colors.Add($Bitmap.GetPixel($x, $y).ToArgb())
            if ($colors.Count -gt 1) { return $true }
        }
    }
    return $false
}

function Read-VerifiedCaptureManifest {
    param([Parameter(Mandatory)][string]$ImagePath)

    $manifestPath = Get-ManifestPath -ImagePath $ImagePath
    if (-not [System.IO.File]::Exists($manifestPath)) {
        throw "Capture provenance manifest does not exist: $manifestPath"
    }
    $manifest = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
    if ($manifest.schema_version -ne 1 -or $manifest.artifact_kind -ne 'forge_window_capture') {
        throw "Capture provenance manifest is unsupported: $manifestPath"
    }
    $actualHash = Get-LowerSha256 -Path $ImagePath
    if ([string]$manifest.image.sha256 -ne $actualHash) {
        throw "Capture provenance hash does not match image: $ImagePath"
    }
    $image = [System.Drawing.Image]::FromFile($ImagePath)
    try {
        if ([int]$manifest.image.width -ne $image.Width -or
            [int]$manifest.image.height -ne $image.Height -or
            [int]$manifest.window.width -ne $image.Width -or
            [int]$manifest.window.height -ne $image.Height) {
            throw "Capture provenance dimensions do not match image: $ImagePath"
        }
    }
    finally {
        $image.Dispose()
    }
    return $manifest
}

if ($PSCmdlet.ParameterSetName -eq 'Capture') {
    if (-not ('ForgeChatVisual.NativeMethods' -as [type])) {
        Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;

namespace ForgeChatVisual {
    public static class NativeMethods {
        [StructLayout(LayoutKind.Sequential)]
        public struct Rect {
            public int Left;
            public int Top;
            public int Right;
            public int Bottom;
        }

        [DllImport("user32.dll")]
        [return: MarshalAs(UnmanagedType.Bool)]
        public static extern bool GetWindowRect(IntPtr hWnd, out Rect rect);

        [DllImport("user32.dll")]
        [return: MarshalAs(UnmanagedType.Bool)]
        public static extern bool IsIconic(IntPtr hWnd);

        [DllImport("user32.dll")]
        [return: MarshalAs(UnmanagedType.Bool)]
        public static extern bool IsWindowVisible(IntPtr hWnd);

        [DllImport("user32.dll")]
        [return: MarshalAs(UnmanagedType.Bool)]
        public static extern bool PrintWindow(IntPtr hWnd, IntPtr hdcBlt, uint flags);

        [DllImport("user32.dll")]
        public static extern int GetSystemMetrics(int index);
    }
}
'@
    }

    $process = $null
    if ($PSBoundParameters.ContainsKey('ProcessId')) {
        $process = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
        if ($null -eq $process) {
            throw "No process exists with PID $ProcessId."
        }
        if ($process.MainWindowHandle -eq 0) {
            throw "Process PID $ProcessId does not own a visible main window."
        }
        if (-not [string]::Equals($process.MainWindowTitle, $WindowTitle, [StringComparison]::OrdinalIgnoreCase)) {
            throw "Process PID $ProcessId owns window '$($process.MainWindowTitle)', not exact Forge title '$WindowTitle'."
        }
    }
    else {
        $matches = @(Get-Process | Where-Object {
            $_.MainWindowHandle -ne 0 -and
            [string]::Equals($_.MainWindowTitle, $WindowTitle, [StringComparison]::OrdinalIgnoreCase)
        })
        if ($matches.Count -eq 0) {
            throw "No visible window exactly matched title '$WindowTitle'."
        }
        if ($matches.Count -ne 1) {
            $pids = ($matches | ForEach-Object Id) -join ', '
            throw "Window title '$WindowTitle' is ambiguous across PIDs $pids; rerun with -ProcessId."
        }
        $process = $matches[0]
    }

    $handle = $process.MainWindowHandle
    $process.Refresh()
    if (-not $process.Responding) {
        throw "Forge window PID $($process.Id) is not responding; capture was rejected."
    }
    if (-not [ForgeChatVisual.NativeMethods]::IsWindowVisible($handle)) {
        throw "Forge window PID $($process.Id) is not visible."
    }
    if ([ForgeChatVisual.NativeMethods]::IsIconic($handle)) {
        throw "Forge window PID $($process.Id) is minimized; restore it before capture."
    }

    $rect = New-Object ForgeChatVisual.NativeMethods+Rect
    if (-not [ForgeChatVisual.NativeMethods]::GetWindowRect($handle, [ref]$rect)) {
        throw "Could not read the bounds of '$($process.MainWindowTitle)'."
    }
    $width = $rect.Right - $rect.Left
    $height = $rect.Bottom - $rect.Top
    if ($width -le 0 -or $height -le 0) {
        throw "Window '$($process.MainWindowTitle)' has invalid bounds ${width}x${height}."
    }

    $virtualLeft = [ForgeChatVisual.NativeMethods]::GetSystemMetrics(76)
    $virtualTop = [ForgeChatVisual.NativeMethods]::GetSystemMetrics(77)
    $virtualWidth = [ForgeChatVisual.NativeMethods]::GetSystemMetrics(78)
    $virtualHeight = [ForgeChatVisual.NativeMethods]::GetSystemMetrics(79)
    $virtualRight = $virtualLeft + $virtualWidth
    $virtualBottom = $virtualTop + $virtualHeight
    if ($virtualWidth -le 0 -or $virtualHeight -le 0 -or
        $rect.Left -lt $virtualLeft -or $rect.Top -lt $virtualTop -or
        $rect.Right -gt $virtualRight -or $rect.Bottom -gt $virtualBottom) {
        throw "Forge window is partly outside the visible virtual desktop; move it fully onscreen before capture."
    }

    $absoluteOutput = Resolve-OutputPath -Path $OutputPath
    $bitmap = New-Object System.Drawing.Bitmap $width, $height
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $hdc = [IntPtr]::Zero
    try {
        $hdc = $graphics.GetHdc()
        # PW_RENDERFULLCONTENT asks Windows to render this HWND directly. Unlike
        # CopyFromScreen it cannot include an overlapping app or notification.
        if (-not [ForgeChatVisual.NativeMethods]::PrintWindow($handle, $hdc, 2)) {
            throw "Windows could not render the Forge window without screen scraping."
        }
    }
    finally {
        if ($hdc -ne [IntPtr]::Zero) { $graphics.ReleaseHdc($hdc) }
        $graphics.Dispose()
    }

    try {
        if (-not (Test-ImageHasVisualContent -Bitmap $bitmap)) {
            throw "Windows returned a blank or uniform Forge-window image; capture was rejected."
        }
        $bitmap.Save($absoluteOutput, [System.Drawing.Imaging.ImageFormat]::Png)
    }
    finally {
        $bitmap.Dispose()
    }

    $imageHash = Get-LowerSha256 -Path $absoluteOutput
    $manifest = [ordered]@{
        schema_version = 1
        artifact_kind = 'forge_window_capture'
        capture_method = 'PrintWindow/PW_RENDERFULLCONTENT'
        chat_delivery_performed = $false
        window = [ordered]@{
            process_id = [int]$process.Id
            handle = $handle.ToInt64().ToString([Globalization.CultureInfo]::InvariantCulture)
            title = [string]$process.MainWindowTitle
            left = [int]$rect.Left
            top = [int]$rect.Top
            width = [int]$width
            height = [int]$height
        }
        image = [ordered]@{
            file = [System.IO.Path]::GetFileName($absoluteOutput)
            width = [int]$width
            height = [int]$height
            sha256 = $imageHash
        }
    }
    Write-DeterministicJson -Path (Get-ManifestPath -ImagePath $absoluteOutput) -Value $manifest
    Write-Output $absoluteOutput
    exit 0
}

$referenceAbsolute = [System.IO.Path]::GetFullPath($ReferencePath)
$alteredAbsolute = [System.IO.Path]::GetFullPath($AlteredPath)
if (-not [System.IO.File]::Exists($referenceAbsolute)) {
    throw "Reference image does not exist: $referenceAbsolute"
}
if (-not [System.IO.File]::Exists($alteredAbsolute)) {
    throw "Altered image does not exist: $alteredAbsolute"
}

$referenceManifest = Read-VerifiedCaptureManifest -ImagePath $referenceAbsolute
$alteredManifest = Read-VerifiedCaptureManifest -ImagePath $alteredAbsolute
$referenceHash = Get-LowerSha256 -Path $referenceAbsolute
$alteredHash = Get-LowerSha256 -Path $alteredAbsolute
if ($referenceHash -eq $alteredHash) {
    throw 'Reference and altered images are identical; comparison proof was rejected.'
}
foreach ($field in @('process_id', 'handle', 'title')) {
    if ([string]$referenceManifest.window.$field -ne [string]$alteredManifest.window.$field) {
        throw "Reference and altered captures have different window identity field '$field'."
    }
}

$comparisonAbsolute = Resolve-OutputPath -Path $ComparisonPath
if ($comparisonAbsolute -eq $referenceAbsolute -or $comparisonAbsolute -eq $alteredAbsolute) {
    throw 'Comparison output must not overwrite either source capture.'
}
$reference = [System.Drawing.Image]::FromFile($referenceAbsolute)
$altered = [System.Drawing.Image]::FromFile($alteredAbsolute)
$headerHeight = 64
$gutter = 8
$panelWidth = [Math]::Max($reference.Width, $altered.Width)
$panelHeight = [Math]::Max($reference.Height, $altered.Height)
$canvasWidth = ($panelWidth * 2) + $gutter
$canvasHeight = $panelHeight + $headerHeight
$canvas = New-Object System.Drawing.Bitmap $canvasWidth, $canvasHeight
$graphics = [System.Drawing.Graphics]::FromImage($canvas)
$background = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(9, 11, 15))
$accent = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(238, 28, 52))
$labelBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::White)
$font = New-Object System.Drawing.Font 'Segoe UI', 20, ([System.Drawing.FontStyle]::Bold)

try {
    $graphics.FillRectangle($background, 0, 0, $canvas.Width, $canvas.Height)
    $graphics.FillRectangle($accent, 0, ($headerHeight - 4), $canvas.Width, 4)
    $graphics.FillRectangle($accent, $panelWidth, 0, $gutter, $canvas.Height)
    $graphics.DrawString('REFERENCE', $font, $labelBrush, 22, 16)
    $graphics.DrawString('ALTERED', $font, $labelBrush, ($panelWidth + $gutter + 22), 16)
    $graphics.DrawImage($reference, 0, $headerHeight, $reference.Width, $reference.Height)
    $graphics.DrawImage($altered, ($panelWidth + $gutter), $headerHeight, $altered.Width, $altered.Height)
    $canvas.Save($comparisonAbsolute, [System.Drawing.Imaging.ImageFormat]::Png)
}
finally {
    $font.Dispose()
    $labelBrush.Dispose()
    $accent.Dispose()
    $background.Dispose()
    $graphics.Dispose()
    $canvas.Dispose()
    $reference.Dispose()
    $altered.Dispose()
}

$comparisonHash = Get-LowerSha256 -Path $comparisonAbsolute
$comparisonManifest = [ordered]@{
    schema_version = 1
    artifact_kind = 'forge_visual_comparison'
    chat_delivery_performed = $false
    window_identity = [ordered]@{
        process_id = [int]$referenceManifest.window.process_id
        handle = [string]$referenceManifest.window.handle
        title = [string]$referenceManifest.window.title
    }
    reference = [ordered]@{
        file = [System.IO.Path]::GetFileName($referenceAbsolute)
        width = [int]$referenceManifest.image.width
        height = [int]$referenceManifest.image.height
        sha256 = $referenceHash
    }
    altered = [ordered]@{
        file = [System.IO.Path]::GetFileName($alteredAbsolute)
        width = [int]$alteredManifest.image.width
        height = [int]$alteredManifest.image.height
        sha256 = $alteredHash
    }
    comparison = [ordered]@{
        file = [System.IO.Path]::GetFileName($comparisonAbsolute)
        width = [int]$canvasWidth
        height = [int]$canvasHeight
        sha256 = $comparisonHash
    }
}
Write-DeterministicJson -Path (Get-ManifestPath -ImagePath $comparisonAbsolute) -Value $comparisonManifest
Write-Output $comparisonAbsolute
