.PHONY: install build test clean dev release

# Install maturin if not present
install-maturin:
	@which maturin > /dev/null || pip install maturin

# Development build (debug)
dev: install-maturin
	maturin develop

# Release build
build: install-maturin
	maturin build --release

# Build wheel for distribution
wheel: install-maturin
	maturin build --release --interpreter python3.8 python3.9 python3.10 python3.11 python3.12

# Test the bindings
test: dev
	python test_example.py

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/
	rm -rf dist/
	rm -rf python/sentrystr.egg-info/

# Install in development mode
install-dev: dev
	pip install -e .

# Build and publish to PyPI (requires authentication)
publish: wheel
	maturin publish

# Format code
fmt:
	cargo fmt

# Check code
check:
	cargo check

# Clippy linting
clippy:
	cargo clippy

# All checks
lint: fmt check clippy

help:
	@echo "Available commands:"
	@echo "  dev          - Build in development mode"
	@echo "  build        - Build in release mode"
	@echo "  wheel        - Build wheels for multiple Python versions"
	@echo "  test         - Run test script"
	@echo "  clean        - Clean build artifacts"
	@echo "  install-dev  - Install in development mode"
	@echo "  publish      - Publish to PyPI"
	@echo "  fmt          - Format Rust code"
	@echo "  check        - Check Rust code"
	@echo "  clippy       - Run clippy linter"
	@echo "  lint         - Run all linting checks"