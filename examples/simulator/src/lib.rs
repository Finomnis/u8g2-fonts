use std::{thread, time::Duration};

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

pub fn init_display(width: u32, height: u32) -> SimulatorDisplay<Rgb888> {
    SimulatorDisplay::new(Size::new(width, height))
}

pub fn show(display: SimulatorDisplay<Rgb888>) -> anyhow::Result<()> {
    let mut window = Window::new(
        "U8g2 Fonts Demo for embedded-graphics",
        &OutputSettings::default(),
    );

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(3));
    }

    Ok(())
}

pub type COLOR = Rgb888;
