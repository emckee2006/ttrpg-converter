param(
    [Parameter(Mandatory=$true)]
  try {
    # Test if fvtt exists and is accessible
    if (-not (Test-Path $fvttPath)) {
        throw "Foundry CLI not found at: $fvttPath"
    }
    
    # Set up package workspace (required for unpack to work)
    Invoke-Fvtt "package workon `"$PackageId`" --type `"$PackageType`""
    
    # Create output directory for extracted files
    $outputDir = Join-Path $env:TEMP "foundry-extract-$PackName-$(Get-Random)"
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
    
    # Extract pack data directly to temp directory (read-only operation)
    # This should not modify any files in the Foundry data directory
    $CommandTimeout = 60  # 60 seconds for unpack
    Invoke-Fvtt "package unpack `"$PackName`" --out `"$outputDir`""me,
    
    [Parameter(Mandatory=$false)]
    [string]$PackageId = "dnd5e",
    
    [Parameter(Mandatory=$false)]
    [string]$PackageType = "System"
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"  # Prevent progress bars from hanging process
$fvttPath = ".\node_modules\.bin\fvtt.cmd"

# Set timeout for individual fvtt commands
$CommandTimeout = 30  # 30 seconds per command

function Invoke-Fvtt {
    param([string]$Command)
    
    try {
        # Use the fvtt command directly
        $processArgs = @{
            FilePath = $fvttPath
            ArgumentList = $Command.Split(' ')
            NoNewWindow = $true
            Wait = $true
            PassThru = $true
        }
        
        $process = Start-Process @processArgs
        
        if ($process.ExitCode -ne 0) {
            throw "fvtt command failed with exit code $($process.ExitCode)"
        }
    }
    catch {
        throw "fvtt $Command failed: $($_.Exception.Message)"
    }
}

try {
    # Test if fvtt exists and is accessible
    if (-not (Test-Path $fvttPath)) {
        throw "Foundry CLI not found at: $fvttPath"
    }
    
    # Configure Foundry CLI (these are quick operations)
    Invoke-Fvtt "configure set installPath `"C:/Program Files/Foundry Virtual Tabletop`""
    Invoke-Fvtt "package workon `"$PackageId`" --type `"$PackageType`""
    
    # Create temp directory and extract pack
    $tempDir = Join-Path $env:TEMP "foundry-extract-$(Get-Random)"
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    
    # This is the potentially slow operation - give it more time
    $CommandTimeout = 60  # 60 seconds for unpack
    Invoke-Fvtt "package unpack `"$PackName`" --out `"$tempDir`""
    
    # Read extracted files
    $files = Get-ChildItem -Path $tempDir -Filter "*.json"
    $records = @()
    
    foreach ($file in $files) {
        $content = Get-Content -Path $file.FullName -Raw | ConvertFrom-Json
        $records += @{
            key = $file.BaseName
            value = $content
        }
    }
    
    # Cleanup
    Remove-Item -Path $tempDir -Recurse -Force
    
    # Output JSON result
    $result = @{
        success = $true
        recordCount = $records.Count
        records = $records
    }
    
    $result | ConvertTo-Json -Depth 10 -Compress
    
} catch {
    # Cleanup on error
    if ($tempDir -and (Test-Path $tempDir)) {
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    
    $errorResult = @{
        success = $false
        error = $_.Exception.Message
    }
    
    $errorResult | ConvertTo-Json -Compress
}
