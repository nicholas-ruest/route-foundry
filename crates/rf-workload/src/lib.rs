use rf_errors::{FoundryError, Result};
use rf_types::{DataClass, WorkloadId};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RoutingWorkload {
    pub workload_id: WorkloadId,
    pub task: String,
    pub data_class: DataClass,
    pub required_region: String,
    pub max_cost_micros: u64,
    pub max_latency_ms: u32,
    pub min_quality: f64,
}
impl RoutingWorkload {
    pub fn validate(&self) -> Result<()> {
        if self.task.trim().len() < 8 || self.task.len() > 2000 {
            return Err(FoundryError::Invalid(
                "task must be 8-2000 characters".into(),
            ));
        }
        if self.required_region.len() < 2
            || self.max_cost_micros == 0
            || self.max_latency_ms == 0
            || !self.min_quality.is_finite()
            || !(0.0..=1.0).contains(&self.min_quality)
        {
            return Err(FoundryError::Invalid("routing constraints invalid".into()));
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_bounds() {
        let w = RoutingWorkload {
            workload_id: WorkloadId::new("abc").unwrap(),
            task: "summarize the case".into(),
            data_class: DataClass::Internal,
            required_region: "ca".into(),
            max_cost_micros: 20,
            max_latency_ms: 400,
            min_quality: 0.8,
        };
        assert!(w.validate().is_ok());
    }
}
