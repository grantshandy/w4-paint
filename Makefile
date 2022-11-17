all:
	cargo build --release
	cp target/wasm32-unknown-unknown/release/w4_paint.wasm .
	wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code w4_paint.wasm -o w4_paint.wasm
	
	# enable wasm-opt for a tiny reduction but it won't run in the native runtime
	# wasm-opt -Oz --strip-producers --dce w4_paint.wasm -o w4_paint.wasm

run: all
	w4 run-native w4_paint.wasm

dev:
	cargo watch -s "make run"