extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::collections::BTreeMap;

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
        .ignore_comments(false)
        .create_reader(BufReader::new(file));
    let mut document = BTreeMap::new();

    let mut depth = 0;
    for e in reader {
        match e {
            Ok(XmlEvent::StartDocument { version, encoding, standalone }) => {
                match standalone {
                    Some(s) => {
                        let xml_str = format!("{} {} {}", version, encoding, s);
                        document.insert(depth,
                                        doc::node::Node::new(xml_str,
                                                             doc::node::NodeType::Document));
                        println!("{} {} {}", version, encoding, s)
                    }
                    None => println!("{} {}", version, encoding),
                }
            }
            Ok(XmlEvent::EndDocument) => {
                println!("End");
            }
            Ok(XmlEvent::StartElement { name, .. }) => {
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
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
