use svgdom::{SVGDom, Handle};
use xml5ever::tree_builder::TreeSink;
use xml5ever::tokenizer::Attribute;

pub struct InkApp {
    pub dom: SVGDom,
    renderables: Vec<RenderShape>,
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
            dom: SVGDom::default(),
            renderables: Vec::new(),
        }
    }

    pub fn get_doc_handle(&mut self) -> Handle {
        self.dom.get_document()
    }

    pub fn add_renderable(&mut self, shape: RenderShape, id: Option<&str>, attrs: &Vec<Attribute>) {
        self.renderables.push(shape);
    }
}