pub fn write_byte(out: &mut Vec<u8>, c: u8) {
    match c {
        b'\0' => out.extend_from_slice(br"\0"),
        b'\n' => out.extend_from_slice(br"\n"),
        b'\r' => out.extend_from_slice(br"\r"),
        b'\t' => out.extend_from_slice(br"\t"),
        b'"' => out.extend_from_slice(br#"\""#),
        b'\\' => out.extend_from_slice(br"\\"),
        32..=126 => out.push(c),
        _ => out.extend_from_slice(format!("\\x{:02x}", c).as_bytes()),
    }
}
