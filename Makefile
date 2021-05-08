.PHONY: help
.DEFAULT_GOAL := help

REGISTRY ?= kenec
IMAGE    ?= pipe
TAG      ?= 0.1.0

help:
	@echo "---------------------------------------------------------------------------------------"
	@echo ""
	@echo "                                     P I P E "
	@echo ""
	@echo "---------------------------------------------------------------------------------------"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development

bench: ## Run internal benchmarks
	@cargo bench --all

build: ## Build the project
	@cargo build

check: check-code check-fmt

check-code: ## Checks code for compilation errors
	@cargo check --all --all-features --all-targets

check-fmt: ## Checks code formatting correctness
	@cargo fmt -- --check

fmt: ## Format code
	@cargo fmt

run: ## Starts Pipe in development mode
	@cargo run

test: ## Spins up Docker resources and runs _every_ test
	@docker-compose up

docker-build: ## Build docker image of Pipe
	@docker build --tag $(REGISTRY)/$(IMAGE):$(TAG) .
