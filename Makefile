.PHONY: test rust-test rust-build fmt clippy lint wasm-build wasm-test js-test \
	lint-js lint-rust fmt-rust fmt-js \
	clean clean-rust clean-js clean-wasm-api

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
	$(MAKE) -C polar-wasm-api build

wasm-test:
	$(MAKE) -C polar-wasm-api test

js-test:
	$(MAKE) -C languages/js parity
	$(MAKE) -C languages/js test

clean: clean-docs clean-rust clean-js clean-wasm-api

clean-rust:
	cargo clean

clean-js:
	$(MAKE) -C languages/js clean

clean-wasm-api:
	$(MAKE) -C wasm-api clean

