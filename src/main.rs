extern crate xml;

use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::RwLock;

use xml::{Event, Parser, Element};

// mod doc;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn main() {
    let mut file = File::open("tests/documents/testrect.svg").unwrap();
    //    let file = BufReader::new(file);

    let mut file_string = String::new();
    if let Err(err) = file.read_to_string(&mut file_string) {
        println!("Reading failed: {}", err);
        std::process::exit(1);
    };
    let mut parser = Parser::new();

    parser.feed_str(&file_string);
    for event in parser {
        match event.unwrap() {
            Event::ElementStart(tag) => println!("<{}>", tag.name),
            Event::ElementEnd(tag) => println!("</{}>", tag.name),
            _ => (),
        }
    }

    //    let reader = parserconfig::new()
    //        .whitespace_to_characters(true)
    //        .ignore_comments(true)
    //        .create_reader(bufreader::new(file));
    //
    //    let document = rwlock::new(doc::document::document::new());
    //
    //    let mut depth = 0;
    //    for e in reader {
    //        match e {
    //            ok(xmlEvent::StartDocument { version, encoding, standalone }) => {
    //                document.write().unwrap().set_version(version);
    //                document.write().unwrap().set_encoding(&encoding);
    //                document.write().unwrap().set_standalone(standalone.unwrap_or_default());
    //            }
    //            Ok(XmlEventa:EndDocument) => {
    //                println!("End");
    //            }
    //            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
    //                let node = doc::node::Node::new(0,
    //                                                attributes,
    //                                                doc::node::NodeType::Element {
    //                                                    elem_type: doc::node::ElementType::Rect,
    //                                                });
    //                document.write().unwrap().add(node);
    //                println!("{}+{}", indent(depth), name);
    //                depth += 1;
    //            }
    //            Ok(XmlEvent::EndElement { name }) => {
    //                depth -= 1;
    //                println!("{}-{}", indent(depth), name);
    //            }
    //            Ok(XmlEvent::CData(data)) => {
    //                println!("{}", data);
    //            }
    //            Ok(XmlEvent::Characters(data)) => {
    //                println!("{}", data);
    //            }
    //            Ok(XmlEvent::ProcessingInstruction { name, data }) => {
    //                println!("{} {}", name, data.unwrap_or_default());
    //            }
    //            Err(e) => {
    //                println!("Error: {}", e);
    //                break;
    //            }
    //            _ => {}
    //        }
    //    }
}
