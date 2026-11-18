# MICROKERN · 微内核操作系统

> 安全从内核开始 — 像 seL4 一样安全，像 Linux 一样好用

MICROKERN 是一个 Rust `no_std` 微内核项目。内核只保留 IPC、内存管理、调度、中断与 Capability；驱动、文件系统和网络栈运行在用户态。

## 当前状态

M1 起步骨架已落地：

- Cargo workspace 与三架构模块目录
- x86_64 `no_std` 内核入口与串口输出
- Buddy 页帧分配器与单元测试
- QEMU 运行脚本与基础 CI

完整设计见 [docs/design/kernel-design.md](docs/design/kernel-design.md)。

## 快速开始

```bash
cargo test -p microkern-kernel
make build ARCH=x86_64
make run ARCH=x86_64
```

`make run` 需要本机安装 `x86_64-unknown-linux-gnu` Rust target 与 QEMU。

## 核心特性

| 特性 | 说明 |
|---|---|
| Rust 编写 | 编译器保证内存安全 |
| 最小内核 | 目标 < 20K 行，极小可信计算基 |
| 完全隔离 | 驱动/服务运行在用户态 |
| 实时调度 | 目标确定性延迟 < 10μs |
| 能力安全 | Capability-based 访问控制 |
| 多架构 | x86_64 / ARM64 / RISC-V |

## 社区

- 主页：https://www.asoe.cn
- 技术文档：https://docs.asoe.cn
- GitHub：https://github.com/techjiang/
- 玲珑论坛：https://forums.asoe.cn/

## License

Apache 2.0 © 2026 科技酱
