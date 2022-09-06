use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*};
use u8g2_fonts::Error;

const MAGIC_FAIL_MESSAGE: &str = "ade19896-433a-4497-9b97-ffdf304271c0";

pub struct FailingDrawTarget {
    size: Size,
}

impl FailingDrawTarget {
    pub fn new(size: Size) -> Self {
        Self { size }
    }

    pub fn assert_passes_on_error<T>(
        render: impl FnOnce(&mut Self) -> Result<T, Error<<Self as DrawTarget>::Error>>,
    ) {
        let result = render(&mut FailingDrawTarget::new(Size::new(10, 10)));
        assert!(matches!(
            result,
            Err(Error::DisplayError(MAGIC_FAIL_MESSAGE))
        ));
    }
}

impl DrawTarget for FailingDrawTarget {
    type Color = Rgb888;

    type Error = &'static str;

    fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        Err(MAGIC_FAIL_MESSAGE)
    }
}

impl OriginDimensions for FailingDrawTarget {
    fn size(&self) -> Size {
        self.size
    }
}
