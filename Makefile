format:
	@cargo fmt

lint:
	@cargo clippy

test:
	@cargo llvm-cov --ignore-filename-regex main
