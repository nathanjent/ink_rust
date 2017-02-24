use errors::*;
use lyon::lyon_core::ArcFlags;
use lyon::math::{ vec2, rad };
use lyon::path_builder::SvgBuilder;
use svgdom::{AttributeId, AttributeValue, Document, ElementId, NodeEdge, NodeType};
use svgdom::types::*;


pub fn build_path_from_dom<Builder: SvgBuilder>(dom: &Document, path: &mut Builder) -> Result<()> {
    if let Some(svg) = dom.svg_element() {
        let mut traversal = svg.traverse();
        loop {
            if let Some(nodeEdge) = traversal.next() {
                match nodeEdge {
                    NodeEdge::Start(node) => {
                        println!("{:?}", node);
                        match node.node_type() {
                            NodeType::Element => {
                                if let Some(tag_id) = node.tag_id() {
                                    println!("{:?}", tag_id);
                                    match tag_id {
                                        ElementId::Path => {
                                        }
                                        ElementId::Circle => {
                                            let mut cx = 0.;
                                            let mut cy = 0.;
                                            let mut r = 1.;
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
                                            build_ellipse(cx, cy, r, r, path); 
                                        }
                                        ElementId::Ellipse => {
                                            let mut cx = 0.;
                                            let mut cy = 0.;
                                            let mut rx = 1.;
                                            let mut ry = 1.;
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
                                            build_ellipse(cx, cy, rx, ry, path); 
                                        }
                                        ElementId::Rect => {
                                            let mut x = 0.;
                                            let mut y = 0.;
                                            let mut rx = 0.;
                                            let mut ry = 0.;
                                            let mut w = 1.;
                                            let mut h = 1.;
                                            let mut transform = None;
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

                                            if let Some(AttributeValue::Transform(t))
                                                = node.attribute_value(AttributeId::Transform) {
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

                                            println!(
                                                "x:{:?} y:{:?} w:{:?} h:{:?} rx:{:?} ry:{:?}",
                                                x, y, w, h, rx, ry);

                                            path.move_to(vec2(x + rx, y));
                                            path.line_to(vec2(x + w - rx, y));
                                            path.line_to(vec2(x + w - rx, y));
                                            path.arc_to( vec2(x + w, y + ry),
                                                rxry, x_axis_rotation, arc_flags);
                                            path.line_to(vec2(x + w, y + h - ry));
                                            path.arc_to(vec2(x + w - rx, y + h),
                                                rxry, x_axis_rotation, arc_flags);
                                            path.line_to(vec2(x + rx, y + h));
                                            path.arc_to(vec2(x, y + h - ry),
                                                rxry, x_axis_rotation, arc_flags);
                                            path.line_to(vec2(x, y + ry));
                                            path.arc_to(vec2(x + rx, y),
                                                rxry, x_axis_rotation, arc_flags);
                                            path.close();

                                            //TODO apply transform
                                            if let Some(t) = transform {
                                                let Transform {
                                                    a: a,
                                                    b: b,
                                                    c: c,
                                                    d: d,
                                                    e: e,
                                                    f: f,
                                                } = t;
                                                println!("{:?}", t);
                                            }
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
                break
            }
        }
    }
    Ok(())
}

fn build_ellipse<Builder: SvgBuilder>(cx: f32, cy: f32, rx: f32, ry: f32, path: &mut Builder) -> Result<()> {
    if rx > 0. && ry > 0. {
        let rxry = vec2(rx, ry);
        let x_axis_rotation = rad(0.);
        let arc_flags = ArcFlags {
            large_arc: false,
            sweep: true,
        };
        path.move_to(vec2(cx + rx, cy));
        path.arc_to( vec2(cx, cy + ry),
            rxry, x_axis_rotation, arc_flags);
        path.arc_to( vec2(cx - rx, cy),
            rxry, x_axis_rotation, arc_flags);
        path.arc_to(vec2(cx, cy - ry),
            rxry, x_axis_rotation, arc_flags);
        path.arc_to(vec2(cx + rx, cy),
            rxry, x_axis_rotation, arc_flags);
        path.close();
    } else if rx < 0. || ry < 0. {
        // error
    }
    Ok(())
}
