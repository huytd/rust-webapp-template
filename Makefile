all: clean asmjs copy

clean:
	rm -rf target/asmjs-unknown-emscripten/debug/wrust.* out

asmjs:
	cargo build --target=asmjs-unknown-emscripten

copy:
	mkdir out
	cp -rf www/* out
	cp target/asmjs-unknown-emscripten/debug/wrust.js out/main.js
