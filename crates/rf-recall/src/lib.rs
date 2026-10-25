use rf_types::Analogue;
pub fn analogue_bonus(a: &[Analogue], provider: &str) -> f64 {
    let n = a
        .iter()
        .filter(|x| x.decision == provider && x.distance < 1.0)
        .count();
    (n as f64 * 0.03).min(0.12)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded() {
        let a = vec![
            Analogue {
                workload_id: "x".into(),
                distance: 0.2,
                decision: "p".into()
            };
            10
        ];
        assert_eq!(analogue_bonus(&a, "p"), 0.12);
    }
}
