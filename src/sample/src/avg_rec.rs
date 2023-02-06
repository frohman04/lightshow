use dxgcap::BGRA8;
use ls_screenshot::Screenshot;
use time::OffsetDateTime;
use tracing::{info, info_span};

use crate::core::{Sample, Sampler};

/// Sampler that outputs the average color of each rectangular region of the edge of the screen.
pub struct AvgRectangleSampler {
    width: usize,
    height: usize,
    region_depth_px: usize,
    regions: Option<Vec<Region>>,
}

impl AvgRectangleSampler {
    /// Create a new sampler that will output `width` number of pixels across the screen and
    /// `height` number of pixels down the screen, measuring `region_depth` pixels into the screen.
    /// The size in pixels along the edge of the image will be set using the dimensions of the
    /// initial screenshot sampled from.
    pub fn new(width: usize, height: usize, region_depth_px: usize) -> Self {
        Self {
            width,
            height,
            region_depth_px,
            regions: None,
        }
    }

    fn gen_regions(
        num_x: usize,
        num_y: usize,
        depth_px: usize,
        img_width_px: usize,
        img_height_px: usize,
    ) -> Vec<Region> {
        let span = info_span!("Generating sampling regions");
        let _guard = span.enter();
        let start = OffsetDateTime::now_utc();

        let cell_width_px = img_width_px / num_x;
        let cell_height_px = img_height_px / num_y;

        let mut regions: Vec<Region> = Vec::with_capacity(num_x * 2 + (num_y - 2) * 2);

        regions.push(Region::new(0, cell_width_px, 0, cell_height_px));
        for i in 1..num_x - 1 {
            regions.push(Region::new(
                cell_height_px * i,
                cell_width_px * (i + 1),
                0,
                depth_px,
            ));
        }
        regions.push(Region::new(
            cell_width_px * (num_x - 1),
            img_width_px,
            0,
            cell_height_px,
        ));
        for i in 1..num_y - 1 {
            regions.push(Region::new(
                img_width_px - depth_px,
                img_width_px,
                cell_height_px * i,
                cell_height_px * (i + 1),
            ));
        }
        regions.push(Region::new(
            cell_width_px * (num_x - 1),
            img_width_px,
            cell_height_px * (num_y - 1),
            img_height_px,
        ));
        for i in (1..num_x - 1).rev() {
            regions.push(Region::new(
                cell_height_px * i,
                cell_width_px * (i + 1),
                img_height_px - depth_px,
                img_height_px,
            ));
        }
        regions.push(Region::new(
            0,
            cell_width_px,
            cell_height_px * (num_y - 1),
            img_height_px,
        ));
        for i in (1..num_y - 1).rev() {
            regions.push(Region::new(
                0,
                depth_px,
                cell_height_px * i,
                cell_height_px * (i + 1),
            ));
        }

        let end = OffsetDateTime::now_utc();
        let duration = (end - start).as_seconds_f64();
        info!(duration);

        regions
    }
}

impl Sampler for AvgRectangleSampler {
    fn sample(&mut self, screenshot: &Screenshot) -> Sample {
        if self.regions.is_none() {
            self.regions = Some(AvgRectangleSampler::gen_regions(
                self.width,
                self.height,
                self.region_depth_px,
                screenshot.width,
                screenshot.height,
            ));
        }

        let regions = self.regions.as_ref().unwrap();
        let pixels = regions
            .iter()
            .map(|region| {
                let sums = region
                    .iter()
                    .map(|(x, y)| {
                        let i = y * screenshot.width + x;
                        let px = screenshot.pixels[i];
                        (px.r, px.g, px.b, px.a)
                    })
                    .fold(
                        (
                            RunningAverage::default(),
                            RunningAverage::default(),
                            RunningAverage::default(),
                            RunningAverage::default(),
                        ),
                        |accum, elem| {
                            (
                                accum.0.push(elem.0),
                                accum.1.push(elem.1),
                                accum.2.push(elem.2),
                                accum.3.push(elem.3),
                            )
                        },
                    );
                let (r, g, b, a) = (sums.0.get(), sums.1.get(), sums.2.get(), sums.3.get());
                BGRA8 { b, g, r, a }
            })
            .collect();

        Sample::new(pixels, self.width, self.height)
    }
}

struct RunningAverage {
    avg: f32,
    count: usize,
}

impl Default for RunningAverage {
    fn default() -> Self {
        RunningAverage {
            avg: 0f32,
            count: 0,
        }
    }
}

impl RunningAverage {
    fn push(mut self, value: u8) -> Self {
        self.avg = ((self.avg * self.count as f32) + value as f32) / (self.count as f32 + 1f32);
        self.count += 1;

        self
    }

    fn get(&self) -> u8 {
        self.avg as u8
    }
}

#[derive(Debug)]
struct Region {
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
}

impl Region {
    fn new(start_x: usize, end_x: usize, start_y: usize, end_y: usize) -> Region {
        Region {
            start_x,
            end_x,
            start_y,
            end_y,
        }
    }

    fn iter(&self) -> RegionIter {
        RegionIter {
            region: self,
            x: self.start_x,
            y: self.start_y,
        }
    }
}

#[derive(Debug)]
struct RegionIter<'a> {
    region: &'a Region,
    x: usize,
    y: usize,
}

impl Iterator for RegionIter<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x + 1 >= self.region.end_x {
            if self.y + 1 >= self.region.end_y {
                None
            } else {
                let out = (self.x, self.y);
                self.x = self.region.start_x;
                self.y += 1;
                Some(out)
            }
        } else {
            let out = (self.x, self.y);
            self.x += 1;
            Some(out)
        }
    }
}
