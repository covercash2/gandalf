config := "test"

run_tunnel:
	cargo run --bin gandalf-tunnel

run_local_tunnel:
	cargo run --bin gandalf-tunnel -- --config ./tunnel/config/tunnel.toml

run_proxy:
	cargo run --bin gandalf -- --config {{config}}

run_local_proxy:
	cargo run --bin gandalf -- --config "test"

run_foundry_proxy:
	cargo run --bin gandalf -- --config foundry

test_all:
	cargo fmt --check
	cargo clippy
	cargo test --all
