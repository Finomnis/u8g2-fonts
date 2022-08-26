//! This is the banner from the README.

use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::Rectangle};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

use u8g2_fonts::types::HorizontalAlignment;
use u8g2_fonts::{
    fonts,
    types::{FontColor, VerticalPosition},
    FontRenderer,
};

fn main() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(400, 150));
    display.clear(Rgb888::CSS_LIGHT_GRAY).unwrap();

    let text = "Agi";

    let font_large = FontRenderer::new::<fonts::u8g2_font_logisoso32_tf>();
    let color_large = Rgb888::BLACK;
    let color_line = Rgb888::CSS_FOREST_GREEN;
    let font_small = FontRenderer::new::<fonts::u8g2_font_t0_13b_tf>();
    let color_small = Rgb888::BLACK;

    let baseline = (display.bounding_box().size.height * 2 / 5) as i32;

    display
        .fill_solid(
            &Rectangle::new(
                Point::new(0, baseline - 1),
                Size::new(display.size().width, 3),
            ),
            color_line,
        )
        .unwrap();

    let text_width = font_large
        .get_rendered_dimensions(text, Point::new(0, 0), VerticalPosition::Baseline)
        .unwrap()
        .bounding_box
        .unwrap()
        .size
        .width as i32;

    let total_whitespace = display.size().width as i32 - 4 * text_width;
    let single_whitespace = total_whitespace / 5;
    let x_start = single_whitespace + text_width / 2;
    let x_steps = single_whitespace + text_width;

    for (id, (name, pos)) in [
        ("Baseline", VerticalPosition::Baseline),
        ("Top", VerticalPosition::Top),
        ("Center", VerticalPosition::Center),
        ("Bottom", VerticalPosition::Bottom),
    ]
    .into_iter()
    .enumerate()
    {
        let x = x_start + id as i32 * x_steps;
        font_large
            .render_aligned(
                text,
                Point::new(x, baseline),
                pos,
                HorizontalAlignment::Center,
                FontColor::Transparent(color_large),
                &mut display,
            )
            .unwrap();

        font_small
            .render_aligned(
                name,
                Point::new(x, 2 * baseline),
                VerticalPosition::Top,
                HorizontalAlignment::Center,
                FontColor::Transparent(color_small),
                &mut display,
            )
            .unwrap();
    }

    println!(
        "data:image/png;base64,{}",
        display
            .to_rgb_output_image(&OutputSettings::default())
            .to_base64_png()
            .unwrap()
    );

    Window::new(
        "Docs image for vertical positioning",
        &OutputSettings::default(),
    )
    .show_static(&display);

    Ok(())
}
