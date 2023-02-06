use dxgcap::BGRA8;
use ls_screenshot::Screenshot;

use crate::core::{Sample, Sampler};

/// Sampler that outputs either red, green, or blue values of increasing intensity.  Useful for
/// basic debugging without the need to doing actual image sampling.
pub struct DummySampler {
    width: usize,
    height: usize,
}

impl DummySampler {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Sampler for DummySampler {
    fn sample(&mut self, _screenshot: &Screenshot) -> Sample {
        let length = self.width * 2 + self.height * 2 - 4;
        let min_color = (u8::MAX as f32 * 0.25).floor() as u8;
        let inc = (u8::MAX - min_color) / length as u8;
        let mut pixels = Vec::with_capacity(length);
        for i in 0..length {
            pixels.push(BGRA8 {
                b: if i % 3 == 0 {
                    0
                } else {
                    inc * i as u8 + min_color
                },
                g: if i % 3 == 1 {
                    0
                } else {
                    inc * i as u8 + min_color
                },
                r: if i % 3 == 2 {
                    0
                } else {
                    inc * i as u8 + min_color
                },
                a: 100,
            });
        }
        Sample::new(pixels, self.width, self.height)
    }
}
