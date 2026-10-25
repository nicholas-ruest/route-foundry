use rf_embedding::DIMENSION;
use rf_errors::{FoundryError, Result};
use rf_types::Analogue;
use ruvector_core::{
    types::{DbOptions, DistanceMetric, SearchQuery, VectorEntry},
    VectorDB,
};
use std::collections::HashMap;
pub struct RouteIndex {
    db: VectorDB,
    len: usize,
}
impl RouteIndex {
    pub fn new() -> Result<Self> {
        let db = VectorDB::new(DbOptions {
            dimensions: DIMENSION,
            distance_metric: DistanceMetric::Euclidean,
            storage_path: "memory://route-foundry".into(),
            hnsw_config: None,
            quantization: None,
        })
        .map_err(|e| FoundryError::Dependency(format!("RuVector create: {e}")))?;
        Ok(Self { db, len: 0 })
    }
    pub fn insert(&mut self, id: &str, vector: Vec<f32>, decision: &str) -> Result<()> {
        let mut metadata = HashMap::new();
        metadata.insert("decision".into(), serde_json::json!(decision));
        self.db
            .insert(VectorEntry {
                id: Some(id.into()),
                vector,
                metadata: Some(metadata),
            })
            .map_err(|e| FoundryError::Dependency(format!("RuVector insert: {e}")))?;
        self.len += 1;
        Ok(())
    }
    pub fn search(&self, vector: &[f32], k: usize) -> Result<Vec<Analogue>> {
        if self.len == 0 {
            return Ok(vec![]);
        }
        let hits = self
            .db
            .search(SearchQuery {
                vector: vector.to_vec(),
                k: k.min(self.len),
                filter: None,
                ef_search: None,
            })
            .map_err(|e| FoundryError::Dependency(format!("RuVector search: {e}")))?;
        Ok(hits
            .into_iter()
            .map(|h| Analogue {
                workload_id: h.id,
                distance: f64::from(h.score),
                decision: h
                    .metadata
                    .and_then(|m| {
                        m.get("decision")
                            .and_then(|v| v.as_str())
                            .map(str::to_owned)
                    })
                    .unwrap_or_default(),
            })
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn executed_insert_search() {
        let mut i = RouteIndex::new().unwrap();
        i.insert("orders", vec![0.0; DIMENSION], "allow").unwrap();
        assert_eq!(
            i.search(&[0.1; DIMENSION], 1).unwrap()[0].workload_id,
            "orders"
        )
    }
}
