#[macro_use]
extern crate conrod;
extern crate xml;
extern crate piston_window;
extern crate find_folder;
extern crate graphics;
extern crate lyon;

mod doc;

use doc::document::{Document};

use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::cell::{RefCell};

use conrod::{Theme, Widget};
use piston_window::*;

use lyon::path_builder::*;

use xml::{Parser, Element, ElementBuilder};

/// Conrod is backend agnostic. Here, we define the `piston_window` backend to use for our `Ui`.
type Backend = (piston_window::G2dTexture<'static>, piston_window::Glyphs);
type Ui = conrod::Ui<Backend>;
type UiCell<'a> = conrod::UiCell<'a, Backend>;

struct InkApp {
    document: RefCell<Option<Document>>,

    elem_sender: mpsc::Sender<(usize, usize, bool)>,
    elem_receiver: mpsc::Receiver<(usize, usize, bool)>,
}

impl InkApp {
    fn new() -> Self {
        let (elem_sender, elem_receiver) = mpsc::channel();
        InkApp {
            document: RefCell::new(None),
            elem_sender: elem_sender,
            elem_receiver: elem_receiver,
        }
    }

    fn set_document(&mut self, document: Document) {
        self.document = RefCell::new(Some(document));
    }
    
    fn get_document(&self) -> Option<Document> {
    	self.document.clone().into_inner()
    }
}

fn main() {

    let mut file = File::open("tests/documents/testrect.svg").unwrap();

    let mut file_string = String::new();
    if let Err(err) = file.read_to_string(&mut file_string) {
        println!("Reading failed: {}", err);
        std::process::exit(1);
    };
    let mut parser = Parser::new();
    let mut eb = ElementBuilder::new();

    parser.feed_str(&file_string);
    let elements = parser.filter_map(|x| eb.handle_event(x)).map(|x| x.unwrap()).collect();

	let mut doc = Document::new();
	doc.set_tree(elements);

    let mut app = InkApp::new();
    app.set_document(doc);

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

	let tree = app.get_document().unwrap().get_tree();
	
    // Generate a unique const `WidgetId` for each widget.
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
