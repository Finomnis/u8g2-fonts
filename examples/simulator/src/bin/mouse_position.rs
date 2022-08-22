//! This example shows dynamically rendered text.

extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb888},
    prelude::*,
    primitives::{Circle, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
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

fn update_mouse_text(pos: Point, display: &mut SimulatorDisplay<BinaryColor>) {
    static prev: Point = Point { x: 0, y: 0 };

    MOUSE_FONT
        .render_text(
            "x: 69",
            TEXT_POS_X,
            FontColor::Transparent(BinaryColor::Off),
            VerticalPosition::Baseline,
            display,
        )
        .unwrap();

    MOUSE_FONT
        .render_text(
            "x: 69",
            TEXT_POS_X,
            FontColor::Transparent(BinaryColor::On),
            VerticalPosition::Baseline,
            display,
        )
        .unwrap();

    MOUSE_FONT
        .render_text_aligned(
            "y: 42",
            TEXT_POS_Y,
            FontColor::Transparent(BinaryColor::Off),
            VerticalPosition::Baseline,
            HorizontalAlignment::Right,
            display,
        )
        .unwrap();

    MOUSE_FONT
        .render_text_aligned(
            "y: 42",
            TEXT_POS_Y,
            FontColor::Transparent(BinaryColor::On),
            VerticalPosition::Baseline,
            HorizontalAlignment::Right,
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

    update_mouse_text(Point::new(0, 0), &mut display);

    // Create and display the window
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Click to move circle", &output_settings);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseMove { point } => {
                    println!("{:?}", point);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
