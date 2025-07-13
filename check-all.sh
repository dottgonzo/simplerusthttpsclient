echo "check for arch aarch64-unknown-linux-gnu" && cargo check --release --target=aarch64-unknown-linux-gnu
echo "check for arch aarch64-unknown-linux-musl" && cargo check --release --target=aarch64-unknown-linux-musl
echo "check for arch x86_64-unknown-linux-gnu" && cargo check --release --target=x86_64-unknown-linux-gnu
echo "check for arch x86_64-unknown-linux-musl" && cargo check --release --target=x86_64-unknown-linux-musl
echo "check for arch x86_64-pc-windows-gnu" && cargo check --release --target=x86_64-pc-windows-gnu
# echo "check for arch x86_64-android-linux-gnu" && cargo check --release --target=x86_64-android-linux-gnu
echo "check for arch wasm32-wasip1-threads" && cargo check --release --target=wasm32-wasip1-threads --no-default-features --features=tls
# echo "check for arch wasm32-wasip2" && cargo check --release --target=wasm32-wasip2
# echo "check for arch wasm32-wasi" && cargo check --release --target=wasm32-wasi
echo "check for arch wasm32-unknown-unknown" && cargo check --release --target=wasm32-unknown-unknown --no-default-features --features=tls