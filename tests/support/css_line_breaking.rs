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
    for overflow in [
        CssOverflowWrap::Normal,
        CssOverflowWrap::BreakWord,
        CssOverflowWrap::Anywhere,
    ] {
        let attrs = Attrs::new().css_line_break(CssLineBreak {
            wrap: true,
            word_break: CssWordBreak::Normal,
            overflow_wrap: overflow,
        });
        let line = ShapeLine::new(
            fs,
            "Ammmmmmmm",
            &AttrsList::new(&attrs),
            Shaping::Advanced,
            8,
        );
        let ordinary = line.layout(32.0, Some(50.0), Wrap::WordOrGlyph, None, None);
        let intrinsic = line.layout(32.0, Some(0.0), Wrap::WordOrGlyphMinContent, None, None);
        result.extend([
            ordinary.len() as f32,
            intrinsic.iter().map(|l| l.w).fold(0.0, f32::max),
        ]);
    }
    result
}

pub fn verify() {
    let mut fs = fonts();
    let actual = scenario(&mut fs);
    assert_eq!(actual[0], 1.0);
    assert!(
        actual[2] > 1.0 && actual[4] > 1.0,
        "emergency wrapping missing: {actual:?}"
    );
    assert!(
        (actual[1] - actual[3]).abs() < 0.01,
        "break-word changed min-content"
    );
    assert!(actual[5] < actual[3], "anywhere did not reduce min-content");
}
