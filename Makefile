ARCH ?= x86_64
TARGET := target/$(ARCH)-microkern/release/kernel
RUSTFLAGS := -C link-arg=-nostartfiles -C link-arg=-no-pie

.PHONY: build run debug test clean

build:
	cargo build --release --bin kernel --target-dir target/$(ARCH)-microkern --target x86_64-unknown-linux-gnu

run: build
	qemu-system-x86_64 -m 512M -smp 2 -kernel $(TARGET) -serial stdio -display none -no-reboot

debug: build
	qemu-system-x86_64 -m 512M -smp 2 -kernel $(TARGET) -serial stdio -display none -no-reboot -s -S

test:
	cargo test -p microkern-kernel

clean:
	cargo clean
