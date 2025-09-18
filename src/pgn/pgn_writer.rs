#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{PieceColor, RepertoireNode, chess::UciMove, fen_key::FenKey};

    /// Helper to build a node with children
    fn node(
        id: u64,
        parent: Option<u64>,
        fen: &str,
        color: PieceColor,
        uci: Option<UciMove>,
        ply: u32,
        children: Vec<u64>,
    ) -> RepertoireNode {
        RepertoireNode {
            id,
            parent,
            fen_key: FenKey::new(fen.to_string(), color),
            last_move_uci: uci,
            ply_depth: ply,
            children,
            signals: Default::default(),
        }
    }

    #[test]
    fn test_single_mainline() {
        // 1. e4 e5 2. Nf3 Nc6
        let n0 = node(0, None, "startpos", PieceColor::White, None, 0, vec![1]);
        let n1 = node(
            1,
            Some(0),
            "fen1",
            PieceColor::Black,
            Some(UciMove::from_uci("e2e4").unwrap()),
            1,
            vec![2],
        );
        let n2 = node(
            2,
            Some(1),
            "fen2",
            PieceColor::White,
            Some(UciMove::from_uci("e7e5").unwrap()),
            2,
            vec![3],
        );
        let n3 = node(
            3,
            Some(2),
            "fen3",
            PieceColor::Black,
            Some(UciMove::from_uci("g1f3").unwrap()),
            3,
            vec![4],
        );
        let n4 = node(
            4,
            Some(3),
            "fen4",
            PieceColor::White,
            Some(UciMove::from_uci("b8c6").unwrap()),
            4,
            vec![],
        );
        let nodes = vec![n0.clone(), n1, n2, n3, n4];
        let writer = PgnWriter;
        let pgn = writer.write_with_nodes(&n0, &nodes).unwrap();
        assert!(pgn.contains("1. e2e4 2. e7e5 3. g1f3 4. b8c6"));
    }

    #[test]
    fn test_variations() {
        // 1. e4 (1. d4)
        let n0 = node(0, None, "startpos", PieceColor::White, None, 0, vec![1, 2]);
        let n1 = node(
            1,
            Some(0),
            "fen1",
            PieceColor::Black,
            Some(UciMove::from_uci("e2e4").unwrap()),
            1,
            vec![],
        );
        let n2 = node(
            2,
            Some(0),
            "fen2",
            PieceColor::Black,
            Some(UciMove::from_uci("d2d4").unwrap()),
            1,
            vec![],
        );
        let nodes = vec![n0.clone(), n1, n2];
        let writer = PgnWriter;
        let pgn = writer.write_with_nodes(&n0, &nodes).unwrap();
        assert!(pgn.contains("1. e2e4 (1. d2d4)"));
    }

    #[test]
    fn test_empty_tree() {
        let n0 = node(0, None, "startpos", PieceColor::White, None, 0, vec![]);
        let writer = PgnWriter;
        let pgn = writer.write_with_nodes(&n0, &[n0.clone()]).unwrap();
        assert!(pgn.contains("*") && !pgn.contains("1."));
    }

    #[test]
    fn test_non_starting_fen() {
        let n0 = node(
            0,
            None,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
            PieceColor::Black,
            None,
            0,
            vec![],
        );
        let writer = PgnWriter;
        let pgn = writer.write_with_nodes(&n0, &[n0.clone()]).unwrap();
        assert!(pgn.contains("[FEN \"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1\"]"));
    }

    #[test]
    fn test_question_mark_for_missing_move() {
        let n0 = node(0, None, "startpos", PieceColor::White, None, 0, vec![1]);
        let n1 = node(1, Some(0), "fen1", PieceColor::Black, None, 1, vec![]);
        let nodes = vec![n0.clone(), n1];
        let writer = PgnWriter;
        let pgn = writer.write_with_nodes(&n0, &nodes).unwrap();
        assert!(pgn.contains("1. ?"));
    }
}
use crate::{
    domain::RepertoireNode,
    pgn::{RepertoireWriter, SanConverter},
};

/// PGN writer that traverses the repertoire tree and outputs moves in PGN format.
/// Supports mainline, variations, and emits FEN tag if not starting position.
#[derive(Default)]
pub struct PgnWriter;

