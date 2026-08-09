.PHONY = test

test:
	@cargo llvm-cov --html --output-dir="target/coverage"
