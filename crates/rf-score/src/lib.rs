pub fn utility(quality: f64, cost: u64, latency: u32) -> f64 {
    (quality * 1.2 - cost as f64 / 1000.0 - latency as f64 / 10000.0).clamp(-1.0, 1.0)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monotonic() {
        assert!(utility(0.9, 10, 100) > utility(0.8, 40, 200));
    }
}
