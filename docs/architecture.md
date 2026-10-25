# Architecture and CRATE_MAP

The browser talks only to the versioned Axum API. Middleware establishes principal, correlation, idempotency, limits, and typed errors. The orchestrator owns ordering: validate → authorize → conformal feasible set → RuVector recall → optimistic selection → synthetic adapter → RVF decision/audit append. RVF is the sole durable store; RuVector is the intelligence index rebuilt from durable decisions.

| Crate | Context / typed API | Caller and evidence |
|---|---|---|
| rf-types | shared value objects / `RoutingResult` | all contexts; serde tests |
| rf-errors | error contract / `FoundryError` | API/adapters; mapping tests |
| rf-config | runtime / `Config::from_env` | CLI/API; validation test |
| rf-provider | assurance / `ProviderCandidate::validate` | orchestrator; invariant tests |
| rf-workload | intake / `RoutingWorkload::validate` | orchestrator/API; boundary tests |
| rf-calibration | safe routing / `conformal_lower` | orchestrator; coverage fixtures |
| rf-bandit | safe routing / `optimistic_score` | orchestrator; exploration tests |
| rf-budget | assurance / `within_hard_limits` | orchestrator; limit tests |
| rf-score | safe routing / `utility` | bandit/evaluation; monotonic tests |
| rf-embedding | intelligence / `embed` | RuVector path; determinism test |
| rf-ruvector | intelligence / `RouteIndex` | orchestrator; real insert/search test |
| rf-recall | intelligence / `analogue_bonus` | orchestrator; boundedness test |
| rf-regorus | policy adapter / `allow_route` | rf-policy; real Rego test |
| rf-policy | policy / `authorize` | orchestrator; denial tests |
| rf-audit | evidence / `audit` | orchestrator; chain test |
| rf-rvf | persistence / `RvfArchive` | repository; append/reopen test |
| rf-repository | persistence / `Repository` | orchestrator/API; retrieval test |
| rf-auth | access / `authenticate` | API; role tests |
| rf-middleware | delivery / idempotency/correlation | API; validation tests |
| rf-orchestrator | application / `FoundryService` | API/CLI; full path tests |
| rf-api | delivery / `router` | CLI/browser; HTTP tests |
| rf-seed | fixtures / `seed_request` | CLI/tests; deterministic test |
| rf-evaluation | evaluation / `run_benchmark` | CLI/CI; ablations |
| rf-cli | executable / serve, seed, benchmark | operator/CI; smoke test |

Every row links to ADR-0001–0025 and the owning context documents. No crate is a re-export shell.
