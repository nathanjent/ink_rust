use errors::*;

use svgdom::Document;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use svg_parser;
use display;

pub struct InkApp {
    // ui: Ui,
    view: [f64; 4], // [x, y, width, height]
    window: [u32; 2], // [width, height]

    // for rebuilding after changes
    rebuild_queued: bool,
    redraw_queued: bool,

    // Upon drawing, we draw once more in the next frame
    redraw_echo_queued: bool,
    pub current_style: Style,
    pub dom: Document,
}

impl InkApp {
    pub fn new() -> Self {
        InkApp {
            view: [-10., -10., 600., 400.],
            window: [640, 480],
            rebuild_queued: false,
            redraw_queued: false,
            redraw_echo_queued: false,
            current_style: Style::new(),
            dom: Document::default(),
        }
    }

    pub fn open<T: Into<String> + AsRef<Path>>(&mut self, file: T) -> Result<()> {
        let mut file = File::open(&file).chain_err(|| "Unable to open file")?;
        let t = load_file(&mut file).chain_err(|| "Unable to load file")?;

        self.dom = match Document::from_data(&t) {
            Ok(doc) => doc,
            Err(e) => bail!("SVG parse error: {}", e),
        };

        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        display::load(self).chain_err(|| "Unable to load display")?;
        Ok(())
    }
}

pub struct Style {
    pub show_points: bool,
    pub fill: [f32; 3],
    pub stroke: [f32; 3],
}

impl Style {
    pub fn new() -> Self {
        Style {
            show_points: false,
            fill: [1., 1., 1.],
            stroke: [0., 0., 0.],
        }
    }
}

fn load_file(file: &mut File) -> Result<Vec<u8>> {
    let mut v = Vec::new();
    file.read_to_end(&mut v)
        .chain_err(|| "Unable to read file")?;
    Ok(v)
}
