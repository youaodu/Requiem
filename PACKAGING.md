# 跨平台打包指南

本文档说明如何为 macOS、Windows 和 Linux 创建安装包。

## 目录

- [Linux (Arch Linux PKGBUILD)](#linux-arch-linux-pkgbuild)
- [macOS (DMG)](#macos-dmg)
- [Windows (NSIS/MSI)](#windows-nsismsi)

---

## Linux (Arch Linux PKGBUILD)

### 构建本地包

```bash
# 1. 确保已安装 base-devel
sudo pacman -S base-devel

# 2. 构建包
makepkg -si

# 3. 仅构建不安装
makepkg

# 4. 清理构建文件后重新构建
makepkg -c

# 5. 更新校验和
updpkgsums
```

### 发布到 AUR

```bash
# 1. 创建 AUR 仓库
git clone ssh://aur@aur.archlinux.org/requiem.git

# 2. 复制 PKGBUILD 和 .SRCINFO
cp PKGBUILD requiem/
cd requiem
makepkg --printsrcinfo > .SRCINFO

# 3. 提交并推送
git add PKGBUILD .SRCINFO
git commit -m "Initial release: requiem 0.0.1"
git push
```

---

## macOS (DMG)

### 前置要求

```bash
# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 添加 macOS 目标 (如果交叉编译)
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### 1. 构建 Universal Binary (推荐)

```bash
# 构建 x86_64 版本
cargo build --release --target x86_64-apple-darwin

# 构建 ARM64 版本
cargo build --release --target aarch64-apple-darwin

# 合并为 Universal Binary
lipo -create \
  target/x86_64-apple-darwin/release/requiem \
  target/aarch64-apple-darwin/release/requiem \
  -output target/release/requiem-universal

# 或者直接构建当前架构
cargo build --release
```

### 2. 创建 .app 包结构

```bash
# 创建目录结构
mkdir -p dist/Requiem.app/Contents/MacOS
mkdir -p dist/Requiem.app/Contents/Resources
mkdir -p dist/Requiem.app/Contents/Resources/locales

# 复制二进制文件
cp target/release/requiem-universal dist/Requiem.app/Contents/MacOS/requiem
# 或者: cp target/release/requiem dist/Requiem.app/Contents/MacOS/requiem

# 赋予执行权限
chmod +x dist/Requiem.app/Contents/MacOS/requiem

# 复制资源文件
cp locales/*.json dist/Requiem.app/Contents/Resources/locales/
# 如果有图标: cp assets/icon.icns dist/Requiem.app/Contents/Resources/
```

### 3. 创建 Info.plist

创建 `dist/Requiem.app/Contents/Info.plist`：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>requiem</string>
    <key>CFBundleIdentifier</key>
    <string>com.youaodu.requiem</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Requiem</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.0.1</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>CFBundleIconFile</key>
    <string>icon</string>
</dict>
</plist>
```

### 4. 创建 DMG

```bash
# 安装 create-dmg (如果需要)
brew install create-dmg

# 创建临时挂载点
mkdir -p dist/dmg

# 复制 .app 到 DMG 目录
cp -R dist/Requiem.app dist/dmg/

# 创建符号链接到 Applications
ln -s /Applications dist/dmg/Applications

# 创建 DMG
hdiutil create -volname "Requiem" \
  -srcfolder dist/dmg \
  -ov -format UDZO \
  dist/Requiem-0.0.1-macOS.dmg

# 或使用 create-dmg (更美观)
create-dmg \
  --volname "Requiem" \
  --volicon "assets/icon.icns" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "Requiem.app" 200 190 \
  --hide-extension "Requiem.app" \
  --app-drop-link 600 185 \
  "dist/Requiem-0.0.1-macOS.dmg" \
  "dist/dmg/"
```

### 5. 代码签名 (可选，但推荐用于分发)

```bash
# 查看证书
security find-identity -v -p codesigning

# 签名 app
codesign --force --deep --sign "Developer ID Application: Your Name" \
  dist/Requiem.app

# 验证签名
codesign -dv --verbose=4 dist/Requiem.app

# 公证 (需要 Apple Developer 账户)
xcrun notarytool submit dist/Requiem-0.0.1-macOS.dmg \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID" \
  --wait

# 装订公证票据
xcrun stapler staple dist/Requiem-0.0.1-macOS.dmg
```

---

## Windows (NSIS/MSI)

### 前置要求

```bash
# 在 Linux/macOS 上交叉编译到 Windows
rustup target add x86_64-pc-windows-gnu

# 安装 MinGW (Linux)
sudo pacman -S mingw-w64-gcc  # Arch
sudo apt install mingw-w64    # Ubuntu

# 或在 Windows 上直接编译
# 安装 Visual Studio Build Tools 或完整的 Visual Studio
```

### 1. 构建 Windows 可执行文件

```bash
# 交叉编译 (Linux/macOS -> Windows)
cargo build --release --target x86_64-pc-windows-gnu

# 或在 Windows 上直接编译
cargo build --release
```

### 2. 准备安装包文件

```bash
# 创建 Windows 打包目录
mkdir -p dist/windows
cp target/x86_64-pc-windows-gnu/release/requiem.exe dist/windows/
# 或: cp target/release/requiem.exe dist/windows/

# 复制资源文件
cp -r locales dist/windows/
cp LICENSE dist/windows/
```

### 3. 使用 NSIS 创建安装程序

安装 NSIS：
- Windows: 下载 https://nsis.sourceforge.io/Download
- Linux: `sudo pacman -S nsis` 或 `sudo apt install nsis`

创建 `installer.nsi`：

```nsis
; Requiem NSIS Installer Script

!define APP_NAME "Requiem"
!define COMP_NAME "youaodu"
!define VERSION "0.0.1"
!define COPYRIGHT "youao.du@gmail.com"
!define DESCRIPTION "A lightweight HTTP client"
!define INSTALLER_NAME "Requiem-0.0.1-Windows-Setup.exe"
!define MAIN_APP_EXE "requiem.exe"
!define INSTALL_TYPE "SetShellVarContext current"

!include "MUI2.nsh"

Name "${APP_NAME}"
Caption "${APP_NAME} ${VERSION} Setup"
OutFile "${INSTALLER_NAME}"
BrandingText "${APP_NAME}"
InstallDirRegKey HKCU "Software\${APP_NAME}" ""
InstallDir "$LOCALAPPDATA\${APP_NAME}"

;--------------------------------
; Modern UI Configuration

!define MUI_ABORTWARNING
!define MUI_UNABORTWARNING
!define MUI_ICON "assets\icon.ico"
!define MUI_UNICON "assets\icon.ico"

;--------------------------------
; Pages

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

;--------------------------------
; Languages

!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "SimpChinese"

;--------------------------------
; Installer Sections

Section "Install"
    SetOutPath "$INSTDIR"

    ; Copy files
    File "dist\windows\requiem.exe"
    File "LICENSE"
    File /r "dist\windows\locales"

    ; Create uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"

    ; Create Start Menu shortcuts
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${MAIN_APP_EXE}"
    CreateShortCut "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe"

    ; Create Desktop shortcut
    CreateShortCut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${MAIN_APP_EXE}"

    ; Registry information
    WriteRegStr HKCU "Software\${APP_NAME}" "" "$INSTDIR"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayName" "${APP_NAME}"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "UninstallString" "$INSTDIR\Uninstall.exe"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayVersion" "${VERSION}"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "Publisher" "${COMP_NAME}"
SectionEnd

;--------------------------------
; Uninstaller Section

Section "Uninstall"
    Delete "$INSTDIR\requiem.exe"
    Delete "$INSTDIR\Uninstall.exe"
    Delete "$INSTDIR\LICENSE"
    RMDir /r "$INSTDIR\locales"
    RMDir "$INSTDIR"

    ; Remove shortcuts
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    Delete "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk"
    Delete "$DESKTOP\${APP_NAME}.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"

    ; Remove registry entries
    DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"
    DeleteRegKey HKCU "Software\${APP_NAME}"
SectionEnd
```

编译 NSIS 安装程序：

```bash
# Windows
makensis installer.nsi

# Linux/macOS (交叉编译)
makensis installer.nsi
```

### 4. 使用 WiX 创建 MSI 安装程序（替代方案）

安装 WiX Toolset (Windows):
```powershell
# 使用 winget
winget install -e --id WixToolset.WixToolset

# 或下载: https://wixtoolset.org/
```

创建 `requiem.wxs`：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
  <Product Id="*" Name="Requiem" Language="1033" Version="0.0.1"
           Manufacturer="youaodu" UpgradeCode="PUT-GUID-HERE">
    <Package InstallerVersion="200" Compressed="yes" InstallScope="perUser" />

    <MajorUpgrade DowngradeErrorMessage="A newer version is already installed." />
    <MediaTemplate EmbedCab="yes" />

    <Feature Id="ProductFeature" Title="Requiem" Level="1">
      <ComponentGroupRef Id="ProductComponents" />
    </Feature>

    <Directory Id="TARGETDIR" Name="SourceDir">
      <Directory Id="LocalAppDataFolder">
        <Directory Id="INSTALLFOLDER" Name="Requiem" />
      </Directory>
      <Directory Id="ProgramMenuFolder">
        <Directory Id="ApplicationProgramsFolder" Name="Requiem"/>
      </Directory>
      <Directory Id="DesktopFolder" Name="Desktop"/>
    </Directory>

    <DirectoryRef Id="ApplicationProgramsFolder">
      <Component Id="ApplicationShortcut" Guid="PUT-GUID-HERE">
        <Shortcut Id="ApplicationStartMenuShortcut"
                  Name="Requiem"
                  Target="[INSTALLFOLDER]requiem.exe"
                  WorkingDirectory="INSTALLFOLDER"/>
        <RemoveFolder Id="ApplicationProgramsFolder" On="uninstall"/>
        <RegistryValue Root="HKCU" Key="Software\Requiem"
                       Name="installed" Type="integer" Value="1" KeyPath="yes"/>
      </Component>
    </DirectoryRef>

    <DirectoryRef Id="DesktopFolder">
      <Component Id="ApplicationDesktopShortcut" Guid="PUT-GUID-HERE">
        <Shortcut Id="ApplicationDesktopShortcut"
                  Name="Requiem"
                  Target="[INSTALLFOLDER]requiem.exe"
                  WorkingDirectory="INSTALLFOLDER"/>
        <RegistryValue Root="HKCU" Key="Software\Requiem"
                       Name="desktop" Type="integer" Value="1" KeyPath="yes"/>
      </Component>
    </DirectoryRef>

    <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
      <Component Id="MainExecutable" Guid="PUT-GUID-HERE">
        <File Source="dist\windows\requiem.exe" />
      </Component>
    </ComponentGroup>
  </Product>
</Wix>
```

编译 MSI：

```powershell
# 编译 .wixobj
candle requiem.wxs

# 链接生成 .msi
light requiem.wixobj -out Requiem-0.0.1-Windows.msi
```

### 5. 代码签名 (Windows)

```powershell
# 需要代码签名证书 (.pfx 文件)

# 使用 signtool 签名
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 requiem.exe

# 验证签名
signtool verify /pa requiem.exe
```

---

## 自动化构建脚本

### 使用 cargo-bundle (跨平台)

```bash
# 安装 cargo-bundle
cargo install cargo-bundle

# 在 Cargo.toml 中添加配置
# [package.metadata.bundle]
# name = "Requiem"
# identifier = "com.youaodu.requiem"
# icon = ["assets/icon.ico", "assets/icon.icns", "assets/icon.png"]

# 构建包
cargo bundle --release  # 根据当前平台自动选择格式
```

### 使用 GitHub Actions 自动打包

创建 `.github/workflows/release.yml`：

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build
        run: cargo build --release

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: requiem-${{ matrix.os }}
          path: target/release/requiem*
```

---

## 依赖说明

### Windows 依赖
- 系统自带：MSVC 运行时 (如果使用 MSVC 工具链) 或 MinGW 运行时
- OpenSSL: 可以静态链接或使用 `rustls` 替代
- 字体：Windows 系统字体通常足够，中文显示无问题

### macOS 依赖
- 系统自带：libSystem, CoreFoundation, CoreGraphics, AppKit
- OpenSSL: macOS 自带或通过 Homebrew 安装
- 字体：系统字体，或打包思源黑体

### Linux 依赖
详见 PKGBUILD 文件

---

## 测试安装包

```bash
# Linux
sudo pacman -U requiem-0.0.1-1-x86_64.pkg.tar.zst

# macOS
open dist/Requiem-0.0.1-macOS.dmg
# 拖动到 Applications 文件夹

# Windows
# 双击运行 Requiem-0.0.1-Windows-Setup.exe
```

---

## 发布清单

- [ ] 更新版本号 (Cargo.toml, PKGBUILD, installer scripts)
- [ ] 运行测试: `cargo test`
- [ ] 更新 CHANGELOG.md
- [ ] 创建 Git tag: `git tag v0.0.1`
- [ ] 构建所有平台的安装包
- [ ] 测试所有安装包
- [ ] 上传到 GitHub Releases
- [ ] 更新 AUR 包 (Linux)
- [ ] 提交到 Homebrew (macOS, 可选)
- [ ] 提交到 Chocolatey (Windows, 可选)
