###############################################################################
#  create-l3-system.sh
#  -------------------
#  USAGE
#    ./create-l3-system.sh            # creates ./l3-system
#    ./create-l3-system.sh my-folder  # creates ./my-folder instead
###############################################################################
#!/usr/bin/env bash
set -euo pipefail

TARGET=${1:-l3-system}
[ -e "$TARGET" ] && { echo "❌  $TARGET already exists"; exit 1; }

echo "📁  creating workspace $TARGET"
mkdir -p "$TARGET/patterns"
cd "$TARGET"

# ── root Cargo.toml ──────────────────────────────────────────────────────────
cat > Cargo.toml <<'ROOT'
[workspace]
members  = ["patterns/*"]
resolver = "2"
ROOT

# ── pattern list ─────────────────────────────────────────────────────────────
PATTERNS=(
  micro_services_mesh
  event_driven_arch
  data_mesh
  saga_workflow
  api_composition
  strangler_system
  mesh_canary
  data_pipeline
)

for p in "${PATTERNS[@]}"; do
  cargo new --lib "patterns/$p" --vcs none
done

echo "✅  crates created"
echo
echo "🔗  Suggested helper libraries (add only where needed):"
cat <<'LIBS'
# micro_services_mesh   -> kube, k8s-openapi, linkerd2-proxy-api
# event_driven_arch     -> nats, async-nats, serde, anyhow
# data_mesh             -> prost, tonic, deltalake, polars
# saga_workflow         -> anyhow, tokio, our layer2 'saga' crate
# api_composition       -> actix-web, reqwest, serde_json
# strangler_system      -> tower, reqwest, envoy-control-plane
# mesh_canary           -> argo-rollouts-rs or kube + custom metrics
# data_pipeline         -> iceberg-rust, arrow, parquet, tokio
LIBS

echo
echo "Run  'cargo clippy --workspace'  and start implementing 🚀"
