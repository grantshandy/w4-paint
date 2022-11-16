all:
	cargo build --release
	wasm-opt target/wasm32-unknown-unknown/release/wasm_fishey.wasm -Oz -o target/wasm32-unknown-unknown/release/wasm_fishey.wasm
	
run: all
	w4 run --no-open --no-qr target/wasm32-unknown-unknown/release/wasm_fishey.wasm

dev:
	cargo watch -s "make run"