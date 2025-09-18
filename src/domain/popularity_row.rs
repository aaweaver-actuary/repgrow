use crate::domain::{PlayRate, chess::UciMove};

#[derive(Clone, Debug)]
pub struct PopularityRow {
    pub uci: UciMove,
    pub play_rate: PlayRate,
    pub games: u32,
}
