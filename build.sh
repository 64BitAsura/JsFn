# shellcheck shell=sh
cargo build --target wasm32-wasi --release && cp ./target/wasm32-wasi/release/JsFn.wasm js-sandbox.wasm