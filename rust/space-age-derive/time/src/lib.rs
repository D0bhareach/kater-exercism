#[derive(Debug)]
pub struct Duration {
    pub seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_time() {
        let t = 1_000_000;
        let d = Duration::from(t);
        assert_eq!(d.seconds, 1_000_000.0);
    }
}
