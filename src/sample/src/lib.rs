use dxgcap::BGRA8;
use ls_screenshot::Screenshot;

/// An edge sampling of the colors of a screenshot, specified as a circle of pixels bordering the
/// screen.
#[derive(Clone, Debug)]
pub struct Sample {
    /// Array of pixels, starting at top-left, going clockwise
    pub pixels: Vec<BGRA8>,
    pub width: usize,
    pub height: usize,
}

impl Sample {
    pub fn new(pixels: Vec<BGRA8>, width: usize, height: usize) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }
}

pub trait Sampler {
    fn sample(&self, screenshot: &Screenshot) -> Sample;
}

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
    fn sample(&self, _screenshot: &Screenshot) -> Sample {
        let length = self.width * 2 + self.height * 2;
        let inc = u8::MAX / length as u8;
        let mut pixels = Vec::with_capacity(length);
        for i in 0..length {
            pixels.push(BGRA8 {
                b: inc * i as u8,
                g: inc * i as u8,
                r: inc * i as u8,
                a: 100,
            });
        }
        Sample::new(pixels, self.width, self.height)
    }
}
