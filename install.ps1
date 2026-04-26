# This script downloads and installs the latest release of fernctl for Windows.
# Run with: irm https://raw.githubusercontent.com/BenSimmers/fernctl/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$TARGET = "x86_64-pc-windows-msvc"
$REPO   = "BenSimmers/fernctl"

# Fetch the latest release metadata.
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest"
$asset   = $release.assets | Where-Object { $_.name -like "fernctl-$TARGET*" } | Select-Object -First 1

if (-not $asset) {
    Write-Error "Could not find a Windows release. Check https://github.com/$REPO/releases"
    exit 1
}

# Determine install directory (~\AppData\Local\Programs\fernctl).
$installDir = "$env:LOCALAPPDATA\Programs\fernctl"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
$dest = Join-Path $installDir "fernctl.exe"

Write-Host "Downloading fernctl for $TARGET..."
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $dest

# Add install directory to user PATH if not already present.
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installDir", "User")
    Write-Host ""
    Write-Host "Added $installDir to your PATH."
    Write-Host "Restart your terminal, then run: fernctl"
} else {
    Write-Host "fernctl installed to $dest"
    Write-Host "You can now run it with: fernctl"
}
