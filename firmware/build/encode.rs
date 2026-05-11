use image::RgbaImage;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

pub const TRANSPARENT_RGB565: u16 = 0x07E0;

pub fn to_rgb565(r: u8, g: u8, b: u8, a: u8) -> u16 {
    if a < 128 {
        return TRANSPARENT_RGB565;
    }
    let v = (((r as u16) & 0xF8) << 8) | (((g as u16) & 0xFC) << 3) | ((b as u16) >> 3);
    if v == TRANSPARENT_RGB565 { v.wrapping_add(1) } else { v }
}

/// Packs an RGBA8 image into little-endian RGB565 bytes.
pub fn image_to_bytes(img: &RgbaImage) -> Vec<u8> {
    let (w, h) = img.dimensions();
    let mut bytes = Vec::with_capacity((w * h * 2) as usize);
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel(x, y).0;
            bytes.extend_from_slice(&to_rgb565(p[0], p[1], p[2], p[3]).to_le_bytes());
        }
    }
    bytes
}

/// Writes `{name}.bin` to `out_dir` and emits `{NAME}_W/_H/_PIXELS` referring to it
/// via `include_bytes!` + a const `transmute` (no hex literal blob).
pub fn emit_image(out: &mut String, out_dir: &Path, name: &str, img: &RgbaImage) {
    let (w, h) = img.dimensions();
    let pixels = (w * h) as usize;
    let bytes = image_to_bytes(img);
    fs::write(out_dir.join(format!("{name}.bin")), &bytes).unwrap();
    writeln!(out, "pub const {name}_W: usize = {w};").unwrap();
    writeln!(out, "pub const {name}_H: usize = {h};").unwrap();
    writeln!(
        out,
        "pub static {name}_PIXELS: [u16; {pixels}] = unsafe \
         {{ ::core::mem::transmute(*include_bytes!(\"{name}.bin\")) }};\n",
    )
    .unwrap();
}
