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

    pub fn fill(&mut self, color: u16) {
        let hi = (color >> 8) as u8;
        let lo = color as u8;
        let mut i = 0;
        while i < FB_LEN {
            self.data[i] = hi;
            self.data[i + 1] = lo;
            i += 2;
        }
    }

    pub fn bytes(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn blit_scaled(
        &mut self,
        src: &[u16],
        src_w: usize,
        src_h: usize,
        dx: i32,
        dy: i32,
        scale: i32,
        transparent: u16,
    ) {
        for sy in 0..src_h {
            for sx in 0..src_w {
                let raw = src[sy * src_w + sx];
                if raw == transparent {
                    continue;
                }
                let hi = (raw >> 8) as u8;
                let lo = raw as u8;
                let x0 = dx + sx as i32 * scale;
                let y0 = dy + sy as i32 * scale;
                for py in 0..scale {
                    let y = y0 + py;
                    if y < 0 || y >= H as i32 {
                        continue;
                    }
                    for px in 0..scale {
                        let x = x0 + px;
                        if x < 0 || x >= W as i32 {
                            continue;
                        }
                        let idx = (y as usize * W + x as usize) * 2;
                        self.data[idx] = hi;
                        self.data[idx + 1] = lo;
                    }
                }
            }
        }
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
