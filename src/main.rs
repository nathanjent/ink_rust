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

    let doc = app.get_doc_handle();

    // parse dom generating shapes for rendering
    walk("", doc);

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

fn walk(prefix: &str, handle: Handle) {
    use conrod::Rectangle;
    use conrod::Dimensions;
    use conrod::ShapeStyle as Style;
    use conrod::color::Color;
    use cssparser::{Parser, DeclarationListParser, DeclarationParser, AtRuleParser, Token,
                    Color as CSSParserColor, Delimiter, parse_important, NumericValue, AtRuleType};

    struct StyleParser;

    struct Property {
        name: String,
        value: String,
    }

    impl DeclarationParser for StyleParser {
        type Declaration = Property;


        fn parse_value(&self, name: &str, input: &mut Parser) -> Result<Property, ()> {
            let mut value = vec![];
            loop {
                let start_position = input.position();
                if let Ok(mut token) = input.next() {
                    if token == Token::Delim('!') {
                        input.reset(start_position);
                        if parse_important(input).is_ok() {
                            if input.is_exhausted() {
                                break;
                            }
                        }
                        input.reset(start_position);
                        token = input.next().unwrap();
                    }
                    value.push(one_component_value_to_string(token, input));
                } else {
                    break;
                }
            }

            Ok(Property {
                name: value[0],
                value: value[1],
            })
        }
    }


    fn component_values(input: &mut Parser) -> Vec<String> {
        let mut values = vec![];
        while let Ok(token) = input.next_including_whitespace() {
            values.push(one_component_value_to_string(token, input));
        }
        values
    }

    fn one_component_value_to_string(token: Token, input: &mut Parser) -> String {
        //        fn numeric(value: NumericValue) -> Vec<f32> {
        //            vec![
        //                Token::Number(value).to_css_string().to_json(),
        //                match value.int_value { Some(i) => i.to_json(), None => value.value.to_json() },
        //                match value.int_value { Some(_) => "integer", None => "number" }.to_json()
        //            ]
        //        }

        match token {
            Token::Ident(v) => v.to_string(),
            // Token::AtKeyword(v) => v,
            Token::Hash(v) => v.to_string(),
            Token::IDHash(v) => v.to_string(),
            //            Token::QuotedString(v) => Property {name: "id", value: v},
            //            Token::UnquotedUrl(v) => Property {name: "id", value: v},
            //            Token::Delim(v) => Property {name: "id", value: v},
            //
            //            Token::Number(v) => Json::Array({
            //                let mut v = vec!["number".to_json()];
            //                v.extend(numeric(v));
            //                v
            //            }),
            //            Token::Percentage(PercentageValue { unit_value, int_value, has_sign }) => Json::Array({
            //                let mut v = vec!["percentage".to_json()];
            //                v.extend(numeric(NumericValue {
            //                    value: unit_value * 100.,
            //                    int_value: int_value,
            //                    has_sign: has_sign,
            //                }));
            //                v
            //            }),
            //            Token::Dimension(value, unit) => Json::Array({
            //                let mut v = vec!["dimension".to_json()];
            //                v.extend(numeric(value));
            //                v.push(unit.to_json());
            //                v
            //            }),
            //
            //            Token::UnicodeRange(start, end) => Property {name: "id", value: v},
            //
            //            Token::WhiteSpace(_) => " ".to_json(),
            //            Token::Comment(_) => "/**/".to_json(),
            Token::Colon => "".to_string(),
            //            Token::Semicolon => ";".to_json(),
            //            Token::Comma => ",".to_json(),
            //            Token::IncludeMatch => "~=".to_json(),
            //            Token::DashMatch => "|=".to_json(),
            //            Token::PrefixMatch => "^=".to_json(),
            //            Token::SuffixMatch => "$=".to_json(),
            //            Token::SubstringMatch => "*=".to_json(),
            //            Token::Column => "||".to_json(),
            //            Token::CDO => "<!--".to_json(),
            //            Token::CDC => "-->".to_json(),
            //
            //            Token::Function(name) => Json::Array({
            //                let mut v = vec!["function".to_json(), name.to_json()];
            //                v.extend(nested(input));
            //                v
            //            }),
            //            Token::ParenthesisBlock => Json::Array({
            //                let mut v = vec!["()".to_json()];
            //                v.extend(nested(input));
            //                v
            //            }),
            //            Token::SquareBracketBlock => Json::Array({
            //                let mut v = vec!["[]".to_json()];
            //                v.extend(nested(input));
            //                v
            //            }),
            //            Token::CurlyBracketBlock => Json::Array({
            //                let mut v = vec!["{}".to_json()];
            //                v.extend(nested(input));
            //                v
            //            }),
            //            Token::BadUrl => Property {name: "id", value: v},
            //            Token::BadString => Property {name: "id", value: v},
            //            Token::CloseParenthesis => Property {name: "id", value: v},
            //            Token::CloseSquareBracket => Property {name: "id", value: v},
            //            Token::CloseCurlyBracket => Property {name: "id", value: v},
            _ => "".to_string(),
        }
    }

    impl AtRuleParser for StyleParser {
        type Prelude = Vec<String>;
        type AtRule = String;

        fn parse_prelude(&self,
                         name: &str,
                         input: &mut Parser)
                         -> Result<AtRuleType<Vec<String>, String>, ()> {
            Ok(AtRuleType::OptionalBlock(component_values(input)))
        }

        fn parse_block(&self, mut prelude: Vec<String>, input: &mut Parser) -> Result<String, ()> {
            prelude.push(component_values(input).into_iter().collect());
            let s: String = prelude.into_iter().collect();
            Ok(s)
        }

        fn rule_without_block(&self, mut prelude: Vec<String>) -> String {
            prelude.push("".to_string());
            "".to_string()
        }
    }

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
                                println!("{:?}", s);
                                let mut parser = Parser::new(&s);
                                let properties = DeclarationListParser::new(&mut parser,
                                                                            StyleParser);
                                let properties: Vec<String> = properties.map(|result| {
                                                                            result.unwrap()
                                                                        })
                                                                        .collect();
                                for property in properties {
                                    println!("{:?}", property);
                                }
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
        walk(&new_indent, child.clone());
    }
}
