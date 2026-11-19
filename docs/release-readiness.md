# Release readiness checklist

本清单用于确认当前版本“能构建、能说明、能追溯、能发布”。

## 构建与测试

- [x] Workspace 可描述所有 crate。
- [x] 内核提供 `no_std` 入口与 panic handler。
- [x] Buddy 分配器带单元测试。
- [x] `Makefile` 提供 build/run/debug/test。
- [x] CI 覆盖 fmt、clippy、单元测试。

## 发布资料

- [x] README 明确当前版本与实现边界。
- [x] RELEASE_NOTES 记录交付、限制和验证命令。
- [x] LICENSE 使用 Apache 2.0。
- [x] Release 目录包含源码归档与 SHA256 校验和。
- [x] GitHub Release 工作流在 tag 推送时生成 Release。

## 上线结论

可作为 M1 开发基线发布；不可作为完整操作系统生产版本发布。每个未实现的子系统必须在后续里程碑完成后重新评估。
