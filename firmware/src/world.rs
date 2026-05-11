use crate::fb::Fb;
use crate::sprites::{
    PROP_BATHTUB_H, PROP_BATHTUB_PIXELS, PROP_BATHTUB_W, PROP_BATH_HANGING_DRAWER_H,
    PROP_BATH_HANGING_DRAWER_PIXELS, PROP_BATH_HANGING_DRAWER_W, PROP_BED_H, PROP_BED_PIXELS,
    PROP_BED_W, PROP_BEDSIDE_TABLE_H, PROP_BEDSIDE_TABLE_PIXELS, PROP_BEDSIDE_TABLE_W,
    PROP_BOOKSHELF_H, PROP_BOOKSHELF_PIXELS, PROP_BOOKSHELF_W, PROP_BOWL_FOOD_H,
    PROP_BOWL_FOOD_PIXELS, PROP_BOWL_FOOD_W, PROP_CHAIR_H, PROP_CHAIR_PIXELS, PROP_CHAIR_W,
    PROP_COUNTER_SET_H, PROP_COUNTER_SET_PIXELS, PROP_COUNTER_SET_W, PROP_FRIDGE_H,
    PROP_FRIDGE_PIXELS, PROP_FRIDGE_W, PROP_HANGING_DRAWER_H, PROP_HANGING_DRAWER_PIXELS,
    PROP_HANGING_DRAWER_W, PROP_KNIFE_H, PROP_KNIFE_PIXELS, PROP_KNIFE_W, PROP_MIRROR_BATH_H,
    PROP_MIRROR_BATH_PIXELS, PROP_MIRROR_BATH_W, PROP_NEON_FAB_H, PROP_NEON_FAB_PIXELS,
    PROP_NEON_FAB_W, PROP_PAINTING_H, PROP_PAINTING_PIXELS, PROP_PAINTING_W, PROP_PLANT_MD_H,
    PROP_PLANT_MD_PIXELS, PROP_PLANT_MD_W, PROP_PLANT_SM_H, PROP_PLANT_SM_PIXELS, PROP_PLANT_SM_W,
    PROP_PORTRAIT_WOMAN_H, PROP_PORTRAIT_WOMAN_PIXELS, PROP_PORTRAIT_WOMAN_W,
    PROP_SHOWER_SHELF_FULL_H, PROP_SHOWER_SHELF_FULL_PIXELS, PROP_SHOWER_SHELF_FULL_W, PROP_SINK_H,
    PROP_SINK_PIXELS, PROP_SINK_W, PROP_SPATULA_H, PROP_SPATULA_HOLES_H,
    PROP_SPATULA_HOLES_PIXELS, PROP_SPATULA_HOLES_W, PROP_SPATULA_PIXELS, PROP_SPATULA_SM_H,
    PROP_SPATULA_SM_PIXELS, PROP_SPATULA_SM_W, PROP_SPATULA_W, PROP_TOILET_H, PROP_TOILET_PIXELS,
    PROP_TOILET_W, PROP_TOWEL_RACK_H, PROP_TOWEL_RACK_PIXELS, PROP_TOWEL_RACK_W, PROP_TP_ROLL_H,
    PROP_TP_ROLL_PIXELS, PROP_TP_ROLL_W, PROP_TRASHCAN_H, PROP_TRASHCAN_PIXELS, PROP_TRASHCAN_W,
    PROP_WINDOW_H, PROP_WINDOW_PIXELS, PROP_WINDOW_W, THEME_BATH_H, THEME_BATH_PIXELS,
    THEME_BATH_W, THEME_BEDROOM_H, THEME_BEDROOM_PIXELS, THEME_BEDROOM_W, THEME_KITCHEN_H,
    THEME_KITCHEN_PIXELS, THEME_KITCHEN_W, TRANSPARENT,
};

// World coordinates are in NATIVE pixels (1×). The renderer scales by SCALE.

pub struct Theme {
    pub w: usize,
    pub h: usize,
    pub pixels: &'static [u16],
}

