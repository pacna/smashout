## ----------------------------------------------------------------------
## The purpose of this Makefile is to simplify common development tasks.
## ----------------------------------------------------------------------
##

.PHONY:run
run: ## run the app
##
	cargo run

.PHONY:watch
watch: ## run the app in watch mode
##
	cargo watch -x "run"

.PHONY:deploy
deploy: ## create wasm and deploy to GH page
##
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./web/ --target web ./target/wasm32-unknown-unknown/release/smashout.wasm

	bash deploy.sh

.PHONY:help
help: ## Show this help
##
	@sed -ne '/@sed/!s/##//p' $(MAKEFILE_LIST)