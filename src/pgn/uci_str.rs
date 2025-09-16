/// UciStr struct for handling/validating UCI strings

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UciStr(String);

impl UciStr {
    /// Creates a new UciStr from a string slice.
    pub fn new(s: &str) -> Self {
        UciStr(s.to_string())
    }

    /// Validates the UCI string format.
    pub fn is_valid(&self) -> bool {
        // Basic validation: UCI strings are typically 4 characters long
        self.0.len() == 4
    }

    /// Returns the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the UciStr and returns the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}
