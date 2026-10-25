use regorus::{Engine, Value};
use rf_errors::{FoundryError, Result};
pub fn allow_route(
    policy: &str,
    class: &str,
    external: bool,
    required: &str,
    region: &str,
) -> Result<bool> {
    let mut e = Engine::new();
    e.add_policy("route.rego".into(), policy.into())
        .map_err(|x| FoundryError::Dependency(format!("Regorus policy: {x}")))?;
    let input = serde_json::json!({"data_class":class,"external":external,"required_region":required,"provider_region":region});
    e.set_input(
        Value::from_json_str(&input.to_string())
            .map_err(|x| FoundryError::Invalid(x.to_string()))?,
    );
    let v = e
        .eval_rule("data.route_foundry.route.allow".into())
        .map_err(|x| FoundryError::Dependency(format!("Regorus eval: {x}")))?;
    Ok(v == Value::Bool(true))
}
#[cfg(test)]
mod tests {
    use super::*;
    const P: &str = include_str!("../../../policies/route.rego");
    #[test]
    fn real_policy() {
        assert!(!allow_route(P, "restricted", true, "ca", "ca").unwrap());
        assert!(allow_route(P, "internal", false, "ca", "ca").unwrap());
    }
}
