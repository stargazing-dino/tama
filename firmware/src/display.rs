use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;
use mipidsi::Builder;
use mipidsi::interface::{Interface, SpiInterface};
use mipidsi::models::GC9A01;
use mipidsi::options::{ColorInversion, ColorOrder};

use crate::fb::{Fb, H, W};

pub struct Display<DI: Interface<Word = u8>, RST: OutputPin> {
    inner: mipidsi::Display<DI, GC9A01, RST>,
}

impl<DI: Interface<Word = u8>, RST: OutputPin> Display<DI, RST> {
    pub fn flush(&mut self, fb: &Fb) -> Result<(), DI::Error> {
        // SAFETY: we only issue RAMWR + pixel data with the address window already
        // set at init. We don't change orientation, sleep state, or any other field
        // mipidsi tracks, so the Display's invariants are preserved.
        unsafe { self.inner.dcs() }.send_command(0x2C, fb.bytes())
    }
}

pub fn init<'b, SPI, DC, RST>(
    spi: SPI,
    dc: DC,
    buffer: &'b mut [u8],
    rst: RST,
    delay: &mut impl DelayNs,
) -> Display<SpiInterface<'b, SPI, DC>, RST>
where
    SPI: SpiDevice<u8>,
    DC: OutputPin,
    RST: OutputPin,
{
    let di = SpiInterface::new(spi, dc, buffer);
    let mut inner = Builder::new(GC9A01, di)
        .reset_pin(rst)
        .invert_colors(ColorInversion::Inverted)
        .color_order(ColorOrder::Bgr)
        .init(delay)
        .unwrap();

    {
        // SAFETY: mipidsi has finished init; we're only setting the address window
        // for full-screen pushes. mipidsi doesn't cache window state.
        let di = unsafe { inner.dcs() };
        set_addr_window(di, 0, 0, (W as u16) - 1, (H as u16) - 1).unwrap();
    }

    Display { inner }
}

fn set_addr_window<DI: Interface<Word = u8>>(
    di: &mut DI,
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
) -> Result<(), DI::Error> {
    di.send_command(
        0x2A,
        &[(x0 >> 8) as u8, x0 as u8, (x1 >> 8) as u8, x1 as u8],
    )?;
    di.send_command(
        0x2B,
        &[(y0 >> 8) as u8, y0 as u8, (y1 >> 8) as u8, y1 as u8],
    )?;
    Ok(())
}
