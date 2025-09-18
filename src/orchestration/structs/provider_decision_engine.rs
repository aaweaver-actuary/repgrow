use std::sync::Arc;

use crate::{
    domain::CandidateRequest,
    orchestration::{ExpansionInput, RawCandidates},
    policy::MovePolicy,
    provider::{MovePopularity, MoveQuality},
};

pub struct ProviderDecisionEngine<'a> {
    pub policy: &'a dyn MovePolicy,
    pub quality: Arc<dyn MoveQuality>,
    pub popularity: Arc<dyn MovePopularity>,
}

impl<'a> ProviderDecisionEngine<'a> {
    pub async fn fetch_raw(
        &self,
        input: &ExpansionInput,
        base_req: &CandidateRequest,
    ) -> anyhow::Result<RawCandidates> {
        // call policy.decide(stm), policy.adjust(req, is_my_side)
        // await provider
        // normalize_* → RawCandidates
    }
}

// Unit test: fake quality/popularity providers → assert the right one is called & req shaped.
