# GitHub Actions 配置清单

使用此清单确保 CI/CD 正确配置。

## ✅ 基础设置（无需配置）

以下功能开箱即用，无需任何配置：

- [x] CI 代码检查和测试
- [x] 多平台构建（Linux/Windows/macOS）
- [x] Arch Linux PKGBUILD 验证
- [x] GitHub Release 创建
- [x] 预构建二进制文件上传
- [x] 依赖自动更新（每周）

**首次使用：** 只需推送代码到 GitHub 即可自动触发 CI！

## 📦 可选配置

### 选项 1: 发布到 AUR

**何时需要：** 想让 Arch Linux 用户通过 `yay -S requiem` 安装

**步骤：**

#### 1. 创建 AUR 账户
- [ ] 访问 https://aur.archlinux.org/register
- [ ] 注册账户并激活

#### 2. 生成 SSH 密钥
```bash
ssh-keygen -t ed25519 -C "your_email@example.com" -f ~/.ssh/aur
```

- [ ] 将公钥（`~/.ssh/aur.pub`）添加到 AUR 账户
  - 登录 AUR
  - 访问 https://aur.archlinux.org/account/
  - 在 "SSH Public Key" 部分粘贴公钥内容

#### 3. 初始化 AUR 仓库（首次）
```bash
# Clone AUR 仓库（首次会是空的）
git clone ssh://aur@aur.archlinux.org/requiem.git aur-requiem
cd aur-requiem

# 复制 PKGBUILD
cp /path/to/Requiem/PKGBUILD .

# 生成 .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# 提交并推送
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: requiem 0.1.0"
git push
```

- [ ] 确认包已在 AUR 上：https://aur.archlinux.org/packages/requiem

#### 4. 配置 GitHub Secrets

在 GitHub 仓库设置中（Settings > Secrets and variables > Actions > New repository secret）：

- [ ] **AUR_SSH_PRIVATE_KEY**
  ```bash
  # 复制私钥内容
  cat ~/.ssh/aur
  ```
  复制完整输出（包括 BEGIN 和 END 行）

- [ ] **AUR_USERNAME**
  ```
  你的 AUR 用户名
  ```

- [ ] **AUR_EMAIL**
  ```
  你的 AUR 注册邮箱
  ```

#### 5. 测试发布

```bash
# 创建测试标签
git tag v0.1.0-test
git push origin v0.1.0-test

# 检查 Actions 页面
# https://github.com/youaodu/Requiem/actions
```

- [ ] 查看 `publish-aur` job 是否成功
- [ ] 确认 AUR 包已更新：https://aur.archlinux.org/packages/requiem

---

### 选项 2: 发布到 crates.io

**何时需要：** 想让其他 Rust 开发者通过 `cargo install requiem` 安装

**步骤：**

#### 1. 创建 crates.io 账户
- [ ] 访问 https://crates.io/
- [ ] 使用 GitHub 账户登录

#### 2. 生成 API Token
- [ ] 访问 https://crates.io/settings/tokens
- [ ] 点击 "New Token"
- [ ] 输入名称（如 "GitHub Actions"）
- [ ] 复制生成的 token（只显示一次！）

#### 3. 配置 GitHub Secret
- [ ] **CARGO_REGISTRY_TOKEN**
  - 访问 GitHub 仓库设置
  - Settings > Secrets and variables > Actions
  - New repository secret
  - 粘贴 crates.io token

#### 4. 首次手动发布

```bash
# 确保 Cargo.toml 配置完整
cargo publish --dry-run

# 实际发布
cargo publish
```

- [ ] 确认包已在 crates.io 上：https://crates.io/crates/requiem

#### 5. 后续自动发布

之后每次推送版本标签时，GitHub Actions 会自动发布到 crates.io

---

## 🧪 验证配置

### 测试 CI 工作流

