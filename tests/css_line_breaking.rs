#[path = "support/css_line_breaking.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
