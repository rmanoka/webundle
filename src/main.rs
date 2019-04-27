#![feature(specialization)]

use std::sync::Arc;
use swc_common::{SourceMap, FilePathMapping, GLOBALS};
use webundle::*;
use failure::Error;

fn main() {
    GLOBALS.set(&swc_common::Globals::new(), || {
        if let Err(e) = run() {
            eprintln!("{}", e);
        }
    });
}

fn run() -> Result<(), Error> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <request>", args[0]);
        std::process::exit(1);
    }

    let cwd = std::env::current_dir()?;
    let req = Request::new(&args[1], &cwd);

    let resolver = resolver::default();
    resolve(req, &cm, &resolver)?;


    // println!("Bundling {}", args[1]);
    // let req = Request::new(&args[1], &cwd);
    // let path = req.path.clone();
    // let src = Source::new(req)?;
    // let module = src.compile(cm.clone())?;
    // let deps = module.bundle();
    // println!("Depedencies:");
    // for d in deps.deps {
    //     println!("{}", d);
    //     let mut req = Request::new(&d, &path);
    //     let res = resolver.resolve(&mut req);
    //     println!("Resolved: {}", req.path.display());
    // }

    Ok(())
}

use std::fmt;
#[macro_use] extern crate failure_derive;
#[derive(Debug, Fail)]
#[fail(display)]
enum Failed {
    #[fail(display = "Unable to resolve {}\n  in context {}", _0, _1)]
    Resolve(String, String),
}

fn resolve(mut req: Request, cm: &Arc<SourceMap>, resolver: &dyn resolver::Context) -> Result<(), Error> {
    println!("Resolving {}", req.path);
    if !resolver.resolve(&mut req) {
        let ctx = req.ctx.to_string_lossy().to_string();
        return Err(Failed::Resolve(req.path, ctx))?;
    }

    println!("Compiling {}", req.path);

    let ctx = std::path::PathBuf::from(&req.path);
    let ctx = ctx.parent().expect("no parent for request");
    let src = Source::new(req)?;
    let module = src.compile(cm.clone())?;
    let deps = module.bundle();

    for d in deps.deps {
        resolve(Request::new(&d, &ctx), cm, resolver)?;
    }
    Ok(())
}
