# Requiem 调试指南

## 快速开始

### 基础调试命令

```bash
# 1. 运行开发版本（最常用）
cargo run

# 2. 带详细日志运行
RUST_LOG=debug cargo run

# 3. 带完整堆栈跟踪
RUST_BACKTRACE=1 cargo run

# 4. 组合使用
RUST_LOG=trace RUST_BACKTRACE=full cargo run

# 5. 使用调试脚本
./debug.sh run        # 标准调试模式
./debug.sh trace      # 完整追踪模式
```

## 日志级别

控制日志输出的详细程度：

```bash
# ERROR 级别（只显示错误）
RUST_LOG=error cargo run

# WARN 级别（警告和错误）
RUST_LOG=warn cargo run

# INFO 级别（一般信息）
RUST_LOG=info cargo run

# DEBUG 级别（调试信息）
RUST_LOG=debug cargo run

# TRACE 级别（所有信息）
RUST_LOG=trace cargo run

# 针对特定模块
RUST_LOG=requiem=debug,reqwest=info cargo run
```

## IDE 调试

### VS Code

1. 安装扩展：
   - `rust-analyzer`
   - `CodeLLDB`

2. 按 F5 启动调试

3. 设置断点：在代码左侧行号处点击

### 命令行 GDB/LLDB

```bash
# 使用 rust-gdb
cargo build
rust-gdb ./target/debug/requiem

# 或使用 rust-lldb
rust-lldb ./target/debug/requiem

# GDB 常用命令：
# (gdb) run              - 运行程序
# (gdb) break main.rs:42 - 在第42行设置断点
# (gdb) continue         - 继续执行
# (gdb) step             - 单步进入
# (gdb) next             - 单步跳过
# (gdb) print variable   - 打印变量
# (gdb) backtrace        - 查看调用栈
```

## 性能分析

### 内存使用分析

```bash
# 使用 heaptrack
./debug.sh mem

# 或手动
cargo build
heaptrack ./target/debug/requiem
heaptrack_gui heaptrack.requiem.*.gz
```

### CPU 性能分析

```bash
# 使用 perf
./debug.sh perf

# 或手动
cargo build --release
perf record -g ./target/release/requiem
perf report
```

### 使用 cargo-flamegraph

```bash
# 安装
cargo install flamegraph

# 生成火焰图
cargo flamegraph

# 查看 flamegraph.svg
```

## 常见问题调试

### 1. 编译错误

```bash
# 详细检查
cargo check --all-targets

# 清理重新编译
cargo clean && cargo build

# 更新依赖
cargo update
```

### 2. 运行时崩溃

```bash
# 启用完整堆栈跟踪
RUST_BACKTRACE=full cargo run

# 使用 GDB 调试
rust-gdb ./target/debug/requiem
(gdb) run
# 程序崩溃后
(gdb) backtrace
```

### 3. UI 渲染问题

```bash
# 检查图形驱动
RUST_LOG=iced=debug cargo run

# 使用软件渲染（如果硬件加速有问题）
ICED_BACKEND=tiny-skia cargo run
```

### 4. HTTP 请求问题

```bash
# 查看 reqwest 日志
RUST_LOG=reqwest=trace cargo run

# 使用 wireshark 抓包
sudo tcpdump -i any -w requiem.pcap port 443
```

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 显示测试输出
cargo test -- --nocapture

# 运行基准测试
cargo bench
```

## 代码质量检查

```bash
# Clippy 检查（代码质量）
cargo clippy

# 格式化检查
cargo fmt -- --check

# 自动格式化
cargo fmt

# 依赖审计
cargo audit
```

## 环境变量参考

| 变量 | 说明 | 示例 |
|------|------|------|
| `RUST_LOG` | 日志级别 | `debug`, `info`, `trace` |
| `RUST_BACKTRACE` | 堆栈跟踪 | `1`, `full` |
| `ICED_BACKEND` | 渲染后端 | `wgpu`, `tiny-skia` |
| `CARGO_PROFILE_RELEASE_DEBUG` | Release 调试符号 | `true` |

## 实用工具

```bash
# 安装调试工具
sudo pacman -S gdb lldb heaptrack perf

# Rust 开发工具
cargo install cargo-watch    # 自动重新编译
cargo install cargo-expand   # 查看宏展开
cargo install cargo-tree     # 查看依赖树
cargo install cargo-audit    # 安全审计
```

## 自动重新编译

```bash
# 代码改变时自动重新运行
cargo watch -x run

# 代码改变时自动测试
cargo watch -x test

# 自定义命令
cargo watch -x 'run --release'
```

## 调试技巧

1. **添加 println! 调试**（最简单）
   ```rust
   println!("Debug: variable = {:?}", variable);
   ```

2. **使用 dbg! 宏**
   ```rust
   let result = dbg!(some_function());
   ```

3. **条件编译**
   ```rust
   #[cfg(debug_assertions)]
   println!("This only runs in debug builds");
   ```

4. **自定义 Debug 输出**
   ```rust
   #[derive(Debug)]
   struct MyStruct {
       field: String,
   }
   ```
