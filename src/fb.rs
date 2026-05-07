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
        for chunk in self.data.chunks_exact_mut(2) {
            chunk[0] = hi;
            chunk[1] = lo;
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
        flip_x: bool,
    ) {
        const ROW_BYTES: usize = W * 2;
        for sy in 0..src_h {
            let y0 = dy + sy as i32 * scale;
            let y_start = y0.max(0);
            let y_end = (y0 + scale).min(H as i32);
            if y_start >= y_end {
                continue;
            }
            let row_count = (y_end - y_start) as usize;

            for sx in 0..src_w {
                let read_x = if flip_x { src_w - 1 - sx } else { sx };
                let raw = src[sy * src_w + read_x];
                if raw == transparent {
                    continue;
                }
                let hi = (raw >> 8) as u8;
                let lo = raw as u8;
                let x0 = dx + sx as i32 * scale;
                let x_start = x0.max(0);
                let x_end = (x0 + scale).min(W as i32);
                if x_start >= x_end {
                    continue;
                }
                let span_bytes = (x_end - x_start) as usize * 2;
                let mut row_off = y_start as usize * ROW_BYTES + x_start as usize * 2;
                for _ in 0..row_count {
                    let dst = &mut self.data[row_off..row_off + span_bytes];
                    for pair in dst.chunks_exact_mut(2) {
                        pair[0] = hi;
                        pair[1] = lo;
                    }
                    row_off += ROW_BYTES;
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
