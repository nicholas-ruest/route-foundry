use rf_errors::{FoundryError, Result};
pub fn validate_idempotency(v: Option<&str>) -> Result<&str> {
    let v = v.ok_or_else(|| FoundryError::Invalid("idempotency-key required".into()))?;
    if v.len() < 8 || v.len() > 128 {
        return Err(FoundryError::Invalid("invalid idempotency-key".into()));
    }
    Ok(v)
}
pub fn validate_correlation(v: Option<&str>) -> Result<String> {
    let v = v.unwrap_or("generated");
    if v.len() > 128 {
        return Err(FoundryError::Invalid("correlation id too long".into()));
    }
    Ok(v.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn idempotency_is_required() {
        assert!(validate_idempotency(None).is_err());
        assert!(validate_idempotency(Some("route-001")).is_ok());
    }
}
