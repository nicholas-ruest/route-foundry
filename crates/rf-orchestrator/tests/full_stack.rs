use rf_orchestrator::{FoundryService, RouteCommand};
use rf_provider::ProviderCandidate;
use rf_repository::Repository;
use rf_types::{DataClass, WorkloadId};
use rf_workload::RoutingWorkload;
#[test]
fn real_ruvector_regorus_rvf_path() {
    let d = tempfile::tempdir().unwrap();
    let service = FoundryService::new(Repository::open(d.path()).unwrap()).unwrap();
    let cmd = RouteCommand {
        workload: RoutingWorkload {
            workload_id: WorkloadId::new("integration-route").unwrap(),
            task: "summarize synthetic support escalation".into(),
            data_class: DataClass::Internal,
            required_region: "ca".into(),
            max_cost_micros: 80,
            max_latency_ms: 500,
            min_quality: 0.7,
        },
        candidates: vec![ProviderCandidate {
            id: "edge-saffron".into(),
            region: "ca".into(),
            external: false,
            healthy: true,
            cost_micros: 18,
            latency_ms: 120,
            quality_samples: vec![0.78, 0.82, 0.84, 0.81],
            pulls: 8,
        }],
    };
    let result = service.route(cmd, "integration-correlation").unwrap();
    assert!(result.synthetic);
    assert!(service.get(&result.workload_id).is_ok());
    assert!(service.audit().unwrap().len() >= 2);
}
