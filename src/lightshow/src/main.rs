use ls_screenshot::Screenshotter;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use time::OffsetDateTime;
use tracing::{info, info_span, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let ansi_enabled = fix_ansi_term();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_ansi(ansi_enabled)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut screenshotter = {
        let span = info_span!("Construct new screenshotter");
        let _guard = span.enter();

        let start = OffsetDateTime::now_utc();
        let screenshotter = Screenshotter::new().expect("Unable to create screenshotter");
        let end = OffsetDateTime::now_utc();
        info!(duration = (end - start).as_seconds_f64());

        screenshotter
    };

    let screenshot = {
        let span = info_span!("Capture screenshot");
        let _guard = span.enter();

        let start = OffsetDateTime::now_utc();
        let screenshot = screenshotter
            .capture()
            .expect("Unable to capture screenshot");
        let end = OffsetDateTime::now_utc();
        info!(duration = (end - start).as_seconds_f64());

        screenshot
    };

    {
        let span = info_span!("Write screenshot to disk");
        let _guard = span.enter();

        let start = OffsetDateTime::now_utc();
        let path = Path::new("screenshot");
        let mut ppm_file = if path.extension() != Some(OsStr::new("ppm")) {
            fs::File::create(path.with_extension("ppm")).expect("Unable to create output file")
        } else {
            fs::File::create(path).expect("Unable to create output file")
        };
        ppm_file
            .write_all("P3\n".as_bytes())
            .expect("Unable to write to file");
        ppm_file
            .write_all(format!("{} {}\n", screenshot.width, screenshot.height).as_bytes())
            .expect("Unable to write to file");
        ppm_file
            .write_all("255\n".as_bytes())
            .expect("Unable to write to file");
        for pixel in screenshot.pixels.iter() {
            ppm_file
                .write_all(format!("{} {} {}\n", pixel.r, pixel.g, pixel.b).as_bytes())
                .expect("Unable to write to file");
        }

        let end = OffsetDateTime::now_utc();
        info!(duration = (end - start).as_seconds_f64());
    };
}

#[cfg(target_os = "windows")]
fn fix_ansi_term() -> bool {
    nu_ansi_term::enable_ansi_support().map_or(false, |()| true)
}

#[cfg(not(target_os = "windows"))]
fn fix_ansi_term() -> bool {
    true
}
