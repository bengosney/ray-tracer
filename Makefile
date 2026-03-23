.PHONY: build deploy clean help

WASM_DIR := wasm-lib
WASM_PKG := $(WASM_DIR)/pkg
NODE_MODULES := node_modules

WASM_SENTINEL := $(WASM_PKG)/wasm_lib.js
JS_SENTINEL := $(NODE_MODULES)/.package-lock.json
PRE_COMMIT_SENTINEL := .git/hooks/pre-commit

NPM := npm
WASM_PACK := wasm-pack
CARGO := cargo

$(WASM_SENTINEL): $(WASM_DIR)/Cargo.toml $(wildcard $(WASM_DIR)/src/*.rs)
	cd $(WASM_DIR) && $(WASM_PACK) build --target web --out-dir pkg

$(JS_SENTINEL): package.json $(WASM_SENTINEL)
	$(NPM) install
	@touch $@

$(PRE_COMMIT_SENTINEL): .pre-commit-config.yaml
	prek install
	@touch $@

build: $(JS_SENTINEL) ## Build the production React app
	$(NPM) run build

deploy: $(JS_SENTINEL) ## Build and deploy to GitHub Pages
	$(NPM) run predeploy
	$(NPM) run deploy

clean: ## Remove build artifacts
	rm -rf build $(NODE_MODULES) $(WASM_PKG) $(WASM_DIR)/target

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'
