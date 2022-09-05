#[cfg(feature = "embedded_graphics_textstyle")]
mod alignment_grid;
#[cfg(feature = "embedded_graphics_textstyle")]
mod util;

#[cfg(feature = "embedded_graphics_textstyle")]
mod textstyle_tests {
    use super::*;
    use embedded_graphics::{
        text::{
            renderer::{CharacterStyle, TextRenderer},
            Alignment, Baseline, Text, TextStyleBuilder,
        },
        Drawable,
    };

    use embedded_graphics_core::{
        pixelcolor::Rgb888,
        prelude::{Dimensions, DrawTarget, Point, Size, WebColors},
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

    #[test]
    fn render_text_with_background_color() {
        let dimensions = TestDrawTarget::expect_image(
            std::include_bytes!("assets/render_text_background.png"),
            |display| {
                let mut character_style =
                    U8g2TextStyle::new(fonts::u8g2_font_10x20_mf, Rgb888::new(255, 255, 254));

                character_style.set_text_color(Some(Rgb888::new(237, 28, 36)));
                character_style.set_background_color(Some(Rgb888::new(1, 1, 1)));

                let text = Text::new("Hello, W0rld!", Point::new(2, 19), character_style);

                text.draw(display).unwrap();

                text.bounding_box()
            },
        );

        assert_eq!(
            dimensions,
            Rectangle::new(Point::new(2, 4), Size::new(130, 20))
        );
    }

    #[test]
    fn render_text_without_text_color() {
        let dimensions =
            TestDrawTarget::expect_image(std::include_bytes!("assets/empty.png"), |display| {
                let mut character_style =
                    U8g2TextStyle::new(fonts::u8g2_font_10x20_mf, Rgb888::new(255, 255, 254));

                character_style.set_text_color(None);

                let text = Text::new("Hello, W0rld!", Point::new(2, 19), character_style);

                text.draw(display).unwrap();

                text.bounding_box()
            });

        assert_eq!(
            dimensions,
            Rectangle::new(Point::new(2, 4), Size::new(130, 20))
        );
    }

    #[test]
    fn render_whitespace_with_background() {
        let foreground_color = Rgb888::new(237, 28, 36);
        let background_color = Rgb888::new(4, 4, 4);
        let whitespace_background_color = Rgb888::new(100, 150, 50);

        let baseline = Baseline::Top;

        let mut character_style = U8g2TextStyle::new(fonts::u8g2_font_10x20_mf, foreground_color);

        let pos_start = Point::new(3, 5);

        // Wrapper function to do the whitespace drawing, to force the &U8g2TextStyle impl.
        // A direct call would automatically dereference.
        fn draw_whitespace<T: TextRenderer, D>(
            text_renderer: T,
            width: u32,
            position: Point,
            baseline: Baseline,
            target: &mut D,
        ) -> Result<Point, D::Error>
        where
            D: DrawTarget<Color = T::Color>,
        {
            T::draw_whitespace(&text_renderer, width, position, baseline, target)
        }

        let pos = TestDrawTarget::expect_image(
            std::include_bytes!("assets/render_whitespace_embedded_graphics.png"),
            |display| {
                let mut pos = pos_start;

                character_style.set_background_color(Some(background_color));

                pos = character_style
                    .draw_string("Ab", pos, baseline, display)
                    .unwrap();

                character_style.set_background_color(Some(whitespace_background_color));

                pos = draw_whitespace(&character_style, 5, pos, baseline, display).unwrap();

                character_style.set_background_color(Some(background_color));

                character_style
                    .draw_string("cd", pos, baseline, display)
                    .unwrap()
            },
        );

        assert_eq!(pos, pos_start + Point::new(45, 0));
    }

    #[test]
    fn render_whitespace_with_zero_width() {
        let foreground_color = Rgb888::new(237, 28, 36);
        let whitespace_background_color = Rgb888::new(100, 150, 50);

        let baseline = Baseline::Top;

        let mut character_style = U8g2TextStyle::new(fonts::u8g2_font_10x20_mf, foreground_color);

        let pos_start = Point::new(3, 5);

        let pos =
            TestDrawTarget::expect_image(std::include_bytes!("assets/empty.png"), |display| {
                character_style.set_background_color(Some(whitespace_background_color));

                character_style
                    .draw_whitespace(0, pos_start, baseline, display)
                    .unwrap()
            });

        assert_eq!(pos, pos_start);
    }

    #[test]
    fn render_whitespace_with_non_monospace_font() {
        let foreground_color = Rgb888::new(237, 28, 36);
        let whitespace_background_color = Rgb888::new(100, 150, 50);

        let baseline = Baseline::Top;

        let mut character_style = U8g2TextStyle::new(fonts::u8g2_font_ncenB14_tr, foreground_color);

        let pos_start = Point::new(3, 5);

        let pos =
            TestDrawTarget::expect_image(std::include_bytes!("assets/empty.png"), |display| {
                character_style.set_background_color(Some(whitespace_background_color));

                character_style
                    .draw_whitespace(5, pos_start, baseline, display)
                    .unwrap()
            });

        assert_eq!(pos, pos_start + Point::new(5, 0));
    }

    #[test]
    fn render_whitespace_without_background_color() {
        let foreground_color = Rgb888::new(237, 28, 36);

        let baseline = Baseline::Top;

        let mut character_style = U8g2TextStyle::new(fonts::u8g2_font_10x20_mf, foreground_color);

        let pos_start = Point::new(3, 5);

        let pos =
            TestDrawTarget::expect_image(std::include_bytes!("assets/empty.png"), |display| {
                character_style.set_background_color(None);

                character_style
                    .draw_whitespace(5, pos_start, baseline, display)
                    .unwrap()
            });

        assert_eq!(pos, pos_start + Point::new(5, 0));
    }
}
