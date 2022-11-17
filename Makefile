all:
	cargo build --release
	wasm-opt target/wasm32-unknown-unknown/release/w4_paint.wasm -Oz -o target/wasm32-unknown-unknown/release/w4_paint.wasm
	
run: all
	w4 run --no-open --no-qr target/wasm32-unknown-unknown/release/w4_paint.wasm

dev:
	cargo watch -s "make run"