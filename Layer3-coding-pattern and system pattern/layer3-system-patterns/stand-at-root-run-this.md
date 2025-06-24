cargo clean                                # optional
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test   --workspace
