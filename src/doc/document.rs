use std::collections::BTreeMap;
use std::cell::{RefCell};

use xml::{Element};

#[derive(Clone, PartialEq)]
pub struct Document {
    version: XmlVersion,
    encoding: String,
    standalone: Option<bool>,
    tree: RefCell<Vec<Element>>,
}

pub type XmlTree = BTreeMap<usize, Element>;

/// XML version enumeration.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum XmlVersion {
    /// XML version 1.0.
    Version10,

    /// XML version 1.1.
    Version11
}

impl Document {
    pub fn new() -> Self {
        Document {
            version: XmlVersion::Version10,
            encoding: "".to_string(),
            standalone: Some(false),
            tree: RefCell::new(Vec::new()),
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
    
    pub fn set_tree(&mut self, mut tree: Vec<Element>) {
    	self.tree.borrow_mut().append(&mut tree);
    }

    pub fn add(&mut self, elem: Element) {
        self.tree.borrow_mut().push(elem);
    }
    
    pub fn get_tree(&self) -> Vec<Element> {
    	self.tree.clone().into_inner()
    }
}
