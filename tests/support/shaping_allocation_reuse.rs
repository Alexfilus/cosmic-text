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
    let text = "A".repeat(16385);
    let lines = shape(fs, &text, Attrs::new(), Wrap::WordOrGlyph, 300.0);
    let glyphs = lines.iter().map(|l| l.glyphs.len()).sum::<usize>();
    vec![glyphs as f32, lines.len() as f32]
}

pub fn verify() {
    let mut fs = fonts();
    let actual = scenario(&mut fs);
    assert_eq!(actual[0], 16385.0, "long word lost glyphs");
    assert!(actual[1] > 1.0, "long word was not wrapped");
}

pub fn benchmark(fs: &mut FontSystem) -> Vec<f32> {
    let text = "Am am ".repeat(100);
    let lines = shape(fs, &text, Attrs::new(), Wrap::WordOrGlyph, 300.0);
    let glyphs = lines.iter().map(|l| l.glyphs.len()).sum::<usize>();
    assert!(glyphs > 0);
    vec![glyphs as f32, lines.iter().map(|l| l.w).sum()]
}
