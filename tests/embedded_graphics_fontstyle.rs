#[cfg(feature = "embedded_graphics_textstyle")]
mod alignment_grid;
#[cfg(feature = "embedded_graphics_textstyle")]
mod util;

#[cfg(feature = "embedded_graphics_textstyle")]
mod textstyle_tests {
    use super::*;
    use embedded_graphics::{
        text::{Alignment, Baseline, Text, TextStyleBuilder},
        Drawable,
    };

    use embedded_graphics_core::{
        pixelcolor::Rgb888,
        prelude::{Dimensions, Point, Size, WebColors},
        primitives::Rectangle,
    };

    use u8g2_fonts::{
        fonts,
        types::{HorizontalAlignment, VerticalPosition},
        U8g2TextStyle,
    };

    use util::TestDrawTarget;

    #[test]
    fn aligned_text() {
        let text = "☃A☃g☃i☃,☃\n☃i☃a☃g☃m☃A☃!☃";
        let character_style = U8g2TextStyle::new(fonts::u8g2_font_ncenB14_tr, Rgb888::CSS_BLUE);

        TestDrawTarget::expect_image(
            std::include_bytes!("assets/aligned_text_dimensions_embedded_graphics.png"),
            |display| {
                alignment_grid::draw(display);

                for (hpos, expected_x, expected_width) in [
                    (HorizontalAlignment::Left, 4, 68),
                    (HorizontalAlignment::Center, 122, 67),
                    (HorizontalAlignment::Right, 238, 67),
                ] {
                    for (vpos, expected_y) in [
                        (VerticalPosition::Top, 8),
                        (VerticalPosition::Center, 78),
                        (VerticalPosition::Bottom, 149),
                        (VerticalPosition::Baseline, 187),
                    ] {
                        let embedded_vpos = match vpos {
                            VerticalPosition::Baseline => Baseline::Alphabetic,
                            VerticalPosition::Top => Baseline::Top,
                            VerticalPosition::Center => Baseline::Middle,
                            VerticalPosition::Bottom => Baseline::Bottom,
                        };
                        let embedded_hpos = match hpos {
                            HorizontalAlignment::Left => Alignment::Left,
                            HorizontalAlignment::Center => Alignment::Center,
                            HorizontalAlignment::Right => Alignment::Right,
                        };

                        let text = Text::with_text_style(
                            text,
                            alignment_grid::get_pos(hpos, vpos),
                            &character_style,
                            TextStyleBuilder::new()
                                .alignment(embedded_hpos)
                                .baseline(embedded_vpos)
                                .build(),
                        );

                        let bounding_box = text.bounding_box();
                        alignment_grid::draw_bounding_box(
                            &bounding_box,
                            Rgb888::new(3, 3, 3),
                            display,
                        );

                        text.draw(display).unwrap();

                        assert_eq!(
                            bounding_box,
                            Rectangle::new(
                                Point::new(expected_x, expected_y),
                                Size::new(expected_width, 39)
                            )
                        );
                    }
                }
            },
        );
    }
}
