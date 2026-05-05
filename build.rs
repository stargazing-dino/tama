use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use image::RgbaImage;

const SPRITESHEET: &str = "assets/Cat Sprite Sheet.png";
const WALL_THEME: &str = "assets/themes/theme09.png";
// Theme spritesheets pack 6 tiles horizontally as A,C,D,E,F,G (index 0..5).
// The wall composition stacks A (ceiling), C (wall trim), F (no-baseboard wall), G (floor).
const WALL_TILE_INDICES: &[u32] = &[0, 1, 4, 5];
const TILE: u32 = 16;

const CELL: u32 = 32;
const TRANSPARENT_RGB565: u16 = 0x07E0;

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

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={SPRITESHEET}");
    println!("cargo:rerun-if-changed={WALL_THEME}");

    let img = image::open(SPRITESHEET)
        .unwrap_or_else(|e| panic!("loading {SPRITESHEET}: {e}"))
        .to_rgba8();

    let mut src = String::new();
    writeln!(&mut src, "pub const SPRITE_W: usize = {CELL};").unwrap();
    writeln!(&mut src, "pub const SPRITE_H: usize = {CELL};").unwrap();
    writeln!(
        &mut src,
        "pub const TRANSPARENT: u16 = 0x{TRANSPARENT_RGB565:04X};\n"
    )
    .unwrap();

    for (name, row, frames) in ANIMS {
        emit_anim(&mut src, &img, name, *row, *frames);
    }

    let theme = image::open(WALL_THEME)
        .unwrap_or_else(|e| panic!("loading {WALL_THEME}: {e}"))
        .to_rgba8();
    let mut wall = RgbaImage::new(TILE, TILE * WALL_TILE_INDICES.len() as u32);
    for (slot, &idx) in WALL_TILE_INDICES.iter().enumerate() {
        let tile = image::imageops::crop_imm(&theme, idx * TILE, 0, TILE, TILE).to_image();
        image::imageops::overlay(&mut wall, &tile, 0, (slot as i64) * TILE as i64);
    }
    emit_image(&mut src, &wall, "WALL");

    fs::write(out.join("tama_sprite.rs"), src).unwrap();
}

fn emit_image(out: &mut String, img: &RgbaImage, name: &str) {
    let (w, h) = img.dimensions();
    let pixels = (w * h) as usize;
    writeln!(out, "pub const {name}_W: usize = {w};").unwrap();
    writeln!(out, "pub const {name}_H: usize = {h};").unwrap();
    writeln!(out, "pub static {name}_PIXELS: [u16; {pixels}] = [").unwrap();
    out.push_str("    ");
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel(x, y);
            let [r, g, b, a] = p.0;
            let rgb565 = if a < 128 {
                TRANSPARENT_RGB565
            } else {
                let v = (((r as u16) & 0xF8) << 8)
                    | (((g as u16) & 0xFC) << 3)
                    | ((b as u16) >> 3);
                if v == TRANSPARENT_RGB565 {
                    v.wrapping_add(1)
                } else {
                    v
                }
            };
            write!(out, "0x{rgb565:04X}, ").unwrap();
        }
        out.push_str("\n    ");
    }
    out.push_str("\n];\n\n");
}

fn emit_anim(out: &mut String, img: &RgbaImage, name: &str, row: u32, frames: u32) {
    let pixels = (CELL * CELL) as usize;
    writeln!(out, "pub const {name}_FRAMES: usize = {frames};").unwrap();
    writeln!(
        out,
        "pub static {name}: [[u16; {pixels}]; {frames}] = ["
    )
    .unwrap();

    for f in 0..frames {
        let x0 = f * CELL;
        let y0 = row * CELL;
        out.push_str("    [\n        ");
        for y in 0..CELL {
            for x in 0..CELL {
                let p = img.get_pixel(x0 + x, y0 + y);
                let [r, g, b, a] = p.0;
                let rgb565 = if a < 128 {
                    TRANSPARENT_RGB565
                } else {
                    let v = (((r as u16) & 0xF8) << 8)
                        | (((g as u16) & 0xFC) << 3)
                        | ((b as u16) >> 3);
                    if v == TRANSPARENT_RGB565 {
                        v.wrapping_add(1)
                    } else {
                        v
                    }
                };
                write!(out, "0x{rgb565:04X}, ").unwrap();
            }
            out.push_str("\n        ");
        }
        out.push_str("\n    ],\n");
    }

    out.push_str("];\n\n");
}