impl PgnWriter {
    /// Recursively writes moves from the repertoire tree in PGN format.
    /// Returns a tuple: (PGN move string, number of plies written)
    pub fn write_with_nodes(
        &self,
        root: &RepertoireNode,
        nodes: &[RepertoireNode],
    ) -> anyhow::Result<String> {
        let mut pgn = String::from("[Event \"Repertoire\"]\n");
        if root.fen_key.fen_string != "startpos" {
            pgn += &format!("[FEN \"{}\"]\n", root.fen_key.fen_string);
        }
        pgn += "\n";
        let (moves, _) = self.write_moves(root, nodes, root.ply_depth);
        pgn += &moves;
        pgn += " *\n";
        Ok(pgn)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn write_moves(
        &self,
        node: &RepertoireNode,
        nodes: &[RepertoireNode],
        mut ply: u32,
    ) -> (String, u32) {
        let mut pgn = String::new();
        let mut current = node;
        while !current.children.is_empty() {
            // Mainline: first child
            let child_id = current.children[0];
            let child = nodes.iter().find(|n| n.id == child_id);
            if let Some(child) = child {
                ply += 1;
                // Output ply number and move
                if current.fen_key.side_to_move.is_white() {
                    pgn += &format!("{}.", ply.div_ceil(2));
                }
                if let Some(ref uci) = child.last_move_uci {
                    pgn += &format!(" {}", uci.to_uci());
                } else {
                    pgn += " ?";
                }
                // Handle variations (other children)
                if current.children.len() > 1 {
                    for &var_id in &current.children[1..] {
                        let var_node = nodes.iter().find(|n| n.id == var_id);
                        if let Some(var_node) = var_node {
                            let (var_pgn, _) = self.write_moves(var_node, nodes, ply);
                            pgn += &format!(" ({})", var_pgn.trim());
                        }
                    }
                }
                current = child;
            } else {
                // Child not found, output ? and break
                pgn += " ?";
                break;
            }
        }
        (pgn, ply)
    }

    pub fn write_with_nodes_and_san<C: SanConverter>(
        &self,
        root: &RepertoireNode,
        nodes: &[RepertoireNode],
        san_converter: &C,
    ) -> anyhow::Result<String> {
        let mut pgn = String::from("[Event \"Repertoire\"]\n");
        if root.fen_key.fen_string != "startpos" {
            pgn += &format!("[FEN \"{}\"]\n", root.fen_key.fen_string);
        }
        pgn += "\n";
        let (moves, _) = self.write_moves_with_san(root, nodes, root.ply_depth, san_converter);
        pgn += &moves;
        pgn += " *\n";
        Ok(pgn)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn write_moves_with_san<C: SanConverter>(
        &self,
        node: &RepertoireNode,
        nodes: &[RepertoireNode],
        mut ply: u32,
        san_converter: &C,
    ) -> (String, u32) {
        let mut pgn = String::new();
        let mut current = node;
        while !current.children.is_empty() {
            let child_id = current.children[0];
            let child = nodes.iter().find(|n| n.id == child_id);
            if let Some(child) = child {
                ply += 1;
                if current.fen_key.side_to_move.is_white() {
                    pgn += &format!("{}.", ply.div_ceil(2));
                }
                if let Some(ref uci) = child.last_move_uci {
                    let san = san_converter.uci_to_san(uci, &child.fen_key.fen_string);
                    pgn += &format!(" {}", san);
                } else {
                    pgn += " ?";
                }
                if current.children.len() > 1 {
                    for &var_id in &current.children[1..] {
                        let var_node = nodes.iter().find(|n| n.id == var_id);
                        if let Some(var_node) = var_node {
                            let (var_pgn, _) =
                                self.write_moves_with_san(var_node, nodes, ply, san_converter);
                            pgn += &format!(" ({})", var_pgn.trim());
                        }
                    }
                }
                current = child;
            } else {
                pgn += " ?";
                break;
            }
        }
        (pgn, ply)
    }
}

impl RepertoireWriter for PgnWriter {
    /// Writes the repertoire tree to PGN format using only the root node (legacy interface).
    /// For full traversal, use write_with_nodes.
    fn write(&self, root: &RepertoireNode) -> anyhow::Result<String> {
        self.write_with_nodes(root, &[root.clone()])
    }
}
