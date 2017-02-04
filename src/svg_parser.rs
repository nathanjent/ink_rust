
use inkapp::{InkApp, RenderShape};
use graphics::types::Color;
use svgdom::{Document, Element, Handle, Text as DomText};

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

pub fn walk(prefix: &str, mut app: &mut InkApp, doc: Handle) {

    let node = doc.borrow();

    print!("{}", prefix);
    match node.node {
        Document => println!("#document"),

        DomText(ref text) => println!("#text {}", escape_default(text)),

        Element(ref name, ref attrs) => {
            let lname = name.local.as_ref();
            println!("{:?}", lname);

            // attribute parsing
            let mut id = None;
            let mut style = Some("");
            let mut pos = [0.; 2];
            let mut size = [1.; 2];
            let mut radii = [0.; 2];
            let mut pos1 = [0.; 2];
            let mut pos2 = [0.; 2];
            let mut points = Vec::new();

            for attr in attrs {
                let key_val = (attr.name.local.as_ref(), attr.value.as_ref());
                match key_val {
                    v @ ("id", _) => {
                        let (_, v) = v;
                        id = Some(v);
                        println!("id: {:?}", id);
                    },
                    v @ ("style", _) => {
                        let (_, v) = v;
                        style = Some(v);
                        println!("style: {:?}", v);
                    },
                    v @ ("x", _) => {
                        let (_, v) = v;
                        pos[0] = v.parse::<f64>().unwrap_or(0.);
                        println!("x: {:?}", pos[0]);
                    },
                    v @ ("y", _) => {
                        let (_, v) = v;
                        pos[1] = v.parse::<f64>().unwrap_or(0.);
                        println!("y: {:?}", pos[1]);
                    },
                    v @ ("width", _) => {
                        let (_, v) = v;
                        size[0] = v.parse::<f64>().unwrap_or(1.);
                        println!("width: {:?}", size[0]);
                    },
                    v @ ("height", _) => {
                        let (_, v) = v;
                        size[1] = v.parse::<f64>().unwrap_or(1.);
                        println!("height: {:?}", size[1]);
                    },
                    v @ ("rx", _) => {
                        let (_, v) = v;
                        radii[0] = v.parse::<f64>().unwrap_or(1.);
                        println!("rx: {:?}", radii[0]);
                    },
                    v @ ("ry", _) => {
                        let (_, v) = v;
                        radii[1] = v.parse::<f64>().unwrap_or(1.);
                        println!("ry: {:?}", radii[1]);
                    },
                    v @ ("cx", _) => {
                        let (_, v) = v;
                        pos[0] = v.parse::<f64>().unwrap_or(0.);
                        println!("cx: {:?}", pos[0]);
                    },
                    v @ ("cy", _) => {
                        let (_, v) = v;
                        pos[1] = v.parse::<f64>().unwrap_or(0.);
                        println!("cy: {:?}", pos[1]);
                    },
                    v @ ("r", _) => {
                        let (_, v) = v;
                        radii = [v.parse::<f64>().unwrap_or(1.); 2];
                        println!("r: {:?}", radii);
                    },
                    v @ ("x1", _) => {
                        let (_, v) = v;
                        pos1[0] = v.parse::<f64>().unwrap_or(0.);
                        println!("x1: {:?}", pos1[0]);
                    },
                    v @ ("y1", _) => {
                        let (_, v) = v;
                        pos1[1] = v.parse::<f64>().unwrap_or(0.);
                        println!("y1: {:?}", pos1[1]);
                    },
                    v @ ("x2", _) => {
                        let (_, v) = v;
                        pos2[0] = v.parse::<f64>().unwrap_or(0.);
                        println!("x2: {:?}", pos2[0]);
                    },
                    v @ ("y2", _) => {
                        let (_, v) = v;
                        pos2[1] = v.parse::<f64>().unwrap_or(0.);
                        println!("y2: {:?}", pos2[1]);
                    },
                    v @ ("points", _) => {
                        let (_, v) = v;
                        points = v.split_whitespace()
                            .map(|s| s.split_at(s.find(',').expect("Point separator error.")))
                            .map(|(x, y)| { (x.parse::<f64>().ok(), y.parse::<f64>().ok()) })
                            .collect();
                        println!("points: {:?}", v);
                    },
                    _ => {},
                }
            }

            // Style parsing
            let mut fill_color = None;
            let mut fill_opacity = None;
            let mut stroke_color = None;
            let mut stroke_opacity = None;
            let mut stroke_width = None;
            let mut stroke_linecap = LineCap::Butt;
            let mut stroke_linejoin = LineJoin::Miter;
            let mut stroke_miterlimit = None;
            let mut stroke_dasharray = DashArray::None;
            for (name, val) in style.unwrap_or("").split_terminator(';')
                .map(|s| s.split_at(s.find(':').unwrap_or(s.len()))) {
                let (_, val) = val.split_at(1);
                match name {
                    "fill" => {
                        if val.starts_with('#') {
                            let (_, hex_str) = val.split_at(1);
                            fill_color = parse_color_hash(hex_str).ok();
                        }
                        println!("fill:#{:?}", val);
                    },
                    "fill-opacity" => {
                        fill_opacity = val.parse::<f64>().ok();
                        println!("fill-opacity:{:?}", val);
                    },
                    "stroke" => {
                        if val.starts_with('#') {
                            let (_, hex_str) = val.split_at(1);
                            stroke_color = parse_color_hash(hex_str).ok();
                        }
                        println!("stroke:#{:?}", val);
                    },
                    "stroke-opacity" => {
                        stroke_opacity = val.parse::<f64>().ok();
                        println!("stroke-opacity:{:?}", val);
                    },
                    "stroke-width" => {
                        stroke_width = val.parse::<f64>().ok();
                        println!("stroke-width:{:?}", val);
                    },
                    "stroke-linecap" => {
                        stroke_linecap = match val {
                            "butt" => LineCap::Butt,
                            "round" => LineCap::Round,
                            "square" => LineCap::Square,
                            "inherit" => LineCap::Inherit,
                            _ => LineCap::Inherit,
                        };
                    },
                    "stroke-linejoin" => {
                        stroke_linejoin = match val {
                            "miter" => LineJoin::Miter,
                            "round" => LineJoin::Round,
                            "bevel" => LineJoin::Bevel,
                            "inherit" => LineJoin::Inherit,
                            _ => LineJoin::Inherit,
                        };
                    },
                    "stroke-miterlimit" => {
                        stroke_miterlimit = val.parse::<f64>().ok();
                    },
                    "stroke-dasharray" => {
                        stroke_dasharray = match val {
                            "none" => DashArray::None,
                            "inherit" => DashArray::Inherit,
                            _ => {
                                let values = val
                                    .split(|c: char| (c.is_whitespace() && c == ','))
                                    .filter_map(|s| s.parse::<f64>().ok())
                                    .collect::<Vec<_>>();
                                DashArray::DashArray(values)
                            }
                        };
                    },
                    _ => continue,
                }
            }

            match lname {
                "rect" => {
                    app.add_renderable(RenderShape::Rectangle, id, attrs);
                },
                "ellipse" => {
                    app.add_renderable(RenderShape::Ellipse, id, attrs);
                },
                "circle" => {
                    app.add_renderable(RenderShape::CircleArc, id, attrs);
                },
                "line" => {
                    app.add_renderable(RenderShape::Line, id, attrs);
                },
                "image" => {
                    app.add_renderable(RenderShape::Image, id, attrs);
                },
                "polygon" => {
                    app.add_renderable(RenderShape::Polygon, id, attrs);
                },
                "text" => {
                    app.add_renderable(RenderShape::Text, id, attrs);
                },
                _ => {},
            }
        },
        _ => {},
    }

    let new_indent = {
        let mut temp = String::new();
        temp.push_str(prefix);
        temp.push_str("    ");
        temp
    };

    for child in node.children
        .iter()
        .filter(|child| match child.borrow().node {
            DomText(_) | Element(_, _) => true,
            _ => false,
        }) {
        walk(&new_indent, &mut app, child.clone());
    }
}