pub struct Prop {
    pub x: i32,
    pub y: i32,
    pub w: usize,
    pub h: usize,
    pub pixels: &'static [u16],
}

pub struct Room {
    pub width: i32,
    pub theme: &'static Theme,
    pub props: &'static [Prop],
}

static THEME_BEDROOM: Theme = Theme {
    w: THEME_BEDROOM_W,
    h: THEME_BEDROOM_H,
    pixels: &THEME_BEDROOM_PIXELS,
};
static THEME_KITCHEN: Theme = Theme {
    w: THEME_KITCHEN_W,
    h: THEME_KITCHEN_H,
    pixels: &THEME_KITCHEN_PIXELS,
};
static THEME_BATH: Theme = Theme {
    w: THEME_BATH_W,
    h: THEME_BATH_H,
    pixels: &THEME_BATH_PIXELS,
};

static BEDROOM_PROPS: &[Prop] = &[
    Prop { x: 2, y: 13, w: PROP_BOOKSHELF_W, h: PROP_BOOKSHELF_H, pixels: &PROP_BOOKSHELF_PIXELS },
    Prop { x: 36, y: 32, w: PROP_PLANT_MD_W, h: PROP_PLANT_MD_H, pixels: &PROP_PLANT_MD_PIXELS },
    Prop { x: 56, y: 8, w: PROP_NEON_FAB_W, h: PROP_NEON_FAB_H, pixels: &PROP_NEON_FAB_PIXELS },
    Prop { x: 54, y: 32, w: PROP_BED_W, h: PROP_BED_H, pixels: &PROP_BED_PIXELS },
    Prop {
        x: 96, y: 4,
        w: PROP_PORTRAIT_WOMAN_W, h: PROP_PORTRAIT_WOMAN_H,
        pixels: &PROP_PORTRAIT_WOMAN_PIXELS,
    },
    Prop {
        x: 96, y: 29,
        w: PROP_BEDSIDE_TABLE_W, h: PROP_BEDSIDE_TABLE_H,
        pixels: &PROP_BEDSIDE_TABLE_PIXELS,
    },
    Prop { x: 126, y: 6, w: PROP_WINDOW_W, h: PROP_WINDOW_H, pixels: &PROP_WINDOW_PIXELS },
    Prop { x: 122, y: 33, w: PROP_CHAIR_W, h: PROP_CHAIR_H, pixels: &PROP_CHAIR_PIXELS },
    Prop { x: 145, y: 37, w: PROP_PLANT_SM_W, h: PROP_PLANT_SM_H, pixels: &PROP_PLANT_SM_PIXELS },
];

static KITCHEN_PROPS: &[Prop] = &[
    Prop { x: 68, y: 2, w: PROP_PAINTING_W, h: PROP_PAINTING_H, pixels: &PROP_PAINTING_PIXELS },
    Prop {
        x: 113, y: 4,
        w: PROP_HANGING_DRAWER_W, h: PROP_HANGING_DRAWER_H,
        pixels: &PROP_HANGING_DRAWER_PIXELS,
    },
    Prop {
        x: 8, y: 3,
        w: PROP_SPATULA_HOLES_W, h: PROP_SPATULA_HOLES_H,
        pixels: &PROP_SPATULA_HOLES_PIXELS,
    },
    Prop { x: 20, y: 4, w: PROP_SPATULA_W, h: PROP_SPATULA_H, pixels: &PROP_SPATULA_PIXELS },
    Prop {
        x: 32, y: 6,
        w: PROP_SPATULA_SM_W, h: PROP_SPATULA_SM_H,
        pixels: &PROP_SPATULA_SM_PIXELS,
    },
    Prop { x: 44, y: 4, w: PROP_KNIFE_W, h: PROP_KNIFE_H, pixels: &PROP_KNIFE_PIXELS },
    Prop {
        x: 0, y: 23,
        w: PROP_COUNTER_SET_W, h: PROP_COUNTER_SET_H,
        pixels: &PROP_COUNTER_SET_PIXELS,
    },
    Prop { x: 80, y: 39, w: PROP_BOWL_FOOD_W, h: PROP_BOWL_FOOD_H, pixels: &PROP_BOWL_FOOD_PIXELS },
    Prop { x: 138, y: 27, w: PROP_FRIDGE_W, h: PROP_FRIDGE_H, pixels: &PROP_FRIDGE_PIXELS },
];

