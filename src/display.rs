use conrod;
use conrod::{widget, Colorable, Positionable, Sizeable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};
use errors::*;

use find_folder;
use std;


use inkapp::InkApp;

pub fn load(app: &mut InkApp) -> Result<()> {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;

    // Build the window.
    let display = match glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Hello Conrod!")
        .build_glium() {
            Ok(d) => d,
            Err(e) => bail!("Could not build display {}", e),
        };

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Generate the widget identifiers.
    widget_ids!(struct Ids {
        text,
        background
    });
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets")
        .chain_err(|| "Asset folder not found")?;
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    let regular = ui.fonts.insert_from_file(font_path)
        .chain_err(|| "Font not found")?;

    // A type used for converting `conrod::render::Primitives` into `Command`s
    // that can be used for drawing to the glium `Surface`.
    let mut renderer = match conrod::backend::glium::Renderer::new(&display) {
            Ok(r) => r,
            Err(e) => bail!("Could not create renderer {:?}", e),
        };

    // The image map describing each of our widget->image mappings
    // (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Poll events from the window.
    let mut last_update = std::time::Instant::now();
    let mut ui_needs_update = true;
    'main: loop {

        // We don't want to loop any faster than 60 FPS,
        // so wait until it has been at least 16ms since the last yield.
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update
            = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events: Vec<_> = display.poll_events().collect();

        // If there are no events and the `Ui` does not need updating,
        // wait for the next event.
        if events.is_empty() && !ui_needs_update {
            events.extend(display.wait_events().next());
        }

        // Reset the needs_update flag and time this update.
        ui_needs_update = false;
        last_update = std::time::Instant::now();

        // Handle all events.
        for event in events {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                ui_needs_update = true;
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed =>
                    break 'main,
                _ => {},
            }
        }

        // Instantiate all widgets in the GUI.
        {
            let ui = &mut ui.set_widgets();

             widget::Canvas::new().color(conrod::color::DARK_RED).set(ids.background, ui);

            // "Hello World!" in the middle of the screen.
            widget::Text::new("Hello World!")
                .middle_of(ui.window)
                .color(conrod::color::WHITE)
                .font_size(32)
                .set(ids.text, ui);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            match renderer.draw(&display, &mut target, &image_map) {
                Ok(_) => {},
                Err(e) => bail!("Could not create renderer {:?}", e),
            }
            target.finish()
                .chain_err(|| "Could not finish {}")?;
        }
    }
    Ok(())
}
