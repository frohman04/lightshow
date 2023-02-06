#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::gui::Framework;
use ls_sample::{DummySampler, Sample, Sampler};
use ls_screenshot::{Screenshot, Screenshotter};
use pixels::{Error, Pixels, SurfaceTexture};
use std::cell::RefCell;
use std::rc::Rc;
use time::OffsetDateTime;
use tracing::{error, info, info_span, Level};
use tracing_subscriber::FmtSubscriber;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;

const IMAGE_WIDTH: u32 = 960;
const IMAGE_HEIGHT: u32 = 540;
const BUFFER: u32 = 50;
const WINDOW_WIDTH: u32 = IMAGE_WIDTH + BUFFER * 2;
const WINDOW_HEIGHT: u32 = IMAGE_HEIGHT + BUFFER * 2;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    screenshotter: Screenshotter,
    sampler: Box<dyn Sampler>,
    screenshot: Option<Screenshot>,
    sample: Option<Sample>,
}

fn main() -> Result<(), Error> {
    let ansi_enabled = fix_ansi_term();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_ansi(ansi_enabled)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Lightshow Sampler Demo")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let world = Rc::new(RefCell::new(World::new()));
    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)?;
        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
            world.clone(),
        );

        (pixels, framework)
    };

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.borrow_mut().update();
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world
                world.borrow_mut().draw(pixels.get_frame_mut());

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if render_result
                    .map_err(|e| error!("pixels.render() failed: {:?}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}

#[cfg(target_os = "windows")]
fn fix_ansi_term() -> bool {
    nu_ansi_term::enable_ansi_support().map_or(false, |()| true)
}

#[cfg(not(target_os = "windows"))]
fn fix_ansi_term() -> bool {
    true
}

impl gui::Capturer for World {
    fn capture(&mut self) -> f64 {
        let span = info_span!("Updating screenshot");
        let _guard = span.enter();

        let start = OffsetDateTime::now_utc();
        match self.screenshotter.capture() {
            Ok(screenshot) => {
                self.screenshot = Some(screenshot);

                self.sample = Some(self.sampler.sample(self.screenshot.as_ref().unwrap()));
            }
            Err(e) => {
                error!("Failed while capturing screenshot: {:?}", e)
            }
        };

        let end = OffsetDateTime::now_utc();
        let duration = (end - start).as_seconds_f64();
        info!(duration);
        duration
    }
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            screenshotter: Screenshotter::new().expect("Unable to create screenshotter"),
            sampler: Box::new(DummySampler::new(37, 22)),
            screenshot: None,
            sample: None,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {}

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        if let (Some(ss), Some(samp)) = (&self.screenshot, &self.sample) {
            let scale_x = ss.width as f32 / IMAGE_WIDTH as f32;
            let scale_y = ss.height as f32 / IMAGE_HEIGHT as f32;

            let buffer = BUFFER as f32;
            let img_start_x = BUFFER as f32;
            let img_end_x = (BUFFER + IMAGE_WIDTH) as f32;
            let img_start_y = BUFFER as f32;
            let img_end_y = (BUFFER + IMAGE_HEIGHT) as f32;

            let pixel_width = (IMAGE_WIDTH as f32 / samp.width as f32).ceil() as i32;
            let pixel_height = (IMAGE_HEIGHT as f32 / samp.height as f32).ceil() as i32;

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let canvas_x = (i % WINDOW_WIDTH as usize) as f32;
                let canvas_y = (i / WINDOW_WIDTH as usize) as f32;

                let in_left_buffer = canvas_x < img_start_x;
                let in_right_buffer = canvas_x >= img_end_x;
                let in_top_buffer = canvas_y < img_start_y;
                let in_bottom_buffer = canvas_y >= img_end_y;

                let rgba = match (
                    in_left_buffer,
                    in_top_buffer,
                    in_right_buffer,
                    in_bottom_buffer,
                ) {
                    // edge lights: top left corner
                    (true, true, false, false) => {
                        let px = samp.pixels[0];
                        [px.r, px.g, px.b, px.a]
                    }
                    // edge lights: top edge, going right
                    (false, true, false, false) => {
                        let px = samp.pixels
                            [((canvas_x - buffer) / pixel_width as f32).floor() as usize];
                        [px.r, px.g, px.b, px.a]
                    }
                    // edge lights: top right corner
                    (false, true, true, false) => {
                        let px = samp.pixels[samp.width - 1];
                        [px.r, px.g, px.b, px.a]
                    }
                    // edge lights: right edge, going down
                    (false, false, true, false) => {
                        let px = samp.pixels[samp.width - 1
                            + ((canvas_y - buffer) / pixel_height as f32).floor() as usize];
                        [px.r, px.g, px.b, px.a]
                    }
                    // edge lights: bottom right corner
                    (false, false, true, true) => {
                        let px = samp.pixels[samp.width - 1 + samp.height - 1];
                        [px.r, px.g, px.b, px.a]
                    }
                    // edge lights: bottom edge, going left
                    (false, false, false, true) => {
                        let px = samp.pixels[samp.width - 1 + samp.height - 1
                            + (samp.width - ((canvas_x - buffer) / pixel_width as f32) as usize)
                            - 1];
                        [px.r, px.g, px.b, px.a]
                    }
                    // edge lights: bottom left corner
                    (true, false, false, true) => {
                        let px = samp.pixels[samp.width - 1 + samp.height - 1 + samp.width - 1];
                        [px.r, px.g, px.b, px.a]
                    }
                    // edge lights: left edge, going up
                    (true, false, false, false) => {
                        let px = samp
                            .pixels
                            .get(
                                samp.width - 1 + samp.height - 1 + samp.width - 1
                                    + (samp.height
                                        - ((canvas_y - buffer) / pixel_height as f32) as usize)
                                    - 1,
                            )
                            .unwrap_or(&samp.pixels[0]);
                        [px.r, px.g, px.b, px.a]
                    }
                    (false, false, false, false) => {
                        let ss_x = ((canvas_x - buffer) * scale_x) as usize;
                        let ss_y = ((canvas_y - buffer) * scale_y) as usize;
                        let ss_i = ss_y * ss.width + ss_x;
                        let ss_pixel = ss.pixels[ss_i];

                        [ss_pixel.r, ss_pixel.g, ss_pixel.b, ss_pixel.a]
                    }
                    a => {
                        panic!("Impossible state (left, top, right, bottom): {a:?}")
                    }
                };

                pixel.copy_from_slice(&rgba);
            }
        }
    }
}
