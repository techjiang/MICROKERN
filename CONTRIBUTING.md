# Contributing

1. 从 Issue 领取任务，一个 PR 只解决一件事。
2. 提交前运行 `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets` 与 `cargo test -p microkern-kernel`。
3. PR 描述说明现状问题、方案依据、改动点和测试结果。
4. 内核代码默认 `no_std`，引入依赖前确认不破坏裸机构建。
