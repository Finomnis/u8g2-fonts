mod util;

use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Point, Size, WebColors},
    primitives::Rectangle,
};
use u8g2_fonts::{
    fonts,
    types::{HorizontalAlignment, VerticalPosition},
    DrawError, FontRenderer,
};

use util::TestDrawTarget;

#[test]
fn letters_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_courB10_tn>().render_glyph(
        'a',
        Point::new(2, 15),
        Rgb888::new(237, 28, 36),
        None,
        VerticalPosition::default(),
        &mut display,
    );

    assert!(matches!(result, Err(DrawError::GlyphNotFound('a'))))
}

#[test]
fn unicode_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>().render_glyph(
        '☃',
        Point::new(2, 15),
        Rgb888::new(237, 28, 36),
        None,
        VerticalPosition::default(),
        &mut display,
    );

    assert!(matches!(result, Err(DrawError::GlyphNotFound('☃'))))
}

#[test]
fn background_color_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>().render_glyph(
        '☃',
        Point::new(2, 15),
        Rgb888::new(237, 28, 36),
        Some(Rgb888::new(1, 1, 1)),
        VerticalPosition::default(),
        &mut display,
    );

    assert!(matches!(
        result,
        Err(DrawError::BackgroundColorNotSupported)
    ))
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
                    VerticalPosition::default(),
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
                    VerticalPosition::default(),
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
                    VerticalPosition::default(),
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
                    VerticalPosition::default(),
                    display,
                )
                .unwrap()
        });

    assert_eq!(advance, Point::new(121, 0));
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
                    VerticalPosition::default(),
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, Point::new(88, 0));
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
                    VerticalPosition::default(),
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, Point::new(130, 0));
}

#[test]
fn render_text_with_vertical_pos() {
    TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_with_vertical_pos.png"),
        |display| {
            display
                .fill_solid(
                    &Rectangle::new(Point::new(0, 25), Size::new(display.size().width, 1)),
                    Rgb888::new(63, 72, 204),
                )
                .unwrap();

            for (x_position, vertical_pos) in [
                VerticalPosition::Center,
                VerticalPosition::Top,
                VerticalPosition::Baseline,
                VerticalPosition::Bottom,
            ]
            .into_iter()
            .enumerate()
            {
                FontRenderer::new::<fonts::u8g2_font_ncenB18_tf>()
                    .render_text(
                        "Agi",
                        Point::new(5 + 50 * x_position as i32, 25),
                        Rgb888::new(237, 28, 36),
                        None,
                        vertical_pos,
                        display,
                    )
                    .unwrap();
            }
        },
    );
}

#[test]
fn render_text_with_newline() {
    let advance = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_newline.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>()
                .render_text(
                    "Hello,\nWorld!",
                    Point::new(2, 15),
                    Rgb888::new(237, 28, 36),
                    None,
                    VerticalPosition::default(),
                    display,
                )
                .unwrap()
        },
    );

    assert_eq!(advance, Point::new(65, 21));
}

