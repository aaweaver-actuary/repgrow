use shakmaty::Color;

use crate::{
    domain::{CandidateRequest, Centipawns, PlayRate},
    policy::{Decision, MovePolicy},
};

/// Default: my side → quality (engine); opponent → popularity (explorer)
pub struct SideSplitPolicy {
    my_side: Color,
    cp_window: Centipawns,
    min_play_rate: PlayRate,
}

impl SideSplitPolicy {
    pub fn new(my_side: Color, cp_window: Centipawns, min_play_rate: PlayRate) -> Self {
        Self {
            my_side,
            cp_window,
            min_play_rate,
        }
    }
}

impl MovePolicy for SideSplitPolicy {
    fn decide(&self, stm: Color) -> Decision {
        if stm == self.my_side {
            Decision::Quality
        } else {
            Decision::Popularity
        }
    }
    fn adjust(&self, req: &mut CandidateRequest, is_my_side: bool) {
        if is_my_side {
            req.cp_window = self.cp_window;
        } else {
            req.min_play_rate = self.min_play_rate;
        }
    }
}
