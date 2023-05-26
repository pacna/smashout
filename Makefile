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

.PHONY:help
help: ## Show this help
##
	@sed -ne '/@sed/!s/##//p' $(MAKEFILE_LIST)