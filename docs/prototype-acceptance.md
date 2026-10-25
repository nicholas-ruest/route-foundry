# Prototype acceptance

| Journey | UI → API → middleware → crates → upstream/persistence → evidence |
|---|---|
| Primary route | `/route`, Route workload → `POST /api/v1/routes` → dev-session + idempotency + Regorus → workload/provider/calibration/bandit/orchestrator → RuVector search+insert and RVF append → selected-provider card, correlation trace, screenshot |
| Denial and recovery | Restricted + external-only request → same endpoint → Regorus deny before execution/append → visible 403 and unchanged audit count; switch to internal provider and resubmit → success → Playwright denial assertion and audit diff |
| Restart retrieval | Save route, refresh, stop/start backend, open `/audit` and route id → `GET` endpoints → auth/correlation → repository → RVF reopen/read → identical receipt rendered → restart script and browser trace |

First-party browser routes are never intercepted. Synthetic provider responses are explicitly labelled; the governing integrations and persistence are real.
