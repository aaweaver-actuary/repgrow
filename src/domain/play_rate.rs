use std::cmp::Ordering;

/// Wrapper type for play rate as a float between 0.0 and 1.0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayRate(pub f32);

impl PlayRate {
    /// Creates a new PlayRate, ensuring the value is clamped between 0.0 and 1.0.
    pub fn new(rate: f32) -> Self {
        PlayRate(rate.clamp(0.0, 1.0))
    }

    /// Returns the inner float value.
    pub fn as_f32(&self) -> f32 {
        self.0
    }

    /// Alias for as_f32.
    pub fn as_float(&self) -> f32 {
        self.as_f32()
    }

    /// Alias for as_f32.
    pub fn value(&self) -> f32 {
        self.as_f32()
    }

    /// Returns the play rate as a percentage (0 to 100), rounded to `round_to` decimal places and formatted as a String.
    pub fn as_pct(&self, round_to: u32) -> String {
        let factor = 10f32.powi(round_to as i32);
        let rounded = (self.0 * 100.0 * factor).round() / factor;
        format!("{:.1}", rounded)
    }

    /// Returns GreaterThan, LessThan, or Equal to another PlayRate.
    pub fn compare(&self, other: &PlayRate) -> Ordering {
        let value1 = self.0;
        let value2 = other.0;
        if value1 < value2 {
            Ordering::Less
        } else if value1 > value2 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Default for PlayRate {
    fn default() -> Self {
        PlayRate(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_rate_creation() {
        let pr = PlayRate::new(0.75);
        assert_eq!(pr.as_f32(), 0.75);
        assert_eq!(pr.as_float(), 0.75);
        assert_eq!(pr.value(), 0.75);
    }

    #[test]
    fn test_play_rate_clamping() {
        let pr_low = PlayRate::new(-0.5);
        assert_eq!(pr_low.as_f32(), 0.0);
        let pr_high = PlayRate::new(1.5);
        assert_eq!(pr_high.as_f32(), 1.0);
        let pr_valid = PlayRate::new(0.5);
        assert_eq!(pr_valid.as_f32(), 0.5);
        assert_eq!(pr_valid.as_float(), 0.5);
        assert_eq!(pr_valid.value(), 0.5);
    }

    #[test]
    fn test_play_rate_as_pct() {
        let pr = PlayRate::new(0.756);
        assert_eq!(pr.as_pct(1), "75.6");
        assert_eq!(pr.as_pct(2), "75.60");
        let pr2 = PlayRate::new(0.1);
        assert_eq!(pr2.as_pct(0), "10");
        let pr3 = PlayRate::new(0.999);
        assert_eq!(pr3.as_pct(2), "99.90");
    }

    #[test]
    fn test_play_rate_compare() {
        let pr1 = PlayRate::new(0.5);
        let pr2 = PlayRate::new(0.75);
        let pr3 = PlayRate::new(0.5);
        assert_eq!(pr1.compare(&pr2), Ordering::Less);
        assert_eq!(pr2.compare(&pr1), Ordering::Greater);
        assert_eq!(pr1.compare(&pr3), Ordering::Equal);
    }
}
