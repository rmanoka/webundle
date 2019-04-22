#![feature(specialization)]

extern crate swc_common;
extern crate swc_ecma_parser;
use std::sync::Arc;
use swc_common::{SourceMap, FilePathMapping, GLOBALS};
use webundle::*;

fn main() {
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
        if let Err(e) = run() {
            eprintln!("{}", e);
        }
    });
}

fn run() -> Result<(), Error> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    println!("Bundling {}", args[1]);
    let req = Request::from_raw(&args[1]);
    let src = Source::from_request(req)?;
    let module = src.compile(cm.clone())?;

    Ok(())
}
