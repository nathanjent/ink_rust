use doc::attribute;

pub struct RectAttribute {
	name: String,
	attr: RectAttributeType,
	value: String,
}

pub enum RectAttributeType {
	None,
	X,
	Y,
	Width,
	Height,
	Rx,
	Ry,
}

pub struct DocumentAttribute {
	name: String,
	attr: DocumentAttributeType,
	value: String,
}

pub enum DocumentAttributeType {
	Version, 
	Encoding, 
	Standalone,
}