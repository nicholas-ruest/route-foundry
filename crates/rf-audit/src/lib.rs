use chrono::Utc;
use rf_errors::Result;
use rf_types::{StoredRecord, WorkloadId};
use sha2::{Digest, Sha256};
pub fn audit(
    id: WorkloadId,
    event: &str,
    correlation: &str,
    previous: &str,
    payload: &[u8],
) -> Result<StoredRecord> {
    let payload_hash = hex::encode(Sha256::digest(payload));
    let receipt = hex::encode(Sha256::digest(format!(
        "{id}|{event}|{correlation}|{previous}|{payload_hash}"
    )));
    Ok(StoredRecord::Audit {
        workload_id: id,
        event: event.into(),
        correlation_id: correlation.into(),
        previous_hash: previous.into(),
        payload_hash,
        recorded_at: Utc::now().to_rfc3339(),
        receipt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn audit_hashes_payload() {
        let a = audit(
            WorkloadId::new("abc").unwrap(),
            "selected",
            "corr",
            "",
            b"payload",
        )
        .unwrap();
        assert!(matches!(a,StoredRecord::Audit{receipt,..}if !receipt.is_empty()));
    }
}
