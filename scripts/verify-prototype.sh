#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.."&&pwd);cd "$ROOT";trap "./scripts/stop-prototype.sh" EXIT
./scripts/check-workspace.sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo deny check
(cd frontend&&npm ci&&npm run typecheck&&npm run lint&&npm test&&npm run build&&npm run contract)
cargo run --locked -p rf-cli -- benchmark | tee evidence/validation/benchmark.json
./scripts/start-prototype.sh
(cd frontend&&npx playwright test)
./scripts/restart-check.sh
echo VERIFY_PROTOTYPE_PASS
