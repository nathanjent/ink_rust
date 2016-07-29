#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate graphics;
extern crate lyon;
extern crate xml5ever;
extern crate encoding;
extern crate cssparser;

use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::default::Default;
use std::iter;
use std::string::String;

use conrod::{Theme, Widget};
use piston_window::*;

use lyon::path_builder::*;

use xml5ever::tendril::SliceExt;
use xml5ever::parse;
use xml5ever::tree_builder::TreeSink;
use xml5ever::rcdom;
use xml5ever::rcdom::{Document, Element, RcDom, Handle};

/// Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
type Backend = (piston_window::G2dTexture<'static>, piston_window::Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

struct InkApp {
    dom: RcDom,
    renderables: Vec<RenderShape>,
    elem_sender: mpsc::Sender<(usize, usize, bool)>,
    elem_receiver: mpsc::Receiver<(usize, usize, bool)>,
}

enum RenderShape {
    Rect_fill(String, conrod::Rectangle),
    Rect_outline(String, conrod::Rectangle),
    Oval_fill(String, conrod::Oval),
    Oval_outline(String, conrod::Oval),
    Line(String, conrod::Line),
    Polyline(String, conrod::PointPath<f32>),
    Polygon(String, conrod::Polygon<f32>),
}

impl InkApp {
    fn new() -> Self {
        let (elem_sender, elem_receiver) = mpsc::channel();
        InkApp {
            dom: RcDom::default(),
            renderables: Vec::new(),
            elem_sender: elem_sender,
            elem_receiver: elem_receiver,
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
    let mut file = File::open("tests/documents/testrect.svg").unwrap();

    let mut file_string = String::new();
    if let Err(err) = file.read_to_string(&mut file_string) {
        println!("Reading failed: {}", err);
        std::process::exit(1);
    };

    let input = file_string.to_tendril();

    let dom: RcDom = parse(iter::once(input), Default::default());

    // Construct the window.
    let mut window: PistonWindow = WindowSettings::new("Inkrust", [720, 360])
                                       .opengl(piston_window::OpenGL::V3_2)
                                       .samples(4)
                                       .exit_on_esc(true)
                                       .build()
                                       .unwrap();

    // Construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
                         .for_folder("assets")
                         .unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        let theme = Theme::default();
        let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.clone()).unwrap();
        Ui::new(glyph_cache, theme)
    };

    let mut app = InkApp::new();
    app.set_dom(dom);

    let doc = app.get_doc_handle();

    // parse dom generating shapes for rendering
    walk("", &mut app, doc);

    window.set_ups(60);

    // Poll events from the window.
    while let Some(event) = window.next() {
        ui.handle_event(event.clone());
        event.update(|_| ui.set_widgets(|mut ui| set_widgets(&mut ui, &mut app)));
        window.draw_2d(&event, |c, g| ui.draw_if_changed(c, g));
    }
}

// Declare the `WidgetId`s and instantiate the widgets.
fn set_widgets(ui: &mut UiCell, app: &mut InkApp) {
    use conrod::{Canvas, Circle, Line, Oval, PointPath, Polygon, Positionable, Rectangle, Text};
    use std::iter::once;

    widget_ids!{
        CANVAS,
        LINE with 64,
        POINT_PATH with 64,
        RECTANGLE_FILL with 64,
        RECTANGLE_OUTLINE with 64,
        TRAPEZOID,
        POLYGON with 64,
        OVAL_FILL with 64,
        OVAL_OUTLINE with 64,
        CIRCLE,
        TEXT with 64,
	};

    // The background canvas upon which we'll place our widgets.
    Canvas::new().pad(80.0).set(CANVAS, ui);

    let ref rs = app.renderables;
    for renderable in rs.iter() {
        match renderable {
            &RenderShape::Rect_fill(ref id, r) => {
                r.set(RECTANGLE_FILL, ui);
            },
            &RenderShape::Rect_outline(ref id, r) => {
                r.set(RECTANGLE_OUTLINE, ui);
            },
            &RenderShape::Oval_fill(ref id, o) => {
                o.set(OVAL_FILL, ui);
            },
            &RenderShape::Oval_outline(ref id, o) => {
                o.set(OVAL_OUTLINE, ui);
            },
            &RenderShape::Line(ref id, l) => {
                l.set(LINE, ui);
            },
            &RenderShape::Polyline(ref id, ref p) => {
                // p.set(LINE, ui);
            },
            &RenderShape::Polygon(ref id, p) => {
                // p.set(POLYGON, ui);
            },
        }
    }

    // Line::centred([-40.0, -40.0], [40.0, 40.0]).top_left_of(CANVAS).set(LINE, ui);

    // let left = [-40.0, -40.0];
    // let top = [0.0, 40.0];
    // let right = [40.0, -40.0];
    // let points = once(left).chain(once(top)).chain(once(right));
    // PointPath::centred(points).down(80.0).set(POINT_PATH, ui);

    // Rectangle::fill([80.0, 80.0]).down(80.0).set(RECTANGLE_FILL, ui);

    // Rectangle::outline([80.0, 80.0]).down(80.0).set(RECTANGLE_OUTLINE, ui);

    // let bl = [-40.0, -40.0];
    // let tl = [-20.0, 40.0];
    // let tr = [20.0, 40.0];
    // let br = [40.0, -40.0];
    // let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    // Polygon::centred_fill(points).right_from(LINE, 80.0).set(TRAPEZOID, ui);

    // Oval::fill([40.0, 80.0]).down(80.0).align_middle_x().set(OVAL_FILL, ui);

    // Oval::outline([80.0, 40.0]).down(100.0).align_middle_x().set(OVAL_OUTLINE, ui);

    // Circle::fill(40.0).down(100.0).align_middle_x().set(CIRCLE, ui);
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn walk(prefix: &str, mut app: &mut InkApp, doc: Handle) {
    use conrod::{Rectangle, Dimensions, Point, ShapeStyle, Positionable, Color};
    // use graphics::Rectangle;
    use graphics::rectangle::{Shape, Border};
    // use graphics::types::Scalar;

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
            let mut style = "";
            let mut pos = [0.; 2];
            let mut size = [1.; 2];
            let mut radius = 1.;
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
                        println!("id: {:?}", v);
                    }
                    v @ ("style", _) => {
                        let (_, v) = v;
                        style = v;
                        println!("{:?}", v);
                    }
                    v @ ("x", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos[0] = v;
                        println!("x: {:?}", v);
                    }
                    v @ ("y", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos[1] = v;
                        println!("y: {:?}", v);
                    }
                    v @ ("width", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        size[0] = v;
                        println!("width: {:?}", v);
                    }
                    v @ ("height", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        size[1] = v;
                        println!("height: {:?}", v);
                    }
                    v @ ("rx", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        radii[0] = v;
                        println!("rx: {:?}", v);
                    }
                    v @ ("ry", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        radii[1] = v;
                        println!("ry: {:?}", v);
                    }
                    v @ ("cx", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos[0] = v;
                        println!("cx: {:?}", v);
                    }
                    v @ ("cy", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos[1] = v;
                        println!("cy: {:?}", v);
                    }
                    v @ ("r", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        radius = v;
                        println!("cy: {:?}", v);
                    }
                    v @ ("x1", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos1[0] = v;
                        println!("x1: {:?}", v);
                    }
                    v @ ("y1", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos1[1] = v;
                        println!("y1: {:?}", v);
                    }
                    v @ ("x2", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos2[0] = v;
                        println!("x2: {:?}", v);
                    }
                    v @ ("y2", _) => {
                        let (_, v) = v;
                        let v = v.parse::<f64>()
                                    .expect("Parse error");
                        pos2[1] = v;
                        println!("y2: {:?}", v);
                    }
                    v @ ("points", _) => {
                        let (_, v) = v;
                        points = v.split_whitespace()
                            .map(|s| s.split_at(s.find(',').unwrap()))
                            .map(|(x, y)| (x.parse::<f64>().expect("Parse error"), y.parse::<f64>().expect("Parse error")))
                            .collect();
                        println!("y2: {:?}", v);
                    }
                    _ => {}
                }
            }

            // Style parsing
            let mut fill_opacity = None;
            let mut fill_color = None;
            let mut stroke_color = None;
            for (name, mut val) in style.split_terminator(';').map(|s| s.split_at(s.find(':').unwrap())) {
                match name {
                    "fill" => {
                        if val.to_string().remove(0) == '#' {
                            let (_, hex_str) = val.split_at(1);
                            val = hex_str
                        }
                        fill_color  = Some(parse_color_hash(val).expect("Error parsing CSS color"));
                    },
                    "stroke" => {
                        if val.to_string().remove(0) == '#' {
                            let (_, hex_str) = val.split_at(1);
                            val = hex_str
                        }
                        stroke_color  = Some(parse_color_hash(val).expect("Error parsing CSS color"));
                    },
                    "fill-opacity" => {
                        fill_opacity = Some(val.parse::<f32>().expect("Error parsing opacity."))
                    },
                    _ => continue,
                }
                println!("{}:{};", name, val);
            }
            match lname {
                "rect" => {
                    match fill_color {
                        Some(c) => {
                            let c = match fill_opacity {
                                Some(o) => c.with_alpha(o),
                                None => c,
                            };
                            app.renderables.push(RenderShape::Rect_fill(id.unwrap().to_string(), Rectangle::fill_with(size, c).xy(pos)))
                        },
                        None => {},
                    }
                    match stroke_color {
                        Some(c) => app.renderables.push(RenderShape::Rect_outline(id.unwrap().to_string(), Rectangle::outline_styled(size, conrod::LineStyle::new().color(c)).xy(pos))),
                        None => {},
                    }
                }
                _ => {}
            }
        }
        _ => {}
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

