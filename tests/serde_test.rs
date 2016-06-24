#[macro_use]
extern crate serde;
extern crate serde_xml;

use std::fmt::Debug;

use serde_xml::from_str;
use serde_xml::value::{Element, from_value};

use serde::de;
use serde::ser;


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct Rect {
	x: isize,
	y: isize,
	width: usize,
	height: usize,
	rx: usize,
	ry: usize,
}

fn test_parse_ok<'a, T>(errors: &[(&'a str, T)])
where T: PartialEq + Debug + ser::Serialize + de::Deserialize,
{
    for &(s, ref value) in errors {
        let v: T = from_str(s).unwrap();
        assert_eq!(v, *value);

        // Make sure we can deserialize into an `Element`.
        let xml_value: Element = from_str(s).unwrap();

        // Make sure we can deserialize from an `Element`.
        let v: T = from_value(xml_value.clone()).unwrap();
        assert_eq!(v, *value);
    }
}

#[test]
fn test_parse_rect() {

    test_parse_ok(&[
    	(
			r#"<rect
         height="1080"
         id="bgfill057"
         style="fill:#A7CCD8"
         width="1920" />"#,
            Rect {
				x: Default::default(),
				y: Default::default(),
				width: 1920,
				height: 1080,
				rx: Default::default(),
				ry: Default::default(),}
    	)
    ]);
}