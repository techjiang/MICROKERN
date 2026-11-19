# Getting started

安装 Rust stable 与 QEMU 后：

```bash
cargo test -p microkern-kernel
make build ARCH=x86_64
make run ARCH=x86_64
```

`make run` 通过串口输出启动日志。v1.0.0 是可运行基线，IPC/调度/多架构/用户态服务仍在规划或占位中。发布范围见 `RELEASE_NOTES.md`。
