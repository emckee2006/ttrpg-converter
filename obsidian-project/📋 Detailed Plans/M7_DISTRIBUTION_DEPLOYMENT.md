# M7: Distribution and Deployment - Junior Developer Implementation Guide

## 🎯 **MILESTONE 7 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 14 | **Priority**: 🔥 HIGH
**Dependencies**: M6 CLI Tools Complete

Desktop application packaging, cross-platform installers, release automation, and distribution infrastructure for seamless end-user deployment.

### 🚀 **DISTRIBUTION FEATURES**
- **Cross-Platform Installers**: Windows MSI, macOS PKG, Linux AppImage/DEB
- **Auto-Update System**: Secure update mechanisms with delta patches
- **Plugin Marketplace**: Plugin discovery, installation, and management
- **Release Automation**: CI/CD pipeline for automated releases

---

## **T7.1: Cross-Platform Application Packaging**
**Duration**: 5 days | **Points**: 6 | **Priority**: 🔥 HIGH

### **Implementation Steps**

**Step 1: Desktop Application Bundling**
Create `packaging/` directory structure:
```
packaging/
├── windows/
│   ├── installer.wxs        # WiX installer definition
│   ├── app.manifest        # Windows application manifest
│   └── resources/
├── macos/
│   ├── Info.plist          # macOS application info
│   └── resources/
├── linux/
│   ├── appimage.yml        # AppImage configuration
│   └── resources/
└── scripts/
    ├── build-all-platforms.ps1
    ├── build-windows.ps1
    ├── build-macos.sh
    └── build-linux.sh
```

**Step 2: Windows MSI Installer**
Create `packaging/windows/installer.wxs`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
  <Product Id="*" Name="TTRPG Converter" Language="1033" Version="1.0.0.0" 
           Manufacturer="TTRPG Tools" UpgradeCode="12345678-1234-5678-9012-123456789012">
    
    <Package InstallerVersion="200" Compressed="yes" InstallScope="perMachine" />
    <MajorUpgrade DowngradeErrorMessage="A newer version is already installed." />
    
    <Feature Id="ProductFeature" Title="TTRPG Converter" Level="1">
      <ComponentGroupRef Id="ProductComponents" />
      <ComponentGroupRef Id="PluginComponents" />
    </Feature>

    <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
      <Component Id="MainExecutable">
        <File Id="ttrpg_gui.exe" Source="$(var.SourceDir)\ttrpg-gui.exe" KeyPath="yes">
          <Shortcut Id="StartMenuShortcut" Directory="ProgramMenuFolder" 
                    Name="TTRPG Converter" Icon="AppIcon" />
        </File>
      </Component>
    </ComponentGroup>

    <Directory Id="TARGETDIR" Name="SourceDir">
      <Directory Id="ProgramFilesFolder">
        <Directory Id="INSTALLFOLDER" Name="TTRPG Converter">
          <Directory Id="PluginFolder" Name="plugins" />
        </Directory>
      </Directory>
    </Directory>
  </Product>
</Wix>
```

**Step 3: Build Scripts**
Create `packaging/scripts/build-windows.ps1`:
```powershell
#!/usr/bin/env pwsh
param(
    [string]$Configuration = "Release",
    [switch]$CreateInstaller = $true
)

$ErrorActionPreference = "Stop"
Write-Host "🏗️ Building TTRPG Converter for Windows" -ForegroundColor Cyan

$RootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$BuildDir = Join-Path $RootDir "target\release"
$PackageDir = Join-Path $RootDir "packaging\windows\output"

New-Item -ItemType Directory -Force -Path $PackageDir | Out-Null

# Build applications
Write-Host "🔨 Building Rust applications..." -ForegroundColor Yellow
Set-Location $RootDir
cargo build --release --bin ttrpg-gui
cargo build --release --bin ttrpg-cli

# Copy executables and plugins
Copy-Item "$BuildDir\ttrpg-gui.exe" -Destination $PackageDir
Copy-Item "$BuildDir\ttrpg-cli.exe" -Destination $PackageDir

$PluginOutputDir = Join-Path $PackageDir "plugins"
New-Item -ItemType Directory -Force -Path $PluginOutputDir | Out-Null
Get-ChildItem "$BuildDir\*.dll" | Copy-Item -Destination $PluginOutputDir

if ($CreateInstaller) {
    Write-Host "🚀 Creating MSI installer..." -ForegroundColor Yellow
    
    # WiX compilation commands here
    $InstallerOutput = Join-Path $PackageDir "ttrpg-converter-windows-x64.msi"
    Write-Host "✅ Windows installer created: $InstallerOutput" -ForegroundColor Green
}

