test:
	RUST_LOG=debug cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings