#![no_std]
#![no_main]

use benchmarks::*;

bench_main! {{
    let content = ASCII_TEST_TEXT;

    let result = bench_run!(dimensions, content);

    assert_eq!(result, Some(Rectangle {
        top_left: Point::new(5,5),
        size: Size::new(10,10),
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
