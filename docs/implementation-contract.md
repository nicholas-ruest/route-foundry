# Implementation contract

The repository contains 24 meaningful Rust crates. Counted crates expose typed APIs, have direct callers or an executable role, and own focused tests. `rf-api` composes the full path; `rf-cli` is only the process entrypoint. React and the generated client do not count toward the Rust floor.

The service fails closed for invalid identity, policy denial, no conformally feasible provider, corrupt RVF, and dependency errors. Denial occurs before a decision append. Every successful decision has a workload id, selected provider, conformal bound, score components, analogue list, synthetic-result label, correlation id, and content-addressed receipt. Cancellation is handled by Axum/Tokio shutdown; request timeout is enforced at the API layer.

Acceptance requires strict Clippy, locked tests, cargo-deny, frontend typecheck/lint/unit/build, OpenAPI drift check, middleware tests, executed upstream tests, benchmark/ablations, and Playwright against the real service. The exact source SHA is rechecked after evidence changes and after merge.
