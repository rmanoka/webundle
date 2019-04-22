use failure::ResultExt;
use swc_common::{CM, FileName, SourceMap,
                 errors::{ColorConfig, Handler}};
use std::sync::Arc;

use super::{Request, Compiled, Error, ErrorKind};

pub type Id = Request;
pub type Result<T> = std::result::Result<T, Error>;

pub trait Transform {
    fn transform(&self, src: &mut Source) -> bool;
}

pub struct Source {
    id: Id,
    content: String,
}
impl Source {
    pub fn from_request(id: Request) -> Result<Self> {
        let content = std::fs::read_to_string(&id.path)
            .context(ErrorKind::FileRead)?;
        Ok(Source{ id, content })
    }
    pub fn compile(self, cm: Arc<SourceMap>) -> Result<Compiled> {

        use swc_ecma_parser::{Parser, Session,
                              SourceFileInput, Syntax};

        let handler = Handler::with_tty_emitter(
            ColorConfig::Auto, true, false, Some(cm.clone()));
        let session = Session { handler: &handler };

        let fm = cm.new_source_file(
            FileName::Custom(self.id.path.to_string_lossy()
                             .to_string()),
            self.content);

        let mut parser = Parser::new(
            session,
            Syntax::Es(Default::default()),
            SourceFileInput::from(&*fm),
            None, // Disable comments
        );

        let module = parser.parse_module()
            .map_err(|mut e| {
                e.emit();
                ErrorKind::ParseError
            })?;

        Ok(Compiled::new(self.id, module))
    }
}
