# Default target
.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help message
	@echo "Available commands:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  make %-10s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY: doc
doc: ## Generate documentation and open in the browser
	cd lr_formatter_rs && cargo doc --no-deps --open

.PHONY: format
format: ## Format project files
	cd lr_formatter_rs && cargo fmt --all

.PHONY: lint
lint: ## Check best code practices
	cd lr_formatter_rs && cargo clippy --all-targets --all-features -- \
	  -A clippy::all \
	  -D clippy::correctness \
	  -D clippy::suspicious \
	  -D clippy::perf \
	  -W clippy::complexity \
	  -W clippy::style

.PHONY: test
test: ## Run unit and system tests
	cd lr_formatter_rs && cargo test --all

.PHONY: precommit
precommit: format lint test ## Run format, lint, and test
