use doc::node;

use std::sync::RwLock;
use std::collections::BTreeMap;

use xml::common::XmlVersion;

type Document<'a> = RwLock<BTreeMap<&'a str, &'a node::Node>>;

pub impl<'a> Document<'a> {
	pub fn new() {
		Document {
			version: XmlVersion::Version10,
			encoding: "",
			standalone: Some(false),
		}
	}
}