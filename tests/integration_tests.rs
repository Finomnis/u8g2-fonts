mod util;

use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, OriginDimensions, Point, Size, WebColors},
    primitives::Rectangle,
};
use u8g2_fonts::{
    fonts,
    types::{HorizontalAlignment, RenderedDimensions, VerticalPosition},
    Error, FontRenderer,
};

use util::TestDrawTarget;

#[test]
fn letters_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_courB10_tn>()
        .render_glyph('a')
        .position(Point::new(2, 15), VerticalPosition::default())
        .color(Rgb888::new(237, 28, 36))
        .draw(&mut display);

    assert!(matches!(result, Err(Error::GlyphNotFound('a'))))
}

#[test]
fn unicode_not_supported() {
    let mut display = TestDrawTarget::new(Size::new(1, 1));

    let result = FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>()
        .render_glyph('☃')
        .position(Point::new(2, 15), VerticalPosition::default())
        .color(Rgb888::new(237, 28, 36))
        .draw(&mut display);

    assert!(matches!(result, Err(Error::GlyphNotFound('☃'))))
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
    let dimensions =
        TestDrawTarget::expect_image(std::include_bytes!("assets/render_glyph.png"), |display| {
            FontRenderer::new::<fonts::u8g2_font_lubBI08_tf>()
                .render_glyph('j')
                .position(Point::new(2, 15), VerticalPosition::default())
                .color(Rgb888::new(237, 28, 36))
                .draw(display)
                .unwrap()
        });

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(4, 0),
            bounding_box: Some(Rectangle::new(Point::new(2, 6), Size::new(6, 11)))
        }
    );
}

#[test]
fn render_glyph_unicode() {
    let dimensions = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_glyph_unicode.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_unifont_t_symbols>()
                .render_glyph('☃')
                .position(Point::new(4, 19), VerticalPosition::default())
                .color(Rgb888::new(237, 28, 36))
                .draw(display)
                .unwrap()
        },
    );

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(16, 0),
            bounding_box: Some(Rectangle::new(Point::new(4, 5), Size::new(16, 16)))
        }
    );
}

#[test]
fn render_glyph_with_background_color() {
    let dimensions = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_glyph_background.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_10x20_mf>()
                .render_glyph('j')
                .position(Point::new(2, 20), VerticalPosition::default())
                .color(Rgb888::new(237, 28, 36))
                .background(Rgb888::new(1, 1, 1))
                .unwrap()
                .draw(display)
                .unwrap()
        },
    );

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(10, 0),
            bounding_box: Some(Rectangle::new(Point::new(2, 4), Size::new(10, 20)))
        }
    );
}

#[test]
fn render_text() {
    let dimensions =
        TestDrawTarget::expect_image(std::include_bytes!("assets/render_text.png"), |display| {
            FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>()
                .render_text("Hello World!")
                .position(Point::new(2, 15), VerticalPosition::default())
                .color(Rgb888::new(237, 28, 36))
                .draw(display)
                .unwrap()
        });

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(121, 0),
            bounding_box: Some(Rectangle::new(Point::new(2, 1), Size::new(120, 14)))
        }
    );
}

#[test]
fn render_text_unicode() {
    let dimensions = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_unicode.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_unifont_t_symbols>()
                .render_text("Snowman: ☃")
                .position(Point::new(5, 20), VerticalPosition::default())
                .color(Rgb888::new(237, 28, 36))
                .draw(display)
                .unwrap()
        },
    );

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(88, 0),
            bounding_box: Some(Rectangle::new(Point::new(6, 6), Size::new(87, 16)))
        }
    );
}

#[test]
fn render_text_with_background_color() {
    let dimensions = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_background.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_10x20_mf>()
                .render_text("Hello, W0rld!")
                .position(Point::new(2, 20), VerticalPosition::default())
                .color(Rgb888::new(237, 28, 36))
                .background(Rgb888::new(1, 1, 1))
                .unwrap()
                .draw(display)
                .unwrap()
        },
    );

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(130, 0),
            bounding_box: Some(Rectangle::new(Point::new(2, 4), Size::new(130, 20)))
        }
    );
}

#[test]
fn render_text_with_vertical_pos() {
    let font = FontRenderer::new::<fonts::u8g2_font_ncenB18_tf>();
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
                let bounding_box = font
                    .render_text("Agi")
                    .position(Point::new(5 + 50 * x_position as i32, 25), vertical_pos)
                    .color(Rgb888::new(237, 28, 36))
                    .draw(display)
                    .unwrap()
                    .bounding_box;

                let expected_bounding_box_y = match vertical_pos {
                    VerticalPosition::Baseline => -18,
                    VerticalPosition::Top => 1,
                    VerticalPosition::Center => -11,
                    VerticalPosition::Bottom => -23,
                };

                assert_eq!(
                    bounding_box,
                    Some(Rectangle::new(
                        Point::new(5 + 50 * x_position as i32, 25 + expected_bounding_box_y),
                        Size::new(42, 23)
                    ))
                );
            }
        },
    );
}

