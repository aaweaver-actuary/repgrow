use once_cell::sync::OnceCell;

static LOG_INIT: OnceCell<()> = OnceCell::new();
#[test]
fn cli_finds_white_moves_after_1e4_e5_2nf3_nc6() {
    LOG_INIT.get_or_init(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    });
    use assert_cmd::Command;
    use std::fs;
    use tempfile::NamedTempFile;

    let out_file = NamedTempFile::new().unwrap();
    let out_path = out_file.path().to_str().unwrap();

    // Run the CLI
    Command::cargo_bin("repgrow")
        .unwrap()
        .args([
            "--start",
            "1. e4 e5 2. Nf3 Nc6",
            "--plies",
            "1",
            "--side",
            "white",
            "--out",
            // out_path,
            "repgrow.pgn",
        ])
        .assert()
        .success();

    // Read and parse the PGN
    let pgn = fs::read_to_string(out_path).unwrap();
    assert!(pgn.contains("1.e4 e5 2.Nf3 Nc6"));
    assert!(pgn.contains("3.Bb5") || pgn.contains("3.Bc4") || pgn.contains("3.d4"));
}
