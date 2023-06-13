export CARGO_FLAGS := env_var_or_default("CARGO_FLAGS", "--dev")

default:
  @just --list

build: extension-clean napi-build extension-build

watch: napi-watch extension-watch

test: napi-test extension-test

lint: napi-lint extension-lint

fmt: napi-fmt extension-fmt nix-fmt

clean: napi-clean extension-clean

# NAPI (autocompletion engine)
napi-build:
  #! /usr/bin/env bash
  set -eu
  pnpm build
  OUT_DIR=./vscode-extension/autocompletion-engine bash -c '(mkdir "$OUT_DIR" 2>&- >/dev/null || true) && mv index.js index.d.ts *.node "$OUT_DIR"'

napi-build-debug:
  pnpm build:debug

napi-watch:
  #! /usr/bin/env bash
  set -eu
  pnpm watch
  OUT_DIR=./vscode-extension/autocompletion-engine bash -c '(mkdir "$OUT_DIR" 2>&- >/dev/null || true) && mv index.js index.d.ts *.node "$OUT_DIR"'

napi-test: napi-build-debug
  cargo test
  pnpm test

napi-lint:
  cargo clippy --all-features --all-targets -- -D warnings

napi-fmt:
  cargo fmt
  pnpm fmt

napi-clean:
  cargo clean
  rm -rf dist index.js index.d.ts

test-watch: napi-build-debug
  pnpm test:watch

artifacts:
  pnpm artifacts

universal:
  pnpm universal

version:
  pnpm version

# JS
extension-build:
  just -f vscode-extension/justfile build

extension-watch:
  just -f vscode-extension/justfile watch

extension-package:
  just -f vscode-extension/justfile package

extension-test: napi-build-debug
  just -f vscode-extension/justfile test

extension-lint:
  just -f vscode-extension/justfile lint

extension-fmt:
  just -f vscode-extension/justfile fmt

extension-clean:
  just -f vscode-extension/justfile clean

# Nix
nix-fmt:
  nix fmt
