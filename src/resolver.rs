use std::path::{PathBuf, Path};
use super::Seq;

pub fn default() -> Box<dyn Context> {
    Box::new(seq![
        DotResolver,
    ])
}

pub trait Context {
    fn resolve(&self, request: &mut Request) -> bool;
}
impl<A: Context, B: Context> Context for Seq<A, B> {
    fn resolve(&self, request: &mut Request) -> bool {
        self.0.resolve(request)
            || self.1.resolve(request)
    }
}
impl<T: Context + ?Sized> Context for Box<T> {
    fn resolve(&self, request: &mut Request) -> bool {
        (**self).resolve(request)
    }
}

use std::collections::HashMap;
pub struct Request {
    pub raw: String,
    pub path: String,
    pub params: HashMap<String, String>,
    pub ctx: PathBuf,
}
impl Request {
    pub fn new(raw: &str, ctx: &Path) -> Self {

        let splits: Vec<_> = raw.rsplitn(2, "?").collect();

        let mut params = HashMap::<String, String>::new();

        let path = if splits.len() == 1 {
            splits[0]
        } else {
            use url::form_urlencoded::parse;
            params.extend(
                parse(splits[0].as_bytes())
                    .map(|(k, v)| (k.to_string(), v.to_string()))
            );
            splits[1]
        }.to_owned();
        let raw = raw.to_owned();
        let ctx = ctx.to_owned();

        Request{ raw, path, params, ctx }
    }
}

pub struct DotResolver;
impl Context for DotResolver {

    fn resolve(&self, request: &mut Request) -> bool {
        // Check if path starts with a `.` or `..`
        let dot = request.path.chars().nth(0)
            .expect("request empty");
        if dot == '.' {

            let can = request.ctx.join(&request.path).canonicalize();
            if let Ok(p) = can {
                std::mem::replace(&mut request.path, p.to_str().unwrap().to_owned());
                return true;
            }
        }

        false
    }

}

pub struct DirResolver {
    base: PathBuf,
}
impl DirResolver {
    pub fn new(base: &Path) -> Self {
        DirResolver { base: base.to_owned() }
    }
}
impl Context for DirResolver {

    fn resolve(&self, request: &mut Request) -> bool {
        // Check if path starts with a normal component
        use std::path::Component;
        let first = request.path.chars().nth(0).expect("request empty");
        if (first == '/') || (first == '.') { return false; }

        let path = self.base.join(&request.path).canonicalize();
        if let Ok(p) = path {
            std::mem::replace(&mut request.path, p.to_str().unwrap().to_owned());
            return true;
        }

        false
    }

}

pub struct NodePackageResolver;
impl Context for NodePackageResolver {
    fn resolve(&self, request: &mut Request) -> bool {
        false
    }
}