#[test]
fn render_text_aligned() {
    let text = "Agi,\niagmA!";
    let font = FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>();

    fn get_x(h: HorizontalAlignment) -> i32 {
        match h {
            HorizontalAlignment::Left => 5,
            HorizontalAlignment::Center => 155,
            HorizontalAlignment::Right => 305,
        }
    }

    fn get_y(v: VerticalPosition) -> i32 {
        match v {
            VerticalPosition::Baseline => 200,
            VerticalPosition::Top => 7,
            VerticalPosition::Center => 87,
            VerticalPosition::Bottom => 167,
        }
    }

    fn get_pos(h: HorizontalAlignment, v: VerticalPosition) -> Point {
        Point::new(get_x(h), get_y(v))
    }

    TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_aligned.png"),
        |display| {
            let vertical_rect = Size::new(
                1,
                (get_y(VerticalPosition::Bottom) - get_y(VerticalPosition::Top) + 1)
                    .try_into()
                    .unwrap(),
            );
            let horizontal_rect = Size::new(
                (get_x(HorizontalAlignment::Right) - get_x(HorizontalAlignment::Left) + 1)
                    .try_into()
                    .unwrap(),
                1,
            );

            for hpos in [
                HorizontalAlignment::Left,
                HorizontalAlignment::Center,
                HorizontalAlignment::Right,
            ] {
                display
                    .fill_solid(
                        &Rectangle::new(
                            get_pos(hpos, VerticalPosition::Top),
                            Size::new(1, display.size().height).try_into().unwrap(),
                        ),
                        Rgb888::CSS_ORANGE,
                    )
                    .unwrap();
            }

            display
                .fill_solid(
                    &Rectangle::new(
                        get_pos(HorizontalAlignment::Left, VerticalPosition::Center),
                        horizontal_rect,
                    ),
                    Rgb888::CSS_ORANGE,
                )
                .unwrap();

            for hpos in [HorizontalAlignment::Left, HorizontalAlignment::Right] {
                display
                    .fill_solid(
                        &Rectangle::new(get_pos(hpos, VerticalPosition::Top), vertical_rect),
                        Rgb888::CSS_RED,
                    )
                    .unwrap();
            }

            for vpos in [VerticalPosition::Top, VerticalPosition::Bottom] {
                display
                    .fill_solid(
                        &Rectangle::new(get_pos(HorizontalAlignment::Left, vpos), horizontal_rect),
                        Rgb888::CSS_RED,
                    )
                    .unwrap();
            }

            display
                .fill_solid(
                    &Rectangle::new(
                        Point::new(0, get_y(VerticalPosition::Baseline)),
                        Size::new(display.size().width, 1),
                    ),
                    Rgb888::CSS_ORANGE,
                )
                .unwrap();

            for hpos in [
                HorizontalAlignment::Left,
                HorizontalAlignment::Center,
                HorizontalAlignment::Right,
            ] {
                for vpos in [
                    VerticalPosition::Top,
                    VerticalPosition::Center,
                    VerticalPosition::Bottom,
                ] {
                    font.render_text_aligned(
                        text,
                        get_pos(hpos, vpos),
                        Rgb888::CSS_DARK_BLUE,
                        None,
                        vpos,
                        hpos,
                        display,
                    )
                    .unwrap();
                }
            }

            for hpos in [
                HorizontalAlignment::Left,
                HorizontalAlignment::Center,
                HorizontalAlignment::Right,
            ] {
                font.render_text_aligned(
                    text,
                    get_pos(hpos, VerticalPosition::Baseline),
                    Rgb888::CSS_DARK_BLUE,
                    None,
                    VerticalPosition::Baseline,
                    hpos,
                    display,
                )
                .unwrap();
            }
        },
    );
}

#[test]
fn get_glyph_dimensions() {
    let font = FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>();

    TestDrawTarget::expect_image(
        std::include_bytes!("assets/glyph_dimensions.png"),
        |display| {
            display
                .fill_solid(
                    &Rectangle::new(Point::new(0, 15), Size::new(display.size().width, 1)),
                    Rgb888::new(63, 72, 204),
                )
                .unwrap();

            for (pos, (ch, vertical_pos)) in [
                ('j', VerticalPosition::Baseline),
                ('A', VerticalPosition::Bottom),
                ('c', VerticalPosition::Top),
                (')', VerticalPosition::Center),
            ]
            .into_iter()
            .enumerate()
            {
                let position = Point::new(2 + 10 * pos as i32, 15);
                let dim = font
                    .get_glyph_dimensions(ch, position, vertical_pos)
                    .unwrap();

                display
                    .fill_solid(&dim.bounding_box.unwrap(), Rgb888::new(2, 2, 2))
                    .unwrap();

                font.render_glyph(
                    ch,
                    position,
                    Rgb888::new(237, 28, 36),
                    None,
                    vertical_pos,
                    display,
                )
                .unwrap();
            }
        },
    );
}

#[test]
fn get_text_dimensions() {
    let font = FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>();

    TestDrawTarget::expect_image(
        std::include_bytes!("assets/text_dimensions.png"),
        |display| {
            let text = "Hello,\nWorld!";
            let position = Point::new(2, 15);
            let vertical_pos = VerticalPosition::default();

            let dim = font
                .get_text_dimensions(text, position, vertical_pos)
                .unwrap();
            assert_eq!(dim.advance, Point::new(65, 21));

            display
                .fill_solid(&dim.bounding_box.unwrap(), Rgb888::new(3, 3, 3))
                .unwrap();

            let advance = font
                .render_text(
                    text,
                    position,
                    Rgb888::new(237, 28, 36),
                    None,
                    vertical_pos,
                    display,
                )
                .unwrap();

            assert_eq!(advance, dim.advance);
        },
    );
}
