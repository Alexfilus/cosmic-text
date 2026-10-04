use cosmic_text::*;

pub fn fonts() -> FontSystem {
    let mut db = fontdb::Database::new();
    db.load_font_data(include_bytes!("../../fonts/ObscuraVFTest.ttf").to_vec());
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}

fn shape(
    fs: &mut FontSystem,
    text: &str,
    attrs: Attrs<'_>,
    wrap: Wrap,
    width: f32,
) -> Vec<LayoutLine> {
    ShapeLine::new(fs, text, &AttrsList::new(&attrs), Shaping::Advanced, 8).layout(
        32.0,
        Some(width),
        wrap,
        None,
        None,
    )
}

pub fn scenario(fs: &mut FontSystem) -> Vec<f32> {
    let mut result = Vec::new();
    for text in ["Am am Am am", "Am	Am", "Am am"] {
        let lines = shape(fs, text, Attrs::new(), Wrap::Word, 100.0);
        result.push(lines.iter().map(|l| l.w).sum());
    }
    result
}

pub fn verify() {
    let mut fs = fonts();
    let actual = scenario(&mut fs);
    let repeated = scenario(&mut fs);
    assert_eq!(actual, repeated, "cached shaping changed advances");
    assert!(actual.iter().all(|v| v.is_finite() && *v > 0.0));
}
