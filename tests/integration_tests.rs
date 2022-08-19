mod util;

use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{Point, Size},
};
use u8g2_fonts::{fonts, Error, FontRenderer};

use util::TestDrawTarget;

#[test]
fn letters_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_courB10_tn>().render_glyph(
        'a',
        Point::new(2, 15),
        Rgb888::new(237, 28, 36),
        None,
        &mut display,
    );

    assert!(matches!(result, Err(Error::GlyphNotFound('a'))))
}

#[test]
fn unicode_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>().render_glyph(
        '☃',
        Point::new(2, 15),
        Rgb888::new(237, 28, 36),
        None,
        &mut display,
    );

    assert!(matches!(result, Err(Error::GlyphNotFound('☃'))))
}

#[test]
fn render_glyph() {
    let advance =
        TestDrawTarget::expect_image(std::include_bytes!("assets/render_glyph.png"), |display| {
            FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>()
                .render_glyph(
                    'j',
                    Point::new(2, 15),
                    Rgb888::new(237, 28, 36),
                    None,
                    display,
                )
                .unwrap()
        });

    assert_eq!(advance, 4);
}

#[test]
fn render_glyph_unicode() {
    let advance = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_glyph_unicode.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_unifont_t_symbols>()
                .render_glyph(
                    '☃',
                    Point::new(2, 15),
                    Rgb888::new(237, 28, 36),
                    None,
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, 4);
}

#[test]
fn render_text() {
    let advance =
        TestDrawTarget::expect_image(std::include_bytes!("assets/render_text.png"), |display| {
            FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>()
                .render_text(
                    "Hello World!",
                    Point::new(2, 15),
                    Rgb888::new(237, 28, 36),
                    None,
                    display,
                )
                .unwrap()
        });

    assert_eq!(advance, 121);
}

#[test]
fn render_text_unicode() {
    let advance = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_unicode.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_unifont_t_symbols>()
                .render_text(
                    "Snowman: ☃",
                    Point::new(2, 15),
                    Rgb888::new(237, 28, 36),
                    None,
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, 4);
}

#[test]
fn dimensions_text() {
    // let mut display = TestDrawTarget::expect_image(std::include_bytes!("assets/boxed.png"));

    // let font = FontRenderer::new::<fonts::u8g2_font_helvR08_tr>();
    // font.render_text("Cage", Point::new(5, 11), Rgb888::GREEN, None, &mut display)
    //     .unwrap();
}

/*
Missing tests:
    - Background
    - Glyph
    - Dimensions
        - text, glyph
    - Unicode
    - getAscent/Descent/height/width/etc
*/
