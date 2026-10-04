#[path = "support/variable_font_instances.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
