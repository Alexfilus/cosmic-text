use cosmic_text::*;
pub fn fonts() -> FontSystem {
    let mut db = fontdb::Database::new();
    db.load_font_data(include_bytes!("../../fonts/NotoSansArabic.ttf").to_vec());
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}
pub fn scenario(fs: &mut FontSystem) -> Vec<f32> {
    [0.0, 0.125]
        .into_iter()
        .map(|spacing| {
            let attrs = Attrs::new()
                .family(Family::Name("Noto Sans Arabic"))
                .letter_spacing(spacing);
            let line = ShapeLine::new(
                fs,
                "خالصة كلمة",
                &AttrsList::new(&attrs),
                Shaping::Advanced,
                8,
            );
            line.layout(32.0, None, Wrap::None, None, None)[0].w
        })
        .collect()
}
pub fn verify() {
    let actual = scenario(&mut fonts());
    assert!(
        (actual[1] - actual[0] - 4.0).abs() < 0.001,
        "only the space should get tracking: {actual:?}"
    );
}
