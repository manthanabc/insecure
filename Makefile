source:
	git archive --prefix=danger_dive_source/ -o danger_dive_source.zip HEAD

release:
	cargo build --release
	rm -rf RELEASE
	mkdir -p RELEASE
	cp target/release/danger_dive RELEASE/
	cp -r assets RELEASE
