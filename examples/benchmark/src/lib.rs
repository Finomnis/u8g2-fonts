#![no_std]
#![no_main]
#![allow(clippy::new_without_default)]

pub use core::convert::Infallible;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    Pixel,
};
use panic_semihosting as _;
use u8g2_fonts::{fonts, FontRenderer};

pub use embedded_graphics_core::pixelcolor::BinaryColor;
pub use u8g2_fonts::types::{RenderedDimensions, VerticalPosition};

pub const ASCII_TEST_TEXT: &str = "Hello, world!\nLorem ipsum dolor sit amet.";
pub const UNICODE_TEST_TEXT: &str = "Привет, мир!\nCъeшь ещё этих мягких булок.";

pub const TEST_FONT: FontRenderer = FontRenderer::new::<fonts::u8g2_font_unifont_t_cyrillic>();
pub const CENTER_POINT: Point = Point::new(320, 240);

#[macro_export]
macro_rules! bench_main {
    ($code:block) => {
        #[::cortex_m_rt::entry]
        fn main() -> ! {
            $code;
            ::cortex_m_semihosting::debug::exit(::cortex_m_semihosting::debug::EXIT_SUCCESS);
            unreachable!();
        }
    };
}

#[macro_export]
macro_rules! bench_run {
    ($fnname:ident $(, $arg:ident)* $(,)?) => {{
        // Make mut so the asm can use them as inout operands.
        $(
            let mut $arg = $arg;
        )*

        // Marks the start of the benchmark
        // and makes args black_box
        #[allow(named_asm_labels)]
        unsafe {
            core::arch::asm!(
                concat!(
                    ".global benchmark_begin\n",
                    "benchmark_begin:\n",
                    $(
                        "/* {} ", stringify!($arg), " */\n",
                    )*
                ),
                $(
                    inout(reg) $arg,
                )*
                options(nostack, preserves_flags),
            );
        }

        let mut result = $fnname($($arg),*);

        // Marks the end of the benchmark
        // and makes result black_box
        #[allow(named_asm_labels)]
        unsafe {
            ::core::arch::asm!(
                ".global benchmark_end",
                "benchmark_end: /* {} */",
                inout(reg) result,
                options(nostack, preserves_flags),
            );
        }

        result
    }};
}

const TEST_DISPLAY_CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

pub struct TestDisplay {
    pub checksum: crc::Digest<'static, u32>,
}

impl TestDisplay {
    pub fn new() -> Self {
        Self {
            checksum: TEST_DISPLAY_CRC.digest(),
        }
    }
}

impl OriginDimensions for TestDisplay {
    fn size(&self) -> Size {
        Size::new(CENTER_POINT.x as u32 * 2, CENTER_POINT.y as u32 * 2)
    }
}
impl DrawTarget for TestDisplay {
    type Color = BinaryColor;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        todo!()
    }
}
