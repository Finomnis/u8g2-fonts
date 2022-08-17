use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

use u8g2_fonts::{create_font_renderer, fonts, FontRenderer};

const FONT: FontRenderer = create_font_renderer::<fonts::u8g2_font_lubBI14_tf>();

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(800, 480));
    let mut window = Window::new("Text rendering demo", &OutputSettings::default());

    let position = Point::new(200, 200);
    Circle::with_center(position, 200)
        .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
        .draw(&mut display)?;

    Line::new(Point::new(0, 20), Point::new(40, 20))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 1))
        .draw(&mut display)?;

    Line::new(Point::new(20, 0), Point::new(20, 40))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 1))
        .draw(&mut display)?;

    let advance = FONT
        .render_glyph(
            'ß',
            Point::new(20, 20),
            Rgb888::CSS_DARK_BLUE,
            Some(Rgb888::CSS_DARK_GRAY),
            &mut display,
        )
        .unwrap();
    let advance = FONT
        .render_glyph(
            'ß',
            Point::new(20 + advance as i32, 20),
            Rgb888::CSS_DARK_BLUE,
            Some(Rgb888::CSS_DARK_GRAY),
            &mut display,
        )
        .unwrap();

    println!("Advance: {}", advance);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}
