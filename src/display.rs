//use conrod;
//use conrod::{widget, Colorable, Positionable, Sizeable, Widget};
//use conrod::backend::glium::glium;
//use conrod::backend::glium::glium::glutin;
//use conrod::backend::glium::glium::index::PrimitiveType;
//use conrod::backend::glium::glium::{DisplayBuild, Surface};

use errors::*;

use glium;
use glium::glutin;
use glium::index::PrimitiveType;
use glium::{DisplayBuild, Surface};

use lyon::extra::rust_logo::build_logo_path;
use lyon::path_builder::*;
use lyon::math::*;
use lyon::tessellation::geometry_builder::{VertexConstructor, VertexBuffers, BuffersBuilder};
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::path_fill::{FillEvents, FillTessellator, FillOptions};
use lyon::tessellation::path_stroke::{StrokeTessellator, StrokeOptions};
use lyon::path::Path;
use lyon::path_iterator::PathIterator;

use find_folder;
use std;

use inkapp::InkApp;
use svg_builder;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    a_position: [f32; 2],
    a_color: [f32; 3],
}

pub struct WithColor(pub [f32; 3]);

impl VertexConstructor<Vec2, Vertex> for WithColor {
    fn new_vertex(&mut self, pos: Vec2) -> Vertex {
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());
        Vertex {
            a_position: pos.array(),
            a_color: self.0,
        }
    }
}

implement_vertex!(Vertex, a_position, a_color);

#[derive(Copy, Clone, Debug)]
struct BgVertex {
    a_position: [f32; 2],
}

struct BgWithColor;
impl VertexConstructor<Vec2, BgVertex> for BgWithColor {
    fn new_vertex(&mut self, pos: Vec2) -> BgVertex {
        BgVertex { a_position: pos.array() }
    }
}

implement_vertex!(BgVertex, a_position);

fn uniform_matrix(m: &Mat4) -> [[f32; 4]; 4] {
    [[m.m11, m.m12, m.m13, m.m14],
     [m.m21, m.m22, m.m23, m.m24],
     [m.m31, m.m32, m.m33, m.m34],
     [m.m41, m.m42, m.m43, m.m44]]
}

