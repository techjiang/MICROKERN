# Architecture

MICROKERN 将内核态限制在 IPC、内存、调度、中断和 Capability。所有复杂服务位于用户态并相互隔离。
