use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyConfig {
    pub my_side: Option<String>,
    pub cp_window: i32,
    pub min_play_rate: f32,
}

impl PolicyConfig {
    pub fn resolve_side_override(&self, cli_side: &str) -> anyhow::Result<shakmaty::Color> {
        let s = if !cli_side.is_empty() {
            Some(cli_side.to_string())
        } else {
            self.my_side.clone()
        };
        match s.as_deref() {
            Some("white") => Ok(shakmaty::Color::White),
            Some("black") => Ok(shakmaty::Color::Black),
            _ => anyhow::bail!("side must be white|black"),
        }
    }
}
