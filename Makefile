build:
	cargo build

run:
	cargo run

release:
	cargo build --release

release-cross-linux:
	cargo build --release --target=x86_64-unknown-linux-musl

fmt:
	cargo fmt
