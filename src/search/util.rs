use crate::domain::{FenKey, PieceColor};
use anyhow::{anyhow, Error, Result};
use shakmaty::CastlingMode;
use shakmaty::{fen::Fen, uci::Uci, Chess, EnPassantMode, Position};

pub fn apply_uci(fen_key: &FenKey, uci: &str) -> Result<(FenKey, shakmaty::Color)> {
    let position: Chess = extract_position_from_fen_key(fen_key)?;
    let extracted_move = extract_move_from_parsed_uci_and_position(uci, &position)?;

    let mut next = position.clone();
    next.play_unchecked(&extracted_move);
    let next_fen = Fen::from_position(next.clone(), EnPassantMode::Legal).to_string();
    Ok((
        FenKey {
            fen_string: next_fen,
            side_to_move: PieceColor::from_shakmaty(next.turn()),
        },
        next.turn(),
    ))
}

pub fn extract_move_from_parsed_uci_and_position(
    uci: &str,
    position: &Chess,
) -> Result<shakmaty::Move, Error> {
    Ok(uci
        .parse::<Uci>()
        .map_err(|_| anyhow!("bad UCI"))?
        .to_move(position)
        .map_err(|_| anyhow!("illegal UCI"))?)
}

pub fn extract_position_from_fen_key(fen_key: &FenKey) -> Result<Chess, Error> {
    Ok(fen_key
        .fen_string
        .parse::<Fen>()?
        .into_position(CastlingMode::Standard)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shakmaty::Color;

    #[test]
    fn test_extract_position_from_fen_key() {
        let fen_key = FenKey {
            fen_string: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            side_to_move: PieceColor::White,
        };
        let position = extract_position_from_fen_key(&fen_key).unwrap();
        assert_eq!(position.turn(), Color::White);
    }
}
