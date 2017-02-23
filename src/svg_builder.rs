use errors::*;
use lyon::lyon_core::ArcFlags;
use lyon::math::{ vec2, rad };
use lyon::path_builder::SvgBuilder;
use svgdom::{AttributeValue, Document, ElementId, NodeEdge, NodeType};
use svgdom::types::*;


pub fn build_path_from_dom<Builder: SvgBuilder>(dom: &Document, path: &mut Builder) -> Result<()> {
    if let Some(svg) = dom.svg_element() {
        let mut traversal = svg.traverse();
        loop {
            if let Some(nodeEdge) = traversal.next() {
                match nodeEdge {
                    NodeEdge::Start(node) => {
                        match node.node_type() {
                            NodeType::Element => {
                                if let Some(tag_id) = node.tag_id() {
                                    match tag_id {
                                        ElementId::Path => {
                                        }
                                        ElementId::Rect => {
                                            let attrs = node.attributes();
                                            let mut x = 0.;
                                            let mut y = 0.;
                                            let mut rx = 0.;
                                            let mut ry = 0.;
                                            let mut w = 1.;
                                            let mut h = 1.;
                                            let mut transform = None;
                                            if let Some(&AttributeValue::Length(Length { num: v, .. }))
                                                = attrs.get_value("x") {
                                                    x = v as f32;
                                            }
                                            if let Some(&AttributeValue::Length(Length { num: v, .. }))
                                                = attrs.get_value("y") {
                                                    y = v as f32;
                                            }
                                            if let Some(&AttributeValue::Length(Length { num: v, .. }))
                                                = attrs.get_value("rx") {
                                                    rx = v as f32;
                                            }
                                            if let Some(&AttributeValue::Length(Length { num: v, .. }))
                                                = attrs.get_value("ry") {
                                                    ry = v as f32;
                                            }
                                            if let Some(&AttributeValue::Length(Length { num: v, .. }))
                                                = attrs.get_value("width") {
                                                    w = v as f32;
                                            }
                                            if let Some(&AttributeValue::Length(Length { num: v, .. }))
                                                = attrs.get_value("height") {
                                                    h = v as f32;
                                            }
                                            if let Some(&AttributeValue::Transform(t))
                                                = attrs.get_value("transform") {
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
                                            let arc_flags = ArcFlags {
                                                large_arc: false,
                                                sweep: true,
                                            };

                                            path.move_to(vec2(x + rx, y));
                                            path.line_to(vec2(x + w - rx, y));
                                            path.line_to(vec2(x + w - rx, y));
                                            path.arc_to(vec2(x + w, y + ry), vec2(rx, ry), rad(0.), arc_flags);
                                            path.line_to(vec2(x + w, y + h - ry));
                                            path.arc_to(vec2(x + w - rx, y + h), vec2(rx, ry), rad(0.), arc_flags);
                                            path.line_to(vec2(x + rx, y + h));
                                            path.arc_to(vec2(x, y + h - ry), vec2(rx, ry), rad(0.), arc_flags);
                                            path.line_to(vec2(x, y + ry));
                                            path.arc_to(vec2(x + rx, y), vec2(rx, ry), rad(0.), arc_flags);
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
