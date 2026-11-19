# MICROKERN · 微内核操作系统

> 安全从内核开始 — 像 seL4 一样安全，像 Linux 一样好用

MICROKERN 是一个 Rust `no_std` 微内核项目。Ring 0 仅保留 IPC、内存管理、调度、中断与 Capability 抽象；驱动、文件系统、网络栈和 POSIX 兼容层规划在用户态独立运行。

**当前发布版本：v1.0.0（M1 可运行基线）**

## 发布范围

v1.0.0 是一个可验证的基础版本，不是完整微内核操作系统：

- 已实现：Cargo workspace、x86_64 裸机入口、COM1 串口输出、Buddy 页帧分配器及单元测试、QEMU 脚本、CI。
- 已规划但未实现：IPC、多架构启动、进程调度、Capability 执行、用户态服务、POSIX 兼容层、ext4 和网络栈。
- 目标场景：架构验证、教学演示、二次开发起点。不要用于生产环境或安全关键场景。

详见 [RELEASE_NOTES.md](RELEASE_NOTES.md) 与 [docs/release-readiness.md](docs/release-readiness.md)。

## 快速开始

```bash
cargo test -p microkern-kernel
make build ARCH=x86_64
make run ARCH=x86_64
```

`make run` 需要 Rust target `x86_64-unknown-linux-gnu` 与 QEMU。串口输出示例：

```text
MICROKERN v1.0.0: boot ok
MICROKERN: first frame allocated
```

## 核心特性（设计目标）

| 特性 | 说明 |
|---|---|
| Rust 编写 | 编译器保证内存安全 |
| 最小内核 | 目标 < 20K 行，极小可信计算基 |
| 完全隔离 | 驱动/服务运行在用户态 |
| 实时调度 | 目标确定性延迟 < 10μs |
| 能力安全 | Capability-based 访问控制 |
| 多架构 | x86_64 / ARM64 / RISC-V |

## 目录

- `kernel/`：内核与架构/子系统骨架
- `services/`：用户态服务占位
- `lib/`：ABI、IPC、syscall、std 适配库骨架
- `apps/`：示例应用
- `tools/`：镜像与调试工具骨架
- `docs/`：设计文档、mdBook、发布核验清单
- `Release/`：源码归档与发布校验信息

## 作者与社区

作者：科技酱 · https://www.asoe.cn  
文档：https://docs.asoe.cn · GitHub：https://github.com/techjiang/  
B 站：https://space.bilibili.com/1768832152 · 邮箱：official@tech.asoe.cn  
玲珑：https://forums.asoe.cn/

QQ 群：291974598 / 474819022 / 631167288

同一个世界，同一个互联网！

## License

Apache 2.0 © 2026 科技酱
