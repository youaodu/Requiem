# 故障排查指南 / Troubleshooting Guide

## Windows 问题 / Windows Issues

### 程序打开后立即关闭 / Application Closes Immediately

如果程序在 Windows 上打开后立即关闭或不显示窗口，请检查日志文件：

**日志文件位置 / Log File Locations:**

```
Windows: C:\Users\<YourUsername>\AppData\Roaming\requiem\logs\requiem-YYYYMMDD.log
macOS:   ~/Library/Application Support/requiem/logs/requiem-YYYYMMDD.log
Linux:   ~/.local/share/requiem/logs/requiem-YYYYMMDD.log
```

### 如何查看日志 / How to View Logs

#### Windows:

1. 按 `Win + R` 打开运行对话框
2. 输入: `%APPDATA%\requiem\logs`
3. 打开最新的日志文件 (requiem-YYYYMMDD.log)

或者通过命令行：
```cmd
cd %APPDATA%\requiem\logs
type requiem-*.log
```

#### 查看 Panic 日志 / View Panic Logs:

如果程序崩溃，会生成 `panic.log` 文件：

```cmd
cd %APPDATA%\requiem\logs
type panic.log
```

### 常见问题 / Common Issues

#### 1. 缺少 DLL 文件 / Missing DLL Files

**症状**: 程序无法启动，提示缺少 .dll 文件

**解决方案**:
- 安装 [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
- 或者从 Release 页面下载包含所有依赖的完整版本

#### 2. GPU/图形驱动问题 / GPU/Graphics Driver Issues

**症状**: 程序启动后黑屏或崩溃

**解决方案**:
1. 更新显卡驱动
2. 尝试使用软件渲染模式（即将支持）

#### 3. 高 DPI 显示问题 / High DPI Display Issues

**症状**: 窗口太小或文字模糊

**解决方案**:
- 右键点击 requiem.exe → 属性 → 兼容性
- 勾选 "替代高 DPI 缩放行为"
- 缩放执行者选择 "应用程序"

---

## macOS 问题 / macOS Issues

### "bad CPU type in executable" 错误

**症状**:
```
zsh: bad CPU type in executable: /path/to/requiem
```

**原因**: 下载的二进制文件架构与您的 Mac 不匹配

**解决方案**:

#### 对于 Apple Silicon (M1/M2/M3) Mac:
下载 `requiem-macos-aarch64` 版本

```bash
# 检查您的 Mac 架构
uname -m
# 如果输出 "arm64"，需要 aarch64 版本
```

#### 对于 Intel Mac:
下载 `requiem-macos-x86_64` 版本

```bash
# 检查您的 Mac 架构
uname -m
# 如果输出 "x86_64"，需要 x86_64 版本
```

### 安全性和隐私问题 / Security & Privacy Issues

**症状**: macOS 阻止应用运行，提示"无法验证开发者"

**解决方案**:

1. **临时允许**:
   ```bash
   xattr -d com.apple.quarantine /path/to/requiem
   ```

2. **通过系统设置允许**:
   - 打开"系统设置" → "隐私与安全性"
   - 找到被阻止的应用，点击"仍要打开"

3. **查看日志**:
   ```bash
   # 查看应用日志
   cat ~/Library/Application\ Support/requiem/logs/requiem-*.log

   # 查看崩溃日志（如果有）
   cat ~/Library/Application\ Support/requiem/logs/panic.log
   ```

---

## Linux 问题 / Linux Issues

### 缺少系统依赖 / Missing System Dependencies

**症状**: 程序无法启动，提示缺少共享库

**解决方案**:

#### Arch Linux:
```bash
sudo pacman -S libfontconfig libfreetype2 libxcb libxkbcommon
```

#### Ubuntu/Debian:
```bash
sudo apt install libfontconfig1 libfreetype6 libxcb-render0 \
                 libxcb-shape0 libxcb-xfixes0 libxkbcommon0
```

#### Fedora:
```bash
sudo dnf install fontconfig freetype libxcb libxkbcommon
```

### 中文显示问题 / Chinese Character Display Issues

**症状**: 中文显示为方框或乱码

**解决方案**:

安装中文字体：

#### Arch Linux:
```bash
sudo pacman -S adobe-source-han-sans-otf-fonts
# 或
sudo pacman -S noto-fonts-cjk
```

#### Ubuntu/Debian:
```bash
sudo apt install fonts-noto-cjk
```

### Wayland vs X11 问题 / Wayland vs X11 Issues

**症状**: 窗口行为异常或无法启动

**解决方案**:

强制使用 X11：
```bash
# 临时切换到 X11
export WAYLAND_DISPLAY=
./requiem
```

或者在 Wayland 下运行：
```bash
# 确保 Wayland 环境变量设置正确
echo $WAYLAND_DISPLAY
./requiem
```

---

## 性能问题 / Performance Issues

### 高内存占用 / High Memory Usage

**症状**: 程序占用超过 200MB 内存

**诊断步骤**:

1. 检查打开的请求数量
2. 检查响应体大小
3. 查看日志中的内存警告

**解决方案**:
- 关闭不需要的请求标签
- 清理大型响应数据
- 重启应用

### 启动缓慢 / Slow Startup

**症状**: 启动时间超过 5 秒

**可能原因**:
1. 大量的保存请求（collection.json 文件过大）
2. 磁盘 I/O 缓慢
3. 杀毒软件扫描

**解决方案**:
1. 清理不需要的请求
2. 添加杀毒软件白名单
3. 检查日志查看具体耗时

---

## 调试模式 / Debug Mode

### 启用详细日志 / Enable Verbose Logging

#### Windows (PowerShell):
```powershell
$env:RUST_LOG="debug"
.\requiem.exe
```

#### Windows (CMD):
```cmd
set RUST_LOG=debug
requiem.exe
```

#### macOS/Linux:
```bash
RUST_LOG=debug ./requiem
```

### 跟踪模式 / Trace Mode (最详细)

```bash
RUST_LOG=trace ./requiem
```

---

## 获取帮助 / Getting Help

如果以上方法都无法解决您的问题，请：

1. **查看日志文件** 并复制相关错误信息
2. **收集系统信息**:
   - 操作系统版本
   - CPU 架构 (x86_64 / ARM64)
   - 显卡型号
3. **提交 Issue**: https://github.com/youaodu/Requiem/issues
   - 附上日志文件内容
   - 描述复现步骤
   - 附上系统信息

---

## 常用命令速查 / Quick Command Reference

### 查看日志 / View Logs

```bash
# Windows (PowerShell)
cat $env:APPDATA\requiem\logs\requiem-*.log

# macOS
cat ~/Library/Application\ Support/requiem/logs/requiem-*.log

# Linux
cat ~/.local/share/requiem/logs/requiem-*.log
```

### 清理数据 / Clean Data

```bash
# Windows (PowerShell)
Remove-Item -Recurse $env:APPDATA\requiem

# macOS
rm -rf ~/Library/Application\ Support/requiem

# Linux
rm -rf ~/.local/share/requiem
```

### 检查架构 / Check Architecture

```bash
# macOS/Linux
uname -m

# Windows (PowerShell)
$env:PROCESSOR_ARCHITECTURE
```
