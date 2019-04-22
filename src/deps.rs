use swc_ecma_ast::{CallExpr, ExprOrSuper, Expr, Lit, Module};
use swc_common::{Fold, FoldWith};

use super::Id;

pub struct Compiled {
    module: Module,
    id: Id,
}
impl Compiled {
    pub fn new(id: Id, module: Module) -> Self {
        Compiled {id, module}
    }
    pub fn bundle(self) -> Bundled {
        let mut t = DependencyFolder::default();
        let module = self.module.fold_with(&mut t);
        Bundled {
            id: self.id,
            deps: t.deps,
            module,
        }
    }
}

pub struct Bundled {
    deps: Vec<String>,
    module: Module,
    id: Id,
}

#[derive(Default)]
pub struct DependencyFolder {
    pub deps: Vec<String>,
}
impl Fold<CallExpr> for DependencyFolder {

    fn fold(&mut self, mut expr: CallExpr) -> CallExpr {

        let callee = match expr.callee {
            ExprOrSuper::Expr(ref c) => c,
            _ => return expr,
        };

        let callee = match **callee {
            Expr::Ident(ref ident) => ident,
            _ => return expr,
        };

        if !callee.as_ref().eq("require") {
            return expr;
        }

        if !expr.args.len() == 1 {
            eprintln!("`require()` with more than one argument!");
            return expr;
        }

        let arg = &expr.args[0];
        if arg.spread.is_some() {
            eprintln!("`require()` with spread argument!")
        }

        let called = match *arg.expr {
            Expr::Lit(Lit::Str(ref s)) => &s.value,
            _ => return expr,
        };

        eprintln!("Dep: {}", called.as_ref());
        self.deps.push(called.as_ref().to_owned());

        let mut ident = callee.clone();
        ident.sym = "__bundle_require".into();

        expr.callee = ExprOrSuper::Expr(
            Box::new(Expr::Ident(ident))
        );

        expr
    }
}
