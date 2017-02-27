use errors::*;

use svgdom::Document;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use svg_parser;
use display;

pub struct InkApp {
    // pub dom: SVGDom,
    renderables: Vec<RenderShape>,
    // ui: Ui,
    view: [f64; 4], // [x, y, width, height]
    window: [u32; 2], // [width, height]

    // for rebuilding after changes
    rebuild_queued: bool,
    redraw_queued: bool,

    // Upon drawing, we draw once more in the next frame
    redraw_echo_queued: bool,
    pub show_points: bool,
    pub dom: Document,
}

pub enum RenderShape {
    Rectangle,
    Line,
    Ellipse,
    CircleArc,
    Image,
    Polygon,
    Text,
}

impl InkApp {
    pub fn new() -> Self {
        InkApp {
            renderables: Vec::new(),
            view: [-10., -10., 600., 400.],
            window: [640, 480],
            rebuild_queued: false,
            redraw_queued: false,
            redraw_echo_queued: false,
            show_points: false,
            dom: Document::default(),
        }
    }

    pub fn open<T: Into<String> + AsRef<Path>>(&mut self, file: T) -> Result<()> {
        let mut file = File::open(&file).chain_err(|| "Unable to open file")?;
        let t = load_file(&mut file).chain_err(|| "Unable to load file")?;

        let svg = svg_parser::parse(&t);
        self.dom = match Document::from_data(&t) {
            Ok(doc) => doc,
            Err(e) => bail!("SVG parse error: {}", e),
        };

        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        display::load(self);
        Ok(())
    }

    pub fn add_renderable(&mut self, renderable: RenderShape) {
        self.renderables.push(renderable);
    }
}

fn load_file(file: &mut File) -> Result<Vec<u8>> {
    let mut v = Vec::new();
    file.read_to_end(&mut v)
        .chain_err(|| "Unable to read file")?;
    Ok(v)
}
