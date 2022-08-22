use simulator::*;

use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyleBuilder};

use u8g2_fonts::types::HorizontalAlignment;
use u8g2_fonts::{
    fonts,
    types::{FontColor, VerticalPosition},
    FontRenderer,
};

fn main() -> anyhow::Result<()> {
    let mut display = init_display(425, 101);

    let center = display.bounding_box().center();

    let text = "Hello, Rust World!\nU8g2 meets embedded-graphics!";

    let line_style = PrimitiveStyleBuilder::new()
        .stroke_color(COLOR::CSS_BLUE)
        .stroke_width(2)
        .fill_color(COLOR::BLACK)
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
        .get_aligned_text_dimensions(
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

    font.render_text_aligned(
        text,
        center,
        FontColor::Transparent(COLOR::CSS_ORANGE),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        &mut display,
    )?;

    show(display)
}
