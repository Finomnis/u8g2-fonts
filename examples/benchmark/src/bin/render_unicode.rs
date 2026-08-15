#![no_std]
#![no_main]

use benchmarks::*;

bench_main! {{
    let display = TestDisplay::new();
    let content = UNICODE_TEST_TEXT;

    let result = bench_run!(render, display, content);

    assert_eq!(result, 0x1d899d9a);
}}

fn render(mut display: TestDisplay, content: &str) -> u32 {
    TEST_FONT
        .render(
            content,
            CENTER_POINT,
            VerticalPosition::Center,
            u8g2_fonts::types::FontColor::Transparent(BinaryColor::On),
            &mut display,
        )
        .unwrap();

    display.checksum.finalize()
}
