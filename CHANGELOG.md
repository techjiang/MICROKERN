# Changelog

All notable changes to MICROKERN are documented here.

## v1.0.0

- Stabilized the M1 bootable skeleton: x86_64 bare-metal entry, COM1 serial output, and Buddy frame allocator with unit tests.
- Added release metadata, GitHub release workflow, and a reproducible source archive layout under `Release/`.
- Clarified architecture, build/run instructions, licensing, and current implementation status across README and docs.
- Added a release readiness checklist so the shipped scope is explicit and verifiable.

## 0.1.0

- Initialized the MICROKERN workspace and kernel skeleton.
- Added x86_64 serial output and Buddy frame allocator.
- Added QEMU build/run scripts and baseline CI.
