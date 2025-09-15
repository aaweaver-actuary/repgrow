use shakmaty::Color;

use crate::{
    domain::CandidateRequest,
    policy::{Decision, MovePolicy},
};

/// Default: my side → quality (engine); opponent → popularity (explorer)
pub struct SideSplitPolicy {
    my_side: Color,
    cp_window: i32,
    min_play_rate: f32,
}

impl SideSplitPolicy {
    pub fn new(my_side: Color, cp_window: i32, min_play_rate: f32) -> Self {
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
            req.cp_window = self.cp_window as f32;
        } else {
            req.min_play_rate = self.min_play_rate as f32;
        }
    }
}
