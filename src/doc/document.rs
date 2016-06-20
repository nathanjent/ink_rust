use doc::node;

use std::collections::BTreeMap;

use xml::common::XmlVersion;
use xml::attribute::OwnedAttribute;

pub struct Document {
    version: XmlVersion,
    encoding: String,
    standalone: Option<bool>,
    tree: XmlTree,
}

type XmlTree = BTreeMap<u32, node::Node>;

impl Document {
    pub fn new() -> Self {
        Document {
            version: XmlVersion::Version10,
            encoding: "".to_string(),
            standalone: Some(false),
            tree: XmlTree::new(),
        }
    }

    pub fn set_version(&mut self, v: XmlVersion) {
        self.version = v;
    }

    pub fn set_encoding(&mut self, e: &str) {
        self.encoding = e.to_owned();
    }

    pub fn set_standalone(&mut self, s: bool) {
        self.standalone = Some(s);
    }

    pub fn add(&mut self, node: node::Node) {
        self.tree.insert(node.id, node);
    }
}
