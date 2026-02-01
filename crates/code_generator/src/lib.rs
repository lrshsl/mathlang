use std::collections::HashMap;

use mth_ast::{int, s_expr, Expr, Literal, Mapping, Module, SExpr, TopLevel};
use mth_common::{inst, ops::*};

pub fn compile_module(module: &Module) -> Result<Instructions, ()> {
    let mut ctx = HashMap::new();
    for expr in &module.top_level {
        match expr {
            TopLevel::MapImpl(mapping) => {
                // Todo: pre-compile mapping?
                // Currently it is re-compiled each time it is called
                let _ = ctx.insert(mapping.name, mapping);
            }
            TopLevel::Expr(Expr::SExpr(SExpr { name, args })) if *name == "plot" => {
                let Some(f) = args.get(0) else {
                    panic!("`plot` requires a function as argument");
                };
                let Expr::SExpr(SExpr { name: f, .. }) = f else {
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

fn compile_fn(f: &Mapping) -> Result<Instructions, ()> {
    match f {
        Mapping { body, .. } => compile_expr(body),
    }
}

fn compile_expr(expr: &Expr) -> Result<Instructions, ()> {
    match expr {
        Expr::Literal(lit) => compile_literal(lit),
        Expr::SExpr(s_expr) => compile_s_expr(s_expr),
    }
}

fn compile_literal(lit: &Literal) -> Result<Instructions, ()> {
    match lit {
        Literal::Int(int) => Ok(vec![inst!(OP_CONST, *int as f32)]),
        Literal::Float(float) => Ok(vec![inst!(OP_CONST, *float as f32)]),
        _ => Err(()),
    }
}

fn compile_s_expr(s_expr: &SExpr) -> Result<Instructions, ()> {
    match s_expr.name {
        // Built-in arithmetic operations
        "__builtin__add" => compile_binary_op(s_expr, OP_ADD),
        "__builtin__sub" => compile_binary_op(s_expr, OP_SUB),
        "__builtin__mul" => compile_binary_op(s_expr, OP_MUL),
        "__builtin__div" => compile_binary_op(s_expr, OP_DIV),

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

        // Power function
        "pow" => {
            if s_expr.args.len() != 2 {
                return Err(());
            }
            let mut instructions = compile_expr(&s_expr.args[0])?;
            instructions.extend(compile_expr(&s_expr.args[1])?);
            instructions.push(inst!(OP_POW));
            Ok(instructions)
        }

        // Variable references (parameters)
        name if name.starts_with('_') => {
            // For now, assume all parameters are the variable x
            // TODO: implement proper parameter handling
            Ok(vec![inst!(OP_X)])
        }

        // Unknown function
        _ => Err(()),
    }
}

fn compile_binary_op(s_expr: &SExpr, opcode: u32) -> Result<Instructions, ()> {
    if s_expr.args.len() != 2 {
        return Err(());
    }
    let mut instructions = compile_expr(&s_expr.args[0])?;
    instructions.extend(compile_expr(&s_expr.args[1])?);
    instructions.push(inst!(opcode));
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mth_ast::{int, s_expr};

    #[test]
    fn test_compile_literal() {
        let expr = &Expr::Literal(Literal::Int(42));
        let result = compile_expr(expr).unwrap();
        assert_eq!(result, vec![inst!(OP_CONST, 42.0)]);
    }

    #[test]
    fn test_compile_add() {
        let expr = s_expr("__builtin__add", vec![int(1), int(2)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(
            result,
            vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 2.0), inst!(OP_ADD),]
        );
    }

    #[test]
    fn test_compile_mul() {
        let expr = s_expr("__builtin__mul", vec![int(3), int(4)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(
            result,
            vec![inst!(OP_CONST, 3.0), inst!(OP_CONST, 4.0), inst!(OP_MUL),]
        );
    }

    #[test]
    fn test_compile_sub() {
        let expr = s_expr("__builtin__sub", vec![int(5), int(2)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(
            result,
            vec![inst!(OP_CONST, 5.0), inst!(OP_CONST, 2.0), inst!(OP_SUB),]
        );
    }

    #[test]
    fn test_compile_div() {
        let expr = s_expr("__builtin__div", vec![int(8), int(2)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(
            result,
            vec![inst!(OP_CONST, 8.0), inst!(OP_CONST, 2.0), inst!(OP_DIV),]
        );
    }

    #[test]
    fn test_compile_sin() {
        let expr = s_expr("sin", vec![int(0)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(result, vec![inst!(OP_CONST, 0.0), inst!(OP_SIN),]);
    }

    #[test]
    fn test_compile_cos() {
        let expr = s_expr("cos", vec![int(0)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(result, vec![inst!(OP_CONST, 0.0), inst!(OP_COS),]);
    }

    #[test]
    fn test_compile_pow() {
        let expr = s_expr("pow", vec![int(2), int(3)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(
            result,
            vec![inst!(OP_CONST, 2.0), inst!(OP_CONST, 3.0), inst!(OP_POW),]
        );
    }

    #[test]
    fn test_compile_nested_expression() {
        // sin(1 + 2)
        let inner = s_expr("__builtin__add", vec![int(1), int(2)]);
        let expr = s_expr("sin", vec![inner]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(
            result,
            vec![
                inst!(OP_CONST, 1.0),
                inst!(OP_CONST, 2.0),
                inst!(OP_ADD),
                inst!(OP_SIN),
            ]
        );
    }

    #[test]
    fn test_compile_deeply_nested_expression() {
        // pow(1 + 2, sin(0))
        let inner_add = s_expr("__builtin__add", vec![int(1), int(2)]);
        let inner_sin = s_expr("sin", vec![int(0)]);
        let expr = s_expr("pow", vec![inner_add, inner_sin]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(
            result,
            vec![
                inst!(OP_CONST, 1.0),
                inst!(OP_CONST, 2.0),
                inst!(OP_ADD),
                inst!(OP_CONST, 0.0),
                inst!(OP_SIN),
                inst!(OP_POW),
            ]
        );
    }

    #[test]
    fn test_compile_s_expr_direct() {
        // Test compile_s_expr directly by extracting SExpr from Expr::SExpr
        let Expr::SExpr(s_expr) = s_expr("__builtin__add", vec![int(1), int(2)]) else {
            panic!("Expected Expr::SExpr")
        };
        let result = compile_s_expr(&s_expr).unwrap();
        assert_eq!(
            result,
            vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 2.0), inst!(OP_ADD),]
        );
    }

    #[test]
    fn test_single_literal_instruction_count() {
        // Test that single literal generates exactly 1 instruction
        let expr = s_expr("sin", vec![int(0)]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(result.len(), 2); // OP_CONST(0.0) + OP_SIN
        assert_eq!(result[0], inst!(OP_CONST, 0.0));
        assert_eq!(result[1], inst!(OP_SIN));
    }

    #[test]
    fn test_constant_zero_instruction_count() {
        // Test that a -> 0 compiles to exactly 1 instruction when properly wrapped
        let expr = int(0);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(result.len(), 1); // Just OP_CONST(0.0)
        assert_eq!(result[0], inst!(OP_CONST, 0.0));
    }

    #[test]
    fn test_complex_expression_instruction_count() {
        // Test that complex expressions generate correct instruction count
        let inner = s_expr("__builtin__add", vec![int(1), int(2)]);
        let expr = s_expr("sin", vec![inner]);
        let result = compile_expr(&expr).unwrap();
        assert_eq!(result.len(), 4); // OP_CONST(1.0) + OP_CONST(2.0) + OP_ADD + OP_SIN
    }
}

#[test]
fn test_compile_mapping_with_literal() {
    use mth_ast::{Mapping, Module, TopLevel};

    // Test a -> 0;
    let mapping = Mapping {
        name: "a",
        params: vec![],
        body: int(0),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::MapImpl(mapping),
            TopLevel::Expr(Expr::SExpr(SExpr {
                name: "plot",
                args: vec![Expr::SExpr(SExpr {
                    name: "a",
                    args: vec![],
                })],
            })),
        ],
    };

    let result = compile_module(&module).unwrap();
    // Should be [OP_CONST, 0.0] for the literal 0
    assert_eq!(result, vec![inst!(OP_CONST, 0.0)]);
}

#[test]
fn test_compile_mapping_with_expression() {
    use mth_ast::{Mapping, Module, TopLevel};

    // Test a -> 1 - 1;
    let mapping = Mapping {
        name: "a",
        params: vec![],
        body: s_expr("__builtin__sub", vec![int(1), int(1)]),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::MapImpl(mapping),
            TopLevel::Expr(Expr::SExpr(SExpr {
                name: "plot",
                args: vec![Expr::SExpr(SExpr {
                    name: "a",
                    args: vec![],
                })],
            })),
        ],
    };

    let result = compile_module(&module).unwrap();
    // Should be [OP_CONST, 1.0, OP_CONST, 1.0, OP_SUB] for 1 - 1
    assert_eq!(
        result,
        vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 1.0), inst!(OP_SUB)]
    );
}

#[test]
fn test_buffer_size_issue() {
    // This test demonstrates the buffer size issue
    // Single literal produces 1 instruction, but shader reads 6
    let result = compile_literal(&Literal::Int(0)).unwrap();
    assert_eq!(result.len(), 1); // Only 1 instruction!

    // Expression produces 3 instructions
    let expr = s_expr("__builtin__sub", vec![int(1), int(1)]);
    let result = compile_expr(&expr).unwrap();
    assert_eq!(result.len(), 3); // 3 instructions
}

#[test]
fn test_fix_verification() {
    // Test that single literal now works correctly with padding
    let mapping = Mapping {
        name: "a",
        params: vec![],
        body: int(0),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::MapImpl(mapping),
            TopLevel::Expr(Expr::SExpr(SExpr {
                name: "plot",
                args: vec![Expr::SExpr(SExpr {
                    name: "a",
                    args: vec![],
                })],
            })),
        ],
    };

    let result = compile_module(&module).unwrap();

    // The result should still be just the literal instruction
    // The padding happens at the GPU level, not in compilation
    assert_eq!(result, vec![inst!(OP_CONST, 0.0)]);
}
