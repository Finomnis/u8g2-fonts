#![no_std]
#![no_main]

use core::convert::Infallible;

use benchmarks::*;

bench_main! {{
    let display = &mut TestDisplay::new();

    bench_run!(render, display).unwrap();

    assert_eq!(display.checksum.clone().finalize(), 1 );
}}

fn render(display: &mut TestDisplay) -> Result<RenderedDimensions, u8g2_fonts::Error<Infallible>> {
    TEST_FONT.render(
        ASCII_TEST_TEXT,
        CENTER_POINT,
        VerticalPosition::Center,
        u8g2_fonts::types::FontColor::Transparent(BinaryColor::On),
        display,
    )
}
