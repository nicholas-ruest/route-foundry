use rf_errors::{FoundryError, Result};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ProviderCandidate {
    pub id: String,
    pub region: String,
    pub external: bool,
    pub healthy: bool,
    pub cost_micros: u64,
    pub latency_ms: u32,
    pub quality_samples: Vec<f64>,
    pub pulls: u64,
}
impl ProviderCandidate {
    pub fn validate(&self) -> Result<()> {
        if self.id.len() < 3 || self.region.len() < 2 {
            return Err(FoundryError::Invalid(
                "provider identity/region invalid".into(),
            ));
        }
        if self.quality_samples.len() < 3
            || self
                .quality_samples
                .iter()
                .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
        {
            return Err(FoundryError::Invalid(
                "provider needs at least three finite quality samples".into(),
            ));
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_uncalibrated() {
        let p = ProviderCandidate {
            id: "edge".into(),
            region: "ca".into(),
            external: false,
            healthy: true,
            cost_micros: 1,
            latency_ms: 1,
            quality_samples: vec![0.9],
            pulls: 1,
        };
        assert!(p.validate().is_err());
    }
}
