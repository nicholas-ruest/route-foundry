# INTEGRATION_MATRIX

| Ingredient | Revision/version/license | Capability and boundary | Owner | Test | Ablation |
|---|---|---|---|---|---|
| ruvnet/ruvector | crate `ruvector-core =2.4.0`, Apache-2.0/MIT | in-process VectorDB insert/search of route vectors | rf-ruvector / Intelligence | executed insert/search + primary | disable analogue bonus |
| ruvnet/ruvector RVF | crate `rvf-runtime =0.3.2`, Apache-2.0/MIT | append/reopen/read canonical JSON records | rf-rvf / Evidence | restart persistence | volatile mode loses restart score |
| microsoft/regorus | crate `regorus =0.13.0`, Apache-2.0 | evaluate `data.route_foundry.route.allow` before effects | rf-regorus / Policy | real rule allow+deny | policy-off produces forbidden route |
| split conformal prediction | Angelopoulos & Bates 2021 | finite-sample lower quality safe set | rf-calibration / Routing | deterministic order-statistic tests | mean-only admits unsafe candidate |
| optimistic contextual routing | Abbasi-Yadkori et al. 2011 mechanism | exploration bonus within feasible set | rf-bandit / Routing | score tests + benchmark | greedy-only reduces utility |
