use clap::Parser;

/// CLI for building a repertoire PGN by composing quality and popularity providers.
#[derive(Parser, Debug)]
pub struct Cli {
    /// Path to config TOML
    #[arg(long, default_value = "config.toml")]
    config: String,
    /// Side for which to optimize (white|black)
    #[arg(long)]
    side: String,
    /// Ply budget
    #[arg(long)]
    plies: u32,
    /// Starting moves in SAN (e.g., "1.e4 e5 2.Nf3 Nc6")
    #[arg(long)]
    start: Option<String>,
    /// Output PGN path
    #[arg(long, default_value = "repertoire.pgn")]
    out: String,
}
