# GitHub Actions CI/CD 配置总结

本项目已配置完整的 CI/CD 流程，支持多平台构建和自动发布。

## 📋 工作流概览

### 1. CI 工作流 (`.github/workflows/ci.yml`)

**触发条件：** Push 到 `main`/`develop` 分支或创建 PR

**包含任务：**

| Job | 平台 | 描述 |
|-----|------|------|
| `check` | Ubuntu | 代码格式检查、Clippy 检查 |
| `test` | Ubuntu/Windows/macOS | 运行测试套件 |
| `build-linux` | Ubuntu | 构建 Linux 二进制文件 |
| `build-windows` | Windows | 构建 Windows 二进制文件 |
| `build-macos` | macOS | 构建 macOS 二进制文件 |
| `verify-pkgbuild` | Arch Linux (容器) | 验证 PKGBUILD，构建 Arch 包 |

**优化特性：**
- ✅ Cargo 缓存（加速后续构建）
- ✅ 并行执行（多个 job 同时运行）
- ✅ 自动安装系统依赖
- ✅ namcap 质量检查（Arch 包）

### 2. Release 工作流 (`.github/workflows/release.yml`)

**触发条件：** 推送版本标签（如 `v0.1.0`）

**发布流程：**

```
Tag Push (v0.1.0)
    ↓
[create-release] 创建 GitHub Release
    ↓
并行执行 ↓↓↓↓
    ├── [build-linux] → Linux tarball
    ├── [build-windows] → Windows zip
    ├── [build-macos] → macOS tarball
    ├── [build-arch] → Arch package (.pkg.tar.zst)
    └── [publish-crates] → crates.io
    ↓
[publish-aur] 自动推送到 AUR
```

**生成的发布资产：**
- `requiem-{version}-linux-x86_64.tar.gz`
- `requiem-{version}-windows-x86_64.zip`
- `requiem-{version}-macos-x86_64.tar.gz`
- `requiem-{version}-1-x86_64.pkg.tar.zst` (Arch Linux)

### 3. 依赖更新工作流 (`.github/workflows/dependency-update.yml`)

**触发条件：**
- 定时：每周一 00:00 UTC
- 手动：GitHub Actions 页面手动触发

**功能：**
- 自动运行 `cargo update`
- 运行测试确保兼容性
- 创建 PR 等待审查

## 🎯 使用场景

### 日常开发

```bash
# 1. 提交代码
git add .
git commit -m "feat: Add new feature"
git push origin main

# 2. CI 自动运行
# ✅ 格式检查
# ✅ Clippy 检查
# ✅ 测试
# ✅ 构建验证
# ✅ Arch PKGBUILD 验证
```

### 发布新版本

```bash
# 1. 更新版本号
vim Cargo.toml  # version = "0.2.0"
git add Cargo.toml
git commit -m "chore: Bump version to 0.2.0"

# 2. 创建标签
git tag v0.2.0
git push origin main --tags

# 3. 自动发布
# ✅ 创建 GitHub Release
# ✅ 构建 4 个平台的二进制
# ✅ 上传到 GitHub Release
# ✅ 发布到 crates.io（可选）
# ✅ 更新 AUR 仓库
```

### 更新依赖

**自动：** 每周一自动检查并创建 PR

**手动：**
1. 访问 Actions 页面
2. 选择 "Dependency Update"
3. 点击 "Run workflow"

## 🔧 配置要求

### 必需的 Secrets

无需配置（基础功能可用）

### 可选的 Secrets

#### 发布到 AUR

在 GitHub 仓库的 Settings > Secrets and variables > Actions 中添加：

| Secret | 说明 | 获取方式 |
|--------|------|----------|
| `AUR_SSH_PRIVATE_KEY` | AUR SSH 私钥 | `ssh-keygen -t ed25519 -f ~/.ssh/aur` |
| `AUR_USERNAME` | AUR 用户名 | 你的 AUR 账户名 |
| `AUR_EMAIL` | AUR 邮箱 | 你的 AUR 注册邮箱 |

