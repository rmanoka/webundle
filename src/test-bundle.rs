#![feature(specialization)]

extern crate swc_common;
extern crate swc_ecma_parser;
use std::sync::Arc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FilePathMapping, SourceMap, FoldWith,
};
use swc_ecma_parser::{Parser, Session, SourceFileInput, Syntax};

use webundle::deps::DependencyFolder;

fn main() {
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
        let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
        let handler = Handler::with_tty_emitter(
            ColorConfig::Auto, true, false, Some(cm.clone()));
        let session = Session { handler: &handler };

        use std::path::Path;
        let fm = cm
            .load_file(Path::new("test.js"))
            .expect("failed to load test.js");

        // let fm = cm.new_source_file(
        //     FileName::Custom("test.js".into()),
        //     "function foo() {}".into(),
        // );

        let mut parser = Parser::new(
            session,
            Syntax::Es(Default::default()),
            SourceFileInput::from(&*fm),
            None, // Disable comments
        );

        let module = parser
            .parse_module()
            .map_err(|mut e| {
                e.emit();
                ()
            })
            .expect("failed to parser module");

        swc_common::CM.set(&cm, || {

            let mut transform = DependencyFolder::default();
            let module = module.fold_with(&mut transform);
            // println!("{:?}", module);

            use std::fs::File;
            let out_file = File::create("output.js")
                .expect("could not create output file");

            use swc_ecma_codegen::{Handlers, Emitter};
            use sourcemap::SourceMapBuilder;
            struct MyHandlers;
            impl Handlers for MyHandlers {}

            // let mut src_map_builder = SourceMapBuilder::new(None);
            let handlers = Box::new(MyHandlers);

            let mut emitter = Emitter {
                cfg: Default::default(),
                comments: None,
                cm: cm.clone(),
                wr: Box::new(swc_ecma_codegen::text_writer::JsWriter::new(
                    cm.clone(),
                    "\n",
                    out_file,
                    None,
                )),
                handlers,
                pos_of_leading_comments: Default::default(),
            };

            emitter.emit_module(&module).
                expect("unable to emit output JS");

            // serde_json::to_writer(std::io::stdout(), &module)
            //     .expect("failed to write json");

        });

    });
}
