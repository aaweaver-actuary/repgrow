use crate::{
    config::SearchConfig,
    orchestration::{ExpansionInput, RawCandidates, SelectedCandidates},
    policy::MovePolicy,
};

pub struct CandidateSelector<'a> {
    pub policy: &'a dyn MovePolicy,
    pub cfg: &'a SearchConfig,
}

impl<'a> CandidateSelector<'a> {
    pub fn select(&self, input: &ExpansionInput, raw: RawCandidates) -> SelectedCandidates {
        // is_my_side? → call policy.post_filter(is_my_side, raw.moves)
        // truncate by cfg.{max_children_my_side|max_children_opp_side}
        // return SelectedCandidates
    }
}

// Unit test: seed mixed candidates (eval/play) → verify stable order and cap applied.
