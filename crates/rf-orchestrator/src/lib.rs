use rf_audit::audit;
use rf_bandit::optimistic_score;
use rf_budget::within_hard_limits;
use rf_calibration::conformal_lower;
use rf_errors::{FoundryError, Result};
use rf_provider::ProviderCandidate;
use rf_repository::Repository;
use rf_ruvector::RouteIndex;
use rf_score::utility;
use rf_types::{CandidateAssessment, RoutingResult, StoredRecord, WorkloadId};
use rf_workload::RoutingWorkload;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteCommand {
    pub workload: RoutingWorkload,
    pub candidates: Vec<ProviderCandidate>,
}
#[derive(Clone)]
pub struct FoundryService {
    repository: Repository,
    index: Arc<Mutex<RouteIndex>>,
}
impl FoundryService {
    pub fn new(repository: Repository) -> Result<Self> {
        let mut idx = RouteIndex::new()?;
        for d in repository.decisions()? {
            let w = RoutingWorkload {
                workload_id: d.workload_id.clone(),
                task: d.synthetic_output.clone(),
                data_class: rf_types::DataClass::Internal,
                required_region: "ca".into(),
                max_cost_micros: d.estimated_cost_micros.max(1),
                max_latency_ms: d.estimated_latency_ms.max(1),
                min_quality: d.conformal_lower,
            };
            idx.insert(
                d.workload_id.as_str(),
                rf_embedding::embed(&w),
                &d.selected_provider,
            )?
        }
        Ok(Self {
            repository,
            index: Arc::new(Mutex::new(idx)),
        })
    }
    pub fn route(&self, cmd: RouteCommand, correlation: &str) -> Result<RoutingResult> {
        cmd.workload.validate()?;
        if cmd.candidates.is_empty() {
            return Err(FoundryError::Invalid(
                "at least one provider required".into(),
            ));
        }
        let vector = rf_embedding::embed(&cmd.workload);
        let analogues = self
            .index
            .lock()
            .map_err(|_| FoundryError::Dependency("index lock poisoned".into()))?
            .search(&vector, 5)?;
        let total = cmd.candidates.iter().map(|p| p.pulls).sum::<u64>().max(1);
        let mut assessments = Vec::new();
        let mut best: Option<(ProviderCandidate, f64, f64)> = None;
        let mut denied = 0;
        for p in &cmd.candidates {
            p.validate()?;
            let mut reasons = Vec::new();
            if let Err(e) = rf_policy::authorize(&cmd.workload, p) {
                denied += 1;
                reasons.push(e.to_string());
                assessments.push(CandidateAssessment {
                    provider_id: p.id.clone(),
                    conformal_lower: 0.0,
                    score: -1.0,
                    eligible: false,
                    reasons,
                });
                continue;
            }
            if !within_hard_limits(&cmd.workload, p) {
                reasons.push("hard cost, latency, health, or residency limit".into());
                assessments.push(CandidateAssessment {
                    provider_id: p.id.clone(),
                    conformal_lower: 0.0,
                    score: -1.0,
                    eligible: false,
                    reasons,
                });
                continue;
            }
            let lower = conformal_lower(&p.quality_samples, 0.2)?;
            if lower < cmd.workload.min_quality {
                reasons.push(format!("conformal lower {lower:0.3} below minimum"));
                assessments.push(CandidateAssessment {
                    provider_id: p.id.clone(),
                    conformal_lower: lower,
                    score: -1.0,
                    eligible: false,
                    reasons,
                });
                continue;
            }
            let bonus = rf_recall::analogue_bonus(&analogues, &p.id);
            let score = optimistic_score(
                utility(lower, p.cost_micros, p.latency_ms),
                p.pulls,
                total,
                bonus,
            );
            reasons.push(format!("eligible lower={lower:0.3} analogue={bonus:0.3}"));
            assessments.push(CandidateAssessment {
                provider_id: p.id.clone(),
                conformal_lower: lower,
                score,
                eligible: true,
                reasons,
            });
            if best.as_ref().map(|x| score > x.1).unwrap_or(true) {
                best = Some((p.clone(), score, lower))
            }
        }
        let (selected, score, lower) = best.ok_or_else(|| {
            if denied == cmd.candidates.len() {
                FoundryError::Forbidden("all providers denied by policy".into())
            } else {
                FoundryError::Invalid("no conformally feasible provider".into())
            }
        })?;
        let output = format!(
            "SYNTHETIC fixture via {}: {}",
            selected.id,
            cmd.workload.task.chars().take(80).collect::<String>()
        );
        let receipt = hex::encode(Sha256::digest(format!(
            "{}|{}|{score:0.8}|{correlation}",
            cmd.workload.workload_id, selected.id
        )));
        let result = RoutingResult {
            workload_id: cmd.workload.workload_id.clone(),
            selected_provider: selected.id.clone(),
            conformal_lower: lower,
            utility_score: score,
            estimated_cost_micros: selected.cost_micros,
            estimated_latency_ms: selected.latency_ms,
            synthetic_output: output,
            synthetic: true,
            reasons: vec![
                "policy allowed before execution".into(),
                "split-conformal feasible set applied".into(),
                "RuVector analogue recall executed".into(),
            ],
            assessments,
            analogues: analogues.clone(),
            receipt,
            correlation_id: correlation.into(),
        };
        self.repository.append(&StoredRecord::Decision {
            result: result.clone(),
        })?;
        let record = audit(
            cmd.workload.workload_id.clone(),
            "route.selected",
            correlation,
            "",
            &serde_json::to_vec(&result).map_err(|e| FoundryError::Persistence(e.to_string()))?,
        )?;
        self.repository.append(&record)?;
        self.index
            .lock()
            .map_err(|_| FoundryError::Dependency("index lock poisoned".into()))?
            .insert(cmd.workload.workload_id.as_str(), vector, &selected.id)?;
        Ok(result)
    }
    pub fn get(&self, id: &WorkloadId) -> Result<RoutingResult> {
        self.repository.decision(id)
    }
    pub fn audit(&self) -> Result<Vec<StoredRecord>> {
        self.repository.records()
    }
    pub fn decision_count(&self) -> Result<usize> {
        Ok(self.repository.decisions()?.len())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rf_types::DataClass;
    fn command(c: DataClass, e: bool) -> RouteCommand {
        RouteCommand {
            workload: RoutingWorkload {
                workload_id: WorkloadId::new("case-1").unwrap(),
                task: "summarize a synthetic support request".into(),
                data_class: c,
                required_region: "ca".into(),
                max_cost_micros: 100,
                max_latency_ms: 500,
                min_quality: 0.7,
            },
            candidates: vec![ProviderCandidate {
                id: "provider-a".into(),
                region: "ca".into(),
                external: e,
                healthy: true,
                cost_micros: 20,
                latency_ms: 100,
                quality_samples: vec![0.8, 0.82, 0.84, 0.86],
                pulls: 3,
            }],
        }
    }
    #[test]
    fn denial_has_no_effect() {
        let d = tempfile::tempdir().unwrap();
        let s = FoundryService::new(Repository::open(d.path()).unwrap()).unwrap();
        assert!(s
            .route(command(DataClass::Restricted, true), "deny")
            .is_err());
        assert_eq!(s.decision_count().unwrap(), 0);
        assert!(s.route(command(DataClass::Internal, false), "ok").is_ok());
    }
}
