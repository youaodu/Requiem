# Arch Linux 支持

Requiem 为 Arch Linux 用户提供全面的打包支持，包括 AUR 发布和预构建包。

## 🚀 快速安装

### 方法 1: 从 AUR 安装（推荐）

```bash
# 使用 yay
yay -S requiem

# 使用 paru
paru -S requiem

# 手动从 AUR 构建
git clone https://aur.archlinux.org/requiem.git
cd requiem
makepkg -si
```

### 方法 2: 从 GitHub Release 安装预构建包

```bash
# 下载最新版本
VERSION="0.1.0"  # 替换为最新版本号
wget https://github.com/youaodu/Requiem/releases/download/v${VERSION}/requiem-${VERSION}-1-x86_64.pkg.tar.zst

# 安装
sudo pacman -U requiem-${VERSION}-1-x86_64.pkg.tar.zst
```

### 方法 3: 从源码构建

```bash
git clone https://github.com/youaodu/Requiem.git
cd Requiem
makepkg -si
```

## 📦 依赖项

### 运行时依赖

- `gcc-libs` - C 运行库
- `glibc` - GNU C 库
- `openssl` - SSL/TLS 支持
- `fontconfig` - 字体配置（iced GUI 框架需要）

### 可选依赖

- `adobe-source-han-sans-otf-fonts` - 中文字符支持
- `nodejs` - AI 功能需要（Claude Code/Codex）
- `npm` - AI 功能需要（Claude Code/Codex）

### 编译依赖

- `rust` - Rust 编译器
- `cargo` - Rust 包管理器

## 🔧 本地测试构建

使用提供的脚本在 Docker 容器中测试 PKGBUILD：

```bash
# 运行测试脚本
./scripts/test-arch-build.sh
```

该脚本会：
1. 启动 Arch Linux Docker 容器
2. 安装所有依赖
3. 验证 PKGBUILD 语法
4. 构建包
5. 运行质量检查
6. 显示包信息

## 📋 包信息

### 包含的文件

```
/usr/bin/requiem                           # 主程序
/usr/share/licenses/requiem/LICENSE        # 许可证
/usr/share/requiem/locales/en.json         # 英文翻译
/usr/share/requiem/locales/zh.json         # 中文翻译
```

### 查看包依赖

```bash
# 查看运行时依赖
ldd /usr/bin/requiem

# 查看 Pacman 记录的依赖
pacman -Qi requiem
```

## 🛠️ 开发者指南

### 更新 PKGBUILD

1. 修改版本号：
   ```bash
   vim PKGBUILD  # 更新 pkgver=
   ```

2. 更新校验和：
   ```bash
   updpkgsums
   ```

3. 生成 .SRCINFO：
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

4. 测试构建：
   ```bash
   makepkg -sf
   ```

5. 验证包质量：
   ```bash
   namcap PKGBUILD
   namcap requiem-*.pkg.tar.zst
   ```

### CI/CD 自动化

项目配置了完整的 GitHub Actions 工作流：

#### CI 检查 (每次 Push/PR)
- ✅ PKGBUILD 语法验证
- ✅ 在 Arch Linux 容器中构建测试
- ✅ namcap 质量检查

#### Release 发布 (推送版本标签)
```bash
git tag v0.2.0
git push origin main --tags
```

自动执行：
1. 创建 GitHub Release
2. 构建 Linux/Windows/macOS 二进制
3. 构建 Arch Linux 包（.pkg.tar.zst）
4. 自动更新 AUR 仓库

详见：[AUR Publishing Guide](.github/workflows/AUR_PUBLISHING.md)

## 🐛 故障排查

### 字体问题

如果遇到中文显示问题：

```bash
# 安装中文字体
sudo pacman -S adobe-source-han-sans-otf-fonts

# 或使用其他中文字体
sudo pacman -S noto-fonts-cjk
```

### 依赖问题

检查缺失的依赖：

```bash
# 查看共享库依赖
ldd /usr/bin/requiem

# 查找缺失的库
pacman -Fs libXXX.so
```

### AI 功能问题

如果需要使用 AI 功能：

```bash
# 安装 Node.js
sudo pacman -S nodejs npm

# 验证安装
npx --version
```

## 📚 相关资源

- [PKGBUILD 文件](/PKGBUILD)
- [AUR 发布指南](.github/workflows/AUR_PUBLISHING.md)
- [CI/CD 配置](.github/workflows/ci.yml)
- [Arch Wiki - AUR](https://wiki.archlinux.org/title/Arch_User_Repository)
- [Arch Wiki - PKGBUILD](https://wiki.archlinux.org/title/PKGBUILD)

## 💬 反馈

遇到问题或有建议？

- 🐛 [报告 Bug](https://github.com/youaodu/Requiem/issues)
- 💡 [功能建议](https://github.com/youaodu/Requiem/discussions)
- 📧 联系：youao.du@gmail.com
