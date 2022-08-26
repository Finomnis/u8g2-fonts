//! This example shows dynamically rendered text.

extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, StrokeAlignment},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use u8g2_fonts::{
    fonts,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

const MOUSE_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_haxrcorp4089_t_cyrillic>();
const TEXT_POS_X: Point = Point::new(6, 62);
const TEXT_POS_Y: Point = Point::new(124, 62);

fn update_mouse_text(pos: Point, prev: Point, display: &mut SimulatorDisplay<BinaryColor>) {
    MOUSE_FONT
        .render(
            format_args!("x: {}", prev.x),
            TEXT_POS_X,
            VerticalPosition::Baseline,
            FontColor::Transparent(BinaryColor::Off),
            display,
        )
        .unwrap();

    MOUSE_FONT
        .render(
            format_args!("x: {}", pos.x),
            TEXT_POS_X,
            VerticalPosition::Baseline,
            FontColor::Transparent(BinaryColor::On),
            display,
        )
        .unwrap();

    MOUSE_FONT
        .render_aligned(
            format_args!("y: {}", prev.y),
            TEXT_POS_Y,
            VerticalPosition::Baseline,
            HorizontalAlignment::Right,
            FontColor::Transparent(BinaryColor::Off),
            display,
        )
        .unwrap();

    MOUSE_FONT
        .render_aligned(
            format_args!("y: {}", pos.y),
            TEXT_POS_Y,
            VerticalPosition::Baseline,
            HorizontalAlignment::Right,
            FontColor::Transparent(BinaryColor::On),
            display,
        )
        .unwrap();
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(130, 70));

    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

    // Draw a 3px wide outline around the display.
    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)?;

    let mut mouse_pos = Point::new(0, 0);
    update_mouse_text(Point::new(0, 0), mouse_pos, &mut display);

    // Create and display the window
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Dynamic text content", &output_settings);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseMove { point } => {
                    let prev = mouse_pos;
                    mouse_pos = point;
                    update_mouse_text(mouse_pos, prev, &mut display);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
