.PHONY: all
all: check lint

.PHONY: ci
ci: check lint test

.PHONY: check
check:
	cargo check

.PHONY: lint
lint:
	cargo clippy
	cargo fmt -- --check

.PHONY: test
test:
	cargo test
