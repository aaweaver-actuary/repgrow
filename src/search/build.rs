use crate::domain::{FenKey, PieceColor, RepertoireNode};
use shakmaty::{fen::Fen, san::San, Chess, EnPassantMode::Legal, Position};

/// Parse a SAN line into a starting FenKey (and stm)
pub fn start_from_san(san_line: Option<&str>) -> anyhow::Result<(FenKey, shakmaty::Color)> {
    let mut pos = Chess::default();
    if let Some(line) = san_line {
        for tok in line.split_whitespace() {
            if tok.contains('.') {
                continue;
            }
            let san: San = tok.parse().map_err(|_| anyhow::anyhow!("bad SAN: {tok}"))?;
            let mv = san
                .to_move(&pos)
                .map_err(|_| anyhow::anyhow!("illegal SAN: {tok}"))?;
            pos.play_unchecked(&mv);
        }
    }
    let fen = Fen::from_position(pos.clone(), Legal).to_string();
    let stm = pos.turn();
    Ok((
        FenKey {
            fen,
            stm: PieceColor::from_shakmaty(stm),
        },
        stm,
    ))
}

/// Create a node struct (id assigned by arena)
pub fn make_node(
    parent: Option<u64>,
    fen_key: &FenKey,
    last_uci: Option<String>,
    ply: u32,
) -> RepertoireNode {
    RepertoireNode {
        id: 0, // arena will set real id
        parent,
        fen_key: fen_key.clone(),
        last_move_uci: last_uci,
        ply_depth: ply,
        children: Vec::new(),
        signals: Default::default(),
    }
}
