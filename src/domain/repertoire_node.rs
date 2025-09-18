/// RepertoireNode represents a node in the repertoire tree.
/// Each node corresponds to a position reached by a sequence of moves.
/// The tree is stored in an arena, with nodes referencing children by their IDs.
use super::fen_key::FenKey;
use crate::domain::{PieceColor, Signals};
// use shakmaty::Color;

#[derive(Clone, Debug)]
pub struct RepertoireNode {
    pub id: u64,
    pub parent: Option<u64>,
    pub fen_key: FenKey,
    pub last_move_uci: Option<String>,
    pub ply_depth: u32,
    pub children: Vec<u64>,
    pub signals: Signals,
}

impl RepertoireNode {
    pub fn new(
        id: u64,
        parent: Option<u64>,
        fen_key: FenKey,
        last_move_uci: Option<String>,
        ply_depth: u32,
    ) -> Self {
        Self {
            id,
            parent,
            fen_key,
            last_move_uci,
            ply_depth,
            children: Vec::new(),
            signals: Signals::default(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

impl Default for RepertoireNode {
    fn default() -> Self {
        Self {
            id: 0,
            parent: None,
            fen_key: FenKey {
                fen_string: String::from("startpos"),
                side_to_move: PieceColor::White,
            },
            last_move_uci: None,
            ply_depth: 0,
            children: Vec::new(),
            signals: Signals::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Centipawns, PieceColor, Signals, fen_key::FenKey};

    #[test]
    fn test_new_node() {
        let fen_key = FenKey {
            fen_string: "8/8/8/8/8/8/8/8 w - - 0 1".to_string(),
            side_to_move: PieceColor::Black,
        };
        let node = RepertoireNode::new(42, Some(1), fen_key.clone(), Some("e2e4".to_string()), 5);
        assert_eq!(node.id, 42);
        assert_eq!(node.parent, Some(1));
        assert_eq!(node.fen_key.fen_string, fen_key.fen_string);
        assert_eq!(node.fen_key.side_to_move, PieceColor::Black);
        assert_eq!(node.last_move_uci, Some("e2e4".to_string()));
        assert_eq!(node.ply_depth, 5);
        assert!(node.children.is_empty());
        assert_eq!(node.signals, Signals::default());
    }

    #[test]
    fn test_default_node() {
        let node = RepertoireNode::default();
        assert_eq!(node.id, 0);
        assert_eq!(node.parent, None);
        assert_eq!(node.fen_key.fen_string, "startpos");
        assert_eq!(node.fen_key.side_to_move, PieceColor::White);
        assert_eq!(node.last_move_uci, None);
        assert_eq!(node.ply_depth, 0);
        assert!(node.children.is_empty());
        assert_eq!(node.signals, Signals::default());
    }

    #[test]
    fn test_is_root() {
        let node = RepertoireNode {
            parent: None,
            ..Default::default()
        };
        assert!(node.is_root());
        let node2 = RepertoireNode {
            parent: Some(7),
            ..Default::default()
        };
        assert!(!node2.is_root());
    }

    #[test]
    fn test_children_mutation() {
        let mut node = RepertoireNode::default();
        node.children.push(10);
        node.children.push(20);
        assert_eq!(node.children, vec![10, 20]);
    }

    #[test]
    fn test_signals_field() {
        let mut node = RepertoireNode::default();
        node.signals.eval_cp = Some(Centipawns::from_float(1.5));
        assert_eq!(node.signals.eval_cp, Some(Centipawns::from_float(1.5)));
    }

    #[test]
    fn test_clone_and_debug() {
        let node = RepertoireNode::new(
            99,
            Some(88),
            FenKey {
                fen_string: "testfen".to_string(),
                side_to_move: PieceColor::Black,
            },
            Some("g1f3".to_string()),
            12,
        );
        let node2 = node.clone();
        assert_eq!(node.id, node2.id);
        let dbg = format!("{:?}", node2);
        assert!(dbg.contains("RepertoireNode"));
        assert!(dbg.contains("id: 99"));
        assert!(dbg.contains("parent: Some(88)"));
        assert!(dbg.contains("fen_key"));
        assert!(dbg.contains("last_move_uci: Some(\"g1f3\")"));
    }
}
