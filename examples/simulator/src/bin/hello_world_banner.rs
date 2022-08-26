//! This is the banner from the README.

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

use u8g2_fonts::types::HorizontalAlignment;
use u8g2_fonts::{
    fonts,
    types::{FontColor, VerticalPosition},
    FontRenderer,
};

fn main() -> anyhow::Result<()> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(425, 101));

    let center = display.bounding_box().center();

    let text = "Hello, Rust World!\nU8g2 meets embedded-graphics!";

    let line_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::CSS_BLUE)
        .stroke_width(2)
        .fill_color(Rgb888::BLACK)
        .build();

    for (start, end) in [
        (
            Point::new(center.x, 0),
            Point::new(center.x, display.size().height as i32),
        ),
        (
            Point::new(0, center.y),
            Point::new(display.size().width as i32, center.y),
        ),
    ] {
        Line::new(start, end)
            .into_styled(line_style)
            .draw(&mut display)?;
    }

    let font = FontRenderer::new::<fonts::u8g2_font_lubI14_tf>();

    let font_bounding_box = font
        .get_rendered_dimensions_aligned(
            text,
            center,
            VerticalPosition::Center,
            HorizontalAlignment::Center,
        )?
        .unwrap();

    font_bounding_box
        .offset(8)
        .into_styled(line_style)
        .draw(&mut display)?;

    font.render_aligned(
        text,
        center,
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        FontColor::Transparent(Rgb888::CSS_ORANGE),
        &mut display,
    )?;

    Window::new(
        "U8g2 Fonts Demo for embedded-graphics",
        &OutputSettings::default(),
    )
    .show_static(&display);

    Ok(())
}
