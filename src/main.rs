extern crate xml;

use std::fs::File;
use std::io::BufReader;

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

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartDocument { version, encoding, standalone }) => {
                match standalone {
                    Some(s) => println!("{} {} {}", version, encoding, s),
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