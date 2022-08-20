use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

use u8g2_fonts::{fonts, types::FontPos, FontRenderer};

const FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_osb21_tf>();

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
        .render_text(
            "Angh Lorem ipsum dolor sit\namet. A 20%!",
            Point::new(20, 50),
            Rgb888::CSS_ORANGE,
            None,
            FontPos::default(),
            &mut display,
        )
        .unwrap();

    let bounding_box = FONT.get_glyph_bounding_box();
    println!("BBox: {:?}", bounding_box);
    bounding_box
        .translate(Point::new(20, 50))
        .offset(1)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_ORANGE, 1))
        .draw(&mut display)
        .unwrap();

    println!("Advance: {}", advance);

    println!("{:#?}", FONT);

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
