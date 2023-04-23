.PHONY: test rust-test rust-build fmt clippy lint wasm-build wasm-test js-test \
	lint-js lint-rust fmt-rust fmt-js \
	clean clean-rust clean-js

test:  rust-test wasm-test js-test

rust-test:
	cargo test --all-targets --all-features

rust-build:
	cargo build

fmt: fmt-rust fmt-js

fmt-rust:
	cargo fmt

fmt-js:
	$(MAKE) -C languages/js fmt

clippy:
	cargo clippy --all-features --all-targets -- -D warnings

lint-js:
	$(MAKE) -C languages/js lint

lint-rust:
	$(MAKE) -C languages/rust lint

lint: clippy lint-js lint-rust
	$(MAKE) fmt

wasm-build:
	$(MAKE) -C autocompletion-engine build OUT_DIR=../vscode-extension/autocompletion-engine

wasm-test:
	$(MAKE) -C autocompletion-engine test

js-build:
	$(MAKE) -C vscode-extension build

js-test:
	echo "TODO: js-test"

clean: clean-rust clean-js

clean-rust:
	cargo clean

clean-js:
	$(MAKE) -C vscode-extension clean
