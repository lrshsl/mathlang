use std::collections::HashMap;

use mth_ast::{Expr, Module, SExpr, TopLevel};
use mth_common::{inst, ops::*};

pub fn compile_module(module: &Module) -> Result<Instructions, ()> {
    let mut ctx = HashMap::new();
    for expr in &module.top_level {
        match expr {
            TopLevel::MapImpl(mapping) => {
                let _ = ctx.insert(mapping.name, mapping);
            }
            TopLevel::Expr(Expr::SExpr(SExpr { name, args })) if *name == "plot" => {
                let Some(f) = args.get(0) else {
                    panic!("plot requires a function as argument");
                };
                return compile_fn(f);
            }
            _ => return Err(()),
        }
    }
    Ok(Vec::from([
        inst!(OP_X_POLY, -1., 3.),
        inst!(OP_CONST, 1.),
        inst!(OP_ADD),
    ]))
}

fn compile_fn(f: &Expr) -> Result<Instructions, ()> {
    Ok(Vec::from([
        inst!(OP_X_POLY, 1., 3.),
        inst!(OP_CONST, 1.),
        inst!(OP_ADD),
    ]))
}
