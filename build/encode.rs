use image::RgbaImage;
use std::fmt::Write as _;

pub const TRANSPARENT_RGB565: u16 = 0x07E0;

pub fn to_rgb565(r: u8, g: u8, b: u8, a: u8) -> u16 {
    if a < 128 {
        return TRANSPARENT_RGB565;
    }
    let v = (((r as u16) & 0xF8) << 8) | (((g as u16) & 0xFC) << 3) | ((b as u16) >> 3);
    if v == TRANSPARENT_RGB565 { v.wrapping_add(1) } else { v }
}

/// Emits `pub const {NAME}_W/_H: usize` and `pub static {NAME}_PIXELS: [u16; W*H]`.
pub fn emit_image(out: &mut String, name: &str, img: &RgbaImage) {
    let (w, h) = img.dimensions();
    let pixels = (w * h) as usize;
    writeln!(out, "pub const {name}_W: usize = {w};").unwrap();
    writeln!(out, "pub const {name}_H: usize = {h};").unwrap();
    writeln!(out, "pub static {name}_PIXELS: [u16; {pixels}] = [").unwrap();
    out.push_str("    ");
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel(x, y).0;
            write!(out, "0x{:04X}, ", to_rgb565(p[0], p[1], p[2], p[3])).unwrap();
        }
        out.push_str("\n    ");
    }
    out.push_str("\n];\n\n");
}
