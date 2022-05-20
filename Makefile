run_debug:
	RUST_LOG=debug cargo run

run:
	./target/release/dlauncher

run_debug_log:
	RUST_LOG=debug ./target/release/dlauncher

build:
	cargo build --release

docs:
	cargo doc --no-deps