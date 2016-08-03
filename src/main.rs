#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate graphics;
extern crate xml5ever;
extern crate encoding;
extern crate cssparser;

use std::fs::File;
use std::io::Read;
use std::default::Default;
use std::iter;
use std::string::String;

use conrod::{self, Colorable, Labelable, Positionable, Sizeable, Theme, Widget};

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

use xml5ever::tendril::SliceExt;
use xml5ever::parse;
use xml5ever::tree_builder::TreeSink;
use xml5ever::rcdom;
use xml5ever::rcdom::{Document, Element, RcDom, Handle};

use self::svg_canvas::SVGCanvas;

struct InkApp {
    dom: RcDom,
    renderables: Vec<RenderShape>,
}

enum RenderShape {
    Rectangle(String, graphics::types::Rectangle),
    Line(String, graphics::types::Line),
    Ellipse(String, graphics::Ellipse),
    CircleArc(String, graphics::CircleArc),
    Image(String, graphics::Image),
//    Polygon(String, graphics::types::Polygon),
    Text(String, graphics::Text),
}

impl InkApp {
    fn new() -> Self {
        InkApp {
            dom: RcDom::default(),
            renderables: Vec::new(),
        }
    }

    fn set_dom(&mut self, dom: RcDom) {
        self.dom = dom;
    }

    fn get_doc_handle(&mut self) -> Handle {
        self.dom.get_document()
    }
}

fn main() {
    let mut file = File::open("tests/documents/testrect.svg").expect("File read error.");

    let mut file_string = String::new();
    if let Err(err) = file.read_to_string(&mut file_string) {
        println!("Reading failed: {}", err);
        std::process::exit(1);
    };

    let input = file_string.to_tendril();

    let dom: RcDom = parse(iter::once(input), Default::default());

    // Construct the window.
    const WIDTH: u32 = 720;
    const HEIGHT: u32 = 360;
    let mut window: PistonWindow = WindowSettings::new("Inkrust", [WIDTH, HEIGHT])
        .opengl(OpenGL::V3_2)
        .samples(4)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();
    window.set_ups(60);

    // construct our `Ui`.
    let mut ui = conrod::Ui::new(conrod::Theme::default());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // No text to draw, so we'll just create an empty text texture cache.
    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    let mut app = InkApp::new();
    app.set_dom(dom);

    let doc = app.get_doc_handle();

    // parse dom generating shapes for rendering
    walk("", &mut app, doc);

    // Poll events from the window.
    while let Some(event) = window.next() {

        // Convert the piston event to a conrod event.
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }
        
        event.update(|_| {
            ui.set_widgets(|ui_cell| set_ui(ui_cell, &mut app))
        });
        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed(&image_map) {
                fn texture_from_image<T>(img: &T) -> &T { img };
                conrod::backend::piston_window::draw(
                    c, g, primitives, &mut text_texture_cache, texture_from_image);
            }
        });
    }
}

fn set_ui(ref mut ui: conrod::UiCell, app: &mut InkApp) {
    use conrod::{Canvas, };
    use std::iter::once;

    widget_ids!{
        BACKGROUND,
        SVG_CANVAS,
	};

    Canvas::new().color(conrod::color::WHITE).set(BACKGROUND, ui);
    SVGCanvas::new()
                .color(conrod::color::rgb(0.0, 0.3, 0.1))
                .middle_of(BACKGROUND)
                .w_h(256.0, 256.0)
                .label_color(conrod::color::BLACK)
                .label("SVG Canvas")
                // This is called when the user clicks the button.
                .react(|| println!("Click"))
                // Add the widget to the conrod::Ui. This schedules the widget it to be
                // drawn when we call Ui::draw.
                .set(SVG_CANVAS, ui);
}

// TODO SVG Canvas will hold these for rendering
    //     let ref rs = self.renderables;
    //     for renderable in rs.iter() {
    //         match renderable {
    //             &RenderShape::Rectangle(ref id, r) => {
    //                 r.draw(ui);
    //             },
    //             &RenderShape::Line(_, l) => {
    //             }
    //             &RenderShape::Ellipse(ref id, o) => {
    //             },
    //             &RenderShape::CircleArc(ref id, o) => {
    //             },
    //             &RenderShape::Image(ref id, l) => {
    //             },
    //             &RenderShape::Polygon(ref id, p) => {
    //             },
    //             &RenderShape::Text(ref id, ref t) => {
    //             },
    //         }
    //     }

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn walk(prefix: &str, mut app: &mut InkApp, doc: Handle) {
    use graphics::{Rectangle, Line, Ellipse, CircleArc, Image, Polygon, Text};

    let node = doc.borrow();

    print!("{}", prefix);
    match node.node {
        Document => println!("#document"),

        rcdom::Text(ref text) => println!("#text {}", escape_default(text)),

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
            let mut shape = None;
            match lname {
                "rect" => {
                    let rect = rectangle::centered([pos[0], pos[1], size[0], size[1]]);
                    rect.color(fill_color.unwrap_or([1.;4]));
                    shape = Some(RenderShape::Rectangle(
                            id.unwrap_or_default().to_string(), 
                            rect));
                },
//                "ellipse" => {
//                },
//                "circle" => {
//                },
                _ => {},
            }
            match shape {
                Some(s) => app.renderables.push(s),
                None => {},
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
            rcdom::Text(_) | Element(_, _) => true,
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

fn parse_color_hash(value: &str) -> Result<graphics::types::Color, ()> {
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

fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Result<graphics::types::Color, ()> {
    Ok([red / 255., green / 255., blue / 255., alpha / 255.])
}

fn rgb(red: f32, green: f32, blue: f32) -> Result<graphics::types::Color, ()> {
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