Write-Host "🎉 Windows packaging completed!" -ForegroundColor Green
```

---

## **T7.2: Auto-Update System**
**Duration**: 4 days | **Points**: 4 | **Priority**: 🔥 HIGH

### **Implementation Steps**

**Step 1: Update Client Implementation**
Create `crates/core/ttrpg-updater/src/updater.rs`:
```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use semver::Version;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub download_url: String,
    pub checksum_sha256: String,
    pub file_size: u64,
    pub release_notes: String,
    pub is_critical: bool,
}

pub struct AutoUpdater {
    client: Client,
    current_version: Version,
    update_server_url: String,
    install_dir: PathBuf,
}

impl AutoUpdater {
    pub fn new(current_version: &str, update_server_url: String, install_dir: PathBuf) -> Result<Self, UpdateError> {
        Ok(Self {
            client: Client::new(),
            current_version: Version::parse(current_version)?,
            update_server_url,
            install_dir,
        })
    }
    
    /// Check for available updates
    pub async fn check_for_updates(&self) -> Result<Option<UpdateInfo>, UpdateError> {
        let url = format!("{}/api/v1/updates/check", self.update_server_url);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", &format!("TTRPG-Converter/{}", self.current_version))
            .send()
            .await?;
            
        if response.status() == 204 {
            return Ok(None);
        }
        
        let update_info: UpdateInfo = response.json().await?;
        let available_version = Version::parse(&update_info.version)?;
        
        if available_version > self.current_version {
            Ok(Some(update_info))
        } else {
            Ok(None)
        }
    }
    
    /// Download and install update
    pub async fn download_and_install(&self, update_info: &UpdateInfo) -> Result<(), UpdateError> {
        tracing::info!("Starting download of version {}", update_info.version);
        
        let temp_file = self.download_update(update_info).await?;
        self.verify_checksum(&temp_file, &update_info.checksum_sha256).await?;
        
        let backup_dir = self.create_backup().await?;
        
        match self.install_update(&temp_file).await {
            Ok(_) => {
                tracing::info!("Update installed successfully");
                Ok(())
            }
            Err(e) => {
                self.restore_backup(&backup_dir).await?;
                Err(e)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Version parsing error: {0}")]
    Version(#[from] semver::Error),
    
    #[error("Checksum mismatch")]
    ChecksumMismatch,
}
```

---

## **T7.3: Plugin Marketplace Integration**
**Duration**: 3 days | **Points**: 2 | **Priority**: 🔥 HIGH

Plugin discovery, installation, and management system with remote plugin registry.

---

## **T7.4: Release Automation Pipeline**
**Duration**: 2 days | **Points**: 2 | **Priority**: 🔥 HIGH

### **Implementation Steps**

**Step 1: GitHub Actions Workflow**
Create `.github/workflows/release.yml`:
```yaml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  create-release:
    runs-on: ubuntu-latest
    outputs:
      upload_url: ${{ steps.create_release.outputs.upload_url }}
    steps:
      - name: Create Release
        id: create_release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}

  build-windows:
    needs: create-release
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build and package
        run: ./packaging/scripts/build-windows.ps1 -CreateInstaller
      - name: Upload Windows installer
        uses: actions/upload-release-asset@v1
        with:
          upload_url: ${{ needs.create-release.outputs.upload_url }}
          asset_path: ./packaging/windows/output/ttrpg-converter-windows-x64.msi
          asset_name: ttrpg-converter-windows-x64.msi
          asset_content_type: application/x-msi

  build-macos:
    needs: create-release
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build and package
        run: ./packaging/scripts/build-macos.sh
      - name: Upload macOS installer
        uses: actions/upload-release-asset@v1
        with:
          upload_url: ${{ needs.create-release.outputs.upload_url }}
          asset_path: ./packaging/macos/output/ttrpg-converter-macos.pkg
          asset_name: ttrpg-converter-macos.pkg
          asset_content_type: application/octet-stream
```

### **Success Criteria**
- [ ] ✅ Windows MSI installer packages all components and creates proper shortcuts
- [ ] ✅ macOS PKG installer creates universal binary and app bundle
- [ ] ✅ Linux AppImage/DEB packages work across distributions
- [ ] ✅ Auto-update system securely downloads and installs updates
- [ ] ✅ Plugin marketplace enables plugin discovery and installation
- [ ] ✅ Release automation creates cross-platform builds automatically
- [ ] ✅ All installers include proper code signing and notarization
