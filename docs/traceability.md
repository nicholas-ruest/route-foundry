# Traceability

| Requirement | Design | Implementation | Verification |
|---|---|---|---|
| permitted per-request routing | ADR-0005, DDD safe-routing | rf-policy, rf-calibration, rf-bandit, rf-orchestrator | primary and denial journeys |
| analogue intelligence | ADR-0008 | rf-embedding, rf-ruvector, rf-recall | executed insert/search + ablation |
| durable replay | ADR-0009 | rf-rvf, rf-repository, rf-audit | restart journey |
| enterprise policy | ADR-0007 | rf-regorus | policy integration and no-effect assertion |
| usable product | ADR-0016–0019 | rf-api, rf-cli, frontend | Playwright desktop/mobile |
| source-bound evidence | ADR-0022–0025 | scripts, manifest, semantic review | exact-SHA readback |
