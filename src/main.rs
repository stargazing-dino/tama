#![no_std]
#![no_main]

mod display;
mod fb;
#[allow(dead_code)]
mod game;
#[allow(dead_code)]
mod sprites;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::bind_interrupts;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::peripherals;
use embassy_nrf::spim::{self, Spim};
use embassy_time::{Delay, Duration, Instant, Timer};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use panic_probe as _;
use static_cell::ConstStaticCell;

use crate::fb::{FB_LEN, Fb, H, W};
use crate::game::{Button, Cat};
use crate::sprites::{SPRITE_H, SPRITE_W, TRANSPARENT, WALL_H, WALL_PIXELS, WALL_W};

bind_interrupts!(struct Irqs {
    SERIAL00 => spim::InterruptHandler<peripherals::SERIAL00>;
});

const SCALE: i32 = 6;
const WALL_SCALE: i32 = 6;
const TICK_MS: u64 = 50;
const BG_COLOR: u16 = 0xFEDD;

static FB_STORAGE: ConstStaticCell<[u8; FB_LEN]> = ConstStaticCell::new([0u8; FB_LEN]);
static SPI_BUF: ConstStaticCell<[u8; 64]> = ConstStaticCell::new([0u8; 64]);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.clock_speed = embassy_nrf::config::ClockSpeed::CK128;
    let p = embassy_nrf::init(nrf_config);
    defmt::info!("tama: here, kitty kitty");

    let mut spi_config = spim::Config::default();
    spi_config.frequency = spim::Frequency::M32;
    spi_config.mode = spim::MODE_0;

    let spim = Spim::new(p.SERIAL00, Irqs, p.P2_01, p.P2_04, p.P2_02, spi_config);
    let cs = Output::new(p.P1_05, Level::High, OutputDrive::Standard);
    let dc = Output::new(p.P1_04, Level::Low, OutputDrive::Standard);
    let rst = Output::new(p.P1_06, Level::High, OutputDrive::Standard);

    let spi_device = ExclusiveDevice::new(spim, cs, NoDelay).unwrap();
    let mut display = display::init(spi_device, dc, SPI_BUF.take(), rst, &mut Delay);

    let mut fb = Fb::new(FB_STORAGE.take());

    let sprite_px = SPRITE_W as i32 * SCALE;
    let dx = (W as i32 - sprite_px) / 2;
    let dy_centered = (H as i32 - sprite_px) / 2;
    // Cat sprite cells have a few empty rows at the bottom; nudge down so feet sit on the floor.
    const CAT_FOOT_NUDGE: i32 = 4;
    let dy = dy_centered + CAT_FOOT_NUDGE;

    let mut cat = Cat::new(Instant::now());

    // D4=feed, D5=pet, D6=play. Active-low with internal pull-up.
    let buttons = [
        (Input::new(p.P1_10, Pull::Up), Button::A),
        (Input::new(p.P1_11, Pull::Up), Button::B),
        (Input::new(p.P2_08, Pull::Up), Button::C),
    ];
    let mut last_high = [true; 3];

    let wall_w_px = WALL_W as i32 * WALL_SCALE;
    // Asset stacks A + C + F + G (ceiling, wall-top, wall, floor). Top of G sits at the cat's nominal feet.
    const FLOOR_SEAM_NATIVE_Y: i32 = 48;
    let floor_anchor_y = dy_centered + SPRITE_H as i32 * SCALE;
    let wall_y = floor_anchor_y - FLOOR_SEAM_NATIVE_Y * WALL_SCALE;
    let n_tiles = (W as i32 + wall_w_px - 1) / wall_w_px;
    let wall_x_start = -(n_tiles * wall_w_px - W as i32) / 2;

    loop {
        let mut press = None;
        for (i, (pin, btn)) in buttons.iter().enumerate() {
            let high = pin.is_high();
            if last_high[i] && !high && press.is_none() {
                press = Some(*btn);
            }
            last_high[i] = high;
        }
        let pixels = cat.tick(Instant::now(), press);
        fb.fill(BG_COLOR);
        let mut wx = wall_x_start;
        while wx < W as i32 {
            fb.blit_scaled(
                &WALL_PIXELS,
                WALL_W,
                WALL_H,
                wx,
                wall_y,
                WALL_SCALE,
                TRANSPARENT,
            );
            wx += wall_w_px;
        }
        fb.blit_scaled(pixels, SPRITE_W, SPRITE_H, dx, dy, SCALE, TRANSPARENT);
        display.flush(&fb).unwrap();
        Timer::after(Duration::from_millis(TICK_MS)).await;
    }
}
