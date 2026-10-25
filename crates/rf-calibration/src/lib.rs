use rf_errors::{FoundryError, Result};
pub fn conformal_lower(samples: &[f64], alpha: f64) -> Result<f64> {
    if samples.len() < 3
        || !(0.0..1.0).contains(&alpha)
        || samples
            .iter()
            .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
    {
        return Err(FoundryError::Invalid("invalid calibration sample".into()));
    }
    let mut v = samples.to_vec();
    v.sort_by(f64::total_cmp);
    let rank = ((alpha * (v.len() + 1) as f64).ceil() as usize)
        .saturating_sub(1)
        .min(v.len() - 1);
    Ok(v[rank])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conservative() {
        assert!(
            (conformal_lower(&[0.81, 0.92, 0.88, 0.86, 0.9], 0.2).unwrap() - 0.86).abs() < 1e-9
        );
    }
}
