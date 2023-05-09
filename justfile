build: napi-build js-build

test: napi-test js-test

lint: napi-lint js-lint

fmt: napi-fmt js-fmt nix-fmt

clean: napi-clean js-clean

# NAPI (autocompletion engine)
napi-build:
	OUT_DIR=../vscode-extension/autocompletion-engine just -f autocompletion-engine/justfile build

napi-test:
	just -f autocompletion-engine/justfile test

napi-lint:
	just -f autocompletion-engine/justfile lint

napi-fmt:
	just -f autocompletion-engine/justfile fmt

napi-clean:
	just -f autocompletion-engine/justfile clean

# JS
js-build:
	just -f vscode-extension/justfile build

js-test:
	just -f vscode-extension/justfile test

js-lint:
	just -f vscode-extension/justfile lint

js-fmt:
	just -f vscode-extension/justfile fmt

js-clean:
	just -f vscode-extension/justfile clean

# Nix
nix-fmt:
	nix fmt
