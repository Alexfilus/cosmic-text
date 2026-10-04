#[path = "support/shaping_dependencies.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
