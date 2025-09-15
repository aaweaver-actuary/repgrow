#[test]
fn pgn_writer_minimal() {
    let writer = repgrow::pgn::writer::PgnWriter::default();
    // Minimal fake node; in practice supply arena & real traversal.
    let root = repgrow::domain::RepertoireNode {
        id: 0, parent: None,
        fen_key: repgrow::domain::FenKey { fen: "dummy".into(), stm: shakmaty::Color::White },
        last_move_uci: None, ply_depth: 0, children: vec![], signals: Default::default()
    };
    let pgn = writer.write(&root).unwrap();
    assert!(pgn.contains("[Event"));
}
