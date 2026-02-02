use mth_ast::{Expr, Literal, Mapping, Module, SExpr, TopLevel, int, s_expr};
use mth_common::{
    inst,
    ops::{OP_ADD, OP_CONST, OP_COS, OP_DIV, OP_MUL, OP_POW, OP_SIN, OP_SUB},
};

use crate::{
    codegen::{compile_expr, compile_s_expr},
    compile_module,
};

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

#[test]
fn test_compile_mapping_with_literal() {
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
