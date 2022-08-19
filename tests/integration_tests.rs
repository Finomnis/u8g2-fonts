mod util;

use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Point, Size},
    primitives::Rectangle,
};
use u8g2_fonts::{fonts, types::FontPos, Error, FontRenderer};

use util::TestDrawTarget;

#[test]
fn letters_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_courB10_tn>().render_glyph(
        'a',
        Point::new(2, 15),
        Rgb888::new(237, 28, 36),
        None,
        FontPos::default(),
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
        FontPos::default(),
        &mut display,
    );

    assert!(matches!(result, Err(Error::GlyphNotFound('☃'))))
}

#[test]
fn background_color_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>().render_glyph(
        '☃',
        Point::new(2, 15),
        Rgb888::new(237, 28, 36),
        Some(Rgb888::new(1, 1, 1)),
        FontPos::default(),
        &mut display,
    );

    assert!(matches!(result, Err(Error::BackgroundColorNotSupported)))
}

#[test]
fn get_ascent_and_descent() {
    let font = FontRenderer::new::<fonts::u8g2_font_osb21_tf>();

    assert_eq!(font.get_ascent(), 21);
    assert_eq!(font.get_descent(), -7);
}

#[test]
fn get_glyph_bounding_box() {
    let font = FontRenderer::new::<fonts::u8g2_font_osb21_tf>();

    assert_eq!(
        font.get_glyph_bounding_box(),
        Rectangle::new(Point::new(-1, -28), Size::new(31, 36))
    );
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
                    FontPos::default(),
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
                    Point::new(4, 19),
                    Rgb888::new(237, 28, 36),
                    None,
                    FontPos::default(),
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, 16);
}

#[test]
fn render_glyph_with_background_color() {
    let advance = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_glyph_background.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_10x20_mf>()
                .render_glyph(
                    'j',
                    Point::new(2, 20),
                    Rgb888::new(237, 28, 36),
                    Some(Rgb888::new(1, 1, 1)),
                    FontPos::default(),
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, 10);
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
                    FontPos::default(),
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
                    Point::new(5, 20),
                    Rgb888::new(237, 28, 36),
                    None,
                    FontPos::default(),
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, 88);
}

#[test]
fn render_text_with_background_color() {
    let advance = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_background.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_10x20_mf>()
                .render_text(
                    "Hello, W0rld!",
                    Point::new(2, 20),
                    Rgb888::new(237, 28, 36),
                    Some(Rgb888::new(1, 1, 1)),
                    FontPos::default(),
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, 130);
}

#[test]
fn render_text_with_font_pos() {
    TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_with_font_pos.png"),
        |display| {
            display
                .fill_solid(
                    &Rectangle::new(Point::new(0, 25), Size::new(display.size().width, 1)),
                    Rgb888::new(63, 72, 204),
                )
                .unwrap();

            FontRenderer::new::<fonts::u8g2_font_ncenB18_tf>()
                .render_text(
                    "Agi",
                    Point::new(5, 25),
                    Rgb888::new(237, 28, 36),
                    None,
                    FontPos::Center,
                    display,
                )
                .unwrap();

            FontRenderer::new::<fonts::u8g2_font_ncenB18_tf>()
                .render_text(
                    "Agi",
                    Point::new(55, 25),
                    Rgb888::new(237, 28, 36),
                    None,
                    FontPos::Top,
                    display,
                )
                .unwrap();

            FontRenderer::new::<fonts::u8g2_font_ncenB18_tf>()
                .render_text(
                    "Agi",
                    Point::new(105, 25),
                    Rgb888::new(237, 28, 36),
                    None,
                    FontPos::Baseline,
                    display,
                )
                .unwrap();

            FontRenderer::new::<fonts::u8g2_font_ncenB18_tf>()
                .render_text(
                    "Agi",
                    Point::new(155, 25),
                    Rgb888::new(237, 28, 36),
                    None,
                    FontPos::Bottom,
                    display,
                )
                .unwrap();
        },
    );
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
