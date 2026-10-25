#!/usr/bin/env bash
set -euo pipefail
test -s docs/specification.md
test -s docs/architecture.md
test -s docs/research.md
test -s docs/implementation-contract.md
test -s docs/traceability.md
test -s docs/core-memory-guidance-map.md
test -s docs/prototype-acceptance.md
test -s docs/integration-matrix.md
adr_count=$(find docs/adrs -maxdepth 1 -name '[0-9][0-9][0-9][0-9].md' | wc -l)
ddd_count=$(find docs/ddd -maxdepth 1 -name '[0-9][0-9].md' | wc -l)
crate_rows=$(grep -c '^| rf-' docs/architecture.md)
test "$adr_count" -eq 25
test "$ddd_count" -eq 12
test "$crate_rows" -ge 20
grep -q 'Ruvnet RuVector' evidence/selection/2026-10-24.md
grep -q 'Microsoft Regorus' evidence/selection/2026-10-24.md
grep -q 'split-conformal' evidence/selection/2026-10-24.md
printf 'STRUCTURAL_DESIGN_PASS crates=%s adrs=%s ddds=%s\n' "$crate_rows" "$adr_count" "$ddd_count"
