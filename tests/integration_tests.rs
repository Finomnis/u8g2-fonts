mod util;

use embedded_graphics_core::{pixelcolor::Rgb888, prelude::Point};
use u8g2_fonts::{fonts, FontRenderer};

use util::TestDrawTarget;

#[test]
fn render_text() {
    let mut display = TestDrawTarget::expect_image(std::include_bytes!("assets/render_text.png"));

    FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>()
        .render_text(
            "Hello World!",
            Point::new(2, 15),
            Rgb888::new(237, 28, 36),
            None,
            &mut display,
        )
        .unwrap();
}

#[test]
fn boxed() {
    // let mut display = TestDrawTarget::expect_image(std::include_bytes!("assets/boxed.png"));

    // let font = FontRenderer::new::<fonts::u8g2_font_helvR08_tr>();
    // font.render_text("Cage", Point::new(5, 11), Rgb888::GREEN, None, &mut display)
    //     .unwrap();
}
