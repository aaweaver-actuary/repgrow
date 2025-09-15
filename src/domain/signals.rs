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
            eval_cp: Some(42.5),
            depth: Some(12),
            play_rate: Some(0.8),
            games: Some(100),
        };
        assert_eq!(s.eval_cp, Some(42.5));
        assert_eq!(s.depth, Some(12));
        assert_eq!(s.play_rate, Some(0.8));
        assert_eq!(s.games, Some(100));
    }

    #[test]
    fn test_signals_clone() {
        let s1 = Signals {
            eval_cp: Some(-3.2),
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
            eval_cp: Some(1.0),
            depth: Some(2),
            play_rate: Some(0.5),
            games: Some(10),
        };
        let dbg = format!("{:?}", s);
        assert!(dbg.contains("Signals"));
        assert!(dbg.contains("eval_cp: Some(1.0)"));
        assert!(dbg.contains("depth: Some(2)"));
        assert!(dbg.contains("play_rate: Some(0.5)"));
        assert!(dbg.contains("games: Some(10)"));
    }
}
/// Signals union carried by candidates; expandable without changing traits.
#[derive(Clone, Debug, Default)]
pub struct Signals {
    pub eval_cp: Option<f32>,
    pub depth: Option<u8>,
    pub play_rate: Option<f32>,
    pub games: Option<u32>,
}
