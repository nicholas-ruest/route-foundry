use rf_errors::{FoundryError, Result};
use rf_rvf::RvfArchive;
use rf_types::{StoredRecord, WorkloadId};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
#[derive(Clone)]
pub struct Repository {
    path: PathBuf,
    lock: Arc<Mutex<()>>,
}
impl Repository {
    pub fn open(dir: &Path) -> Result<Self> {
        fs::create_dir_all(dir).map_err(|e| FoundryError::Persistence(e.to_string()))?;
        Ok(Self {
            path: dir.join("route-foundry.rvf"),
            lock: Arc::new(Mutex::new(())),
        })
    }
    pub fn append(&self, r: &StoredRecord) -> Result<String> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| FoundryError::Persistence("archive lock poisoned".into()))?;
        let b = serde_json::to_vec(r).map_err(|e| FoundryError::Persistence(e.to_string()))?;
        let id = self.records_unlocked()?.len() as u64 + 1;
        RvfArchive::append(&self.path, id, &b)?;
        Ok(hex::encode(Sha256::digest(b)))
    }
    fn records_unlocked(&self) -> Result<Vec<StoredRecord>> {
        RvfArchive::read_all(&self.path)?
            .into_iter()
            .map(|(_, b)| {
                serde_json::from_slice(&b).map_err(|e| FoundryError::Persistence(e.to_string()))
            })
            .collect()
    }
    pub fn records(&self) -> Result<Vec<StoredRecord>> {
        let _g = self
            .lock
            .lock()
            .map_err(|_| FoundryError::Persistence("archive lock poisoned".into()))?;
        self.records_unlocked()
    }
    pub fn decision(&self, id: &WorkloadId) -> Result<rf_types::RoutingResult> {
        self.records()?
            .into_iter()
            .rev()
            .find_map(|r| match r {
                StoredRecord::Decision { result } if &result.workload_id == id => Some(result),
                _ => None,
            })
            .ok_or_else(|| FoundryError::NotFound(id.to_string()))
    }
    pub fn decisions(&self) -> Result<Vec<rf_types::RoutingResult>> {
        Ok(self
            .records()?
            .into_iter()
            .filter_map(|r| {
                if let StoredRecord::Decision { result } = r {
                    Some(result)
                } else {
                    None
                }
            })
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let d = tempfile::tempdir().unwrap();
        assert!(Repository::open(d.path())
            .unwrap()
            .records()
            .unwrap()
            .is_empty());
    }
}
