# MICROKERN v1.0.0 Release Notes

发布日期：2026-02-18

## 已交付

- x86_64 裸机内核入口、COM1 串口日志、panic 处理和 HLT 停机循环。
- Buddy 页帧分配器，包含分配/释放与边界条件单元测试。
- Cargo workspace、`Makefile` 的 build/run/debug/test 目标。
- GitHub Actions CI：格式检查、clippy、内核单元测试。
- 设计文档、mdBook 与发布核验清单。
- `Release/` 下的源码归档、校验和与归档说明。

## 未交付 / 后续里程碑

- IPC、调度器、Capability 执行逻辑仍为设计骨架。
- ARM64、RISC-V 启动路径仍为预留模块。
- 用户态服务、POSIX 兼容层、网络栈与 ext4 仍为占位。

## 验证方式

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p microkern-kernel
make build ARCH=x86_64
make run ARCH=x86_64
```

## 使用限制

v1.0.0 仅用于架构验证、教学演示和二次开发。由于核心子系统尚未完整实现，不适合生产环境或安全关键场景。