```bash
# 1. 创建测试分支
git checkout -b test-ci

# 2. 做一个小改动
echo "# Test" >> README.md

# 3. 提交并推送
git add README.md
git commit -m "test: Verify CI workflow"
git push origin test-ci

# 4. 创建 PR
# 访问 GitHub 仓库，创建 Pull Request

# 5. 检查 CI 状态
# PR 页面会显示所有 CI 检查结果
```

- [ ] `check` job 通过（格式和 Clippy）
- [ ] `test` job 通过（所有平台）
- [ ] `build-*` jobs 通过（所有平台）
- [ ] `verify-pkgbuild` job 通过（Arch 包）

### 测试 Release 工作流

```bash
# 1. 更新版本号
vim Cargo.toml  # version = "0.1.0"

# 2. 提交
git add Cargo.toml
git commit -m "chore: Release v0.1.0"
git push origin main

# 3. 创建标签
git tag v0.1.0
git push origin v0.1.0

# 4. 检查 Release 页面
# https://github.com/youaodu/Requiem/releases
```

- [ ] GitHub Release 已创建
- [ ] 4 个平台的二进制文件已上传
- [ ] Arch Linux 包已上传
- [ ] AUR 已更新（如果配置）
- [ ] crates.io 已更新（如果配置）

### 测试本地 Arch 构建

```bash
# 使用提供的脚本
./scripts/test-arch-build.sh
```

- [ ] Docker 容器成功启动
- [ ] 依赖安装成功
- [ ] PKGBUILD 验证通过
- [ ] 包构建成功
- [ ] namcap 检查通过

---

## 📋 维护清单

### 每次发布新版本

- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 更新 `CHANGELOG.md`（如果有）
- [ ] 确保所有测试通过：`cargo test`
- [ ] 本地构建测试：`cargo build --release`
- [ ] 提交更改
- [ ] 创建并推送标签

### 定期维护（建议每月）

- [ ] 检查依赖更新 PR
- [ ] 审查 GitHub Actions 日志
- [ ] 更新 README 和文档
- [ ] 检查 AUR 包反馈（如果发布到 AUR）

### PKGBUILD 维护

当添加新的依赖时：

```bash
# 1. 检查运行时依赖
ldd target/release/requiem

# 2. 更新 PKGBUILD 的 depends 数组
# 3. 本地测试
makepkg -sf

# 4. 验证
namcap PKGBUILD
namcap requiem-*.pkg.tar.zst

# 5. 提交更改
git add PKGBUILD
git commit -m "build: Update PKGBUILD dependencies"
```

---

## 🆘 获取帮助

### 文档资源

- [GitHub Actions 总览](.github/GITHUB_ACTIONS_SUMMARY.md)
- [AUR 发布指南](.github/workflows/AUR_PUBLISHING.md)
- [Arch Linux 指南](.github/ARCH_LINUX.md)

### 遇到问题？

1. **查看 Actions 日志**
   - https://github.com/youaodu/Requiem/actions
   - 点击失败的 job 查看详细日志

2. **本地重现问题**
   - CI 失败：本地运行相同命令
   - Arch 构建失败：使用 `./scripts/test-arch-build.sh`

3. **寻求帮助**
   - 📝 [创建 Issue](https://github.com/youaodu/Requiem/issues)
   - 💬 [讨论区](https://github.com/youaodu/Requiem/discussions)

---

## ✨ 快速命令参考

```bash
# CI 相关
cargo fmt --all              # 格式化代码
cargo clippy --all-features  # Lint 检查
cargo test --workspace       # 运行测试
cargo build --release        # 构建 release

# Arch 包相关
./scripts/test-arch-build.sh # 本地测试 Arch 构建
makepkg -sf                  # 构建包
namcap PKGBUILD              # 检查 PKGBUILD
updpkgsums                   # 更新校验和

# 发布相关
git tag v0.1.0               # 创建标签
git push origin --tags       # 推送标签
cargo publish --dry-run      # 测试发布到 crates.io
```

---

**状态：**
- [x] 基础 CI/CD 已配置
- [ ] AUR 发布已配置（可选）
- [ ] crates.io 发布已配置（可选）

祝你构建愉快！🎉
