use crate::domain::RepertoireNode;

/// Writer interface for alternate outputs later (JSON, DB, etc.)
pub trait RepertoireWriter {
    fn write(&self, root: &RepertoireNode) -> anyhow::Result<String>;
}
