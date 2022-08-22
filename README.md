# u8g2-fonts

[![Hello World text rendered by this crate](examples/hello_world.png?raw=true)](examples/simulator/src/bin/hello_world_banner.rs)

[![Crates.io](https://img.shields.io/crates/v/u8g2-fonts)](https://crates.io/crates/u8g2-fonts)
[![Crates.io](https://img.shields.io/crates/d/u8g2-fonts)](https://crates.io/crates/u8g2-fonts)
[![License](https://img.shields.io/crates/l/u8g2-fonts)](https://github.com/Finomnis/u8g2-fonts/blob/main/LICENSE)
[![Build Status](https://img.shields.io/github/workflow/status/Finomnis/u8g2-fonts/CI/main)](https://github.com/Finomnis/u8g2-fonts/actions/workflows/ci.yml?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/u8g2-fonts)](https://docs.rs/u8g2-fonts)
[![Coverage Status](https://img.shields.io/coveralls/github/Finomnis/u8g2-fonts/main)](https://coveralls.io/github/Finomnis/u8g2-fonts?branch=main)

This crate is a pure Rust reimplementation of the font subsystem of [U8g2](https://github.com/olikraus/u8g2).

It is intended for the [embedded-graphics](https://crates.io/crates/embedded-graphics) ecosystem.


## Licensing

While this crate is MIT / Apache-2.0 licensed, note that the fonts themselves *are not*.

For more information about the font licenses, read the [license agreement](https://github.com/olikraus/u8g2/blob/master/LICENSE) of U8g2.


## Example

```rust
let mut display = init_display(150, 50);

let font = FontRenderer::new::<fonts::u8g2_font_lubI14_tf>();

font.render_text_aligned(
    "Hello, World!",
    display.bounding_box().center(),
    FontColor::Transparent(COLOR::GREEN),
    VerticalPosition::Center,
    HorizontalAlignment::Center,
    &mut display,
)?;

show(display);
```

If you [run this code](examples/simulator/src/bin/readme_example.rs), you will get the following output:

![Hello World text rendered by this crate](examples/readme_example.png?raw=true)
