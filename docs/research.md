# Research and hypothesis

RuVector Core 2.4.0 provides the executed vector index used to recall structurally similar workloads and prior chosen routes. RVF Runtime 0.3.2 provides the append/reopen/read evidence envelope. Microsoft Regorus 0.13.0 executes Rego in-process and is the enterprise policy authority; application code cannot reinterpret a denial.

The frontier mechanism is split-conformal safe-set routing. For each candidate, calibration residuals are ordered and a conservative lower quality statistic is selected at `ceil((n+1)*(1-alpha))`. Only candidates whose lower bound meets the workload threshold enter the action set. Within that set, a bounded optimism score combines quality, normalized cost/latency, exploration, and an analogue bonus. This differs from ordinary model routers that optimize an average prediction without finite-sample coverage framing.

Primary sources: `ruvnet/ruvector` release 2.4.0 and RVF Runtime 0.3.2 source; `microsoft/regorus` 0.13.0 (Apache-2.0); Angelopoulos & Bates, *A Gentle Introduction to Conformal Prediction and Distribution-Free Uncertainty Quantification* (2021); Abbasi-Yadkori et al., *Improved Algorithms for Linear Stochastic Bandits* (NeurIPS 2011). The hypothesis is that policy + conformal feasibility + recalled analogues yields higher feasible utility and zero forbidden external routes on the deterministic corpus.
