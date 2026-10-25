pub fn optimistic_score(base: f64, pulls: u64, total: u64, analogue: f64) -> f64 {
    let explore = (2.0 * ((total.max(2) as f64).ln()) / (pulls.max(1) as f64))
        .sqrt()
        .min(0.8);
    base + explore + analogue.clamp(0.0, 0.15)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explores() {
        assert!(optimistic_score(0.5, 1, 20, 0.0) > optimistic_score(0.5, 20, 20, 0.0));
    }
}