#[test]
fn get_text_dimensions_with_vertical_pos() {
    let font = FontRenderer::new::<fonts::u8g2_font_ncenB18_tf>();

    for (x_position, vertical_pos) in [
        VerticalPosition::Center,
        VerticalPosition::Top,
        VerticalPosition::Baseline,
        VerticalPosition::Bottom,
    ]
    .into_iter()
    .enumerate()
    {
        let bounding_box = font
            .render_text("Agi")
            .position(Point::new(5 + 50 * x_position as i32, 25), vertical_pos)
            .compute_dimensions()
            .unwrap()
            .bounding_box;

        let expected_bounding_box_y = match vertical_pos {
            VerticalPosition::Baseline => -18,
            VerticalPosition::Top => 1,
            VerticalPosition::Center => -11,
            VerticalPosition::Bottom => -23,
        };

        assert_eq!(
            bounding_box,
            Some(Rectangle::new(
                Point::new(5 + 50 * x_position as i32, 25 + expected_bounding_box_y),
                Size::new(42, 23)
            ))
        );
    }
}

#[test]
fn render_text_with_newline() {
    let dimensions = TestDrawTarget::expect_image(
        std::include_bytes!("assets/render_text_newline.png"),
        |display| {
            FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>()
                .render_text("Hello,\nWorld!")
                .position(Point::new(2, 15), VerticalPosition::default())
                .color(Rgb888::new(237, 28, 36))
                .draw(display)
                .unwrap()
        },
    );

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(65, 21),
            bounding_box: Some(Rectangle::new(Point::new(1, 1), Size::new(65, 35)))
        }
    );
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

            for (hpos, expected_x, expected_width) in [
                (HorizontalAlignment::Left, 5, 68),
                (HorizontalAlignment::Center, 122, 67),
                (HorizontalAlignment::Right, 238, 67),
            ] {
                for (vpos, expected_y) in [
                    (VerticalPosition::Top, 8),
                    (VerticalPosition::Center, 68),
                    (VerticalPosition::Bottom, 128),
                    (VerticalPosition::Baseline, 186),
                ] {
                    let bounding_box = font
                        .render_text(text)
                        .position(get_pos(hpos, vpos), vpos)
                        .alignment(hpos)
                        .color(Rgb888::CSS_DARK_BLUE)
                        .draw(display)
                        .unwrap();

                    assert_eq!(
                        bounding_box,
                        Some(Rectangle::new(
                            Point::new(expected_x, expected_y),
                            Size::new(expected_width, 39)
                        ))
                    );
                }
            }
        },
    );
}

#[test]
fn render_args() {
    let font = FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>();
    let dimensions =
        TestDrawTarget::expect_image(std::include_bytes!("assets/render_args.png"), |display| {
            font.render_args(format_args!("!{}?", 42.69))
                .position(Point::new(2, 15), VerticalPosition::Baseline)
                .color(Rgb888::new(237, 28, 36))
                .draw(display)
                .unwrap()
        });

    assert_eq!(
        dimensions,
        RenderedDimensions {
            advance: Point::new(65, 0),
            bounding_box: Some(Rectangle::new(Point::new(3, 1), Size::new(63, 14)))
        }
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
                let glyph_drawer = font
                    .render_glyph(ch)
                    .position(position, vertical_pos)
                    .color(Rgb888::new(237, 28, 36));

                let dim = glyph_drawer.compute_dimensions().unwrap();

                display
                    .fill_solid(&dim.bounding_box.unwrap(), Rgb888::new(2, 2, 2))
                    .unwrap();

                let rendered_dim = glyph_drawer.draw(display).unwrap();

                assert_eq!(dim, rendered_dim);
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

            let text_drawer = font
                .render_text(text)
                .position(position, vertical_pos)
                .color(Rgb888::new(237, 28, 36));

            let dim = text_drawer.compute_dimensions().unwrap();
            assert_eq!(dim.advance, Point::new(65, 21));

            display
                .fill_solid(&dim.bounding_box.unwrap(), Rgb888::new(3, 3, 3))
                .unwrap();

            let rendered_dim = text_drawer.draw(display).unwrap();

            assert_eq!(dim, rendered_dim);
        },
    );
}

#[test]
fn get_aligned_text_dimensions() {
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
        std::include_bytes!("assets/aligned_text_dimensions.png"),
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
                    VerticalPosition::Baseline,
                ] {
                    let text_drawer = font
                        .render_text(text)
                        .position(get_pos(hpos, vpos), vpos)
                        .alignment(hpos)
                        .color(Rgb888::CSS_BLUE);

                    let bounding_box = text_drawer.compute_dimensions().unwrap().unwrap();

                    display
                        .fill_solid(&bounding_box, Rgb888::new(3, 3, 3))
                        .unwrap();

                    let rendered_bounding_box = text_drawer.draw(display).unwrap().unwrap();

                    assert_eq!(bounding_box, rendered_bounding_box);
                }
            }
        },
    );
}
