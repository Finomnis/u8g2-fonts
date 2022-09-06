use embedded_graphics_core::{
    pixelcolor::{Rgb888, WebColors},
    prelude::{DrawTarget, OriginDimensions, Point, Size},
    primitives::Rectangle,
};
use u8g2_fonts::types::{HorizontalAlignment, VerticalPosition};

pub fn get_x(h: HorizontalAlignment) -> i32 {
    match h {
        HorizontalAlignment::Left => 5,
        HorizontalAlignment::Center => 155,
        HorizontalAlignment::Right => 305,
    }
}

pub fn get_y(v: VerticalPosition) -> i32 {
    match v {
        VerticalPosition::Baseline => 200,
        VerticalPosition::Top => 7,
        VerticalPosition::Center => 87,
        VerticalPosition::Bottom => 167,
    }
}

pub fn get_pos(h: HorizontalAlignment, v: VerticalPosition) -> Point {
    Point::new(get_x(h), get_y(v))
}

pub fn draw<Display>(display: &mut Display)
where
    Display: DrawTarget<Color = Rgb888> + OriginDimensions,
    Display::Error: core::fmt::Debug,
{
    let vertical_rect = Size::new(
        1,
        (get_y(VerticalPosition::Bottom) - get_y(VerticalPosition::Top) + 1)
            .try_into()
            .unwrap(),
    );
    let horizontal_rect = Size::new(
        (get_x(HorizontalAlignment::Right) - get_x(HorizontalAlignment::Left) + 1)
            .try_into()
            .unwrap(),
        1,
    );

    for hpos in [
        HorizontalAlignment::Left,
        HorizontalAlignment::Center,
        HorizontalAlignment::Right,
    ] {
        display
            .fill_solid(
                &Rectangle::new(
                    get_pos(hpos, VerticalPosition::Top),
                    Size::new(1, display.size().height).try_into().unwrap(),
                ),
                Rgb888::CSS_ORANGE,
            )
            .unwrap();
    }

    display
        .fill_solid(
            &Rectangle::new(
                get_pos(HorizontalAlignment::Left, VerticalPosition::Center),
                horizontal_rect,
            ),
            Rgb888::CSS_ORANGE,
        )
        .unwrap();

    for hpos in [HorizontalAlignment::Left, HorizontalAlignment::Right] {
        display
            .fill_solid(
                &Rectangle::new(get_pos(hpos, VerticalPosition::Top), vertical_rect),
                Rgb888::CSS_RED,
            )
            .unwrap();
    }

    for vpos in [VerticalPosition::Top, VerticalPosition::Bottom] {
        display
            .fill_solid(
                &Rectangle::new(get_pos(HorizontalAlignment::Left, vpos), horizontal_rect),
                Rgb888::CSS_RED,
            )
            .unwrap();
    }

    display
        .fill_solid(
            &Rectangle::new(
                Point::new(0, get_y(VerticalPosition::Baseline)),
                Size::new(display.size().width, 1),
            ),
            Rgb888::CSS_ORANGE,
        )
        .unwrap();
}

pub fn draw_bounding_box<Display>(bounding_box: &Rectangle, color: Rgb888, display: &mut Display)
where
    Display: DrawTarget<Color = Rgb888> + OriginDimensions,
    Display::Error: core::fmt::Debug,
{
    display
        .fill_solid(
            &Rectangle::new(bounding_box.top_left, Size::new(bounding_box.size.width, 1)),
            color,
        )
        .unwrap();

    display
        .fill_solid(
            &Rectangle::new(
                bounding_box.top_left,
                Size::new(1, bounding_box.size.height),
            ),
            color,
        )
        .unwrap();

    display
        .fill_solid(
            &Rectangle::new(
                bounding_box.top_left + Point::new(0, bounding_box.size.height as i32 - 1),
                Size::new(bounding_box.size.width, 1),
            ),
            color,
        )
        .unwrap();

    display
        .fill_solid(
            &Rectangle::new(
                bounding_box.top_left + Point::new(bounding_box.size.width as i32 - 1, 0),
                Size::new(1, bounding_box.size.height),
            ),
            color,
        )
        .unwrap();
}
