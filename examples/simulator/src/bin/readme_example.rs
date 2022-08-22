use simulator::*;

use embedded_graphics::prelude::*;

use u8g2_fonts::{
    fonts,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

fn main() -> anyhow::Result<()> {
    let mut display = init_display(150, 50);

    let font = FontRenderer::new::<fonts::u8g2_font_lubI14_tf>();

    font.render_text_aligned(
        "Hello, World!",
        display.bounding_box().center(),
        FontColor::Transparent(COLOR::GREEN),
        VerticalPosition::Center,
        HorizontalAlignment::Center,
        &mut display,
    )?;

    show(display)
}
