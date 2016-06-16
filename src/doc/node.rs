use std::cell::{Ref, RefMut, RefCell};
use std::rc::Rc;

pub struct Node<T> {
    elem: T,
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
    Element { elem_type: ElementType },
    Text,
    Comment,
    Pi,
}

pub enum ElementType {
	Rect,
	Svg,
}

impl<T> Node<T> {
    pub fn new(elem: T, node_type: NodeType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            node_type: node_type,
        }))
    }
}