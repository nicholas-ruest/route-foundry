use rf_provider::ProviderCandidate;
use rf_workload::RoutingWorkload;
pub fn within_hard_limits(w: &RoutingWorkload, p: &ProviderCandidate) -> bool {
    p.healthy
        && p.cost_micros <= w.max_cost_micros
        && p.latency_ms <= w.max_latency_ms
        && p.region == w.required_region
}

#[cfg(test)]
mod tests {
    use super::*;
    use rf_types::{DataClass, WorkloadId};
    #[test]
    fn rejects_slow_provider() {
        let w = RoutingWorkload {
            workload_id: WorkloadId::new("abc").unwrap(),
            task: "summarize request".into(),
            data_class: DataClass::Internal,
            required_region: "ca".into(),
            max_cost_micros: 10,
            max_latency_ms: 50,
            min_quality: 0.5,
        };
        let p = ProviderCandidate {
            id: "slow".into(),
            region: "ca".into(),
            external: false,
            healthy: true,
            cost_micros: 9,
            latency_ms: 51,
            quality_samples: vec![0.8, 0.8, 0.8],
            pulls: 1,
        };
        assert!(!within_hard_limits(&w, &p));
    }
}
