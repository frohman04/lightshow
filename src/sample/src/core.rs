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
    fn sample(&mut self, screenshot: &Screenshot) -> Sample;
}
