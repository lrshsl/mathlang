use std::collections::HashMap;

use mth_ast::{Expr, Function, FunctionCall, Literal, Module, TopLevel};
use mth_common::{inst, ops::*};

pub fn compile_module(module: &Module) -> Result<Vec<Instruction>, String> {
    let mut ctx = HashMap::new();
    for expr in &module.top_level {
        match expr {
            TopLevel::Function(mapping) => {
                // Todo: pre-compile mapping?
                // Currently it is re-compiled each time it is called
                let _ = ctx.insert(mapping.name, mapping);
            }
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name,
                args,
                is_negated,
            })) if *name == "plot" => {
                let Some(f) = args.get(0) else {
                    panic!("`plot` requires a function as argument");
                };
                let Expr::FunctionCall(FunctionCall { name: f, .. }) = f else {
                    panic!("`plot` requires a function as argument");
                };
                let Some(mapping) = ctx.get(f) else {
                    panic!("Could not resolve function `{f:?}`");
                };

                let mut instructions = compile_fn(mapping)?;
                if *is_negated {
                    instructions.extend([inst![OP_CONST, -1.0], inst![OP_MUL]]);
                }
                return Ok(instructions);
            }
            other => return Err(format!("Invalid top-level: {other:?}")),
        }
    }
    Ok(vec![])
}

pub fn compile_fn(f: &Function) -> Result<Vec<Instruction>, String> {
    match f {
        Function { body, .. } => compile_expr(body),
    }
}

pub fn compile_expr(expr: &Expr) -> Result<Vec<Instruction>, String> {
    match expr {
        Expr::Literal(lit) => compile_literal(lit),
        Expr::FunctionCall(s_expr) => compile_s_expr(s_expr),
    }
}

pub fn compile_literal(lit: &Literal) -> Result<Vec<Instruction>, String> {
    match lit {
        Literal::Int(int) => Ok(vec![inst!(OP_CONST, *int as f32)]),
        Literal::Float(float) => Ok(vec![inst!(OP_CONST, *float as f32)]),
        _ => Err(format!("Invalid literal: {lit:?}")),
    }
}

pub fn compile_s_expr(s_expr: &FunctionCall) -> Result<Vec<Instruction>, String> {
    let mut instructions = match s_expr.name {
        // Built-in arithmetic ops
        "+" => compile_binary_op(s_expr, OP_ADD),
        "-" => compile_binary_op(s_expr, OP_SUB),
        "*" => compile_binary_op(s_expr, OP_MUL),
        "/" => compile_binary_op(s_expr, OP_DIV),
        "^" => compile_binary_op(s_expr, OP_POW),

        // Logical ops
        "or" => compile_binary_op(s_expr, OP_OR),
        "and" => compile_binary_op(s_expr, OP_AND),

        // Bitwise ops
        "bitwise_or" => compile_binary_op(s_expr, OP_BW_OR),
        "bitwise_xor" => compile_binary_op(s_expr, OP_BW_XOR),
        "bitwise_and" => compile_binary_op(s_expr, OP_BW_AND),

        // Comparison ops
        "==" => compile_binary_op(s_expr, OP_EQ),
        "!=" => compile_binary_op(s_expr, OP_NE),
        "<" => compile_binary_op(s_expr, OP_LT),
        "<=" => compile_binary_op(s_expr, OP_LE),
        ">" => compile_binary_op(s_expr, OP_GT),
        ">=" => compile_binary_op(s_expr, OP_GE),

        // Mathematical functions
        "sin" | "cos" | "tan" | "log" => {
            if s_expr.args.len() != 1 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            let mut instructions = compile_expr(&s_expr.args[0])?;
            let opcode = match s_expr.name {
                "sin" => OP_SIN,
                "cos" => OP_COS,
                "tan" => OP_TAN,
                "log" => OP_LOG,
                _ => unreachable!(),
            };
            instructions.push(inst!(opcode));
            Ok(instructions)
        }

        "pi" => {
            if s_expr.args.len() != 0 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            Ok(vec![inst!(OP_CONST, std::f32::consts::PI)])
        }

        "x" => {
            if s_expr.args.len() != 0 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            Ok(vec![inst!(OP_X)])
        }

        "y" => {
            if s_expr.args.len() != 0 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            Ok(vec![inst!(OP_Y)])
        }

        _ => Err(format!("Unknown function: {}", s_expr.name)),
    }?;

    if s_expr.is_negated {
        instructions.extend([inst![OP_CONST, -1.0], inst![OP_MUL]]);
    }
    return Ok(instructions);
}

pub fn compile_binary_op(s_expr: &FunctionCall, opcode: u32) -> Result<Vec<Instruction>, String> {
    if s_expr.args.len() != 2 {
        return Err(format!("Wrong number of arguments for {}", s_expr.name));
    }
    let mut instructions = compile_expr(&s_expr.args[0])?;
    instructions.extend(compile_expr(&s_expr.args[1])?);
    instructions.push(inst!(opcode));
    Ok(instructions)
}
