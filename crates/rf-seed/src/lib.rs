use rf_orchestrator::RouteCommand;
use rf_provider::ProviderCandidate;
use rf_types::{DataClass, WorkloadId};
use rf_workload::RoutingWorkload;
pub fn seed_request(id: &str) -> RouteCommand {
    RouteCommand {
        workload: RoutingWorkload {
            workload_id: WorkloadId::new(id).expect("seed id"),
            task: "Summarize the synthetic support escalation and propose next actions".into(),
            data_class: DataClass::Internal,
            required_region: "ca".into(),
            max_cost_micros: 80,
            max_latency_ms: 500,
            min_quality: 0.76,
        },
        candidates: vec![
            ProviderCandidate {
                id: "edge-saffron".into(),
                region: "ca".into(),
                external: false,
                healthy: true,
                cost_micros: 18,
                latency_ms: 120,
                quality_samples: vec![0.78, 0.82, 0.84, 0.81, 0.86],
                pulls: 8,
            },
            ProviderCandidate {
                id: "cloud-cobalt".into(),
                region: "ca".into(),
                external: true,
                healthy: true,
                cost_micros: 34,
                latency_ms: 180,
                quality_samples: vec![0.88, 0.9, 0.91, 0.89, 0.92],
                pulls: 15,
            },
            ProviderCandidate {
                id: "cloud-violet".into(),
                region: "us".into(),
                external: true,
                healthy: true,
                cost_micros: 12,
                latency_ms: 90,
                quality_samples: vec![0.8, 0.83, 0.82, 0.84],
                pulls: 3,
            },
        ],
    }
}
