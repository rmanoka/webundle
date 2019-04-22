use std::path::{PathBuf, Path};
use super::Seq;

pub trait Context {
    fn resolve(&self, request: &mut Request) -> bool;
}
impl<A: Context, B: Context> Context for Seq<A, B> {
    fn resolve(&self, request: &mut Request) -> bool {
        self.0.resolve(request)
            || self.1.resolve(request)
    }
}

use std::collections::HashMap;
pub struct Request {
    pub raw: String,
    pub path: PathBuf,
    pub params: HashMap<String, String>,
}
impl Request {
    pub fn from_raw(raw: &str) -> Self {

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
        }.into();
        let raw = raw.to_owned();

        Request {raw, path, params}
    }
}

pub struct DotResolver {
    base: PathBuf,
}
impl DotResolver {
    pub fn new(base: &Path) -> Self {
        DotResolver { base: base.to_owned() }
    }
}
impl Context for DotResolver {

    fn resolve(&self, request: &mut Request) -> bool {
        // Check if path starts with a `.` or `..`
        use std::path::Component;
        let dot = match request.path.components().nth(0) {
            Some(d) => d,
            None => return false,
        };

        if (dot == Component::CurDir)
            || (dot == Component::ParentDir) {
                let can = self.base.join(&request.path).canonicalize();
                if let Ok(p) = can {
                    std::mem::replace(&mut request.path, p);
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
        let dot = match request.path.components().nth(0) {
            Some(d) => d,
            None => return false,
        };

        if let Component::Normal(_) = dot {
            let can = self.base.join(&request.path).canonicalize();
            if let Ok(p) = can {
                std::mem::replace(&mut request.path, p);
                return true;
            }
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