static BATH_PROPS: &[Prop] = &[
    Prop {
        x: 6, y: 8,
        w: PROP_SHOWER_SHELF_FULL_W, h: PROP_SHOWER_SHELF_FULL_H,
        pixels: &PROP_SHOWER_SHELF_FULL_PIXELS,
    },
    Prop {
        x: 44, y: 22,
        w: PROP_TOWEL_RACK_W, h: PROP_TOWEL_RACK_H,
        pixels: &PROP_TOWEL_RACK_PIXELS,
    },
    Prop {
        x: 101, y: 4,
        w: PROP_MIRROR_BATH_W, h: PROP_MIRROR_BATH_H,
        pixels: &PROP_MIRROR_BATH_PIXELS,
    },
    Prop {
        x: 140, y: 14,
        w: PROP_BATH_HANGING_DRAWER_W, h: PROP_BATH_HANGING_DRAWER_H,
        pixels: &PROP_BATH_HANGING_DRAWER_PIXELS,
    },
    Prop { x: 78, y: 31, w: PROP_TP_ROLL_W, h: PROP_TP_ROLL_H, pixels: &PROP_TP_ROLL_PIXELS },
    Prop { x: 4, y: 34, w: PROP_BATHTUB_W, h: PROP_BATHTUB_H, pixels: &PROP_BATHTUB_PIXELS },
    Prop { x: 63, y: 33, w: PROP_TOILET_W, h: PROP_TOILET_H, pixels: &PROP_TOILET_PIXELS },
    Prop { x: 98, y: 32, w: PROP_SINK_W, h: PROP_SINK_H, pixels: &PROP_SINK_PIXELS },
    Prop { x: 145, y: 34, w: PROP_TRASHCAN_W, h: PROP_TRASHCAN_H, pixels: &PROP_TRASHCAN_PIXELS },
];

pub static WORLD: &[Room] = &[
    Room { width: 160, theme: &THEME_BEDROOM, props: BEDROOM_PROPS },
    Room { width: 160, theme: &THEME_KITCHEN, props: KITCHEN_PROPS },
    Room { width: 160, theme: &THEME_BATH, props: BATH_PROPS },
];

pub fn world_width() -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < WORLD.len() {
        sum += WORLD[i].width;
        i += 1;
    }
    sum
}

pub fn draw(fb: &mut Fb, cam_x: i32, view_native_w: i32, wall_screen_y: i32, scale: i32) {
    let mut room_x0 = 0;
    for room in WORLD {
        let room_x1 = room_x0 + room.width;
        let on_screen = room_x1 > cam_x && room_x0 < cam_x + view_native_w;
        if on_screen {
            let theme = room.theme;
            let tile_w = theme.w as i32;
            let mut tx = room_x0;
            while tx < room_x1 {
                let screen_x = (tx - cam_x) * scale;
                fb.blit_scaled(
                    theme.pixels,
                    theme.w,
                    theme.h,
                    screen_x,
                    wall_screen_y,
                    scale,
                    TRANSPARENT,
                    false,
                );
                tx += tile_w;
            }
            for prop in room.props {
                // prop.x is room-local; prop.y is native px down from the top of the wall art.
                let screen_x = (room_x0 + prop.x - cam_x) * scale;
                let screen_y = wall_screen_y + prop.y * scale;
                fb.blit_scaled(
                    prop.pixels,
                    prop.w,
                    prop.h,
                    screen_x,
                    screen_y,
                    scale,
                    TRANSPARENT,
                    false,
                );
            }
        }
        room_x0 = room_x1;
    }
}
