use std::collections::HashMap;

use mth_ast::{Expr, FunctionCall, Literal, Mapping, Module, TopLevel};
use mth_common::{inst, ops::*};

pub fn compile_module(module: &Module) -> Result<Vec<Instruction>, ()> {
    let mut ctx = HashMap::new();
    for expr in &module.top_level {
        match expr {
            TopLevel::MapImpl(mapping) => {
                // Todo: pre-compile mapping?
                // Currently it is re-compiled each time it is called
                let _ = ctx.insert(mapping.name, mapping);
            }
            TopLevel::Expr(Expr::FunctionCall(FunctionCall { name, args })) if *name == "plot" => {
                let Some(f) = args.get(0) else {
                    panic!("`plot` requires a function as argument");
                };
                let Expr::FunctionCall(FunctionCall { name: f, .. }) = f else {
                    panic!("`plot` requires a function as argument");
                };
                let Some(mapping) = ctx.get(f) else {
                    panic!("Could not resolve function `{f:?}`");
                };

                return compile_fn(mapping);
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

pub fn compile_fn(f: &Mapping) -> Result<Vec<Instruction>, ()> {
    match f {
        Mapping { body, .. } => compile_expr(body),
    }
}

pub fn compile_expr(expr: &Expr) -> Result<Vec<Instruction>, ()> {
    match expr {
        Expr::Literal(lit) => compile_literal(lit),
        Expr::FunctionCall(s_expr) => compile_s_expr(s_expr),
    }
}

pub fn compile_literal(lit: &Literal) -> Result<Vec<Instruction>, ()> {
    match lit {
        Literal::Int(int) => Ok(vec![inst!(OP_CONST, *int as f32)]),
        Literal::Float(float) => Ok(vec![inst!(OP_CONST, *float as f32)]),
        _ => Err(()),
    }
}

pub fn compile_s_expr(s_expr: &FunctionCall) -> Result<Vec<Instruction>, ()> {
    match s_expr.name {
        // Built-in arithmetic operations
        "+" => compile_binary_op(s_expr, OP_ADD),
        "-" => compile_binary_op(s_expr, OP_SUB),
        "*" => compile_binary_op(s_expr, OP_MUL),
        "/" => compile_binary_op(s_expr, OP_DIV),
        "^" => compile_binary_op(s_expr, OP_POW),
        "==" => compile_binary_op(s_expr, OP_EQ),

        // Mathematical functions
        "sin" | "cos" | "tan" | "log" => {
            if s_expr.args.len() != 1 {
                return Err(());
            }
            let mut instructions = compile_expr(&s_expr.args[0])?;
            let opcode = match s_expr.name {
                "sin" => OP_SIN,
                "cos" => OP_COS,
                "tan" => OP_TAN,
                "log" => OP_LOG,
                _ => return Err(()),
            };
            instructions.push(inst!(opcode));
            Ok(instructions)
        }

        "pi" => {
            if s_expr.args.len() != 0 {
                return Err(());
            }
            Ok(vec![inst!(OP_CONST, std::f32::consts::PI)])
        }

        "x" => {
            if s_expr.args.len() != 0 {
                return Err(());
            }
            Ok(vec![inst!(OP_X)])
        }

        "y" => {
            if s_expr.args.len() != 0 {
                return Err(());
            }
            Ok(vec![inst!(OP_Y)])
        }

        // Unknown function
        _ => Err(()),
    }
}

pub fn compile_binary_op(s_expr: &FunctionCall, opcode: u32) -> Result<Vec<Instruction>, ()> {
    if s_expr.args.len() < 2 {
        return Err(());
    }

    let mut instructions = compile_expr(&s_expr.args[0])?;
    for arg in s_expr.args[1..].iter() {
        let inst = compile_expr(arg)?;

        instructions.reserve(inst.len() + 1);
        instructions.extend(inst);
        instructions.push(inst![opcode])
    }

    Ok(instructions.to_vec())
}
