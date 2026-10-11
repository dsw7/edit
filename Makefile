.PHONY = lint test install

lint:
	@cargo fmt
	@cargo check
	@cargo clippy

test:
	@cargo llvm-cov --html --output-dir="target/coverage"

install:
	@cargo install --path . --root ~/.local
