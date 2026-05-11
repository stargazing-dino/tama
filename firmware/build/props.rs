use crate::encode::emit_image;
use image::{RgbaImage, imageops};
use std::path::Path;

const CELL: u32 = 64;

// Each entry is (name, col, row) within a 64×64 cell grid. Each cell is
// auto-trimmed to its opaque bounding box before emission, so const W/H
// reflect the artwork, not the source cell.

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
    ("PROP_MINI_FRIDGE", 2, 1),
    ("PROP_MINI_FRIDGE_OPEN", 3, 1),
    ("PROP_WINDOW_LG", 4, 1),
    ("PROP_WINDOW", 5, 1),
    ("PROP_CHAIR", 6, 1),
    ("PROP_MIRROR", 7, 1),
    ("PROP_BEDSIDE_TABLE", 8, 1),
];

const KITCHEN_PROPS: &[(&str, u32, u32)] = &[
    ("PROP_COUNTER_SET", 0, 0),
    ("PROP_COUNTER_SET_OPEN", 1, 0),
    ("PROP_COUNTER", 2, 0),
    ("PROP_COUNTER_OPEN", 3, 0),
    ("PROP_FRIDGE", 4, 0),
    ("PROP_SPATULA", 5, 0),
    ("PROP_SPATULA_SM", 6, 0),
    ("PROP_SPATULA_HOLES", 7, 0),
    ("PROP_KNIFE", 8, 0),
    ("PROP_DISH", 9, 0),
    ("PROP_DISH_APPLE", 10, 0),
    ("PROP_APPLE", 11, 0),
    ("PROP_DISH_CARROT", 0, 1),
    ("PROP_CARROT", 1, 1),
    ("PROP_DISH_MEAT", 2, 1),
    ("PROP_BOWL", 3, 1),
    ("PROP_BOWL_FOOD", 4, 1),
    ("PROP_HANGING_DRAWER", 5, 1),
    ("PROP_HANGING_DRAWER_OPEN", 6, 1),
    ("PROP_DISH_RACK", 7, 1),
    ("PROP_CUTTING_BOARD", 8, 1),
    ("PROP_PAINTING", 9, 1),
];

const BATH_PROPS: &[(&str, u32, u32)] = &[
    ("PROP_TOILET", 0, 0),
    ("PROP_TP_ROLL", 1, 0),
    ("PROP_TOWEL_RACK", 2, 0),
    ("PROP_BATHTUB", 3, 0),
    ("PROP_MIRROR_BATH", 4, 0),
    ("PROP_SINK_LONG", 5, 0),
    ("PROP_TRASHCAN", 6, 0),
    ("PROP_SHOWER_SHELF", 7, 0),
    ("PROP_SHOWER_SHELF_FULL", 8, 0),
    ("PROP_SOAP_RED", 9, 0),
    ("PROP_SOAP_ORANGE", 10, 0),
    ("PROP_SOAP_GREEN", 11, 0),
    ("PROP_SINK", 0, 1),
    ("PROP_BATH_HANGING_DRAWER", 1, 1),
    ("PROP_BATH_HANGING_DRAWER_OPEN", 2, 1),
];

const SHEETS: &[(&str, &[(&str, u32, u32)])] = &[
    ("../DayOff/Objects/Bedroom/64x64 Bedroom.png", BEDROOM_PROPS),
    ("../DayOff/Objects/Kitchen/64x64 Kitchen.png", KITCHEN_PROPS),
    ("../DayOff/Objects/Bathroom/64x64 Bathroom.png", BATH_PROPS),
];

pub fn emit(out: &mut String, out_dir: &Path) {
    for (path, props) in SHEETS {
        println!("cargo:rerun-if-changed={path}");
        let sheet = image::open(path)
            .unwrap_or_else(|e| panic!("loading {path}: {e}"))
            .to_rgba8();
        for (name, col, row) in *props {
            let cell = imageops::crop_imm(&sheet, col * CELL, row * CELL, CELL, CELL).to_image();
            let cropped = trim_transparent(&cell);
            emit_image(out, out_dir, name, &cropped);
        }
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
