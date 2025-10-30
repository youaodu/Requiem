# Application Icon Guide

本文档说明如何为 Requiem 创建和使用应用程序图标。

## 📁 图标文件位置

```
assets/
├── icon.png     # 源图标 (已有，来自 src/resources/logo.png)
├── icon.ico     # Windows 图标 (自动生成)
└── icon.icns    # macOS 图标 (在 macOS 上生成)
```

## 🎨 图标要求

### 源文件 (icon.png)
- **格式**: PNG
- **推荐尺寸**: 1024x1024 像素
- **透明背景**: 推荐（但非必须）
- **位置**: `assets/icon.png`

### Windows 图标 (.ico)
- **格式**: ICO
- **包含尺寸**: 16, 32, 48, 64, 128, 256 像素
- **自动生成**: 由 `scripts/convert-icons.sh` 或 CI 自动创建

### macOS 图标 (.icns)
- **格式**: ICNS
- **包含尺寸**: 16, 32, 64, 128, 256, 512 像素 + @2x 版本
- **自动生成**: 在 macOS 系统上由脚本自动创建

---

## 🛠️ 本地生成图标

### 方法 1: 使用自动化脚本（推荐）

```bash
# 在项目根目录执行
./scripts/convert-icons.sh
```

**前置要求**:
- **Linux/macOS**: ImageMagick (`brew install imagemagick` 或 `sudo apt install imagemagick`)
- **Windows**: ImageMagick (`choco install imagemagick`)
- **macOS (.icns)**: macOS 系统自带的 `sips` 和 `iconutil` 工具

### 方法 2: 手动转换

#### Windows (.ico)

**使用 ImageMagick**:
```bash
convert assets/icon.png \
  -define icon:auto-resize=256,128,64,48,32,16 \
  assets/icon.ico
```

**或使用在线工具**:
- https://convertio.co/png-ico/
- https://www.icoconverter.com/

#### macOS (.icns)

**在 macOS 上使用系统工具**:
```bash
# 创建 iconset 目录
mkdir -p assets/icon.iconset

# 生成各种尺寸
for size in 16 32 64 128 256 512; do
  size2x=$((size * 2))
  sips -z $size $size assets/icon.png \
    --out "assets/icon.iconset/icon_${size}x${size}.png"
  sips -z $size2x $size2x assets/icon.png \
    --out "assets/icon.iconset/icon_${size}x${size}@2x.png"
done

# 转换为 .icns
iconutil -c icns assets/icon.iconset -o assets/icon.icns
rm -rf assets/icon.iconset
```

**或使用在线工具**:
- https://cloudconvert.com/png-to-icns

---

## 🚀 CI/CD 自动化

GitHub Actions 会在发布时自动生成图标：

### Windows
```yaml
- name: Install ImageMagick
  run: choco install imagemagick -y

- name: Convert icon
  run: magick assets/icon.png -define icon:auto-resize=... assets/icon.ico
```

### macOS
```yaml
- name: Create macOS icon
  run: |
    # 使用 sips 和 iconutil 生成 .icns
    # (详见 .github/workflows/release.yml)
```

---

## 🔧 如何图标被使用

### Windows

1. **构建时嵌入**:
   - `build.rs` 使用 `winres` crate 将图标嵌入到 `.exe` 文件中
   - 在 Windows 资源中设置应用程序元数据

2. **MSI 安装包**:
   - 安装程序会从 `.exe` 中提取图标
   - 用于桌面快捷方式和开始菜单

3. **效果**:
   - ✅ 任务栏显示应用图标
   - ✅ 文件资源管理器显示图标
   - ✅ 不再显示控制台图标

### macOS

1. **.app Bundle**:
   - 图标放在 `Requiem.app/Contents/Resources/icon.icns`
   - `Info.plist` 中引用: `<key>CFBundleIconFile</key><string>icon</string>`

2. **效果**:
   - ✅ Dock 显示应用图标
   - ✅ Finder 显示图标
   - ✅ 应用程序切换器显示图标

### Linux

Linux 通常使用桌面文件 (`.desktop`) 指定图标，目前通过 PKGBUILD 安装时可以配置。

---

## 🎯 当前状态

### ✅ 已完成
- [x] Windows 图标自动转换和嵌入
- [x] macOS 图标自动转换
- [x] GitHub Actions 自动化
- [x] 本地构建脚本

### 📝 说明
- Windows `.ico` 已在 Linux CI 上生成 ✅
- macOS `.icns` 需要在 macOS 系统上生成（CI 自动完成）✅
- `build.rs` 会在 Windows 构建时自动嵌入图标 ✅

---

## 🐛 故障排除

### Windows 图标未显示

**问题**: 构建后图标仍是控制台样式

**解决方案**:
1. 确保 `assets/icon.ico` 存在
2. 确保在 **Release 模式**构建: `cargo build --release`
3. 检查 `build.rs` 是否正确执行
4. 查看构建日志中是否有 winres 警告

### macOS 图标未显示

**问题**: .app 显示默认图标

**解决方案**:
1. 确保 `assets/icon.icns` 存在于 `.app/Contents/Resources/`
2. 检查 `Info.plist` 中是否有 `CFBundleIconFile`
3. 清除图标缓存:
   ```bash
   sudo rm -rf /Library/Caches/com.apple.iconservices.store
   sudo find /private/var/folders/ -name com.apple.dock.iconcache -exec rm {} \;
   killall Dock
   ```

### 图标转换失败

**问题**: `convert-icons.sh` 脚本失败

**解决方案**:
1. 安装 ImageMagick: `brew install imagemagick`
2. 确保 `assets/icon.png` 存在
3. 使用在线工具手动转换

---

## 📚 参考资料

- [Windows Icon Format (.ico)](https://en.wikipedia.org/wiki/ICO_(file_format))
- [macOS Icon Format (.icns)](https://en.wikipedia.org/wiki/Apple_Icon_Image_format)
- [winres crate documentation](https://docs.rs/winres/)
- [Apple Icon Guidelines](https://developer.apple.com/design/human-interface-guidelines/app-icons)
