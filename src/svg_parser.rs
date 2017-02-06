use svgparser::*;
// use graphics::types::Color;
use errors::*;

use std::collections::HashMap;
use std::str;

// use inkapp::{InkApp, RenderShape};
// use svgdom::{Document, Element, Handle, Text as DomText};

#[derive(Debug)]
struct SvgElement<'dom> {
    elem: Option<ElementId>,
    attrs: HashMap<AttributeId, AttributeValue<'dom>>,
    text: Option<Stream<'dom>>,
    children: Vec<SvgElement<'dom>>,
}

impl<'dom> SvgElement<'dom> {
    fn new() -> Self {
        SvgElement {
            elem: None,
            attrs: HashMap::new(),
            text: None,
            children: Vec::new(),
        }
    }
}

pub fn parse(svg_str: &[u8]) -> Result<()> {
    let mut p = svg::Tokenizer::new(&svg_str);
    let mut element = SvgElement::new();
    let mut elements = Vec::new();
    loop {
        if let Ok(svg_token) = p.parse_next() {
            match svg_token {
                svg::Token::ElementStart(elem) => {
                    element.elem = ElementId::from_name(u8_to_str!(elem));
                }
                svg::Token::ElementEnd(end) => {
                    match end {
                        svg::ElementEnd::Open => {}
                        svg::ElementEnd::Close(elem) => {
                            element.elem = ElementId::from_name(u8_to_str!(elem));
                            if element.elem.is_some() {
                                elements.push(element);
                            }
                            element = SvgElement::new();
                        }
                        svg::ElementEnd::Empty => {
                            if element.elem.is_some() {
                                elements.push(element);
                            }
                            element = SvgElement::new();
                        }
                    }
                }
                svg::Token::Attribute(name, mut value) => {
                    match name {
                        b"style" => {
                            let mut style = style::Tokenizer::new(value);
                            loop {
                                if let Ok(style_token) = style.parse_next() {
                                    match style_token {
                                        style::Token::Attribute(name, mut value) => {
                                            if let Some(attr) =
                                                AttributeId::from_name(u8_to_str!(name)) {
                                                if let Some(elem) = element.elem {
                                                    // println!("{:?}", id);
                                                    if let Ok(value) =
                                                        AttributeValue::from_stream(elem,
                                                                                    attr,
                                                                                    &mut value) {
                                                        element.attrs.insert(attr, value);
                                                    }
                                                }
                                            } else {
                                                // FEATURE process attributes not in spec
                                                // TODO retain for file saving
                                                // println!("{:?}:{:?}", name, value);
                                            }
                                        }
                                        style::Token::EntityRef(_entity) => {
                                            // TODO retain for file saving
                                        }
                                        style::Token::EndOfStream => break,
                                    }
                                } else {
                                    // TODO handle warning
                                    break;
                                }
                            }
                        }
                        b"d" => {
                            let mut p = path::Tokenizer::new(value);
                            loop {
                                if let Ok(segment_token) = p.parse_next() {
                                    match segment_token {
                                        path::SegmentToken::Segment(_segment) => {
                                            // println!("  {:?}", segment)
                                        }
                                        path::SegmentToken::EndOfStream => break,
                                    }
                                } else {
                                    // By SVG spec, invalid data occurred in the path should
                                    // stop parsing of this path, but not the whole document.
                                    // So we just show a warning and continue parsing.
                                    // println!("Warning: {:?}.", e);
                                    // TODO handle warning
                                    break;
                                }
                            }
                        }
                        _ => {
                            if let Some(attr) = AttributeId::from_name(u8_to_str!(name)) {
                                if let Some(elem) = element.elem {
                                    // println!("{:?}", id);
                                    if let Ok(value) = AttributeValue::from_stream(elem,
                                                                                   attr.clone(),
                                                                                   &mut value) {
                                        element.attrs.insert(attr, value);
                                    }
                                }
                            }
                        }
                    }
                }
                svg::Token::Text(text) => {
                    element.text = Some(text);
                }
                svg::Token::Cdata(_stream) => {
                    // TODO retain for file saving
                }
                svg::Token::Whitespace(_u8slice) => {}
                svg::Token::Comment(_u8slice) => {
                    // TODO retain for file saving
                }
                svg::Token::DtdEmpty(_u8slice) => {}
                svg::Token::DtdStart(_u8slice) => {}
                svg::Token::DtdEnd => {}
                svg::Token::Entity(_u8slice, _stream) => {}
                svg::Token::Declaration(_u8slice) => {
                    // TODO Fail politely when not xml 1.0 and utf-8
                }
                svg::Token::EndOfStream => break,
            }
        } else {
            bail!("SVG parsing error.");
        }
    }
    println!("{:?}", elements);
    Ok(())
}
