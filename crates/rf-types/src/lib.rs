use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkloadId(String);
impl WorkloadId {
    pub fn new(v: impl Into<String>) -> Result<Self, String> {
        let v = v.into();
        if !(3..=64).contains(&v.len())
            || !v
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            Err("workload id must be 3-64 safe characters".into())
        } else {
            Ok(Self(v))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DataClass {
    Public,
    Internal,
    Confidential,
    Restricted,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analogue {
    pub workload_id: String,
    pub distance: f64,
    pub decision: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CandidateAssessment {
    pub provider_id: String,
    pub conformal_lower: f64,
    pub score: f64,
    pub eligible: bool,
    pub reasons: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RoutingResult {
    pub workload_id: WorkloadId,
    pub selected_provider: String,
    pub conformal_lower: f64,
    pub utility_score: f64,
    pub estimated_cost_micros: u64,
    pub estimated_latency_ms: u32,
    pub synthetic_output: String,
    pub synthetic: bool,
    pub reasons: Vec<String>,
    pub assessments: Vec<CandidateAssessment>,
    pub analogues: Vec<Analogue>,
    pub receipt: String,
    pub correlation_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StoredRecord {
    Decision {
        result: RoutingResult,
    },
    Audit {
        workload_id: WorkloadId,
        event: String,
        correlation_id: String,
        previous_hash: String,
        payload_hash: String,
        recorded_at: String,
        receipt: String,
    },
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_identity() {
        assert!(WorkloadId::new("route-42").is_ok());
        assert!(WorkloadId::new("..").is_err());
    }
}
