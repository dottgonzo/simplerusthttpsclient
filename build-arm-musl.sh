export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc
export CC=aarch64-linux-gnu-gcc
cargo build --release --target=aarch64-unknown-linux-musl