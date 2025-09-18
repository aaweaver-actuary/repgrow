use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, PartialOrd)]
pub struct Centipawns(f32);

impl Centipawns {
    /// Creates a new Centipawns instance.
    /// # Arguments
    /// * `value` - The centipawn value as a float.
    /// # Returns
    /// * `Centipawns` - A new Centipawns instance.
    /// # Examples
    /// ```
    /// use repgrow::domain::Centipawns;
    /// let cp = Centipawns::new(42.0);
    /// assert_eq!(cp.value(), 42.0);
    /// let cp_neg = Centipawns::new(-15.5);
    /// assert_eq!(cp_neg.value(), -15.5);
    /// ```
    pub fn new(value: f32) -> Self {
        Centipawns(value)
    }

    /// Creates a Centipawns instance from an integer value.
    /// # Arguments
    /// * `value` - The centipawn value as an integer.
    /// # Returns
    /// * `Centipawns` - A new Centipawns instance.
    /// # Examples
    /// ```
    /// use repgrow::domain::Centipawns;
    /// let cp = Centipawns::from_int(100);
    /// assert_eq!(cp.value(), 100.0);
    /// let cp_neg = Centipawns::from_int(-50);
    /// assert_eq!(cp_neg.value(), -50.0);
    /// ```
    pub fn from_int(value: i32) -> Self {
        Centipawns(value as f32)
    }

    /// Alias for `new`.
    /// # Arguments
    /// * `value` - The centipawn value as a float.
    /// # Returns
    /// * `Centipawns` - A new Centipawns instance.
    /// # Examples
    /// ```
    /// use repgrow::domain::Centipawns;
    /// let cp = Centipawns::from_float(75.5);
    /// assert_eq!(cp.value(), 75.5);
    /// let cp_neg1 = Centipawns::from_float(-20.0);
    /// let cp_neg2 = Centipawns::new(-20.0);
    /// assert_eq!(cp_neg1, cp_neg2);
    /// ```
    pub fn from_float(value: f32) -> Self {
        Centipawns::new(value)
    }

    /// Returns the inner float value.
    /// # Returns
    /// * `f32` - The centipawn value.
    /// # Examples
    /// ```
    /// use repgrow::domain::Centipawns;
    /// let cp = Centipawns::new(42.0);
    /// assert_eq!(cp.value(), 42.0);
    /// let cp_neg = Centipawns::new(-15.5);
    /// assert_eq!(cp_neg.value(), -15.5);
    /// ```
    pub fn value(&self) -> f32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centipawns() {
        let cp = Centipawns::new(42.0);
        assert_eq!(cp.value(), 42.0);
    }

    #[test]
    fn test_centipawns_negative() {
        let cp = Centipawns::new(-15.5);
        assert_eq!(cp.value(), -15.5);
    }

    #[test]
    fn test_centipawns_can_be_serialized() {
        let cp = Centipawns::new(10.0);
        let serialized = serde_json::to_string(&cp).unwrap();
        assert_eq!(serialized, "10.0");
    }

    #[test]
    fn test_centipawns_can_be_deserialized() {
        let data = "25.5";
        let cp: Centipawns = serde_json::from_str(data).unwrap();
        assert_eq!(cp.value(), 25.5);
    }
}
