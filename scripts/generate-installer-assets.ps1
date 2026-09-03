param(
  [string]$Source = (Join-Path $PSScriptRoot "..\src-tauri\icons\icon.png"),
  [string]$OutputDirectory = (Join-Path $PSScriptRoot "..\src-tauri\nsis")
)

$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Drawing

New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$sourceImage = [System.Drawing.Image]::FromFile((Resolve-Path $Source))

function New-InstallerBitmap {
  param(
    [int]$Width,
    [int]$Height,
    [string]$Path,
    [scriptblock]$DrawContent
  )

  $bitmap = New-Object System.Drawing.Bitmap($Width, $Height, [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
  $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
  $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
  $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
  $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
  & $DrawContent $graphics $Width $Height
  $bitmap.Save($Path, [System.Drawing.Imaging.ImageFormat]::Bmp)
  $graphics.Dispose()
  $bitmap.Dispose()
}

try {
  New-InstallerBitmap 150 57 (Join-Path $OutputDirectory "installer-header.bmp") {
    param($graphics, $width, $height)
    $graphics.Clear([System.Drawing.Color]::White)
    $graphics.FillRectangle((New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(31, 41, 51))), 0, 0, 6, $height)
    $graphics.FillRectangle((New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(228, 231, 236))), 6, ($height - 2), ($width - 6), 2)
    $graphics.DrawImage($sourceImage, [System.Drawing.Rectangle]::new(16, 8, 41, 41))
  }

  New-InstallerBitmap 164 314 (Join-Path $OutputDirectory "installer-sidebar.bmp") {
    param($graphics, $width, $height)
    $graphics.Clear([System.Drawing.Color]::FromArgb(245, 246, 247))
    $graphics.FillRectangle((New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(31, 41, 51))), 0, 0, 9, $height)
    $graphics.FillRectangle((New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(184, 68, 62))), 9, ($height - 18), ($width - 9), 9)
    $graphics.FillRectangle((New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(240, 180, 41))), 9, ($height - 9), ($width - 9), 9)
    $graphics.DrawImage($sourceImage, [System.Drawing.Rectangle]::new(18, 83, 128, 128))
  }
}
finally {
  $sourceImage.Dispose()
}
