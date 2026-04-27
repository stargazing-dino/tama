#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use nrf_pac as pac;
use pac::gpio::vals::Dir;
use panic_probe as _;

#[entry]
fn main() -> ! {
    defmt::info!("hello from tama");

    // Onboard LED0: P2.00, active LOW.
    pac::P2_S.pin_cnf(0).write(|w| w.set_dir(Dir::OUTPUT));
    // XIAO header D0: P1.04, treating external LED as active HIGH.
    pac::P1_S.pin_cnf(4).write(|w| w.set_dir(Dir::OUTPUT));

    let mut on = false;
    loop {
        on = !on;
        if on {
            pac::P2_S.outclr().write(|w| w.set_pin(0, true)); // onboard on
            pac::P1_S.outset().write(|w| w.set_pin(4, true)); // D0 on
            defmt::info!("LEDs on");
        } else {
            pac::P2_S.outset().write(|w| w.set_pin(0, true)); // onboard off
            pac::P1_S.outclr().write(|w| w.set_pin(4, true)); // D0 off
            defmt::info!("LEDs off");
        }
        cortex_m::asm::delay(32_000_000);
    }
}
