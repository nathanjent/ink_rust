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
    renderables: Vec<Rectangle>,
    elem_sender: mpsc::Sender<(usize, usize, bool)>,
    elem_receiver: mpsc::Receiver<(usize, usize, bool)>,
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


    // parse dom generating shapes for rendering
    walk("", &app);

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
    use conrod::{Canvas, Circle, Line, Oval, PointPath, Polygon, Positionable, Rectangle};
    use std::iter::once;

    widget_ids!{
        CANVAS,
        LINE,
        POINT_PATH,
        RECTANGLE_FILL,
        RECTANGLE_OUTLINE,
        TRAPEZOID,
        OVAL_FILL,
        OVAL_OUTLINE,
        CIRCLE,
	};

    // The background canvas upon which we'll place our widgets.
    Canvas::new().pad(80.0).set(CANVAS, ui);

    Line::centred([-40.0, -40.0], [40.0, 40.0]).top_left_of(CANVAS).set(LINE, ui);

    let left = [-40.0, -40.0];
    let top = [0.0, 40.0];
    let right = [40.0, -40.0];
    let points = once(left).chain(once(top)).chain(once(right));
    PointPath::centred(points).down(80.0).set(POINT_PATH, ui);

    Rectangle::fill([80.0, 80.0]).down(80.0).set(RECTANGLE_FILL, ui);

    Rectangle::outline([80.0, 80.0]).down(80.0).set(RECTANGLE_OUTLINE, ui);

    let bl = [-40.0, -40.0];
    let tl = [-20.0, 40.0];
    let tr = [20.0, 40.0];
    let br = [40.0, -40.0];
    let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    Polygon::centred_fill(points).right_from(LINE, 80.0).set(TRAPEZOID, ui);

    Oval::fill([40.0, 80.0]).down(80.0).align_middle_x().set(OVAL_FILL, ui);

    Oval::outline([80.0, 40.0]).down(100.0).align_middle_x().set(OVAL_OUTLINE, ui);

    Circle::fill(40.0).down(100.0).align_middle_x().set(CIRCLE, ui);
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn walk(prefix: &str, &mut app: InkApp) {
    use conrod::{Rectangle, Dimensions, Point, ShapeStyle, Positionable};
    use conrod::color::Color;

    let doc = app.get_doc_handle();
    let node = doc.borrow();

    print!("{}", prefix);
    match node.node {
        Document => println!("#document"),

        rcdom::Text(ref text) => println!("#text {}", escape_default(text)),

        Element(ref name, ref attrs) => {
            let lname = name.local.as_ref();
            println!("{:?}", lname);
            match lname {
                "rect" => {
                    let mut dim: Dimensions = [0.0, 0.0];
                    let mut pos: Point = [0.0, 0.0];
                    let mut color: Color = Color::Rgba(1.0, 1.0, 1.0, 1.0);
                    for attr in attrs {
                        let key_val = (attr.name.local.as_ref(), attr.value.as_ref());
                        match key_val {
                            x @ ("x", _) => {
                                let (_, x) = x;
                                let x = x.parse::<f64>()
                                         .expect("Parse error");
                                pos[0] = x;
                                println!("{:?}", x);
                            }
                            y @ ("y", _) => {
                                let (_, y) = y;
                                let y = y.parse::<f64>()
                                         .expect("Parse error");
                                pos[1] = y;
                                println!("{:?}", y);
                            }
                            w @ ("width", _) => {
                                let (_, w) = w;
                                let w = w.parse::<f64>()
                                         .expect("Parse error");
                                dim[0] = w;
                                println!("{:?}", w);
                            }
                            h @ ("height", _) => {
                                let (_, h) = h;
                                let h = h.parse::<f64>()
                                         .expect("Parse error");
                                dim[1] = h;
                                println!("{:?}", h);
                            }
                            i @ ("id", _) => {
                                let (_, i) = i;
                                println!("{:?}", i);
                            }
                            s @ ("style", _) => {
                                let (_, s) = s;
                                println!("{:?}", s);
                                // let mut parser = Parser::new(&s);
                                // 	while !parser.is_exhausted() {
                                // 		// TODO parse property string into (key,value)
                                // 				let property = parser.parse_until_before(
                                // 					Delimiter::Semicolon | Delimiter::None,
                                // 					|mut t| loop {
                                // 						match t.next_including_whitespace_and_comments() {
                                // 					Ok(Token::WhiteSpace(_)) |
                                // 					Ok(Token::Colon) |
                                // 					Ok(Token::Comment(_)) => {},
                                // 			                r => return r
                                // 						}
                                // 			        }
                                // 				).expect("Style parse error");
                                // 				match property {
                                // 					(Token::Ident(k), Token::Hash(v)) |
                                // 						(Token::Ident(k), Token::IDHash(v)) => {
                                // 								println!("{:?}:{:?}", k, v);
                                // 								let color = CSSParserColor::parse(&mut Parser::new(&s))
                                // 								.expect("Color parse error");
                                // 							style.set_color(Color::Rgba(
                                // 									color.red,
                                // 									color.green,
                                // 									color.blue,
                                // 									color.alpha));
                                // 					}
                                // 					Token::AtKeyword(s) => println!("{:?} k", s),
                                // 					Token::QuotedString(s) => println!("{:?} q", s),
                                // 					Token::UnquotedUrl(s) => println!("{:?} u", s),
                                // 					Token::Number(s) => println!("{:?} n", s),
                                // 					Token::UnicodeRange(v, s) => println!("{:?}{:?} r", v, s),
                                // 					Token::WhiteSpace(s) => println!("{:?} w", s),
                                // 					Token::Dimension(v, s) => println!("{:?}{:?} d", v, s),
                                // 					_ => { }
                                // 				}
                                // 	}
                            }
                            _ => {}
                        }
                    }
                    let rect = Rectangle::styled(dim, ShapeStyle::fill_with(color)).left(pos[0]).down(pos[1]);
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
        walk(&new_indent, &app);
    }
}
