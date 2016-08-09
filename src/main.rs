#[macro_use]
extern crate conrod;
extern crate piston_window;
extern crate find_folder;
extern crate graphics;
extern crate xml5ever;
extern crate encoding;
extern crate cssparser;
extern crate nom;

mod inkapp;
mod svg_canvas;
mod svg_parser;

use std::fs::File;
use std::io::Read;
use std::default::Default;
use std::iter;
use std::string::String;

use conrod::{Colorable, Labelable, Positionable, Sizeable, Widget};

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings};

use xml5ever::tendril::SliceExt;
use xml5ever::parse;
use xml5ever::tree_builder::TreeSink;
use xml5ever::rcdom::{RcDom, Handle};

use inkapp::InkApp;
use svg_canvas::SVGCanvas;
use svg_parser::walk;


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
    app.dom = dom;

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