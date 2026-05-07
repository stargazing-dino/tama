use crate::encode::emit_image;
use image::{RgbaImage, imageops};

const SHEET: &str = "DayOff/Objects/Bedroom/64x64 Bedroom.png";
const CELL: u32 = 64;

// (name, col, row) within the 64×64 cell grid. Each cell is auto-trimmed to
// its opaque bounding box before emission, so const W/H reflect the artwork,
// not the source cell.
const BEDROOM_PROPS: &[(&str, u32, u32)] = &[
    ("PROP_BED", 0, 0),
    ("PROP_PLANT_LG", 1, 0),
    ("PROP_PLANT_MD", 2, 0),
    ("PROP_PLANT_SM", 3, 0),
    ("PROP_STOOL", 4, 0),
    ("PROP_BOOK_SIDE", 5, 0),
    ("PROP_LAMP", 6, 0),
    ("PROP_BOOKSHELF", 7, 0),
    ("PROP_PORTRAIT_WOMAN", 8, 0),
    ("PROP_PORTRAIT_MAN", 9, 0),
    ("PROP_NEON_FAB", 10, 0),
    ("PROP_DOOR", 11, 0),
    ("PROP_WARDROBE", 0, 1),
    ("PROP_WARDROBE_OPEN", 1, 1),
    ("PROP_FRIDGE", 2, 1),
    ("PROP_FRIDGE_OPEN", 3, 1),
    ("PROP_WINDOW_LG", 4, 1),
    ("PROP_WINDOW", 5, 1),
    ("PROP_CHAIR", 6, 1),
    ("PROP_MIRROR", 7, 1),
    ("PROP_BEDSIDE_TABLE", 8, 1),
];

pub fn emit(out: &mut String) {
    println!("cargo:rerun-if-changed={SHEET}");
    let sheet = image::open(SHEET)
        .unwrap_or_else(|e| panic!("loading {SHEET}: {e}"))
        .to_rgba8();
    for (name, col, row) in BEDROOM_PROPS {
        let cell = imageops::crop_imm(&sheet, col * CELL, row * CELL, CELL, CELL).to_image();
        let cropped = trim_transparent(&cell);
        emit_image(out, name, &cropped);
    }
}

fn trim_transparent(img: &RgbaImage) -> RgbaImage {
    let (w, h) = img.dimensions();
    let mut min_x = w;
    let mut min_y = h;
    let mut max_x = 0u32;
    let mut max_y = 0u32;
    let mut any = false;
    for y in 0..h {
        for x in 0..w {
            if img.get_pixel(x, y).0[3] >= 128 {
                any = true;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }
    if !any {
        return RgbaImage::new(1, 1);
    }
    imageops::crop_imm(img, min_x, min_y, max_x - min_x + 1, max_y - min_y + 1).to_image()
}
