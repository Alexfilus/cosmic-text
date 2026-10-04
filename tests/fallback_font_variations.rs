#[path = "support/fallback_font_variations.rs"]
mod support;

#[test]
fn regression() {
    support::verify();
}