fn parse_color_hash(value: &str) -> Result<conrod::Color, ()> {
    let value = value.as_bytes();
    match value.len() {
        8 => rgba(
            (try!(from_hex(value[0])) * 16 + try!(from_hex(value[1]))) as f32,
            (try!(from_hex(value[2])) * 16 + try!(from_hex(value[3]))) as f32,
            (try!(from_hex(value[4])) * 16 + try!(from_hex(value[5]))) as f32,
            (try!(from_hex(value[6])) * 16 + try!(from_hex(value[7]))) as f32,
        ),
        6 => rgb(
            (try!(from_hex(value[0])) * 16 + try!(from_hex(value[1]))) as f32,
            (try!(from_hex(value[2])) * 16 + try!(from_hex(value[3]))) as f32,
            (try!(from_hex(value[4])) * 16 + try!(from_hex(value[5]))) as f32,
        ),
        4 => rgba(
            (try!(from_hex(value[0])) * 17) as f32,
            (try!(from_hex(value[1])) * 17) as f32,
            (try!(from_hex(value[2])) * 17) as f32,
            (try!(from_hex(value[3])) * 17) as f32,
        ),
        3 => rgb(
            (try!(from_hex(value[0])) * 17) as f32,
            (try!(from_hex(value[1])) * 17) as f32,
            (try!(from_hex(value[2])) * 17) as f32,
        ),
        _ => Err(())
    }
}

fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Result<conrod::Color, ()> {
    Ok(conrod::Color::Rgba(red / 255., green / 255., blue / 255., alpha / 255.,))
}

fn rgb(red: f32, green: f32, blue: f32) -> Result<conrod::Color, ()> {
    Ok(conrod::Color::Rgba(red / 255., green / 255., blue / 255., 1.,))
}

fn from_hex(c: u8) -> Result<u8, ()> {
    match c {
        b'0' ... b'9' => Ok(c - b'0'),
        b'a' ... b'f' => Ok(c - b'a' + 10),
        b'A' ... b'F' => Ok(c - b'A' + 10),
        _ => Err(())
    }
}
