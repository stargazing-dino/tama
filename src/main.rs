#![no_std]
#![no_main]

mod display;
mod fb;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::bind_interrupts;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::peripherals;
use embassy_nrf::spim::{self, Spim};
use embassy_time::{Delay, Instant};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyleBuilder};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use panic_probe as _;
use static_cell::ConstStaticCell;

use crate::fb::{FB_LEN, Fb, H, W};

bind_interrupts!(struct Irqs {
    SERIAL00 => spim::InterruptHandler<peripherals::SERIAL00>;
});

const BALL_R: i32 = 16;

static FB_STORAGE: ConstStaticCell<[u8; FB_LEN]> = ConstStaticCell::new([0u8; FB_LEN]);
static SPI_BUF: ConstStaticCell<[u8; 64]> = ConstStaticCell::new([0u8; 64]);

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.clock_speed = embassy_nrf::config::ClockSpeed::CK128;
    let p = embassy_nrf::init(nrf_config);
    defmt::info!("tama: bringing up GC9A01 @ 128 MHz");

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

    let ball_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::CYAN)
        .stroke_width(3)
        .fill_color(Rgb565::MAGENTA)
        .build();

    let mut x: i32 = 80;
    let mut y: i32 = 60;
    let mut vx: i32 = 4;
    let mut vy: i32 = 3;

    let mut frames: u32 = 0;
    let mut window_start = Instant::now();
    let mut t_clear: u64 = 0;
    let mut t_draw: u64 = 0;
    let mut t_push: u64 = 0;

    loop {
        let t0 = Instant::now();
        fb.clear();

        let t1 = Instant::now();
        Circle::new(Point::new(x, y), (BALL_R * 2) as u32)
            .into_styled(ball_style)
            .draw(&mut fb)
            .unwrap();

        let t2 = Instant::now();
        display.flush(&fb).unwrap();
        let t3 = Instant::now();

        t_clear += (t1 - t0).as_micros();
        t_draw += (t2 - t1).as_micros();
        t_push += (t3 - t2).as_micros();

        x += vx;
        y += vy;
        if x <= 0 || x + BALL_R * 2 >= W as i32 {
            vx = -vx;
            x = x.clamp(0, W as i32 - BALL_R * 2);
        }
        if y <= 0 || y + BALL_R * 2 >= H as i32 {
            vy = -vy;
            y = y.clamp(0, H as i32 - BALL_R * 2);
        }

        frames += 1;
        if frames >= 30 {
            let elapsed_ms = window_start.elapsed().as_millis();
            let fps = (frames as u64 * 1000) / elapsed_ms.max(1);
            defmt::info!(
                "{} frames in {}ms = {} fps | per-frame avg: clear={}us draw={}us push={}us",
                frames,
                elapsed_ms,
                fps,
                t_clear / frames as u64,
                t_draw / frames as u64,
                t_push / frames as u64,
            );
            frames = 0;
            t_clear = 0;
            t_draw = 0;
            t_push = 0;
            window_start = Instant::now();
        }
    }
}
