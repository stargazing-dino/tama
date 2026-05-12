use crate::encode::{TRANSPARENT_RGB565, to_rgb565};
use image::RgbaImage;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

const SPRITESHEET: &str = "../elthen-cats/Cat Sprite Sheet.png";
const CELL: u32 = 32;

// (name, row, frame_count)
const ANIMS: &[(&str, u32, u32)] = &[
    ("IDLE_A", 0, 4),
    ("IDLE_B", 1, 4),
    ("CLEAN_A", 2, 4),
    ("CLEAN_B", 3, 4),
    ("WALK_A", 4, 8),
    ("WALK_B", 5, 8),
    ("SLEEP", 6, 4),
    ("PAW", 7, 6),
    ("JUMP", 8, 7),
    ("SCARED", 9, 8),
];

pub fn emit(out: &mut String, out_dir: &Path) {
    println!("cargo:rerun-if-changed={SPRITESHEET}");
    let img = image::open(SPRITESHEET)
        .unwrap_or_else(|e| panic!("loading {SPRITESHEET}: {e}"))
        .to_rgba8();

    writeln!(out, "pub const SPRITE_W: usize = {CELL};").unwrap();
    writeln!(out, "pub const SPRITE_H: usize = {CELL};").unwrap();
    writeln!(out, "pub const TRANSPARENT: u16 = 0x{TRANSPARENT_RGB565:04X};\n").unwrap();

    for (name, row, frames) in ANIMS {
        emit_anim(out, out_dir, &img, name, *row, *frames);
    }
}

fn emit_anim(
    out: &mut String,
    out_dir: &Path,
    img: &RgbaImage,
    name: &str,
    row: u32,
    frames: u32,
) {
    let pixels = (CELL * CELL) as usize;
    let mut bytes = Vec::with_capacity(pixels * frames as usize * 2);
    for f in 0..frames {
        let x0 = f * CELL;
        let y0 = row * CELL;
        for y in 0..CELL {
            for x in 0..CELL {
                let p = img.get_pixel(x0 + x, y0 + y).0;
                bytes.extend_from_slice(&to_rgb565(p[0], p[1], p[2], p[3]).to_le_bytes());
            }
        }
    }
    fs::write(out_dir.join(format!("{name}.bin")), &bytes).unwrap();
    writeln!(out, "pub const {name}_FRAMES: usize = {frames};").unwrap();
    writeln!(
        out,
        "pub static {name}: [[u16; {pixels}]; {frames}] = unsafe \
         {{ ::core::mem::transmute(*include_bytes!(\"{name}.bin\")) }};\n",
    )
    .unwrap();
}
