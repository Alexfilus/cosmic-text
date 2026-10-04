#[path = "support/shaping_fast_paths.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
