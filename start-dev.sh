#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$ROOT/tmp" "$ROOT/logs"

echo "Starting backend (dev)..."
(
  cd "$ROOT/backend"
  # dev backend (uses dotenv in backend/src/main.rs)
  RUST_LOG=info cargo run > "$ROOT/logs/backend.log" 2>&1 &
  echo $! > "$ROOT/tmp/backend.pid"
)

echo "Starting frontend (dev)..."
(
  cd "$ROOT/frontend"
  # make sure dependencies are installed locally first (npm install)
  # ensure PUBLIC_BACKEND_URL is correct if you want to override
  npm run dev > "$ROOT/logs/frontend.log" 2>&1 &
  echo $! > "$ROOT/tmp/frontend.pid"
)

echo "Started:"
echo "  backend pid: $(cat "$ROOT/tmp/backend.pid") (logs: $ROOT/logs/backend.log)"
echo "  frontend pid: $(cat "$ROOT/tmp/frontend.pid") (logs: $ROOT/logs/frontend.log)"
echo "Stop with: $ROOT/stop.sh"