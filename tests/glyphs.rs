use std::collections::BTreeSet;

use embedded_graphics_core::prelude::Point;
use u8g2_fonts::{fonts, types::VerticalPosition, FontRenderer};

const PRINTABLE_ASCII: &str = concat!(
    " !\"#$%&'()*+,-./0123456789:;<=>?",
    "@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_",
    "`abcdefghijklmnopqrstuvwxyz{|}~",
);

/// Runs `check` on a selection of fonts with different glyph ranges, bit widths and
/// encodings. `check` receives the font name and a renderer of that font.
fn for_each_test_font(check: impl Fn(&str, &FontRenderer)) {
    macro_rules! check_font {
        ($font:ty) => {
            check(stringify!($font), &FontRenderer::new::<$font>())
        };
    }

    check_font!(fonts::u8g2_font_ncenB14_tr);
    check_font!(fonts::u8g2_font_courB10_tn);
    check_font!(fonts::u8g2_font_lubBI08_tf);
    check_font!(fonts::u8g2_font_u8glib_4_tf);
    check_font!(fonts::u8g2_font_4x6_mr);
    check_font!(fonts::u8g2_font_10x20_me);
    check_font!(fonts::u8g2_font_haxrcorp4089_t_cyrillic);
    check_font!(fonts::u8g2_font_unifont_t_symbols);
}

#[test]
fn yields_the_exact_glyph_set_of_a_font() {
    let font = FontRenderer::new::<fonts::u8g2_font_courB10_tn>();

    assert_eq!(
        font.glyphs().collect::<String>(),
        " *+,-./0123456789:".to_string()
    );
}

#[test]
fn yields_printable_ascii_of_ascii_only_fonts() {
    macro_rules! check_font {
        ($font:ty) => {
            assert_eq!(
                FontRenderer::new::<$font>().glyphs().collect::<String>(),
                PRINTABLE_ASCII,
                "{}",
                stringify!($font)
            )
        };
    }

    check_font!(fonts::u8g2_font_ncenB14_tr);
    check_font!(fonts::u8g2_font_4x6_mr);
    check_font!(fonts::u8g2_font_helvB08_tr);
    check_font!(fonts::u8g2_font_fub25_tr);
}

#[test]
fn yields_unicode_glyphs() {
    let font = FontRenderer::new::<fonts::u8g2_font_haxrcorp4089_t_cyrillic>();
    let glyphs = font.glyphs().collect::<Vec<_>>();

    // The printable ASCII characters come first, the unicode characters after them.
    assert_eq!(
        glyphs.iter().take(95).collect::<String>(),
        PRINTABLE_ASCII.to_string()
    );

    let unicode = &glyphs[95..];
    assert_eq!(unicode.len(), 63);
    assert_eq!(unicode[0], '\u{410}');
    assert_eq!(unicode[unicode.len() - 1], '\u{44f}');
    assert!(unicode.iter().all(|ch| *ch > '\u{ff}'));
}

#[test]
fn yields_ascending_and_unique_characters() {
    for_each_test_font(|name, font| {
        let glyphs = font.glyphs().collect::<Vec<_>>();

        assert!(!glyphs.is_empty(), "{name} has no glyphs");
        assert!(
            glyphs.windows(2).all(|w| w[0] < w[1]),
            "{name} is not sorted ascending"
        );
        assert_eq!(
            glyphs.len(),
            glyphs.iter().collect::<BTreeSet<_>>().len(),
            "{name} yields duplicates"
        );
    });
}

#[test]
fn every_yielded_character_is_retrievable() {
    for_each_test_font(|name, font| {
        for ch in font.glyphs() {
            font.get_rendered_dimensions(ch, Point::new(0, 0), VerticalPosition::Baseline)
                .unwrap_or_else(|e| panic!("{name}: {ch:?} is not retrievable: {e}"));
        }
    });
}

#[test]
fn characters_that_are_not_yielded_are_not_retrievable() {
    for_each_test_font(|name, font| {
        let glyphs = font.glyphs().collect::<BTreeSet<_>>();

        for ch in '\u{0}'..='\u{ff}' {
            if ch == '\n' {
                // Newlines are line breaks rather than glyphs.
                continue;
            }

            let retrievable = font
                .get_rendered_dimensions(ch, Point::new(0, 0), VerticalPosition::Baseline)
                .is_ok();

            assert_eq!(
                retrievable,
                glyphs.contains(&ch),
                "{name}: {ch:?} retrievable: {retrievable}, yielded: {}",
                glyphs.contains(&ch)
            );
        }
    });
}

#[test]
fn unicode_characters_that_are_not_yielded_are_not_retrievable() {
    let font = FontRenderer::new::<fonts::u8g2_font_haxrcorp4089_t_cyrillic>();
    let glyphs = font.glyphs().collect::<BTreeSet<_>>();

    for ch in '\u{100}'..='\u{500}' {
        let retrievable = font
            .get_rendered_dimensions(ch, Point::new(0, 0), VerticalPosition::Baseline)
            .is_ok();

        assert_eq!(retrievable, glyphs.contains(&ch), "{ch:?}");
    }
}

#[test]
fn is_debug_and_clone() {
    let font = FontRenderer::new::<fonts::u8g2_font_courB10_tn>();
    let mut glyphs = font.glyphs();

    assert!(!format!("{glyphs:?}").is_empty());

    glyphs.next().unwrap();
    assert_eq!(
        glyphs.clone().collect::<String>(),
        glyphs.collect::<String>()
    );
}

#[test]
fn the_glyph_list_terminator_is_not_a_glyph() {
    let font = FontRenderer::new::<fonts::u8g2_font_ncenB14_tr>();

    assert!(!font.glyphs().any(|ch| ch == '\u{0}'));
}
