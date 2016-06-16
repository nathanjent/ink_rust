pub struct Node<T> {
    elem: T,
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
    Element,
    Text,
    Comment,
    Pi,
}
