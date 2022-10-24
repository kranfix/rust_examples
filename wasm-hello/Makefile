build:
	cargo build --target wasm32-wasi --examples

wasmer_hello_there:
	wasmer run target/wasm32-wasi/debug/examples/hello_there.wasm

wasmtime_hello_there:
	wasmtime target/wasm32-wasi/debug/examples/hello_there.wasm

pkg:
	wasm-pack build

test:
	#wasm-pack test --firefox
	#wasm-pack test --chrome
	wasm-pack test --headless --chrome