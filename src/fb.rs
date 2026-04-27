use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

pub const W: usize = 240;
pub const H: usize = 240;
pub const FB_LEN: usize = W * H * 2;

pub struct Fb<'a> {
    data: &'a mut [u8; FB_LEN],
}

impl<'a> Fb<'a> {
    pub fn new(data: &'a mut [u8; FB_LEN]) -> Self {
        Self { data }
    }

    pub fn clear(&mut self) {
        unsafe {
            core::ptr::write_bytes(self.data.as_mut_ptr(), 0, FB_LEN);
        }
    }

    pub fn bytes(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl OriginDimensions for Fb<'_> {
    fn size(&self) -> Size {
        Size::new(W as u32, H as u32)
    }
}

impl DrawTarget for Fb<'_> {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(p, c) in pixels {
            if p.x >= 0 && p.x < W as i32 && p.y >= 0 && p.y < H as i32 {
                let idx = (p.y as usize * W + p.x as usize) * 2;
                let raw = c.into_storage();
                self.data[idx] = (raw >> 8) as u8;
                self.data[idx + 1] = raw as u8;
            }
        }
        Ok(())
    }
}
