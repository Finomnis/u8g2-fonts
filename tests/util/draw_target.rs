use std::{convert::Infallible, io::Cursor};

use embedded_graphics_core::{pixelcolor::Rgb888, prelude::*};
use image::RgbImage;

pub struct TestDrawTarget {
    size: Size,
    data: Vec<<Self as DrawTarget>::Color>,
    expected_image: RgbImage,
}

impl TestDrawTarget {
    pub fn expect_image(image_data: &'static [u8]) -> Self {
        let image_reader = image::io::Reader::new(Cursor::new(image_data))
            .with_guessed_format()
            .expect("Reference image data is not a recognized image format!");
        let image: RgbImage = image_reader
            .decode()
            .expect("Reference image data content is invalid")
            .into_rgb8();

        let size = Size::new(image.width(), image.height());

        Self {
            size,
            data: vec![
                <Self as DrawTarget>::Color::WHITE;
                size.width as usize * size.height as usize
            ],
            expected_image: image,
        }
    }

    pub fn get_pixel(&self, p: Point) -> Option<<Self as DrawTarget>::Color> {
        if p.x >= 0 && p.y >= 0 && (p.x as u32) < self.size.width && (p.y as u32) < self.size.height
        {
            self.data
                .get(((p.y as u32) * self.size.height + p.x as u32) as usize)
                .cloned()
        } else {
            None
        }
    }

    fn set_pixel(&mut self, p: Point, color: <Self as DrawTarget>::Color) {
        if p.x >= 0 && p.y >= 0 && (p.x as u32) < self.size.width && (p.y as u32) < self.size.height
        {
            if let Some(value) = self
                .data
                .get_mut(((p.y as u32) * self.size.height + p.x as u32) as usize)
            {
                *value = color;
            }
        }
    }
}

impl Drop for TestDrawTarget {
    fn drop(&mut self) {
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let image::Rgb(expected) = *self.expected_image.get_pixel(x, y);
                let actual = self.get_pixel(Point::new(x as i32, y as i32)).unwrap();
                let actual = [actual.r(), actual.g(), actual.b()];
                if expected != actual {
                    panic!(
                        "Expectation not met!\n\nPixel at position ({}, {}) does not match!\n    Expected: {:?}\n    Actual:   {:?}\n\n",
                        x, y, expected, actual
                    );
                }
            }
        }
    }
}

impl DrawTarget for TestDrawTarget {
    type Color = Rgb888;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, color) in pixels {
            self.set_pixel(p, color);
        }

        Ok(())
    }
}

impl OriginDimensions for TestDrawTarget {
    fn size(&self) -> Size {
        self.size
    }
}
