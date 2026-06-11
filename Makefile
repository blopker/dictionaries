.PHONY: help all crawl build generate patch validate readme test lint

help: ## Show this help
	@grep -E '^[a-z]+:.*##' $(MAKEFILE_LIST) | awk -F ':.*## ' '{printf "  %-10s %s\n", $$1, $$2}'

all: ## Full pipeline: crawl, build, generate, readme, validate
	cargo run --release -- all

crawl: ## Download and extract upstream sources (archive/, source/)
	cargo run --release -- crawl

build: ## Run make/configure for sources that build their hunspell files
	cargo run --release -- build

generate: ## Decode, normalize, patch, and write dictionaries/
	cargo run --release -- generate

validate: ## Parse every dictionary with spellbook (offline)
	cargo run --release -- validate

readme: ## Regenerate the tables in readme.md
	cargo run --release -- readme

test: ## Run unit tests
	cargo test

lint: ## Check formatting and clippy
	cargo fmt --check
	cargo clippy -- -D warnings
