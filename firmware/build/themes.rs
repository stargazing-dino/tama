use crate::encode::emit_image;
use image::{RgbaImage, imageops};
use std::path::Path;

const TILE: u32 = 16;
// Theme spritesheets pack 6 tiles horizontally as A,C,D,E,F,G (index 0..5).
// Stack: D (plain wall) ×2, F (wall, no-baseboard), G (floor) — uniform wall, no trim.
const WALL_TILE_INDICES: &[u32] = &[2, 2, 4, 5];

const THEMES: &[(&str, &str)] = &[
    ("THEME_BEDROOM", "assets/themes/theme09.png"),
    ("THEME_KITCHEN", "assets/themes/theme10.png"),
    ("THEME_BATH", "assets/themes/theme12.png"),
];

pub fn emit(out: &mut String, out_dir: &Path) {
    for (name, path) in THEMES {
        println!("cargo:rerun-if-changed={path}");
        let theme = image::open(path)
            .unwrap_or_else(|e| panic!("loading {path}: {e}"))
            .to_rgba8();
        let mut wall = RgbaImage::new(TILE, TILE * WALL_TILE_INDICES.len() as u32);
        for (slot, &idx) in WALL_TILE_INDICES.iter().enumerate() {
            let tile = imageops::crop_imm(&theme, idx * TILE, 0, TILE, TILE).to_image();
            imageops::overlay(&mut wall, &tile, 0, (slot as i64) * TILE as i64);
        }
        emit_image(out, out_dir, name, &wall);
    }
}
