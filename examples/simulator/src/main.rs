use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

use u8g2_fonts::{fonts, FontRenderer};

const FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_unifont_t_symbols>();

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(800, 480));
    let mut window = Window::new("Text Rendering Demo", &OutputSettings::default());

    let position = Point::new(200, 200);
    Circle::with_center(position, 200)
        .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
        .draw(&mut display)?;

    Line::new(Point::new(0, 50), Point::new(40, 50))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
        .draw(&mut display)?;

    Line::new(Point::new(20, 0), Point::new(20, 100))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
        .draw(&mut display)?;

    let advance = FONT
        .render_glyph(
            ' ',
            Point::new(20, 20),
            Rgb888::CSS_YELLOW,
            None, //Some(Rgb888::CSS_DARK_GRAY),
            &mut display,
        )
        .unwrap();
    FONT.render_glyph(
        'ß',
        Point::new(20 + advance as i32, 20),
        Rgb888::CSS_YELLOW,
        None, //Some(Rgb888::CSS_DARK_GRAY),
        &mut display,
    )
    .unwrap();

    let advance = FONT
        .render_text(
            "Angh Lorem ipsum dolor sit amet. A 20%! \u{2603}",
            Point::new(20, 50),
            Rgb888::CSS_ORANGE,
            None,
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

    println!("Shutting down ...");

    Ok(())
}