pub fn load(app: &mut InkApp) -> Result<()> {
    //const WIDTH: u32 = 400;
    //const HEIGHT: u32 = 200;

    //// Build the window.
    //let display = match glium::glutin::WindowBuilder::new()
    //    .with_vsync()
    //    .with_dimensions(WIDTH, HEIGHT)
    //    .with_title("Hello Conrod!")
    //    .build_glium() {
    //    Ok(d) => d,
    //    Err(e) => bail!("Could not build display {}", e),
    //};

    //// construct our `Ui`.
    //let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    //// Generate the widget identifiers.
    //widget_ids!(struct Ids {
    //    text,
    //    background
    //});
    //let ids = Ids::new(ui.widget_id_generator());

    //// Add a `Font` to the `Ui`'s `font::Map` from file.
    //let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets")
    //    .chain_err(|| "Asset folder not found")?;
    //let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    //let regular = ui.fonts
    //    .insert_from_file(font_path)
    //    .chain_err(|| "Font not found")?;

    //// A type used for converting `conrod::render::Primitives` into `Command`s
    //// that can be used for drawing to the glium `Surface`.
    //let mut renderer = match conrod::backend::glium::Renderer::new(&display) {
    //    Ok(r) => r,
    //    Err(e) => bail!("Could not create renderer {:?}", e),
    //};

    //// The image map describing each of our widget->image mappings
    //// (in our case, none).
    //let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    //// Poll events from the window.
    //let mut last_update = std::time::Instant::now();
    //let mut ui_needs_update = true;
    //'main: loop {

    //    // We don't want to loop any faster than 60 FPS,
    //    // so wait until it has been at least 16ms since the last yield.
    //    let sixteen_ms = std::time::Duration::from_millis(16);
    //    let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
    //    if duration_since_last_update < sixteen_ms {
    //        std::thread::sleep(sixteen_ms - duration_since_last_update);
    //    }

    //    // Collect all pending events.
    //    let mut events: Vec<_> = display.poll_events().collect();

    //    // If there are no events and the `Ui` does not need updating,
    //    // wait for the next event.
    //    if events.is_empty() && !ui_needs_update {
    //        events.extend(display.wait_events().next());
    //    }

    //    // Reset the needs_update flag and time this update.
    //    ui_needs_update = false;
    //    last_update = std::time::Instant::now();

    //    // Handle all events.
    //    for event in events {

    //        // Use the `winit` backend feature to convert the winit event to a conrod one.
    //        if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
    //            ui.handle_event(event);
    //            ui_needs_update = true;
    //        }

    //        match event {
    //            // Break from the loop upon `Escape`.
    //            glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
    //            glium::glutin::Event::Closed => break 'main,
    //            _ => {}
    //        }
    //    }

    //    // Instantiate all widgets in the GUI.
    //    {
    //        let ui = &mut ui.set_widgets();

    //        widget::Canvas::new().color(conrod::color::DARK_RED).set(ids.background, ui);

    //        // "Hello World!" in the middle of the screen.
    //        widget::Text::new("Hello World!")
    //            .middle_of(ui.window)
    //            .color(conrod::color::WHITE)
    //            .font_size(32)
    //            .set(ids.text, ui);
    //    }

    //    // Render the `Ui` and then display it on the screen.
    //    if let Some(primitives) = ui.draw_if_changed() {
    //        renderer.fill(&display, primitives, &image_map);
    //        let mut target = display.draw();
    //        target.clear_color(0.0, 0.0, 0.0, 1.0);
    //        match renderer.draw(&display, &mut target, &image_map) {
    //            Ok(_) => {}
    //            Err(e) => bail!("Could not create renderer {:?}", e),
    //        }
    //        target.finish()
    //            .chain_err(|| "Could not finish {}")?;
    //    }
    //}

    let mut buffers: VertexBuffers<Vertex> = VertexBuffers::new();

    svg_builder::fill_buffer_from_dom(&app, &mut buffers)
        .chain_err(|| "Path build from dom error.")?;

    let (indices, vertices) = (buffers.indices, buffers.vertices);
    println!(" -- {} vertices {} indices", vertices.len(), indices.len());

    let mut bg_buffers: VertexBuffers<BgVertex> = VertexBuffers::new();
    tessellate_rectangle(&Rect::new(vec2(-1.0, -1.0), size(2.0, 2.0)),
                         &mut BuffersBuilder::new(&mut bg_buffers, BgWithColor));

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_dimensions(700, 700)
        .with_title("Inkrust".to_string())
        // NoAvailablePixelFormat error when uncommented
        //.with_multisampling(8)
        .with_vsync()
        .build_glium()
        .unwrap();
    //.chain_err(|| "Could not build display")?;

    let model_vbo = glium::VertexBuffer::new(&display, &vertices[..]).unwrap();
    let model_ibo = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices[..])
        .unwrap();

    let bg_vbo = glium::VertexBuffer::new(&display, &bg_buffers.vertices[..]).unwrap();
    let bg_ibo = glium::IndexBuffer::new(&display,
                                         PrimitiveType::TrianglesList,
                                         &bg_buffers.indices[..])
        .unwrap();

    // compiling shaders and linking them together
    let bg_program = program!(&display,
        140 => {
            vertex: "
                #version 140
                in vec2 a_position;
                out vec2 v_position;
                void main() {
                    gl_Position = vec4(a_position, 0.0, 1.0);
                    v_position = a_position;
                }
            ",
            fragment: "
                #version 140
                uniform vec2 u_resolution;
                in vec2 v_position;
                out vec4 f_color;
                void main() {
                    vec2 px_position = (v_position * vec2(1.0, -1.0)    + vec2(1.0, 1.0))
                                     * 0.5 * u_resolution;
                    // #005fa4
                    float vignette = clamp(0.0, 1.0, (0.7*length(v_position)));
                    f_color = mix(
                        vec4(0.0, 0.47, 0.9, 1.0),
                        vec4(0.0, 0.1, 0.64, 1.0),
                        vignette
                    );
                    if (mod(px_position.x, 20.0) <= 1.0 ||
                        mod(px_position.y, 20.0) <= 1.0) {
                        f_color *= 1.2;
                    }
                    if (mod(px_position.x, 100.0) <= 1.0 ||
                        mod(px_position.y, 100.0) <= 1.0) {
                        f_color *= 1.2;
                    }
                }
            "
        },
    )
        .unwrap();

    // compiling shaders and linking them together
    let model_program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform vec2 u_resolution;
                uniform mat4 u_matrix;
                in vec2 a_position;
                in vec3 a_color;
                out vec3 v_color;
                void main() {
                    gl_Position = u_matrix * vec4(a_position, 0.0, 1.0);// / vec4(u_resolution, 1.0, 1.0);
                    v_color = a_color;
                }
            ",
            fragment: "
                #version 140
                in vec3 v_color;
                out vec4 f_color;
                void main() {
                    f_color = vec4(v_color, 1.0);
                }
            "
        },
    ).unwrap();

    let mut target_zoom = 1.0;
    let mut zoom = 1.0;
    let mut target_pos = vec2(0.0, 0.0);
    let mut pos = vec2(0.0, 0.0);
    loop {
        zoom += (target_zoom - zoom) / 3.0;
        pos = pos + (target_pos - pos) / 3.0;

        let mut target = display.draw();

        let (w, h) = target.get_dimensions();
        let resolution = vec2(w as f32, h as f32);

        let model_mat = Mat4::identity();
        let mut view_mat = Mat4::identity();

        view_mat = view_mat.pre_translated(-1.0, 1.0, 0.0);
        view_mat = view_mat.pre_scaled(5.0 * zoom, 5.0 * zoom, 0.0);
        view_mat = view_mat.pre_scaled(2.0 / resolution.x, -2.0 / resolution.y, 1.0);
        view_mat = view_mat.pre_translated(pos.x, pos.y, 0.0);

        let uniforms = uniform! {
            u_resolution: resolution.array(),
            u_matrix: uniform_matrix(&model_mat.pre_mul(&view_mat))
        };

        target.clear_color(0.75, 0.75, 0.75, 1.0);
        target.draw(&bg_vbo,
                  &bg_ibo,
                  &bg_program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.draw(&model_vbo,
                  &model_ibo,
                  &model_program,
                  &uniforms,
                  &Default::default())
            .unwrap();
        target.finish().unwrap();

        let mut should_close = false;
        for event in display.poll_events() {
            should_close |= match event {
                glutin::Event::Closed => true,
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => true,
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::PageDown)) => {
                    target_zoom *= 0.8;
                    false
                }
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::PageUp)) => {
                    target_zoom *= 1.25;
                    false
                }
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Left)) => {
                    target_pos.x += 5.0 / target_zoom;
                    false
                }
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Right)) => {
                    target_pos.x -= 5.0 / target_zoom;
                    false
                }
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Up)) => {
                    target_pos.y += 5.0 / target_zoom;
                    false
                }
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Down)) => {
                    target_pos.y -= 5.0 / target_zoom;
                    false
                }
                _evt => {
                    //println!("{:?}", _evt);
                    false
                }
            };
        }
        if should_close {
            break;
        }
    }

    Ok(())
}
