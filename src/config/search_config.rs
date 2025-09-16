use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchConfig {
    /// Number of concurrent searches to run.
    pub concurrency: usize,
    /// Maximum number of nodes to explore in total.
    pub max_total_nodes: Option<usize>,
    /// Maximum number of children to explore on the side to move.
    pub max_children_my_side: Option<usize>,
    /// Maximum number of children to explore on the opponent's side.
    pub max_children_opp_side: Option<usize>,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            max_total_nodes: Some(1000000),
            max_children_my_side: Some(10000),
            max_children_opp_side: Some(10000),
        }
    }
}
