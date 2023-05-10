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
	OUT_DIR=../vscode-extension/autocompletion-engine just -f autocompletion-engine/justfile build

napi-watch:
	OUT_DIR=../vscode-extension/autocompletion-engine just -f autocompletion-engine/justfile watch

napi-test:
	just -f autocompletion-engine/justfile test

napi-lint:
	just -f autocompletion-engine/justfile lint

napi-fmt:
	just -f autocompletion-engine/justfile fmt

napi-clean:
	just -f autocompletion-engine/justfile clean

# JS
extension-build:
	just -f vscode-extension/justfile build

extension-watch:
	just -f vscode-extension/justfile watch

extension-package:
	just -f vscode-extension/justfile package

extension-test:
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
