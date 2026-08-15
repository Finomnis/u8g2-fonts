#![no_std]
#![no_main]

use benchmarks::*;

bench_main! {{
    let content = UNICODE_TEST_TEXT;

    let result = bench_run!(dimensions, content);

    assert_eq!(result, RenderedDimensions {
        advance: Point::new(224,17),
        bounding_box: Some(Rectangle {
            top_left: Point::new(321,226),
            size: Size::new(220,29),
        })
    });
}}

fn dimensions(content: &str) -> RenderedDimensions {
    TEST_FONT
        .get_rendered_dimensions(content, CENTER_POINT, VerticalPosition::Center)
        .unwrap()
}
