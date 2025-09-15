use repgrow::{config::AppConfig, policy::SideSplitPolicy, provider, search::orchestrator::Orchestrator, pgn::writer::PgnWriter};
use shakmaty::{Chess, san::San};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = AppConfig::load(&cli.config)?;

    // Build infra
    let infra = repgrow::infra::build_infra(&cfg)?;

    // Build providers from config (factory)
    let quality = provider::build_quality(&cfg.quality, &infra)?;
    let popularity = provider::build_popularity(&cfg.popularity, &infra)?;

    // Build policy (default: my side → quality; opp → popularity)
    let my_side = cfg.policy.resolve_side_override(cli.side.as_str())?;
    let policy = SideSplitPolicy::new(my_side, cfg.policy.cp_window, cfg.policy.min_play_rate);

    // Orchestrator
    let mut orch = Orchestrator::new(cfg.search.clone(), policy, quality, popularity, infra.clone());
    let root = orch.build_from_start(cli.start.as_deref(), cli.plies).await?;

    // Write PGN
    let writer = PgnWriter::default();
    let pgn = writer.write(&root)?;
    std::fs::write(&cli.out, pgn)?;
    eprintln!("Wrote {}", cli.out);
    Ok(())
}
