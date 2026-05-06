use crate::sprites::{
    THEME_BATH_H, THEME_BATH_PIXELS, THEME_BATH_W, THEME_KITCHEN_H, THEME_KITCHEN_PIXELS,
    THEME_KITCHEN_W, THEME_LIVING_H, THEME_LIVING_PIXELS, THEME_LIVING_W,
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
