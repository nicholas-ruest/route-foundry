use rf_errors::{FoundryError, Result};
use std::{net::SocketAddr, path::PathBuf};
#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    pub bind: SocketAddr,
    pub data_dir: PathBuf,
    pub namespace: String,
    pub auth_mode: String,
}
impl RuntimeConfig {
    pub fn from_env() -> Result<Self> {
        let bind = std::env::var("RF_BIND")
            .unwrap_or_else(|_| "127.0.0.1:18180".into())
            .parse()
            .map_err(|e| FoundryError::Invalid(format!("RF_BIND: {e}")))?;
        let data_dir =
            PathBuf::from(std::env::var("RF_DATA_DIR").unwrap_or_else(|_| "data/prototype".into()));
        let namespace =
            std::env::var("RF_NAMESPACE").unwrap_or_else(|_| "route-foundry-prototype".into());
        let auth_mode = std::env::var("RF_AUTH_MODE").unwrap_or_else(|_| "dev-session".into());
        Ok(Self {
            bind,
            data_dir,
            namespace,
            auth_mode,
        })
    }
    pub fn namespace_dir(&self) -> PathBuf {
        self.data_dir.join(&self.namespace)
    }
}
