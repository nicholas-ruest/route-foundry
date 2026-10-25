use rf_errors::{FoundryError, Result};
#[derive(Clone, Debug)]
pub struct Principal {
    pub role: String,
}
pub fn authenticate(cookie: Option<&str>, mode: &str) -> Result<Principal> {
    if mode != "dev-session" {
        return Err(FoundryError::Unauthorized);
    }
    let role = cookie
        .and_then(|h| {
            h.split(';')
                .map(str::trim)
                .find_map(|p| p.strip_prefix("rf_session="))
        })
        .ok_or(FoundryError::Unauthorized)?;
    if !["steward", "analyst"].contains(&role) {
        return Err(FoundryError::Unauthorized);
    }
    Ok(Principal { role: role.into() })
}
pub fn require(principal: &Principal, role: &str) -> Result<()> {
    if principal.role == role {
        Ok(())
    } else {
        Err(FoundryError::Forbidden(format!("{role} role required")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dev_cookie_authenticates() {
        assert!(authenticate(Some("rf_session=steward"), "dev-session").is_ok());
        assert!(authenticate(None, "dev-session").is_err());
    }
}
