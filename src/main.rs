#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate conrod;
#[macro_use]
extern crate svgparser;
extern crate piston_window as pw;
extern crate graphics;
extern crate find_folder;
extern crate encoding;

mod inkapp;
mod svg_parser;
//mod svg_canvas;
//mod svgdom;

mod errors {
    error_chain!{}
}

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // Use with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

#[cfg(feature = "yaml")]
fn run() -> Result<()> {
    use inkapp::InkApp;
    use clap::App;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let ink_app;
    if let Some(filename) = m.value_of("INPUT") {
        ink_app = InkApp::open(filename).chain_err(|| "Unable to open Inkrust")?;
    } else {
        ink_app = InkApp::new();
    }

    ink_app.start()?;
    Ok(())
}

#[cfg(not(feature = "yaml"))]
fn run() -> Result<()> {
    use inkapp::InkApp;

    // Just load example asset for now
    let ink_app =
        InkApp::open("tests/documents/testrect.svg").chain_err(|| "Unable to open Inkrust")?;

    ink_app.start()?;
    Ok(())
}
