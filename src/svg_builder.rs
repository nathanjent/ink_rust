use errors::*;
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::tessellation::basic_shapes::*;
use lyon::path::Path;
use lyon::path_iterator::PathIterator;
use lyon::lyon_core::ArcFlags;
use lyon::math::{point, rad, vec2};
use lyon::path_builder::*;
use lyon::tessellation::path_fill::{FillEvents, FillTessellator, FillOptions};
use lyon::tessellation::path_stroke::{StrokeTessellator, StrokeOptions};

use svgdom::{AttributeId, AttributeValue, Document, ElementId, NodeEdge, NodeType};
use svgdom::types::*;

use inkapp::InkApp;
use display;

pub fn fill_buffer_from_dom(app: &InkApp,
                            buffers: &mut VertexBuffers<display::Vertex>)
                            -> Result<()> {
    let mut builder = SvgPathBuilder::new(Path::builder());

    // TODO traverse dom build paths for each element and adding to buffers
    if let Some(svg) = app.dom.svg_element() {
        let mut traversal = svg.traverse();
        loop {
            if let Some(node_edge) = traversal.next() {
                match node_edge {
                    NodeEdge::Start(node) => {
                        //println!("{:?}", node);
                        match node.node_type() {
                            NodeType::Element => {
                                if let Some(tag_id) = node.tag_id() {
                                    println!("{:?}", tag_id);
                                    match tag_id {
                                        ElementId::Path => {
                                            let mut fill_color =  app.current_style.fill;
                                            let mut stroke_color = app.current_style.stroke;

                                            if let Some(AttributeValue::Path(path::Path {
                                                d: segments
                                            })) = node.attribute_value(AttributeId::D) {
                                                println!("{:?}", segments);
                                                build_from_segments(&segments[..],
                                                                    &mut builder);
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Fill) {
                                                fill_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Stroke) {
                                                stroke_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }

                                            build_buffers(&mut builder,
                                                          buffers,
                                                          stroke_color,
                                                          fill_color,
                                                          app.current_style.show_points)
                                                .chain_err(|| "Build buffers error")?;
                                        }
                                        ElementId::Circle => {
                                            let mut cx = 0.;
                                            let mut cy = 0.;
                                            let mut r = 1.;
                                            let mut fill_color =  app.current_style.fill;
                                            let mut stroke_color = app.current_style.stroke;
                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Cx) {
                                                cx = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Cy) {
                                                cy = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::R) {
                                                r = v as f32;
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Fill) {
                                                fill_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Stroke) {
                                                stroke_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }

                                            build_ellipse(cx, cy, r, r, &mut builder)
                                                .chain_err(|| "Build ellipse error")?;

                                            build_buffers(&mut builder,
                                                          buffers,
                                                          stroke_color,
                                                          fill_color,
                                                          app.current_style.show_points)
                                                .chain_err(|| "Build buffers error")?;
                                        }
                                        ElementId::Ellipse => {
                                            let mut cx = 0.;
                                            let mut cy = 0.;
                                            let mut rx = 1.;
                                            let mut ry = 1.;
                                            let mut fill_color =  app.current_style.fill;
                                            let mut stroke_color = app.current_style.stroke;
                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Cx) {
                                                cx = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Cy) {
                                                cy = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Rx) {
                                                rx = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Ry) {
                                                ry = v as f32;
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Fill) {
                                                fill_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Stroke) {
                                                stroke_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }
                                            build_ellipse(cx, cy, rx, ry, &mut builder)
                                                .chain_err(|| "Build ellipse error")?;
                                            build_buffers(&mut builder,
                                                          buffers,
                                                          stroke_color,
                                                          fill_color,
                                                          app.current_style.show_points)
                                                .chain_err(|| "Build buffers error")?;
                                        }
                                        ElementId::Rect => {
                                            let mut x = 0.;
                                            let mut y = 0.;
                                            let mut rx = 0.;
                                            let mut ry = 0.;
                                            let mut w = 1.;
                                            let mut h = 1.;
                                            let mut transform = None;
                                            let mut fill_color =  app.current_style.fill;
                                            let mut stroke_color = app.current_style.stroke;
                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::X) {
                                                x = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Y) {
                                                y = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Width) {
                                                w = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Height) {
                                                h = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Rx) {
                                                rx = v as f32;
                                            }

                                            if let Some(AttributeValue::Length(Length {
                                                num: v, ..
                                            })) = node.attribute_value(AttributeId::Ry) {
                                                ry = v as f32;
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Fill) {
                                                fill_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }

                                            if let Some(AttributeValue::Color(Color {
                                                red: r,
                                                green: g,
                                                blue: b,
                                            })) = node.attribute_value(AttributeId::Stroke) {
                                                stroke_color = [
                                                    r as f32 / 256.,
                                                    g as f32 / 256.,
                                                    b as f32 / 256.,];
                                            }

                                            if let Some(AttributeValue::Transform(t)) =
                                                node.attribute_value(AttributeId::Transform) {
                                                transform = Some(t);
                                            }
                                            if rx == 0. && ry > 0. {
                                                rx = ry;
                                            } else if ry == 0. && rx > 0. {
                                                ry = rx;
                                            }
                                            if rx > w / 2. {
                                                rx = w / 2.;
                                            }
                                            if ry > h / 2. {
                                                ry = h / 2.;
                                            }
                                            let rxry = vec2(rx, ry);
                                            let x_axis_rotation = rad(0.);
                                            let arc_flags = ArcFlags {
                                                large_arc: false,
                                                sweep: true,
                                            };

                                            builder.move_to(vec2(x + rx, y));
                                            builder.line_to(vec2(x + w - rx, y));
                                            builder.line_to(vec2(x + w - rx, y));
                                            builder.arc_to(vec2(x + w, y + ry),
                                                           rxry,
                                                           x_axis_rotation,
                                                           arc_flags);
                                            builder.line_to(vec2(x + w, y + h - ry));
                                            builder.arc_to(vec2(x + w - rx, y + h),
                                                           rxry,
                                                           x_axis_rotation,
                                                           arc_flags);
                                            builder.line_to(vec2(x + rx, y + h));
                                            builder.arc_to(vec2(x, y + h - ry),
                                                           rxry,
                                                           x_axis_rotation,
                                                           arc_flags);
                                            builder.line_to(vec2(x, y + ry));
                                            builder.arc_to(vec2(x + rx, y),
                                                           rxry,
                                                           x_axis_rotation,
                                                           arc_flags);
                                            builder.close();

                                            //TODO apply transform
                                            if let Some(t) = transform {
                                                let Transform { a: a,
                                                                b: b,
                                                                c: c,
                                                                d: d,
                                                                e: e,
                                                                f: f } = t;
                                                println!("{:?},{:?},{:?},{:?},{:?},{:?}",
                                                         a,
                                                         b,
                                                         c,
                                                         d,
                                                         e,
                                                         f);
                                            }
                                            build_buffers(&mut builder,
                                                          buffers,
                                                          stroke_color,
                                                          fill_color,
                                                          app.current_style.show_points)
                                                .chain_err(|| "Build buffers error")?;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            NodeType::Text => {}
                            _ => {}
                        }
                    }
                    NodeEdge::End(_) => {}
                }
            } else {
                break;
            }
        }
    }
    Ok(())
}

fn build_ellipse<Builder: SvgBuilder>(cx: f32,
                                      cy: f32,
                                      rx: f32,
                                      ry: f32,
                                      path: &mut Builder)
                                      -> Result<()> {
    if rx > 0. && ry > 0. {
        let rxry = vec2(rx, ry);
        let x_axis_rotation = rad(0.);
        let arc_flags = ArcFlags {
            large_arc: false,
            sweep: true,
        };
        path.move_to(vec2(cx + rx, cy));
        path.arc_to(vec2(cx, cy + ry), rxry, x_axis_rotation, arc_flags);
        path.arc_to(vec2(cx - rx, cy), rxry, x_axis_rotation, arc_flags);
        path.arc_to(vec2(cx, cy - ry), rxry, x_axis_rotation, arc_flags);
        path.arc_to(vec2(cx + rx, cy), rxry, x_axis_rotation, arc_flags);
        path.close();
    } else if rx < 0. || ry < 0. {
        // error
    }
    Ok(())
}

fn build_from_segments<Builder: SvgBuilder>(segments: &[path::Segment],
                                            path: &mut Builder)
                                            -> Result<()> {
    for segment in segments {
        println!("{:?}", segment);
        match *segment.data() {
            path::SegmentData::MoveTo { x, y } => {
                let xy = vec2(x as f32, y as f32);
                if segment.is_absolute() {
                    path.move_to(xy);
                } else {
                    path.relative_move_to(xy);
                }
            }
            path::SegmentData::LineTo { x, y } => {
                let xy = vec2(x as f32, y as f32);
                if segment.is_absolute() {
                    path.line_to(xy);
                } else {
                    path.relative_line_to(xy);
                }
            }
            path::SegmentData::HorizontalLineTo { x } => {
                if segment.is_absolute() {
                    path.horizontal_line_to(x as f32);
                } else {
                    path.relative_horizontal_line_to(x as f32);
                }
            }
            path::SegmentData::VerticalLineTo { y } => {
                if segment.is_absolute() {
                    path.vertical_line_to(y as f32);
                } else {
                    path.relative_vertical_line_to(y as f32);
                }
            }
            path::SegmentData::CurveTo { x1, y1, x2, y2, x, y } => {
                let x1y1 = vec2(x1 as f32, y1 as f32);
                let x2y2 = vec2(x2 as f32, y2 as f32);
                let xy = vec2(x as f32, y as f32);
                if segment.is_absolute() {
                    path.cubic_bezier_to(x1y1, x2y2, xy);
                } else {
                    path.relative_cubic_bezier_to(x1y1, x2y2, xy);
                }
            }
            path::SegmentData::SmoothCurveTo { x2, y2, x, y } => {
                let x2y2 = vec2(x2 as f32, y2 as f32);
                let xy = vec2(x as f32, y as f32);
                if segment.is_absolute() {
                    path.smooth_cubic_bezier_to(x2y2, xy);
                } else {
                    path.smooth_relative_cubic_bezier_to(x2y2, xy);
                }
            }
            path::SegmentData::Quadratic { x1, y1, x, y } => {
                let x1y1 = vec2(x1 as f32, y1 as f32);
                let xy = vec2(x as f32, y as f32);
                if segment.is_absolute() {
                    path.quadratic_bezier_to(x1y1, xy);
                } else {
                    path.relative_quadratic_bezier_to(x1y1, xy);
                }
            }
            path::SegmentData::SmoothQuadratic { x, y } => {
                let xy = vec2(x as f32, y as f32);
                if segment.is_absolute() {
                    path.smooth_quadratic_bezier_to(xy);
                } else {
                    path.smooth_relative_quadratic_bezier_to(xy);
                }
            }
            path::SegmentData::EllipticalArc { rx,
                                               ry,
                                               x_axis_rotation,
                                               large_arc,
                                               sweep,
                                               x,
                                               y } => {
                let arc_flags = ArcFlags {
                    sweep: sweep,
                    large_arc: large_arc,
                };
                let x_axis_rotation = rad(x_axis_rotation as f32);
                let xy = vec2(x as f32, y as f32);
                let rxry = vec2(rx as f32, ry as f32);
                if segment.is_absolute() {
                    path.arc_to(xy, rxry, x_axis_rotation, arc_flags);
                } else {
                    path.relative_arc_to(xy, rxry, x_axis_rotation, arc_flags);
                }
            }
            path::SegmentData::ClosePath => {
                path.close();
            }
        }
    }
    Ok(())
}

fn build_buffers<Builder: SvgBuilder<PathType=Path>>(builder: &mut Builder,
                                      buffers: &mut VertexBuffers<display::Vertex>,
                                      stroke_color: [f32; 3],
                                      fill_color: [f32; 3],
                                      show_points: bool)
-> Result<()> {
    let path: Path = builder.build_and_reset();

    let events = FillEvents::from_iter(path.path_iter()
        .flattened(0.03));
    FillTessellator::new()
        .tessellate_events(&events,
                           &FillOptions::default(),
                           &mut BuffersBuilder::new(buffers, display::WithColor(fill_color)))
        .unwrap();

    StrokeTessellator::new()
        .tessellate(path.path_iter().flattened(0.03),
                    &StrokeOptions::stroke_width(1.0),

                    &mut BuffersBuilder::new(buffers, display::WithColor(stroke_color)))
        .unwrap();
    if show_points {
        for p in path.as_slice().iter() {
            if let Some(to) = p.destination() {
                tessellate_ellipsis(to,
                                    vec2(1.0, 1.0),
                                    16,
                                    &mut BuffersBuilder::new(buffers,
                                                             display::WithColor([0.0, 0.0, 0.0])));
                tessellate_ellipsis(to,
                                    vec2(0.5, 0.5),
                                    16,
                                    &mut BuffersBuilder::new(buffers,
                                                             display::WithColor([0.0, 1.0, 0.0])));
            }
        }
    }
    Ok(())
}
