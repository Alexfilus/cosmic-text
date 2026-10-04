#[path = "support/cluster_letter_spacing.rs"]
mod support;
#[test]
fn regression() {
    support::verify();
}
