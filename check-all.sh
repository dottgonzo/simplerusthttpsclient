cargo check --release --target=aarch64-unknown-linux-gnu
cargo check --release --target=aarch64-unknown-linux-musl
cargo check --release --target=x86_64-unknown-linux-gnu
cargo check --release --target=x86_64-unknown-linux-musl
cargo check --release --target=x86_64-pc-windows-gnu
cargo check --release --target=wasm32-wasi-preview1-threads --no-default-features