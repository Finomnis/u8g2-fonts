#![cfg(feature = "avr-progmem")]

#[path = "util/draw_target.rs"]
mod draw_target;

use avr_progmem::progmem;
use draw_target::TestDrawTarget;
use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_core::primitives::Rectangle;
use u8g2_fonts::{
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer, LookupError,
};
use u8g2_fonts::types::RenderedDimensions;

progmem! {
    static progmem UNICODE_FONT: [u8; include_bytes!("../src/fonts/u8g2_font_unifont_t_symbols.u8g2font").len()] =
        *include_bytes!("../src/fonts/u8g2_font_unifont_t_symbols.u8g2font");
    static progmem MONOSPACE_FONT: [u8; include_bytes!("../src/fonts/u8g2_font_10x20_mf.u8g2font").len()] =
        *include_bytes!("../src/fonts/u8g2_font_10x20_mf.u8g2font");
}

#[test]
fn render_text_unicode() {
    let font = FontRenderer::new_from_progmem(UNICODE_FONT.as_slice());

    let dimensions = TestDrawTarget::expect_image(
        include_bytes!("assets/render_text_unicode.png"),
        |display| {
            font.render(
                "Snowman: ☃",
                Point::new(5, 20),
                VerticalPosition::default(),
                FontColor::Transparent(Rgb888::new(237, 28, 36)),
                display,
            )
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
        include_bytes!("assets/render_text_background.png"),
        |display| {
            FontRenderer::new_from_progmem(MONOSPACE_FONT.as_slice())
                .render(
                    "Hello, W0rld!",
                    Point::new(2, 20),
                    VerticalPosition::default(),
                    FontColor::WithBackground {
                        fg: Rgb888::new(237, 28, 36),
                        bg: Rgb888::new(1, 1, 1),
                    },
                    display,
                )
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
fn progmem_alignment_and_metrics_match_slice() {
    let flash = FontRenderer::new_from_progmem(UNICODE_FONT.as_slice()).with_line_height(24);
    let slice = FontRenderer::new_from_raw_data(include_bytes!(
        "../src/fonts/u8g2_font_unifont_t_symbols.u8g2font"
    ))
    .with_line_height(24);

    assert_eq!(flash.get_ascent(), slice.get_ascent());
    assert_eq!(flash.get_descent(), slice.get_descent());
    assert_eq!(
        flash.get_default_line_height(),
        slice.get_default_line_height()
    );

    let position = Point::new(64, 32);
    assert_eq!(
        flash.get_font_bounding_box(VerticalPosition::Center),
        slice.get_font_bounding_box(VerticalPosition::Center)
    );

    let mut expected = TestDrawTarget::new(Size::new(128, 64));
    let mut actual = TestDrawTarget::new(Size::new(128, 64));
    for (font, display) in [(&slice, &mut expected), (&flash.clone(), &mut actual)] {
        let dimensions = font
            .render_aligned(
                format_args!("aA☃\n{}", 42),
                position,
                VerticalPosition::Center,
                HorizontalAlignment::Center,
                FontColor::Transparent(Rgb888::CSS_RED),
                display,
            )
            .unwrap();
        assert_eq!(
            dimensions,
            slice
                .get_rendered_dimensions_aligned(
                    "aA☃\n42",
                    position,
                    VerticalPosition::Center,
                    HorizontalAlignment::Center,
                )
                .unwrap()
        );
    }
    assert_pixels_match(&actual, &expected);
}

fn assert_pixels_match(actual: &TestDrawTarget, expected: &TestDrawTarget) {
    for y in 0..actual.size().height as i32 {
        for x in 0..actual.size().width as i32 {
            let position = Point::new(x, y);
            assert_eq!(actual.get_pixel(position), expected.get_pixel(position));
        }
    }
}

#[cfg(feature = "embedded_graphics_textstyle")]
#[test]
fn textstyle_reads_progmem() {
    use embedded_graphics::text::{renderer::TextRenderer, Baseline};
    use u8g2_fonts::U8g2TextStyle;

    let style = U8g2TextStyle::new_from_progmem(UNICODE_FONT.as_slice(), Rgb888::new(237, 28, 36));
    TestDrawTarget::expect_image(
        include_bytes!("assets/render_text_unicode.png"),
        |display| {
            style
                .draw_string(
                    "Snowman: ☃",
                    Point::new(5, 19),
                    Baseline::Alphabetic,
                    display,
                )
                .unwrap()
        },
    );
}
