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
    elem_sender: mpsc::Sender<(usize, usize, bool)>,
    elem_receiver: mpsc::Receiver<(usize, usize, bool)>,
}

impl InkApp {
    fn new() -> Self {
        let (elem_sender, elem_receiver) = mpsc::channel();
        InkApp {
            dom: RcDom::default(),
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
    use conrod::{
    	Canvas, 
    	Rectangle, 
    	Positionable
    };

    widget_ids!{
        CANVAS,
        RECTANGLE_FILL,
        RECTANGLE_OUTLINE,
    };
    
    Canvas::new().pad(80.0).set(CANVAS, ui);
    
    Rectangle::fill([40.0, 40.0]).down(40.0).set(RECTANGLE_FILL, ui);

    Rectangle::outline([40.0, 40.0]).down(40.0).set(RECTANGLE_OUTLINE, ui);
    
    let doc = app.get_doc_handle();
    walk("", ui, doc);
}

pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn walk(prefix: &str, ui: &mut UiCell, handle: Handle) {
	use conrod::Rectangle;
	use conrod::Dimensions;
	use conrod::ShapeStyle as Style;
	use conrod::color::Color;
	use cssparser::{Parser, Token, Color as CSSParserColor};
	
    let node = handle.borrow();

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
                	let mut style = Style::fill();
                	for attr in attrs {
                		let key_val = (attr.name.local.as_ref(), attr.value.as_ref());
                		match key_val {
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
                				let mut parser = Parser::new(&s);
                				while !parser.is_exhausted() {
									let token = parser
									.next_including_whitespace_and_comments()
									.expect("Style parsing error.");
									match token {
										Token::Ident(id) => println!("{:?} id", id),
										Token::Hash(s) | Token::IDHash(s) => {
											println!("{:?} i", s);
										}
										Token::AtKeyword(s) => println!("{:?} k", s),
										Token::QuotedString(s) => println!("{:?} q", s),
										Token::UnquotedUrl(s) => println!("{:?} u", s),
										Token::Number(s) => println!("{:?} n", s),
										Token::UnicodeRange(v, s) => println!("{:?}{:?} r", v, s),
										Token::WhiteSpace(s) => println!("{:?} w", s),
										Token::Dimension(v, s) => println!("{:?}{:?} d", v, s),
										_ => { }
									}
//									let color = CSSParserColor::parse(&mut Parser::new(&s))
//									.expect("Color parse error");
//        							style.set_color(Color::Rgba(
//        									color.red,
//        									color.green,
//        									color.blue,
//        									color.alpha));
                				}
			                    println!("{:?}", s);
							}
							_ => {}
                		}
                	}
                	let rect = Rectangle::styled(dim, style);
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
        walk(&new_indent, ui, child.clone());
    }
}
