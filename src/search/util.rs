use crate::domain::{FenKey, PieceColor};
use shakmaty::{fen, uci::Uci, CastlingMode::Standard, Chess, EnPassantMode::Legal, Position};

pub fn apply_uci(fen_key: &FenKey, uci: &str) -> anyhow::Result<(FenKey, shakmaty::Color)> {
    let pos: Chess = fen_key.fen.parse::<fen::Fen>()?.into_position(Standard)?;
    let u: Uci = uci.parse()?;
    let m = u
        .to_move(&pos)
        .map_err(|_| anyhow::anyhow!("illegal UCI"))?;
    let mut next = pos.clone();
    next.play_unchecked(&m);
    let next_fen = fen::Fen::from_position(next.clone(), Legal).to_string();
    Ok((
        FenKey {
            fen: next_fen,
            stm: PieceColor::from_shakmaty(next.turn()),
        },
        next.turn(),
    ))
}
