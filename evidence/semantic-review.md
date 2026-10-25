# Semantic evidence review

Candidate source SHA: `CI_REBIND_REQUIRED`.

The repository was read back after compilation and browser execution. Cargo metadata reports 24 workspace members. All 24 contain implemented Rust behavior; none is an empty re-export crate. The application call path is `rf-cli -> rf-api -> rf-auth/rf-middleware -> rf-orchestrator -> rf-workload/rf-provider/rf-policy/rf-calibration/rf-budget/rf-bandit/rf-score/rf-recall -> rf-regorus/rf-ruvector -> rf-repository/rf-audit/rf-rvf`. `rf-seed` and `rf-evaluation` have useful executable callers in the CLI and CI.

| Crates | Observed implementation | Caller/test evidence | Result |
|---|---|---|---|
| rf-types, rf-errors, rf-config | value objects, explicit errors, validated runtime config | workspace callers; unit and startup paths | PASS |
| rf-provider, rf-workload | candidate/workload aggregates and invariants | orchestrator; focused tests | PASS |
| rf-calibration, rf-bandit, rf-budget, rf-score | conformal order statistic, optimism, hard limits, utility | orchestrator; monotonic/boundary tests and benchmark ablations | PASS |
| rf-embedding, rf-ruvector, rf-recall | deterministic vector, real VectorDB insert/search, analogue bonus | full-stack integration plus executed upstream test | PASS |
| rf-regorus, rf-policy | real Engine policy load/eval and typed authorization | allow/deny test, Rust no-effect test, browser recovery journey | PASS |
| rf-audit, rf-rvf, rf-repository | hash record, real RVF append/reopen/read, typed lookup | archive tests and restart retrieval | PASS |
| rf-auth, rf-middleware | dev-session principal, role/idempotency/correlation validation | focused tests and every browser request | PASS |
| rf-orchestrator, rf-api, rf-cli | ordered use case, versioned HTTP service, process lifecycle | integration test, health, Playwright, restart | PASS |
| rf-seed, rf-evaluation | deterministic realistic corpus and five measured modes | CLI benchmark and tests | PASS |

Small algorithm crates are narrow by deliberate decision, not arbitrary subdivisions: each owns a distinct invariant, has a real orchestrator caller, and is independently ablated or tested. No copied Purpose Lattice domain types, covenant APIs, or purpose-governance workflows remain in the Rust call path. Grep/readback found no stale `pl-` workspace package names. The React controls call the real API through Vite proxy; Playwright has no route interception.

Checks observed on this candidate: structural pass (24/25/12), strict Clippy pass, locked workspace tests pass, cargo-deny pass after explicitly allowing OSI-approved MIT-0 required by Regorus, frontend typecheck/lint/unit/build/contract pass, three browser tests pass, and restart retrieval pass. CI replaces the source marker with the exact `github.sha`, verifies both semantic review and manifest bindings, executes the complete verifier plus clean container build, and uploads the resolved evidence.
