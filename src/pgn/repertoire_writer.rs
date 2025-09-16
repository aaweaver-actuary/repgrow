use crate::domain::RepertoireNode;
use anyhow::Result;

/// Writer interface for alternate outputs later (JSON, DB, etc.)
pub trait RepertoireWriter {
    fn write(&self, root: &RepertoireNode) -> Result<String>;
}
