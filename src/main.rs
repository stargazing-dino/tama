#![no_std]
#![no_main]

mod display;
mod fb;
#[allow(dead_code)]
mod cat;
mod input;
#[allow(dead_code)]
mod sprites;
mod world;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::yield_now;
use embassy_nrf::bind_interrupts;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::peripherals;
use embassy_nrf::spim::{self, Spim};
use embassy_time::{Delay, Instant};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use panic_probe as _;
use static_cell::ConstStaticCell;

use crate::fb::{FB_LEN, Fb, H, W};
use crate::cat::{Action, Cat};
use crate::input::Button;
use crate::sprites::{SPRITE_H, SPRITE_W, TRANSPARENT};

bind_interrupts!(struct Irqs {
    SERIAL00 => spim::InterruptHandler<peripherals::SERIAL00>;
});

const SCALE: i32 = 6;
const BG_COLOR: u16 = 0xFEDD;
// Wall stack is A+C+F+G (4 tiles tall, native 64px). Floor seam sits at native y=48.
const FLOOR_SEAM_NATIVE_Y: i32 = 48;

static FB_STORAGE: ConstStaticCell<[u8; FB_LEN]> = ConstStaticCell::new([0u8; FB_LEN]);
static SPI_BUF: ConstStaticCell<[u8; 64]> = ConstStaticCell::new([0u8; 64]);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
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
    let dy_centered = (H as i32 - sprite_px) / 2;
    // Cat sprite cells have a few empty rows at the bottom; nudge down so feet sit on the floor.
    const CAT_FOOT_NUDGE: i32 = 4;
    let cat_screen_y = dy_centered + CAT_FOOT_NUDGE;

    // Top of the wall art (in screen px). Floor seam aligns with the cat's nominal feet.
    let floor_anchor_y = dy_centered + SPRITE_H as i32 * SCALE;
    let wall_screen_y = floor_anchor_y - FLOOR_SEAM_NATIVE_Y * SCALE;

    let world_w = world::world_width();
    let view_native_w = W as i32 / SCALE;
    let mut cat = Cat::new(Instant::now(), world_w / 2);

    // D4=feed, D5=pet, D6=play. Active-low with internal pull-up.
    input::spawn(
        &spawner,
        Input::new(p.P1_10, Pull::Up),
        Input::new(p.P1_11, Pull::Up),
        Input::new(p.P2_08, Pull::Up),
    );

    loop {
        let action = match input::EVENTS.try_receive() {
            Ok(Button::A) => Some(Action::Feed),
            Ok(Button::B) => Some(Action::Pet),
            Ok(Button::C) => Some(Action::Play),
            Err(_) => None,
        };
        let pixels = cat.tick(Instant::now(), action, world_w);

        // Camera: keep cat near screen center, clamp to world bounds.
        let max_cam = (world_w - view_native_w).max(0);
        let cam_x = (cat.world_x - view_native_w / 2).clamp(0, max_cam);

        fb.fill(BG_COLOR);
        world::draw(&mut fb, cam_x, view_native_w, wall_screen_y, SCALE);

        let cat_screen_x = (cat.world_x - cam_x) * SCALE - sprite_px / 2;
        fb.blit_scaled(
            pixels,
            SPRITE_W,
            SPRITE_H,
            cat_screen_x,
            cat_screen_y,
            SCALE,
            TRANSPARENT,
            cat.facing < 0,
        );
        display.flush(&fb).unwrap();

        yield_now().await;
    }
}
