use crate::fb::Fb;
use crate::sprites::{
    THEME_BATH_H, THEME_BATH_PIXELS, THEME_BATH_W, THEME_KITCHEN_H, THEME_KITCHEN_PIXELS,
    THEME_KITCHEN_W, THEME_LIVING_H, THEME_LIVING_PIXELS, THEME_LIVING_W, TRANSPARENT,
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

static THEME_LIVING: Theme = Theme {
    w: THEME_LIVING_W,
    h: THEME_LIVING_H,
    pixels: &THEME_LIVING_PIXELS,
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

pub static WORLD: &[Room] = &[
    Room { width: 80, theme: &THEME_LIVING, props: &[] },
    Room { width: 80, theme: &THEME_KITCHEN, props: &[] },
    Room { width: 80, theme: &THEME_BATH, props: &[] },
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
