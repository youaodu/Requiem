# GitHub Actions 快速参考

## 🚀 一键启动

```bash
# 1. 推送代码 → 自动触发 CI
git push origin main

# 2. 发布版本 → 自动构建和发布
git tag v0.1.0 && git push origin --tags
```

就这么简单！无需任何配置。

## 📊 工作流状态

| 工作流 | 何时触发 | 耗时 | 产出 |
|--------|---------|------|------|
| **CI** | Push/PR | ~5-10min | 代码检查、测试、构建验证 |
| **Release** | 版本标签 | ~10-15min | 二进制文件、GitHub Release |
| **依赖更新** | 每周一 | ~5min | 自动 PR |

## 🎯 常用命令

```bash
# === CI 相关 ===
cargo fmt --all                    # 格式化
cargo clippy --all-features        # Lint
cargo test --workspace             # 测试
cargo build --release              # 构建

# === Arch Linux ===
./scripts/test-arch-build.sh       # 本地测试 Arch 构建
makepkg -sf                        # 构建包
namcap PKGBUILD                    # 检查 PKGBUILD
updpkgsums                         # 更新校验和

# === 发布流程 ===
vim Cargo.toml                     # 更新版本号
git add Cargo.toml
git commit -m "chore: Bump version to 0.2.0"
git tag v0.2.0
git push origin main --tags        # 触发 Release 工作流
```

## 📦 发布清单

- [ ] 更新 `Cargo.toml` 版本号
- [ ] 本地测试：`cargo test && cargo build --release`
- [ ] 提交：`git commit -m "chore: Bump version"`
- [ ] 标签：`git tag v0.X.Y`
- [ ] 推送：`git push origin --tags`
- [ ] 等待 Actions 完成（~10-15 分钟）
- [ ] 检查 [Releases 页面](https://github.com/youaodu/Requiem/releases)

## 🔧 可选配置

### AUR 自动发布

需要 3 个 Secrets（Settings > Secrets > Actions）：
- `AUR_SSH_PRIVATE_KEY` - SSH 私钥
- `AUR_USERNAME` - AUR 用户名
- `AUR_EMAIL` - AUR 邮箱

### crates.io 自动发布

需要 1 个 Secret：
- `CARGO_REGISTRY_TOKEN` - crates.io API token

📖 详细步骤：[设置清单](SETUP_CHECKLIST.md)

## 🐛 故障排查

| 问题 | 解决方案 |
|------|---------|
| 格式检查失败 | `cargo fmt --all` |
| Clippy 警告 | 修复警告或在代码中添加 `#[allow(...)]` |
| 测试失败 | 本地运行 `cargo test` 调试 |
| Arch 构建失败 | `./scripts/test-arch-build.sh` 本地测试 |
| AUR 发布失败 | 检查 Secrets 配置，查看 Actions 日志 |

## 📚 完整文档

- [GitHub Actions 总览](GITHUB_ACTIONS_SUMMARY.md) - 完整的 CI/CD 文档
- [设置清单](SETUP_CHECKLIST.md) - 分步配置指南
- [AUR 发布](workflows/AUR_PUBLISHING.md) - AUR 发布详细指南
- [Arch Linux](ARCH_LINUX.md) - Arch Linux 用户指南

## 💡 提示

- CI 缓存生效后，构建速度提升 50%+
- Arch 包自动包含 locale 文件
- Release 自动生成 4 个平台的二进制
- 依赖更新每周自动检查

## 🔗 快速链接

- [Actions 日志](https://github.com/youaodu/Requiem/actions)
- [Releases](https://github.com/youaodu/Requiem/releases)
- [Issues](https://github.com/youaodu/Requiem/issues)
