use nafi_syntax::SourceFile;

#[test]
fn parse_snapshot_tests() {
    insta::glob!("snapshots/*.nafi", |path| {
        let input = std::fs::read_to_string(path).unwrap();
        insta::assert_debug_snapshot!(SourceFile::parse(&input).syntax_tree());
    });
}
