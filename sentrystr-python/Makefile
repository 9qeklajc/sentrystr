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
lint: fmt check clippy py-lint

# Python development setup
py-install:
	pip install -e ".[dev]"
	pre-commit install

# Python linting
py-lint:
	ruff check python/ tests/

# Python formatting
py-format:
	ruff format python/ tests/

# Python fix linting issues
py-fix:
	ruff check --fix python/ tests/
	ruff format python/ tests/

# Python type checking
py-check:
	mypy python/ tests/ --ignore-missing-imports

# Python tests
py-test:
	pytest tests/

# All Python checks
py-all: py-lint py-check

# Clean Python cache
py-clean:
	rm -rf .ruff_cache/
	rm -rf .mypy_cache/
	rm -rf __pycache__/
	find . -name "*.pyc" -delete
	find . -name "__pycache__" -type d -exec rm -rf {} +

# Run pre-commit on all files
pre-commit:
	pre-commit run --all-files

help:
	@echo "Available commands:"
	@echo "Rust commands:"
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
	@echo "  lint         - Run all linting checks (Rust + Python)"
	@echo ""
	@echo "Python commands:"
	@echo "  py-install   - Install Python dev dependencies"
	@echo "  py-lint      - Run Python linting with ruff"
	@echo "  py-format    - Format Python code with ruff"
	@echo "  py-fix       - Fix Python linting issues and format"
	@echo "  py-check     - Run Python type checking with mypy"
	@echo "  py-test      - Run Python tests with pytest"
	@echo "  py-all       - Run all Python checks (lint + type)"
	@echo "  py-clean     - Clean Python cache files"
	@echo "  pre-commit   - Run pre-commit hooks on all files"