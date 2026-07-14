param(
    [Parameter(Mandatory = $true)]
    [string]$Output
)

# Generates a self-contained 32px 32-bit ICO without a graphics dependency.
$width = 32
$height = 32
$pixelBytes = $width * $height * 4
$maskBytes = $height * 4
$imageBytes = 40 + $pixelBytes + $maskBytes
$stream = [System.IO.MemoryStream]::new()
$writer = [System.IO.BinaryWriter]::new($stream)

$writer.Write([UInt16]0) # reserved
$writer.Write([UInt16]1) # icon
$writer.Write([UInt16]1) # one image
$writer.Write([Byte]$width)
$writer.Write([Byte]$height)
$writer.Write([Byte]0)
$writer.Write([Byte]0)
$writer.Write([UInt16]1)
$writer.Write([UInt16]32)
$writer.Write([UInt32]$imageBytes)
$writer.Write([UInt32]22)

$writer.Write([UInt32]40) # BITMAPINFOHEADER size
$writer.Write([Int32]$width)
$writer.Write([Int32]($height * 2))
$writer.Write([UInt16]1)
$writer.Write([UInt16]32)
$writer.Write([UInt32]0)
$writer.Write([UInt32]($pixelBytes + $maskBytes))
$writer.Write([Int32]0)
$writer.Write([Int32]0)
$writer.Write([UInt32]0)
$writer.Write([UInt32]0)

for ($y = $height - 1; $y -ge 0; $y--) {
    for ($x = 0; $x -lt $width; $x++) {
        $edge = [Math]::Min([Math]::Min($x, $y), [Math]::Min($width - 1 - $x, $height - 1 - $y))
        $inside = $edge -ge 2
        if ($inside) {
            $blue = [Byte](110 + (($x + $y) % 40))
            $green = [Byte](60 + (($x * 2) % 45))
            $red = [Byte](25 + (($y * 2) % 35))
            $writer.Write($blue); $writer.Write($green); $writer.Write($red); $writer.Write([Byte]255)
        } else {
            $writer.Write([Byte]0); $writer.Write([Byte]0); $writer.Write([Byte]0); $writer.Write([Byte]0)
        }
    }
}

for ($row = 0; $row -lt $height; $row++) { $writer.Write([UInt32]0) }
$writer.Flush()
[System.IO.Directory]::CreateDirectory([System.IO.Path]::GetDirectoryName($Output)) | Out-Null
[System.IO.File]::WriteAllBytes($Output, $stream.ToArray())
