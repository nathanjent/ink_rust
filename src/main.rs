#![recursion_limit = "1024"]

#[macro_use]
extern crate conrod;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate svgparser;
extern crate svgdom;
extern crate find_folder;
extern crate encoding;

#[macro_use]
extern crate glium;
extern crate lyon;

mod inkapp;
mod svg_parser;
mod svg_builder;
mod display;

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

fn run() -> Result<()> {
    use inkapp::InkApp;
    use clap::App;

    //let yaml = load_yaml!("cli.yml");
    //let matches = App::from_yaml(yaml).get_matches();

    let mut ink_app = InkApp::new();
    let filename = "tests/documents/testrect.svg";
    //if let Some(f) = matches.value_of("INPUT") {
    //    filename = f;
    //}
    ink_app.open(filename).chain_err(|| "Unable to open file")?;

    ink_app.start()
        .chain_err(|| "Unable to start Inkrust")?;
    Ok(())
}
