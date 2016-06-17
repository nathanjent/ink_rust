extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::{EventWriter, ParserConfig};
use xml::reader::{EventReader, XmlEvent};

mod doc;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn main() {
    let file = File::open("tests/documents/testrect.svg").unwrap();
    let file = BufReader::new(file);
    let reader = ParserConfig::new()
        .whitespace_to_characters(true)
        .ignore_comments(true)
        .create_reader(BufReader::new(file));
        
    let document = doc::document::Document::new();

    let mut depth = 0;
    for e in reader {
        match e {
            Ok(XmlEvent::StartDocument { version, encoding, standalone }) => {
            	let sa = standalone.map_or_else(|n| "no", |s| if s { "yes" } else { "no" });
//            	let attributes = vec!(
//            		xml::attribute::OwnedAttribute::new(
//            			xml::name::OwnedName::new("version", , )
//            			, version),
//            		xml::attribute::OwnedAttribute::new("encoding", encoding),
//            		xml::attribute::OwnedAttribute::new("standalone", sa),
//            	);
                let xml_str = format!("{} {} {}", version, encoding, sa);
                let mut doc = document.write().unwrap();
                doc.insert(depth,
                                doc::node::Node::new(xml_str,
                                                     doc::node::NodeType::Document));
                println!("{} {} {}", version, encoding, sa)
            }
            Ok(XmlEvent::EndDocument) => {
                println!("End");
            }
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                let mut doc = document.write().unwrap();
            	let elem = doc::node::Node::new(attributes, 
            		doc::node::NodeType::Element{
            			elem_type: doc::node::ElementType::Rect,
            		});
            	
            	doc.insert(depth + 0, elem);
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Ok(XmlEvent::CData(data)) => {
                println!("{}", data);
            }
            Ok(XmlEvent::Characters(data)) => {
                println!("{}", data);
            }
            Ok(XmlEvent::ProcessingInstruction(name, data)) => {
                println!("{} {}",name , data);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
