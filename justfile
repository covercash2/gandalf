
run_tunnel:
	cargo run --bin gandalf-tunnel

run_proxy:
	cargo run --bin gandalf

test_all:
	cargo fmt --check
	cargo clippy
	cargo test --all
