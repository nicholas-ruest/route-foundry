use rf_workload::RoutingWorkload;
use sha2::{Digest, Sha256};
pub const DIMENSION: usize = 32;
pub fn embed(w: &RoutingWorkload) -> Vec<f32> {
    Sha256::digest(format!(
        "{}|{}|{}|{}",
        w.task, w.required_region, w.max_cost_micros, w.max_latency_ms
    ))
    .iter()
    .map(|b| f32::from(*b) / 255.0)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_types::{DataClass, WorkloadId};
    #[test]
    fn stable_dimension() {
        let w = RoutingWorkload {
            workload_id: WorkloadId::new("abc").unwrap(),
            task: "summarize request".into(),
            data_class: DataClass::Internal,
            required_region: "ca".into(),
            max_cost_micros: 10,
            max_latency_ms: 50,
            min_quality: 0.5,
        };
        assert_eq!(embed(&w), embed(&w));
        assert_eq!(embed(&w).len(), DIMENSION);
    }
}
