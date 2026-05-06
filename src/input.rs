use embassy_executor::Spawner;
use embassy_nrf::gpio::Input;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};

use crate::game::Button;

pub static EVENTS: Channel<CriticalSectionRawMutex, Button, 4> = Channel::new();

pub fn spawn(spawner: &Spawner, a: Input<'static>, b: Input<'static>, c: Input<'static>) {
    spawner.spawn(watch(a, Button::A).unwrap());
    spawner.spawn(watch(b, Button::B).unwrap());
    spawner.spawn(watch(c, Button::C).unwrap());
}

#[embassy_executor::task(pool_size = 3)]
async fn watch(mut pin: Input<'static>, btn: Button) {
    loop {
        pin.wait_for_falling_edge().await;
        // Reject transients (e.g. SPI coupling on neighboring port pins): a real press
        // holds the line low. If it bounces back high within 5 ms, drop it.
        Timer::after(Duration::from_millis(5)).await;
        if pin.is_high() {
            continue;
        }
        let _ = EVENTS.try_send(btn);
        // Wait for release + settle before re-arming.
        pin.wait_for_high().await;
        Timer::after(Duration::from_millis(20)).await;
    }
}
