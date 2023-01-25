export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

# Raspberry Pi

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"