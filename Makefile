# Common tasks for tpt-gov-nz. On Windows, run these from Git Bash or install `make`.
# `new-dept` is intentionally absent: the generator in scripts/ rewrites existing
# files. Read the generator and CONTRIBUTING.md before running it.

.PHONY: help install doctor dev demo stack stack-down seed reset typecheck lint test test-rust test-ts build

help:
	@echo "make install     Install JavaScript dependencies (pnpm)"
	@echo "make doctor      Check toolchain, .env and configuration"
	@echo "make dev         Run all portals in development mode"
	@echo "make demo        Run the one-command citizen demo (Docker)"
	@echo "make stack       Start the full Phase 1 stack (Docker, detached)"
	@echo "make stack-down  Stop the full Phase 1 stack"
	@echo "make seed        Start the stack and wait for health; dev seeds run as migrations"
	@echo "make reset       Delete Phase 1 database volumes and start fresh (CONFIRM=yes required)"
	@echo "make typecheck   Type-check all TypeScript packages"
	@echo "make lint        Lint all TypeScript packages"
	@echo "make test        Run TypeScript and Rust unit tests"
	@echo "make build       Build all TypeScript packages"

install:
	pnpm install

doctor:
	pnpm doctor

dev:
	pnpm dev

demo:
	docker compose -f docker/demo.yml up

stack:
	docker compose -f docker/phase1.yml up -d

stack-down:
	docker compose -f docker/phase1.yml down

# Each service applies its migrations, including the dev seed, on startup.
seed:
	docker compose -f docker/phase1.yml up -d --wait

# Destructive: removes every Phase 1 database volume.
reset:
	@test "$(CONFIRM)" = "yes" || (echo "This deletes all Phase 1 database volumes. Run: make reset CONFIRM=yes" && exit 1)
	docker compose -f docker/phase1.yml down -v
	docker compose -f docker/phase1.yml up -d --wait

typecheck:
	pnpm typecheck

lint:
	pnpm lint

build:
	pnpm build

test: test-ts test-rust

test-ts:
	pnpm test

# Unit tests only. Integration tests need a database (see TODO.md, "Postgres service container").
test-rust:
	cargo test --workspace --lib
