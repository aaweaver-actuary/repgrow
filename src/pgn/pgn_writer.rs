use crate::{domain::RepertoireNode, pgn::RepertoireWriter};

/// Minimal PGN writer that walks the node arena from the root.
/// For brevity, this example assumes node 0 is root and siblings are variations.
#[derive(Default)]
pub struct PgnWriter;

impl RepertoireWriter for PgnWriter {
    fn write(&self, _root: &RepertoireNode) -> anyhow::Result<String> {
        // TODO: traverse stored arena (you may want to give writer a ref to arena)
        // Emit tag pairs, SetUp/FEN when not initial, then SAN tokens + variations.
        Ok(String::from("[Event \"Repertoire\"]\n\n1. e4 e5 *\n"))
    }
}
