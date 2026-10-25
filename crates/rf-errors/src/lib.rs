use serde::Serialize;
#[derive(Debug, thiserror::Error)]
pub enum FoundryError {
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("policy denied: {0}")]
    PolicyDenied(String),
    #[error("authentication failed")]
    Unauthorized,
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("dependency failed: {0}")]
    Dependency(String),
    #[error("persistence failed: {0}")]
    Persistence(String),
}
pub type Result<T> = std::result::Result<T, FoundryError>;
#[derive(Debug, Serialize)]
pub struct Problem {
    pub kind: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub correlation_id: String,
}
impl FoundryError {
    pub fn problem(&self, id: String) -> Problem {
        let status = match self {
            Self::Invalid(_) => 400,
            Self::Unauthorized => 401,
            Self::Forbidden(_) | Self::PolicyDenied(_) => 403,
            Self::NotFound(_) => 404,
            Self::Conflict(_) => 409,
            Self::Dependency(_) => 503,
            Self::Persistence(_) => 500,
        };
        Problem {
            kind: format!("urn:route-foundry:{status}"),
            title: self.to_string(),
            status,
            detail: self.to_string(),
            correlation_id: id,
        }
    }
}
