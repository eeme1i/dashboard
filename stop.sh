#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
PID_DIR="$ROOT/tmp"

if [ ! -d "$PID_DIR" ]; then
  echo "No pid dir ($PID_DIR). Nothing to stop."
  exit 0
fi

stopped=false
for f in "$PID_DIR"/*.pid; do
  [ -e "$f" ] || continue
  pid="$(cat "$f" 2>/dev/null || true)"
  if [ -n "$pid" ]; then
    echo "Stopping pid $pid from $f..."
    if kill "$pid" 2>/dev/null; then
      echo "  killed $pid"
      rm -f "$f"
      stopped=true
    else
      echo "  failed to kill $pid (may already be dead)"
      rm -f "$f"
    fi
  else
    rm -f "$f"
  fi
done

if [ "$stopped" = false ]; then
  echo "No running pids found."
fi