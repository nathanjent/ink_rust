#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate clap;
#[macro_use] extern crate conrod;
extern crate piston_window as pw;
extern crate find_folder;
extern crate encoding;
extern crate svgparser;

mod inkapp;
//mod svg_canvas;
//mod svgdom;

mod errors {
    error_chain! { }
}

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
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
    use std::fs::File;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(filename) = m.value_of("INPUT") {
        InkApp::open("tests/documents/testrect.svg")
            .chain_err(|| "Unable to open Inkrust")?;
    } else {
    }


    Ok(())
}

#[cfg(not(feature = "yaml"))]
fn run() -> Result<()> {
    use inkapp::InkApp;
    use clap::App;
    use std::fs::File;

    // Just load example asset for now
    InkApp::open("tests/documents/testrect.svg")
        .chain_err(|| "Unable to open Inkrust")?;

    Ok(())
}
