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
    for weight in [100.0, 900.0] {
        let attrs = Attrs::new()
            .family(Family::Name("Absent requested family"))
            .font_weight_axis(weight)
            .font_optical_size(32.0);
        let lines = shape(fs, "Am am", attrs, Wrap::None, 1000.0);
        assert!(lines
            .iter()
            .flat_map(|l| &l.glyphs)
            .all(|g| g.font_is_variable));
        result.push(lines[0].w);
    }
    result
}

pub fn verify() {
    let mut fs = fonts();
    let actual = scenario(&mut fs);
    assert!(
        actual[1] > actual[0] + 1.0,
        "fallback axes did not change advances: {actual:?}"
    );
}
