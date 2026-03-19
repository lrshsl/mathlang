use std::collections::HashMap;

use mth_ast::{Expr, Function, FunctionCall, Literal, Module, TopLevel};
use mth_common::{
    N_PLOTS, PLOT_TYPE_EQUATION, PLOT_TYPE_FN_GRAPH, inst, ops::*, plot_desc::PlotDesc,
};

type CResult = Result<(u32, u32), String>;

pub fn compile_module(module: &Module) -> Result<(Vec<Instruction>, [PlotDesc; N_PLOTS]), String> {
    let mut ctx = HashMap::new();
    let mut instructions = Vec::new();
    let mut plot_descs = [PlotDesc::default(); N_PLOTS];
    let mut plot_index = 0usize;

    for expr in &module.top_level {
        match expr {
            TopLevel::Function(mapping) => {
                ctx.insert(mapping.name, mapping);
            }
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name,
                args,
                is_negated,
            })) if *name == "plot" => {
                if plot_index >= N_PLOTS {
                    return Err("Too many plots".to_string());
                }
                let Some(f) = args.get(0) else {
                    panic!("`plot` requires a function as argument");
                };
                let Expr::FunctionCall(FunctionCall { name: f, .. }) = f else {
                    panic!("`plot` requires a function as argument");
                };
                let Some(mapping) = ctx.get(f) else {
                    panic!("Could not resolve function `{f:?}`");
                };

                let (len, plot_type) = compile_fn(mapping, &mut instructions)?;
                if *is_negated {
                    instructions.push(inst!(OP_CONST, -1.0));
                    instructions.push(inst!(OP_MUL));
                }

                plot_descs[plot_index] = PlotDesc {
                    length: if *is_negated { len + 2 } else { len },
                    type_id: plot_type,
                    ..Default::default()
                };
                plot_index += 1;
            }
            other => return Err(format!("Invalid top-level: {other:?}")),
        }
    }

    Ok((instructions, plot_descs))
}

pub fn compile_fn(f: &Function, buf: &mut Vec<Instruction>) -> CResult {
    match f {
        Function { body, .. } => compile_expr(body, buf),
    }
}

pub fn compile_expr(expr: &Expr, buf: &mut Vec<Instruction>) -> CResult {
    match expr {
        Expr::Literal(lit) => compile_literal(lit, buf),
        Expr::FunctionCall(s_expr) => compile_s_expr(s_expr, buf),
    }
}

pub fn compile_literal(lit: &Literal, buf: &mut Vec<Instruction>) -> CResult {
    match lit {
        Literal::Int(int) => {
            buf.push(inst!(OP_CONST, *int as f32));
            Ok((1, PLOT_TYPE_FN_GRAPH))
        }
        Literal::Float(float) => {
            buf.push(inst!(OP_CONST, *float as f32));
            Ok((1, PLOT_TYPE_FN_GRAPH))
        }
        _ => Err(format!("Invalid literal: {lit:?}")),
    }
}

pub fn compile_s_expr(s_expr: &FunctionCall, buf: &mut Vec<Instruction>) -> CResult {
    let start_len = buf.len();
    let plot_type = match s_expr.name {
        "+" => {
            compile_binary_op(s_expr, OP_ADD, buf)?;
            PLOT_TYPE_FN_GRAPH
        }
        "-" => {
            compile_binary_op(s_expr, OP_SUB, buf)?;
            PLOT_TYPE_FN_GRAPH
        }
        "*" => {
            compile_binary_op(s_expr, OP_MUL, buf)?;
            PLOT_TYPE_FN_GRAPH
        }
        "/" => {
            compile_binary_op(s_expr, OP_DIV, buf)?;
            PLOT_TYPE_FN_GRAPH
        }
        "^" => {
            compile_binary_op(s_expr, OP_POW, buf)?;
            PLOT_TYPE_FN_GRAPH
        }

        "or" => {
            compile_binary_op(s_expr, OP_OR, buf)?;
            PLOT_TYPE_FN_GRAPH
        }
        "and" => {
            compile_binary_op(s_expr, OP_AND, buf)?;
            PLOT_TYPE_FN_GRAPH
        }

        "bitwise_or" => {
            compile_binary_op(s_expr, OP_BW_OR, buf)?;
            PLOT_TYPE_FN_GRAPH
        }
        "bitwise_xor" => {
            compile_binary_op(s_expr, OP_BW_XOR, buf)?;
            PLOT_TYPE_FN_GRAPH
        }
        "bitwise_and" => {
            compile_binary_op(s_expr, OP_BW_AND, buf)?;
            PLOT_TYPE_FN_GRAPH
        }

        "==" => {
            compile_binary_op(s_expr, OP_EQ, buf)?;
            PLOT_TYPE_EQUATION
        }
        "!=" => {
            compile_binary_op(s_expr, OP_NE, buf)?;
            PLOT_TYPE_EQUATION
        }
        "<" => {
            compile_binary_op(s_expr, OP_LT, buf)?;
            PLOT_TYPE_EQUATION
        }
        "<=" => {
            compile_binary_op(s_expr, OP_LE, buf)?;
            PLOT_TYPE_EQUATION
        }
        ">" => {
            compile_binary_op(s_expr, OP_GT, buf)?;
            PLOT_TYPE_EQUATION
        }
        ">=" => {
            compile_binary_op(s_expr, OP_GE, buf)?;
            PLOT_TYPE_EQUATION
        }

        "sin" | "cos" | "tan" | "log" => {
            if s_expr.args.len() != 1 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            let (_inner_len, _) = compile_expr(&s_expr.args[0], buf)?;
            let opcode = match s_expr.name {
                "sin" => OP_SIN,
                "cos" => OP_COS,
                "tan" => OP_TAN,
                "log" => OP_LOG,
                _ => unreachable!(),
            };
            buf.push(inst!(opcode));
            PLOT_TYPE_FN_GRAPH
        }

        "abs" => {
            if s_expr.args.len() != 1 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            let (_inner_len, _) = compile_expr(&s_expr.args[0], buf)?;
            buf.push(inst!(OP_ABS));
            PLOT_TYPE_FN_GRAPH
        }

        "pi" => {
            if s_expr.args.len() != 0 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            buf.push(inst!(OP_CONST, std::f32::consts::PI));
            PLOT_TYPE_FN_GRAPH
        }

        "x" => {
            if s_expr.args.len() != 0 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            buf.push(inst!(OP_X));
            PLOT_TYPE_FN_GRAPH
        }

        "y" => {
            if s_expr.args.len() != 0 {
                return Err(format!("Wrong number of arguments for {}", s_expr.name));
            }
            buf.push(inst!(OP_Y));
            PLOT_TYPE_FN_GRAPH
        }

        _ => return Err(format!("Unknown function: {}", s_expr.name)),
    };

    if s_expr.is_negated {
        buf.push(inst!(OP_CONST, -1.0));
        buf.push(inst!(OP_MUL));
    }

    let len = (buf.len() - start_len) as u32;
    Ok((len, plot_type))
}

pub fn compile_binary_op(
    s_expr: &FunctionCall,
    opcode: u32,
    buf: &mut Vec<Instruction>,
) -> Result<u32, String> {
    if s_expr.args.len() != 2 {
        return Err(format!("Wrong number of arguments for {}", s_expr.name));
    }
    let start_len = buf.len();
    let (_len1, _) = compile_expr(&s_expr.args[0], buf)?;
    let (_len2, _) = compile_expr(&s_expr.args[1], buf)?;
    buf.push(inst!(opcode));
    Ok((buf.len() - start_len) as u32)
}
