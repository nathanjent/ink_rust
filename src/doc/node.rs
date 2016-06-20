use xml;

pub struct Node {
    pub id: u32,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    node_type: NodeType,
}

/// Document: Top-level node. Do not confuse with the root node.
/// Element: Regular node, e.g. <group/>.
/// Text: e.g. "Some text" in <group>Some text</group> is
/// represented by a text node.
/// Comment: e.g. <!-- some comment -->
/// PI: Processing instruction,
/// e.g. <?xml version="1.0" encoding="utf-8" standalone="no"?>
pub enum NodeType {
    Document,
    Element {
        elem_type: ElementType,
    },
    Text,
    Comment,
    Pi,
}

pub enum ElementType {
    Rect,
    Svg,
}

impl Node {
    pub fn new(id: u32,
               attributes: Vec<xml::attribute::OwnedAttribute>,
               node_type: NodeType)
               -> Self {
        Node {
            id: id,
            attributes: attributes,
            node_type: node_type,
        }
    }
}
