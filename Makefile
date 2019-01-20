.PHONY: build
build:
	cargo fmt --all
	cargo +stable build

.PHONY: test
test:
	cargo +stable test --all-targets --all-features -p oak
	cargo +stable test --target wasm32-unknown-unknown --all-features -p oak

SOURCES := $(shell find crates -name "*.rs")

target/wasm32-unknown-unknown/release/%.wasm: \
	examples/%/Cargo.toml \
	examples/%/src/*.rs \
	examples/%/src/**/*.rs \
	$(SOURCES)
	cargo +stable build -vv --release --target=wasm32-unknown-unknown --package $*

target/wasm-bindgen:
	mkdir -p target/wasm-bindgen

target/wasm-bindgen/%_bg.wasm: \
	target/wasm32-unknown-unknown/release/%_gc_opt.wasm \
	target/wasm-bindgen
	wasm-bindgen \
		target/wasm32-unknown-unknown/release/$*.wasm \
		--out-dir target/wasm-bindgen

gh-pages:
	git worktree remove --force ./gh-pages || exit 0
	git worktree add ./gh-pages gh-pages

gh-pages/%: \
	gh-pages \
	target/wasm-bindgen/%_gc_opt_bg.wasm \
	examples/%/static/* \
	examples/%/static/**/*
	`yarn bin`/webpack --env.pkg $*

.PHONY: start
start:
	`yarn bin`/webpack-dev-server --env.watch

.SECONDARY: