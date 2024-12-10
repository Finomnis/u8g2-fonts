# u8g2-fonts

[![Hello World text rendered by this crate](assets/hello_world.png?raw=true)](examples/simulator/src/bin/hello_world_banner.rs)

[![Crates.io](https://img.shields.io/crates/v/u8g2-fonts)](https://crates.io/crates/u8g2-fonts)
[![Crates.io](https://img.shields.io/crates/d/u8g2-fonts)](https://crates.io/crates/u8g2-fonts)
[![License](https://img.shields.io/crates/l/u8g2-fonts)](https://github.com/Finomnis/u8g2-fonts/blob/main/LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/Finomnis/u8g2-fonts/ci.yml)](https://github.com/Finomnis/u8g2-fonts/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/u8g2-fonts)](https://docs.rs/u8g2-fonts)
[![Coverage Status](https://img.shields.io/coveralls/github/Finomnis/u8g2-fonts/main)](https://coveralls.io/github/Finomnis/u8g2-fonts?branch=main)

This crate is a pure Rust reimplementation of the font subsystem of [U8g2](https://github.com/olikraus/u8g2).

It is intended for the [embedded-graphics](https://crates.io/crates/embedded-graphics) ecosystem.


## Licensing

While this crate is MIT / Apache-2.0 licensed, note that the fonts themselves *are not*.

For more information about the font licenses, read the [license agreement](https://github.com/olikraus/u8g2/blob/master/LICENSE) of U8g2.


## Example

```rust
let font = FontRenderer::new::<fonts::u8g2_font_haxrcorp4089_t_cyrillic>();
let text = "embedded-graphics";

font.render_aligned(
    text,
    display.bounding_box().center() + Point::new(0, 16),
    VerticalPosition::Baseline,
    HorizontalAlignment::Center,
    FontColor::Transparent(BinaryColor::On),
    &mut display,
)
.unwrap();
```

This example is based on the `hello-world` of the official [embedded-graphics examples](https://github.com/embedded-graphics/examples).

If we [replace the text rendering section](examples/simulator/src/bin/embedded_graphics_hello_world.rs) of the example with the code above, it produces this output:

![Embedded-graphics example with our U8g2 font](assets/embedded_graphics_hello_world.png?raw=true)

Note that the letter `i` sits snug in between the `h` and the `c`, compared to the original example. This is not a monospace font.
