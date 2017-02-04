//use svgdom::{SVGDom, Handle};
use svgparser::svg;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use errors::*;

pub struct InkApp {
//    pub dom: SVGDom,
    renderables: Vec<RenderShape>,
//    ui: Ui,
    view: [f64; 4], // [x, y, width, height]
    window: [u32; 2], // [width, height]
    
    // for rebuilding after changes
    rebuild_queued: bool,
    redraw_queued: bool,

    // Upon drawing, we draw once more in the next frame
    redraw_echo_queued: bool,
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
    pub fn open<T: Into<String> + AsRef<Path>>(file: T) -> Result<()> {
        let mut file = File::open(&file)
            .chain_err(|| "Unable to open file")?;
        let t = load_file(&mut file)
        .chain_err(|| "Unable to load file")?;

        let mut p = svg::Tokenizer::new(&t);
        Ok(())
    }
}

fn load_file(file: &mut File) -> Result<Vec<u8>> {
    let mut v = Vec::new();
    file.read_to_end(&mut v)
        .chain_err(|| "Unable to read file")?;
    Ok(v)
}
