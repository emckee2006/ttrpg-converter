#
# Extracts a Foundry VTT compendium pack to a temporary directory and outputs the directory path.
# This script is called by the FoundryCliBridge in the TTRPGConverter application.
#

param(
    [Parameter(Mandatory=$true)]
    [string]$PackName,

    [Parameter(Mandatory=$true)]
    [string]$PackageId,

    [Parameter(Mandatory=$true)]
    [string]$PackageType,

    [Parameter(Mandatory=$false)]
    [switch]$ShowDebug
)

# Set strict error handling
$ErrorActionPreference = "Stop"

# Function for logging if in debug mode
function Write-DebugLog {
    param([string]$Message)
    if ($ShowDebug) {
        # Write to host to avoid polluting stdout, which is used for the output path
        Write-Host "DEBUG: $Message"
    }
}

$tempDir = ""

try {
    # The script is executed from the solution root, where node_modules should be.
    $fvttPath = ".\node_modules\.bin\fvtt.cmd"
    Write-DebugLog "Looking for fvtt at (Get-Location)\$fvttPath"
    if (-not (Test-Path $fvttPath)) {
        throw "Foundry CLI (fvtt.cmd) not found. Make sure you have run 'npm install' in the project root."
    }

    # Create a unique temporary directory for the output
    $tempDir = Join-Path $env:TEMP "foundry-extract-$PackName-$(Get-Random)"
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    Write-DebugLog "Created temporary output directory at $tempDir"

    # Set the package context for the unpack operation. This is a fast operation.
    $workonCommand = "package workon `"$PackageId`" --type `"$PackageType`""
    Write-DebugLog "Running: fvtt $workonCommand"
    & $fvttPath $workonCommand.Split(' ') | Out-Null

    # Unpack the compendium pack to our temporary directory. This can be slow.
    $unpackCommand = "package unpack `"$PackName`" --out `"$tempDir`""
    Write-DebugLog "Running: fvtt $unpackCommand"
    & $fvttPath $unpackCommand.Split(' ') | Out-Null

    # The C# code expects ONLY the directory path to be written to standard output.
    # This is the primary success output of the script.
    Write-Output $tempDir
    Write-DebugLog "Successfully unpacked to $tempDir. Emitting path to stdout."

} catch {
    # On any error, write the error message to the error stream and exit with a non-zero code.
    $errorMessage = "Failed to extract pack '{0}'. Error: {1}" -f $PackName, $_.Exception.Message
    Write-Error $errorMessage
    
    # Cleanup the temp directory on failure
    if ($tempDir -and (Test-Path $tempDir)) {
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    
    exit 1
}
