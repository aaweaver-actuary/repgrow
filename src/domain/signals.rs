use crate::domain::{Centipawns, PlayRate};

/// Signals union carried by candidates; expandable without changing traits.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Signals {
    /// Centipawn evaluation from engine analysis, positive for White, negative for Black. None if no evaluation available.
    pub eval_cp: Option<Centipawns>,

    /// Depth of the engine analysis that produced eval_cp. None if no analysis available.
    pub depth: Option<u8>,

    /// Play rate of this move in the given position, as a fraction between 0.0 and 1.0. None if no data available.
    pub play_rate: Option<PlayRate>,

    /// Number of games played in this position. None if no data available.
    pub games: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signals_default() {
        let s = Signals::default();
        assert_eq!(s.eval_cp, None);
        assert_eq!(s.depth, None);
        assert_eq!(s.play_rate, None);
        assert_eq!(s.games, None);
    }

    #[test]
    fn test_signals_new_values() {
        let s = Signals {
            eval_cp: Some(Centipawns::from_float(42.5)),
            depth: Some(12),
            play_rate: Some(PlayRate::new(0.8)),
            games: Some(100),
        };
        assert_eq!(s.eval_cp, Some(Centipawns::from_float(42.5)));
        assert_eq!(s.depth, Some(12));
        assert_eq!(s.play_rate, Some(PlayRate::new(0.8)));
        assert_eq!(s.games, Some(100));
    }

    #[test]
    fn test_signals_clone() {
        let s1 = Signals {
            eval_cp: Some(Centipawns::from_float(-3.2)),
            depth: Some(5),
            play_rate: None,
            games: Some(7),
        };
        let s2 = s1.clone();
        assert_eq!(s1.eval_cp, s2.eval_cp);
        assert_eq!(s1.depth, s2.depth);
        assert_eq!(s1.play_rate, s2.play_rate);
        assert_eq!(s1.games, s2.games);
    }

    #[test]
    fn test_signals_debug() {
        let s = Signals {
            eval_cp: Some(Centipawns::from_float(1.0)),
            depth: Some(2),
            play_rate: Some(PlayRate::new(0.5)),
            games: Some(10),
        };
        let dbg = format!("{:?}", s);
        println!("Results from the debug macro:\n{}", dbg);
        assert!(dbg.contains("Signals"));
        assert!(dbg.contains("eval_cp: Some(Centipawns(1.0))"));
        assert!(dbg.contains("depth: Some(2)"));
        assert!(dbg.contains("play_rate: Some(PlayRate(0.5))"));
        assert!(dbg.contains("games: Some(10)"));
    }
}
