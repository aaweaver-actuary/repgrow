use async_trait::async_trait;

#[async_trait]
pub trait TerminationPolicyPort: Send + Sync {
    fn should_expand(&self, current_ply: u32, max_plies: u32) -> bool;
    fn notify_children_created(&self, n: usize); // optional for global caps
}

pub struct SimpleTerminationPolicy {/* counters, max_total_nodes */}

// Unit test: feed boundaries → expect allow/deny expansion; verify counter increments.
