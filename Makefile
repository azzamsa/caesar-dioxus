help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	rustup target add wasm32-unknown-unknown

	trunk --version | cargo install trunk
	microserver --version | cargo install microserver
	git-cliff --version | cargo install git-cliff

clean: ## Clean build artifacts.
	trunk clean

dev: ## Develop the app.
	pnpx tailwindcss --config tailwind.config.js --input styles/main.scss  --output styles/compiled.scss
	trunk serve

build: ## Build the app.
	NODE_ENV=production pnpx tailwindcss --config tailwind.config.js --input styles/main.scss --output styles/compiled.scss
	trunk build --release

preview-build: ## Preview the build output.
	microserver dist/

fmt: ## Format the codebase.
	cargo fmt
	prettier --config prettier.config.js --ignore-path .prettierignore --write .

fmt_check: ## Check is the codebase properly formatted.
	cargo fmt --check
	prettier --config prettier.config.js --ignore-path .prettierignore --check .

lint: ## Lint the codebase.
	cargo clippy

test: ## Lint the codebase.
	cargo test

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.

check:  fmt_check lint test ## Check if the repository comply with the rules. Mostly used in CI.


release:  ## Create a release
	bash scripts/release.sh $(version)

##
## Misc

update_dependencies: ## Check outdated dependencies.
	cargo update
	cargo outdated --root-deps-only
