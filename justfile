build: napi-build js-build

test: napi-test js-test

lint: napi-lint js-lint

fmt: napi-fmt js-fmt

clean: napi-clean js-clean

# NAPI (autocompletion engine)
napi-build:
	$(MAKE) -C autocompletion-engine build OUT_DIR=../vscode-extension/autocompletion-engine

napi-test:
	$(MAKE) -C autocompletion-engine test

napi-lint:
	$(MAKE) -C autocompletion-engine lint

napi-fmt:
	$(MAKE) -C autocompletion-engine fmt

napi-clean:
	$(MAKE) -C autocompletion-engine clean

# JS
js-build:
	$(MAKE) -C vscode-extension build

js-test:
	$(MAKE) -C vscode-extension test

js-lint:
	$(MAKE) -C vscode-extension lint

js-fmt:
	$(MAKE) -C vscode-extension fmt

js-clean:
	$(MAKE) -C vscode-extension clean
