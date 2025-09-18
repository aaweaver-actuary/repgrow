use crate::domain::chess::UciMove;

pub trait SanConverter {
    fn uci_to_san(&self, uci: &UciMove, fen: &str) -> String;
}

pub struct MockSanConverter;

impl SanConverter for MockSanConverter {
    /// A mock implementation that converts a few common UCI moves to SAN.
    /// For other moves, it returns the UCI string as-is. This is a stub and
    /// will be used only for testing purposes.
    fn uci_to_san(&self, uci: &UciMove, _fen: &str) -> String {
        match uci.to_uci().as_str() {
            "e2e4" => "e4".to_string(),
            "e7e5" => "e5".to_string(),
            "g1f3" => "Nf3".to_string(),
            "b8c6" => "Nc6".to_string(),
            "d2d4" => "d4".to_string(),
            _ => uci.to_uci(),
        }
    }
}
