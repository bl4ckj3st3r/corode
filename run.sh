#!/usr/bin/env bash
set -euo pipefail

# Clean and build
cargo clean
cargo build --release --target riscv64gc-unknown-none-elf

# Test in QEMU
qemu-system-riscv64 \
  -machine virt \
  -nographic \
  -bios none \
  -kernel target/riscv64gc-unknown-none-elf/release/trickster-core
