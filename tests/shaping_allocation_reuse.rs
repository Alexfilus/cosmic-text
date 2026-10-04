#[path = "support/shaping_allocation_reuse.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
