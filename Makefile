.PHONY = lint test

lint:
	@cargo fmt
	@cargo check
	@cargo clippy

test:
	@cargo llvm-cov --html --output-dir="target/coverage"
