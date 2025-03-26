# Lion CLI installer for Windows
$VERSION = "0.2.91"
$DOWNLOAD_URL = "https://github.com/TeenCoder159/lion/releases/download/v${VERSION}/lion-windows.exe"
$INSTALL_DIR = "$env:LOCALAPPDATA\Programs\Lion"

# Create directory if it doesn't exist
if (-not (Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
}

# Download Lion CLI
Write-Host "Downloading Lion CLI..."
Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile "$INSTALL_DIR\lion.exe"

# Add to PATH if not already there
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not $currentPath.Contains($INSTALL_DIR)) {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$INSTALL_DIR", "User")
    Write-Host "Added Lion CLI to PATH"
}

# Create a batch file for lion-cli
$batchFile = @"
@echo off
"%LOCALAPPDATA%\Programs\Lion\lion.exe" %*
"@
Set-Content -Path "$INSTALL_DIR\lion-cli.bat" -Value $batchFile

Write-Host "Lion CLI installed successfully! Please restart your terminal and run 'lion help' to get started."
