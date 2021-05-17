#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compiles.rs");
    t.pass("tests/defaults.rs");
}