enum LineCap {
    Butt,
    Round,
    Square,
    Inherit,
}

enum LineJoin {
    Miter,
    Round,
    Bevel,
    Inherit,
}

enum DashArray {
    DashArray(Vec<f64>),
    None,
    Inherit,
}

fn parse_color_hash(value: &str) -> Result<Color, ()> {
    let value = value.as_bytes();
    match value.len() {
        8 => {
            rgba((try!(from_hex(value[0])) * 16 + try!(from_hex(value[1]))) as f32,
                 (try!(from_hex(value[2])) * 16 + try!(from_hex(value[3]))) as f32,
                 (try!(from_hex(value[4])) * 16 + try!(from_hex(value[5]))) as f32,
                 (try!(from_hex(value[6])) * 16 + try!(from_hex(value[7]))) as f32)
        }
        6 => {
            rgb((try!(from_hex(value[0])) * 16 + try!(from_hex(value[1]))) as f32,
                (try!(from_hex(value[2])) * 16 + try!(from_hex(value[3]))) as f32,
                (try!(from_hex(value[4])) * 16 + try!(from_hex(value[5]))) as f32)
        }
        4 => {
            rgba((try!(from_hex(value[0])) * 17) as f32,
                 (try!(from_hex(value[1])) * 17) as f32,
                 (try!(from_hex(value[2])) * 17) as f32,
                 (try!(from_hex(value[3])) * 17) as f32)
        }
        3 => {
            rgb((try!(from_hex(value[0])) * 17) as f32,
                (try!(from_hex(value[1])) * 17) as f32,
                (try!(from_hex(value[2])) * 17) as f32)
        }
        _ => Err(()),
    }
}

fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Result<Color, ()> {
    Ok([red / 255., green / 255., blue / 255., alpha / 255.])
}

fn rgb(red: f32, green: f32, blue: f32) -> Result<Color, ()> {
    Ok([red / 255., green / 255., blue / 255., 1.])
}

fn from_hex(c: u8) -> Result<u8, ()> {
    match c {
        b'0'...b'9' => Ok(c - b'0'),
        b'a'...b'f' => Ok(c - b'a' + 10),
        b'A'...b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}