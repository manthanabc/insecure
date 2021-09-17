source:
	git archive --prefix=danger_dive_source/ -o danger_dive_source.zip HEAD

release:
	cargo make --profile release build-native
	rm -rf RELEASE
	mkdir -p RELEASE
	cp target/release/danger_dive RELEASE/
	cp -r assets RELEASE
	zip -r RELEASE.zip RELEASE

wasm:
	cargo make --profile release build-web
	rm -rf WASM
	mkdir -p WASM/target/wasm32-unknown-unknown/release
	cp index.html WASM/
	cp -r assets/ WASM/
	cp target/wasm.js target/wasm_bg.wasm WASM/target/
	cp target/wasm32-unknown-unknown/release/danger_dive.wasm  WASM/target/wasm32-unknown-unknown/release/
	zip -r WASM.zip WASM
