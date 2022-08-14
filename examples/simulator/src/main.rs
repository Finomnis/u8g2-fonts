use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(800, 480));
    let mut window = Window::new("Click to move circle", &OutputSettings::default());

    let position = Point::new(200, 200);
    Circle::with_center(position, 200)
        .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
        .draw(&mut display)?;

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
