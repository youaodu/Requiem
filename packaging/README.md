# Requiem 打包指南

本目录包含用于创建跨平台安装包的配置文件和脚本。

## 目录结构

```
packaging/
├── README.md                    # 本文档
├── macos/
│   ├── Info.plist              # macOS .app bundle 配置
│   └── create-dmg.sh           # DMG 创建脚本
└── windows/
    └── build-msi.ps1           # MSI 创建脚本（PowerShell）
```

根目录还包含：
- `requiem.wxs` - WiX Toolset 配置文件（用于生成 Windows MSI）

---

## 自动构建（推荐）

### 通过 GitHub Actions 发布

当你推送一个版本标签时，GitHub Actions 会自动构建所有平台的安装包：

```bash
# 更新版本号
# 编辑 Cargo.toml, requiem.wxs, packaging/macos/Info.plist

# 创建并推送标签
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions 将会生成：
- ✅ **Windows**: `Requiem-0.1.0-Windows.msi` + `.zip` (x86_64)
- ✅ **macOS**: `Requiem-0.1.0-macOS-Universal.dmg` + `.tar.gz` (Intel + ARM64)
- ✅ **Linux**: `requiem-0.1.0-linux-x86_64.tar.gz`

这些文件会自动上传到 GitHub Releases。

---

## 手动构建

### Windows MSI

#### 前置要求

```powershell
# 安装 WiX Toolset v5
dotnet tool install --global wix --version 5.0.1

# 或者下载安装器：https://wixtoolset.org/
```

#### 构建步骤

```powershell
# 1. 构建 Rust 二进制
cargo build --release

# 2. 使用脚本构建 MSI
cd packaging/windows
.\build-msi.ps1 -Version "0.1.0"

# 或手动构建
wix build ../../requiem.wxs -out dist/Requiem-0.1.0-Windows.msi
```

生成的 MSI 位于 `dist/Requiem-0.1.0-Windows.msi`

#### 测试安装

```powershell
# 安装
msiexec /i dist\Requiem-0.1.0-Windows.msi

# 卸载
msiexec /x dist\Requiem-0.1.0-Windows.msi
```

---

### macOS DMG

#### 前置要求

```bash
# macOS 系统自带所有必要工具
# 无需额外安装
```

#### 构建 Universal Binary（推荐）

```bash
# 1. 安装目标架构
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# 2. 构建两个架构
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# 3. 合并为 Universal Binary
lipo -create \
  target/x86_64-apple-darwin/release/requiem \
  target/aarch64-apple-darwin/release/requiem \
  -output target/release/requiem

# 4. 创建 DMG
cd packaging/macos
./create-dmg.sh 0.1.0
```

生成的 DMG 位于 `dist/Requiem-0.1.0-macOS.dmg`

#### 仅构建当前架构

```bash
# 1. 构建当前架构
cargo build --release

# 2. 创建 DMG
cd packaging/macos
./create-dmg.sh 0.1.0
```

#### 测试安装

```bash
# 打开 DMG
open dist/Requiem-0.1.0-macOS.dmg

# 拖动到 Applications 文件夹
# 然后从 Launchpad 或 Applications 启动
```

---

### Linux (Arch Linux PKGBUILD)

```bash
# 1. 编辑 PKGBUILD 更新版本号
vim PKGBUILD

# 2. 构建包
makepkg -si

# 3. 安装
sudo pacman -U requiem-0.1.0-1-x86_64.pkg.tar.zst
```

详见根目录的 `PKGBUILD` 和 `PACKAGING.md`。

---

## 版本更新清单

发布新版本时，需要更新以下文件中的版本号：

- [ ] `Cargo.toml` - `version = "0.1.0"`
- [ ] `requiem.wxs` - `Version="0.1.0"`（GitHub Actions 会自动替换）
- [ ] `packaging/macos/Info.plist` - `<string>0.1.0</string>`（脚本会自动替换）
- [ ] `PKGBUILD` - `pkgver=0.1.0`

然后：
```bash
# 更新 Cargo.lock
cargo update

# 提交更改
git add .
git commit -m "chore: Bump version to 0.1.0"

# 创建标签
git tag v0.1.0
git push origin main --tags
```

---

## 故障排除

### Windows

**问题**: `wix: command not found`
```powershell
# 解决方案：确保 WiX 在 PATH 中
echo $env:PATH
# 手动添加：C:\Users\<YourName>\.dotnet\tools
```

**问题**: `candle.exe` 或 `light.exe` 找不到
```powershell
# 这是 WiX v3 的命令，请使用 WiX v5：
wix build requiem.wxs
```

### macOS

**问题**: `lipo` 失败
```bash
# 解决方案：检查二进制文件是否存在
file target/x86_64-apple-darwin/release/requiem
file target/aarch64-apple-darwin/release/requiem
```

**问题**: DMG 挂载失败
```bash
# 解决方案：清理旧的挂载点
hdiutil detach /Volumes/Requiem
rm -rf dist/dmg
```

### Linux

**问题**: `makepkg` 失败
```bash
# 解决方案：更新校验和
updpkgsums

# 或者跳过校验和检查（仅开发时）
makepkg --skipinteg
```

---

## 代码签名（可选）

### Windows

```powershell
# 需要代码签名证书 (.pfx)
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 requiem.exe
```

### macOS

```bash
# 需要 Apple Developer 账户
codesign --force --deep --sign "Developer ID Application: Your Name" dist/Requiem.app

# 公证
xcrun notarytool submit dist/Requiem-0.1.0-macOS.dmg \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID" \
  --wait

# 装订
xcrun stapler staple dist/Requiem-0.1.0-macOS.dmg
```

---

## 文件大小参考

| 平台 | 格式 | 预估大小 |
|------|------|---------|
| Windows | MSI | ~15-25 MB |
| Windows | ZIP | ~10-15 MB |
| macOS | DMG | ~20-30 MB |
| macOS | tarball | ~10-15 MB |
| Linux | tarball | ~10-15 MB |
| Linux | PKGBUILD | ~10-15 MB |

实际大小取决于：
- Rust 编译优化设置 (`release` profile)
- 是否使用 `strip` 移除调试符号
- 是否包含额外资源（字体、图标等）

---

## 参考资料

- [WiX Toolset 文档](https://wixtoolset.org/docs/)
- [macOS App Bundle 指南](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/BundleTypes/BundleTypes.html)
- [Arch Linux PKGBUILD 手册](https://wiki.archlinux.org/title/PKGBUILD)
- [GitHub Actions 自动发布](https://docs.github.com/en/actions/publishing-packages)
