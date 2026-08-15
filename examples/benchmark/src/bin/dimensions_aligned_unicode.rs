#![no_std]
#![no_main]

use benchmarks::*;

bench_main! {{
    let content = UNICODE_TEST_TEXT;

    let result = bench_run!(dimensions, content);

    assert_eq!(result, Some(Rectangle {
        top_left: Point::new(210,226),
        size: Size::new(220,29),
    }));
}}

fn dimensions(content: &str) -> Option<Rectangle> {
    TEST_FONT
        .get_rendered_dimensions_aligned(
            content,
            CENTER_POINT,
            VerticalPosition::Center,
            HorizontalAlignment::Center,
        )
        .unwrap()
}
