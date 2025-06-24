mkdir layer2-system-pattern && cd layer2-system-pattern
cat > Cargo.toml <<'ROOT'
[workspace]
members  = ["patterns/*"]
resolver = "2"
ROOT

# list exactly matches the table order
PATTERNS=(
  api_gateway bff service_registry sidecar circuit_breaker retry_backoff bulkhead
  rate_limiter token_propagation policy_gatekeeper cache_aside write_through
  cqrs_split event_sourcing saga idempotent_receiver message_broker pub_sub
  distributed_tracing metrics blue_green canary iac auto_scaler
)

for p in "${PATTERNS[@]}"; do
  cargo new --lib "patterns/$p" --vcs none
done

echo "✅ workspace ready: cd infra-patterns && cargo check --workspace"

### and run this command
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test   --workspace

