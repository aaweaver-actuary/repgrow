use crate::domain::{FenKey, PieceColor};
use anyhow::{anyhow, Result};
use shakmaty::{fen::Fen, san::San, Chess, Color, EnPassantMode, Position};

/// Parse SAN into starting FEN (and side to move)
pub fn start_from_san(san_line: Option<&str>) -> Result<(FenKey, Color)> {
    let mut pos = Chess::default();
    if let Some(line) = san_line {
        for tok in line.split_whitespace() {
            if tok.contains('.') {
                continue;
            }
            let san: San = tok.parse().map_err(|_| anyhow!("bad SAN: {tok}"))?;
            let mv = san
                .to_move(&pos)
                .map_err(|_| anyhow!("illegal SAN: {tok}"))?;
            pos.play_unchecked(&mv);
        }
    }
    let fen = Fen::from_position(pos.clone(), EnPassantMode::Legal).to_string();
    let stm = pos.turn();
    Ok((
        FenKey {
            fen_string: fen,
            side_to_move: PieceColor::from_shakmaty(stm),
        },
        stm,
    ))
}
