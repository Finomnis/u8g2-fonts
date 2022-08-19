mod util;

use embedded_graphics_core::{
    pixelcolor::Rgb888,
    prelude::{Point, RgbColor},
};
use u8g2_fonts::{fonts, FontRenderer};

use util::TestDrawTarget;

#[test]
fn boxed() {
    let mut display = TestDrawTarget::expect_image(std::include_bytes!("assets/boxed.png"));

    let font = FontRenderer::new::<fonts::u8g2_font_helvR08_tr>();
    font.render_text("Cage", Point::new(5, 11), Rgb888::GREEN, None, &mut display)
        .unwrap();
}
