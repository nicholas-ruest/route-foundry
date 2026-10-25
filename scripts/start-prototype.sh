#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.."&&pwd);cd "$ROOT";mkdir -p .run .route-foundry-data
cargo run --locked -p rf-cli -- --data-dir .route-foundry-data serve --addr 127.0.0.1:18080 >.run/backend.log 2>&1 & echo $! >.run/backend.pid
(cd frontend; npm run dev -- --host 127.0.0.1 >../.run/frontend.log 2>&1 & echo $! >../.run/frontend.pid)
for _ in $(seq 1 90);do curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1&&curl -fsS http://127.0.0.1:14273 >/dev/null 2>&1&&{ echo "READY http://127.0.0.1:14273";exit 0;};sleep 1;done
exit 1
