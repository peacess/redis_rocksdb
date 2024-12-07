
.PHONY: clean upgrade format check build build-all rebuild release test build_test

clean:
	cargo clean
upgrade:
	cargo upgrade
	cargo update
format:
	cargo +nightly fmt
check:
	cargo check
build:
	cargo build

rebuild: clean build

release:
	cargo build --release
test:
	cargo test
build_test:
	cargo test --no-run
