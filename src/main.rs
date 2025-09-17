use clap_builder::Parser;
use repgrow::pgn::RepertoireWriter;
use repgrow::{
    cli::Cli,
    config::AppConfig,
    infra::build_infra,
    pgn::PgnWriter,
    policy::SideSplitPolicy,
    provider::{build_popularity, build_quality},
    search::Orchestrator,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = AppConfig::load(&cli.config)?;

    // Build infra
    let infra = build_infra(&cfg)?;

    // Build providers from config (factory)
    let quality = build_quality(&cfg.quality, &infra)?;
    let popularity = build_popularity(&cfg.popularity, &infra)?;

    // Build policy (default: my side → quality; opp → popularity)
    let my_side = cfg.policy.resolve_side_override(cli.side.as_str())?;
    let policy = SideSplitPolicy::new(my_side, cfg.policy.cp_window, cfg.policy.min_play_rate);

    // Orchestrator
    let orch = Orchestrator::new(cfg.search.clone(), policy, quality, popularity);
    let root = orch
        .build_from_start(cli.start.as_deref(), cli.plies)
        .await?;

    // Write PGN
    let writer = PgnWriter;
    let pgn = writer.write(&root)?;
    std::fs::write(&cli.out, pgn)?;
    eprintln!("Wrote {}", cli.out);
    Ok(())
}
