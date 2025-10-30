# AUR Publishing Guide

本文档说明如何配置自动发布到 Arch User Repository (AUR)。

## 前置要求

### 1. 创建 AUR 账户

访问 https://aur.archlinux.org/ 并注册账户。

### 2. 配置 SSH 密钥

```bash
# 生成专用于 AUR 的 SSH 密钥
ssh-keygen -t ed25519 -C "your_email@example.com" -f ~/.ssh/aur

# 将公钥添加到 AUR 账户
cat ~/.ssh/aur.pub
# 复制内容到 https://aur.archlinux.org/account/ 的 "SSH Public Key" 部分
```

### 3. 初始化 AUR 仓库

首次发布需要手动初始化 AUR 仓库：

```bash
# Clone AUR 仓库（首次会是空的）
git clone ssh://aur@aur.archlinux.org/requiem.git aur-requiem
cd aur-requiem

# 复制 PKGBUILD
cp ../PKGBUILD .

# 生成 .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# 提交并推送
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: requiem 0.1.0"
git push origin master
```

## GitHub Actions 配置

### 配置 GitHub Secrets

在 GitHub 仓库设置中添加以下 Secrets（Settings > Secrets and variables > Actions）：

1. **AUR_SSH_PRIVATE_KEY**
   ```bash
   # 复制私钥内容
   cat ~/.ssh/aur
   ```
   将完整的私钥内容（包括 `-----BEGIN OPENSSH PRIVATE KEY-----` 和 `-----END OPENSSH PRIVATE KEY-----`）粘贴到 Secret 中。

2. **AUR_USERNAME**
   ```
   你的 AUR 用户名
   ```

3. **AUR_EMAIL**
   ```
   你的 AUR 注册邮箱
   ```

### 可选：Crates.io Token

如果要自动发布到 crates.io，添加：

4. **CARGO_REGISTRY_TOKEN**
   - 访问 https://crates.io/settings/tokens
   - 创建新 token
   - 粘贴到 Secret 中

## 工作流说明

### CI 工作流 (`.github/workflows/ci.yml`)

每次 Push 或 PR 时会：
- ✅ 验证 PKGBUILD 语法（使用 namcap）
- ✅ 在 Arch Linux 容器中构建包
- ✅ 检查包的质量
- ✅ 上传构建的包作为 artifact

### Release 工作流 (`.github/workflows/release.yml`)

当推送版本标签时（例如 `v0.1.0`）会：

1. **build-arch** job
   - 在 Arch Linux 容器中构建包
   - 自动更新 PKGBUILD 的版本号
   - 上传 `.pkg.tar.zst` 包到 GitHub Release
   - 生成更新后的 PKGBUILD（包含正确的 checksums）

2. **publish-aur** job
   - 下载构建生成的 PKGBUILD
   - 生成 .SRCINFO
   - 自动推送到 AUR

## 发布流程

### 发布新版本

```bash
# 1. 更新版本号
vim Cargo.toml  # 修改 version = "0.2.0"

# 2. 提交更改
git add Cargo.toml
git commit -m "chore: Bump version to 0.2.0"

# 3. 创建标签并推送
git tag v0.2.0
git push origin main --tags
```

GitHub Actions 会自动：
1. 创建 GitHub Release
2. 构建 Linux/Windows/macOS 二进制文件
3. 构建 Arch Linux 包（`.pkg.tar.zst`）
4. 更新 AUR 仓库

### 手动更新 AUR（如需要）

```bash
cd aur-requiem

# 更新 PKGBUILD
vim PKGBUILD

# 更新 checksums
updpkgsums

# 生成 .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# 测试构建
makepkg -sf

# 提交并推送
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.2.0"
git push origin master
```

## 包质量检查

使用 namcap 检查包质量：

```bash
# 检查 PKGBUILD
namcap PKGBUILD

# 检查构建的包
namcap requiem-*.pkg.tar.zst
```

常见检查项：
- 依赖项是否完整
- 文件权限是否正确
- LICENSE 文件是否安装
- 是否有不必要的文件

## Arch Linux 用户安装方式

### 从 AUR 安装

```bash
# 使用 yay
yay -S requiem

# 使用 paru
paru -S requiem

# 手动安装
git clone https://aur.archlinux.org/requiem.git
cd requiem
makepkg -si
```

### 从 GitHub Release 安装

```bash
# 下载预构建的包
wget https://github.com/youaodu/Requiem/releases/download/v0.1.0/requiem-0.1.0-1-x86_64.pkg.tar.zst

# 安装
sudo pacman -U requiem-0.1.0-1-x86_64.pkg.tar.zst
```

## 故障排查

### AUR 推送失败

1. 检查 SSH 密钥是否正确配置
2. 确认 AUR 账户已激活
3. 检查 .SRCINFO 是否正确生成
4. 确保 PKGBUILD 语法正确

### 构建失败

1. 检查依赖项是否完整（在 PKGBUILD 的 `depends` 中）
2. 使用 `ldd target/release/requiem` 查看运行时依赖
3. 在本地 Arch Linux 环境测试构建

### GitHub Actions 失败

1. 检查 Secrets 是否正确配置
2. 查看 Actions 日志了解具体错误
3. 可以在本地使用 Docker 测试：
   ```bash
   docker run -it --rm -v $(pwd):/build archlinux:latest bash
   cd /build
   pacman -Syu --noconfirm
   pacman -S --noconfirm base-devel rust cargo
   useradd -m builder
   chown -R builder:builder .
   su builder -c "makepkg -sf"
   ```

## 参考资源

- [AUR Submission Guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [PKGBUILD Manual](https://man.archlinux.org/man/PKGBUILD.5)
- [namcap Documentation](https://wiki.archlinux.org/title/Namcap)
- [Arch Package Guidelines](https://wiki.archlinux.org/title/Arch_package_guidelines)