详见：[AUR Publishing Guide](workflows/AUR_PUBLISHING.md)

#### 发布到 crates.io

| Secret | 说明 | 获取方式 |
|--------|------|----------|
| `CARGO_REGISTRY_TOKEN` | Crates.io API Token | https://crates.io/settings/tokens |

## 📊 状态徽章

可以在 README.md 中添加：

```markdown
[![CI](https://github.com/youaodu/Requiem/workflows/CI/badge.svg)](https://github.com/youaodu/Requiem/actions)
[![Release](https://github.com/youaodu/Requiem/workflows/Release/badge.svg)](https://github.com/youaodu/Requiem/releases)
[![Dependency Update](https://github.com/youaodu/Requiem/workflows/Dependency%20Update/badge.svg)](https://github.com/youaodu/Requiem/actions)
```

## 🐛 故障排查

### CI 失败

**格式检查失败：**
```bash
# 本地运行
cargo fmt --all -- --check

# 自动修复
cargo fmt --all
```

**Clippy 警告：**
```bash
# 本地运行
cargo clippy --all-targets --all-features -- -D warnings

# 修复后重新提交
```

**测试失败：**
```bash
# 本地运行所有测试
cargo test --all-features --workspace
```

### Arch 构建失败

**本地测试：**
```bash
# 使用 Docker 测试
./scripts/test-arch-build.sh

# 或手动测试
docker run -it --rm -v $(pwd):/build archlinux:latest bash
cd /build
pacman -Syu --noconfirm
pacman -S --noconfirm base-devel rust cargo
useradd -m builder && chown -R builder:builder .
su builder -c "makepkg -sf"
```

**常见问题：**
1. 依赖缺失 → 更新 PKGBUILD 的 `depends` 或 `makedepends`
2. 版本号不匹配 → 确保 PKGBUILD 和 Cargo.toml 版本一致
3. 校验和错误 → 运行 `updpkgsums`

### AUR 发布失败

**检查清单：**
- [ ] AUR SSH 密钥已添加到 GitHub Secrets
- [ ] AUR 账户已激活
- [ ] 首次发布已手动完成（初始化 AUR 仓库）
- [ ] .SRCINFO 正确生成

**手动发布：**
```bash
cd /tmp
git clone ssh://aur@aur.archlinux.org/requiem.git
cd requiem
cp /path/to/Requiem/PKGBUILD .
makepkg --printsrcinfo > .SRCINFO
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.2.0"
git push
```

## 📈 性能优化

### 缓存策略

工作流已配置 Cargo 缓存：
- Registry 缓存（`~/.cargo/registry`）
- Index 缓存（`~/.cargo/git`）
- Build 缓存（`target/`）

**效果：**
- 首次构建：~5-10 分钟
- 缓存命中：~2-3 分钟

### 并行执行

多个 job 并行运行，充分利用 GitHub Actions 的并发能力：
- 测试在 3 个平台同时进行
- 构建任务并行执行
- 总耗时取决于最慢的 job

## 🎓 学习资源

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Arch Linux 打包指南](https://wiki.archlinux.org/title/Creating_packages)
- [AUR 提交指南](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [Cargo 发布指南](https://doc.rust-lang.org/cargo/reference/publishing.html)

## 📝 工作流文件

| 文件 | 描述 |
|------|------|
| [ci.yml](workflows/ci.yml) | 持续集成（代码检查、测试、构建） |
| [release.yml](workflows/release.yml) | 发布自动化（多平台构建、AUR） |
| [dependency-update.yml](workflows/dependency-update.yml) | 依赖自动更新 |

## 🤝 贡献

欢迎改进工作流配置！提交 PR 时请确保：
- [ ] 测试过修改后的工作流
- [ ] 更新相关文档
- [ ] 说明改动原因

## 📧 支持

遇到 CI/CD 相关问题？
- 📝 [查看 Actions 日志](https://github.com/youaodu/Requiem/actions)
- 🐛 [报告问题](https://github.com/youaodu/Requiem/issues)
- 💬 [讨论](https://github.com/youaodu/Requiem/discussions)
