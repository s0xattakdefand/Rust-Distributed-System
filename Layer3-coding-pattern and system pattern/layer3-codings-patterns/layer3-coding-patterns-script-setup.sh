###############################################################################
# 1) copy–paste everything into your terminal
# 2) chmod +x create-l3.sh && ./create-l3.sh     # default dir = ./l3-coding
###############################################################################
#!/usr/bin/env bash
set -euo pipefail

TARGET=${1:-l3-coding}
[ -e "$TARGET" ] && { echo "❌  $TARGET already exists"; exit 1; }

echo "📁  creating workspace $TARGET"
mkdir -p "$TARGET/patterns"
cd "$TARGET"

# ---------- root Cargo.toml --------------------------------------------------
cat > Cargo.toml <<'ROOT'
[workspace]
members  = ["patterns/*"]
resolver = "2"
ROOT

# ---------- the pattern list -------------------------------------------------
PATTERNS=(
  hexagonal
  clean
  plugin_kernel
  cqrs_event_sourcing
  mvc
  strangler_refactor
)

for p in "${PATTERNS[@]}"; do
  cargo new --lib "patterns/$p" --vcs none
done

echo "✅  crates created"
echo
echo "🔗  Now add their key libraries:"
echo
cat <<'LIBS'
# Hexagonal / Clean – usually just traits and domain code
# (No external deps needed yet)

# Plugin-kernel (dynamic loading)
cargo add -p plugin_kernel libloading@0.8      # or `wasmtime` for WASM plugins

# CQRS + Event Sourcing
cargo add -p cqrs_event_sourcing serde serde_json            # serialization
cargo add -p cqrs_event_sourcing anyhow                      # error bubbling
cargo add -p cqrs_event_sourcing event-store@0.16            # append-only log
# Optionally persistence driver: sqlx, mongodb, scylla-rs …

# MVC (desktop/web GUI demo)
cargo add -p mvc yew@0.21 --features csr           # or `egui` for desktop

# Strangler-Fig Refactor helper
cargo add -p strangler_refactor reqwest@0.12        # HTTP façade to legacy svc
cargo add -p strangler_refactor tower@0.4           # middleware split-points
LIBS

echo
echo "Run \`cargo clippy --workspace\` and start filling in code! 🚀"
