#Requires -Version 5.1
Write-Host @"
 ██████   ██████  █████████    █████████             █████     ███                                                            ■ ■                  ■ ■                                                                                  
░░██████ ██████  ███░░░░░███  ███░░░░░███           ░░███     ░░░                                                              \ \   /\_/\_/\_/\  / /
 ░███░█████░███ ░███    ░░░  ░███    ░███   ██████  ███████   ████  █████ █████  ██████  ████████   █████   ██████               \ (='_`  _  `_`=) /
 ░███░░███ ░███ ░░█████████  ░███████████  ███░░███░░░███░   ░░███ ░░███ ░░███  ███░░███░░███░░███ ███░░   ███░░███               `-\___________/-' 
 ░███ ░░░  ░███  ░░░░░░░░███ ░███░░░░░███ ░███ ░░░   ░███     ░███  ░███  ░███ ░███████  ░███ ░░░ ░░█████ ░███████                    /_/   \_\
 ░███      ░███  ███    ░███ ░███    ░███ ░███  ███  ░███ ███ ░███  ░░███ ███  ░███░░░   ░███      ░░░░███░███░░░  
 █████     █████░░█████████  █████   █████░░██████   ░░█████  █████  ░░█████   ░░██████  █████     ██████ ░░██████ 
░░░░░     ░░░░░  ░░░░░░░░░  ░░░░░   ░░░░░  ░░░░░░     ░░░░░  ░░░░░    ░░░░░     ░░░░░░  ░░░░░     ░░░░░░   ░░░░░░  
Downloading....
"@
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "[*] Relaunching as Administrator..." -ForegroundColor Yellow
    Start-Process powershell -Verb RunAs `
        -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`""
    exit
}

if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64" -or $env:PROCESSOR_ARCHITEW6432 -eq "ARM64") {
    $arch = "arm64"
} elseif ([Environment]::Is64BitOperatingSystem) {
    $arch = "x86_64"
} else {
    Write-Error "Unsupported architecture: 32-bit (x86) operating systems are not supported."
    Read-Host "`nPress Enter to exit"
    exit 1
}

$apiUrl  = "https://api.github.com/repos/mrrabyss/MSActiverse/releases/latest"
$headers = @{ "User-Agent" = "PS-MSActiverse-Installer" }

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

try {
    $release = Invoke-RestMethod -Uri $apiUrl -Headers $headers -ErrorAction Stop
} catch {
    Write-Error "GitHub API error: $_"
    Read-Host "`nPress Enter to exit"
    exit 1
}

$version  = $release.tag_name -replace "^v"
$fileName = "MSActiverse-$version-$arch.exe"
$asset    = $release.assets | Where-Object { $_.name -eq $fileName } | Select-Object -First 1

if (-not $asset) {
    Write-Error "Asset '$fileName' not found in release $version."
    Write-Host "`nAvailable assets:" -ForegroundColor Yellow
    $release.assets | ForEach-Object { Write-Host "  - $($_.name)" }
    Read-Host "`nPress Enter to exit"
    exit 1
}

$outDir  = Join-Path $env:TEMP "MSActiverse"
$null    = New-Item -ItemType Directory -Path $outDir -Force
$outFile = Join-Path $outDir $asset.name

$ProgressPreference = "SilentlyContinue"
try {
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $outFile -Headers $headers -ErrorAction Stop
    Write-Host "[+] Download complete." -ForegroundColor Green
} catch {
    Write-Error "Download failed: $_"
    Read-Host "`nPress Enter to exit"
    exit 1
}

Write-Host "[*] Launching..." -ForegroundColor Cyan
if (Test-Path $outFile) {
    Start-Process -FilePath $outFile -Verb RunAs -Wait
} else {
    Write-Error "Executable file not found at $outFile"
    Read-Host "`nPress Enter to exit"
    exit 1
}

Read-Host "`nPress Enter to exit"