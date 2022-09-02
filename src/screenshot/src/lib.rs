use dxgcap::{DXGIManager, BGRA8};

pub struct Screenshot {
    pub pixels: Vec<BGRA8>,
    pub width: usize,
    pub height: usize,
}

impl Screenshot {
    fn new(pixels: Vec<BGRA8>, width: usize, height: usize) -> Screenshot {
        Screenshot {
            pixels,
            width,
            height,
        }
    }
}

pub struct Screenshotter {
    manager: DXGIManager,
}

impl Screenshotter {
    pub fn new() -> anyhow::Result<Screenshotter> {
        Ok(Screenshotter {
            manager: DXGIManager::new(50).map_err(anyhow::Error::msg)?,
        })
    }

    pub fn capture(&mut self) -> anyhow::Result<Screenshot> {
        let ss = self
            .manager
            .capture_frame()
            .map_err(|err| anyhow::Error::msg(format!("{:?}", err)))?;
        Ok(Screenshot::new(ss.0, ss.1 .0, ss.1 .1))
    }
}
