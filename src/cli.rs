use clap::Parser;

/// CLI for building a repertoire PGN by composing quality and popularity providers.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to config TOML
    #[arg(long, default_value = "config.toml")]
    pub config: String,
    /// Side for which to optimize (white|black)
    #[arg(long)]
    pub side: String,
    /// Ply budget
    #[arg(long)]
    pub plies: u32,
    /// Starting moves in SAN (e.g., "1.e4 e5 2.Nf3 Nc6")
    #[arg(long)]
    pub start: Option<String>,
    /// Output PGN path
    #[arg(long, default_value = "repertoire.pgn")]
    pub out: String,
}
