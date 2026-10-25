use rf_errors::{FoundryError, Result};
use rf_provider::ProviderCandidate;
use rf_types::DataClass;
use rf_workload::RoutingWorkload;
const POLICY: &str = include_str!("../../../policies/route.rego");
pub fn authorize(w: &RoutingWorkload, p: &ProviderCandidate) -> Result<()> {
    let c = match w.data_class {
        DataClass::Public => "public",
        DataClass::Internal => "internal",
        DataClass::Confidential => "confidential",
        DataClass::Restricted => "restricted",
    };
    if rf_regorus::allow_route(POLICY, c, p.external, &w.required_region, &p.region)? {
        Ok(())
    } else {
        Err(FoundryError::Forbidden(format!(
            "policy denied provider {}",
            p.id
        )))
    }
}
