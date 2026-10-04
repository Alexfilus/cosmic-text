use cosmic_text::*;

pub fn fonts() -> FontSystem {
    let mut db = fontdb::Database::new();
    db.load_font_data(include_bytes!("../../fonts/ObscuraVFTest.ttf").to_vec());
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}

pub fn scenario(fs: &mut FontSystem) -> Vec<f32> {
    let mut face =
        rustybuzz::Face::from_slice(include_bytes!("../../fonts/ObscuraVFTest.ttf"), 0).unwrap();
    face.set_variations(&[
        rustybuzz::Variation {
            tag: rustybuzz::ttf_parser::Tag::from_bytes(b"wght"),
            value: 700.0,
        },
        rustybuzz::Variation {
            tag: rustybuzz::ttf_parser::Tag::from_bytes(b"opsz"),
            value: 32.0,
        },
    ]);
    let mut input = rustybuzz::UnicodeBuffer::new();
    input.push_str("A A");
    input.guess_segment_properties();
    let glyphs = rustybuzz::shape(&face, &[], input);
    std::hint::black_box(fs);
    glyphs
        .glyph_positions()
        .iter()
        .map(|p| p.x_advance as f32)
        .collect()
}

pub fn verify() {
    let mut fs = fonts();
    let actual = scenario(&mut fs);
    assert_eq!(actual.len(), 3);
    assert!(
        actual[1] > 0.0,
        "space lost its advance under two variation axes: {actual:?}"
    );
    assert_eq!(actual[0], actual[2]);
}
