use mth_ast::{function_call, int, Expr, Function, FunctionCall, Literal, Module, TopLevel};
use mth_common::{inst, ops::*, N_PLOTS, PLOT_TYPE_EQUATION, PLOT_TYPE_FN_GRAPH};

use crate::codegen::{compile_expr, compile_s_expr};

#[test]
fn test_compile_literal() {
    let expr = &Expr::Literal(Literal::Int(42));
    let mut buf = Vec::new();
    let result = compile_expr(expr, &mut buf).unwrap();
    assert_eq!(result, (1, PLOT_TYPE_FN_GRAPH));
    assert_eq!(buf, vec![inst!(OP_CONST, 42.0)]);
}

#[test]
fn test_compile_add() {
    let expr = function_call("+", vec![int(1), int(2)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (3, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
        vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 2.0), inst!(OP_ADD)]
    );
}

#[test]
fn test_compile_mul() {
    let expr = function_call("*", vec![int(3), int(4)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (3, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
        vec![inst!(OP_CONST, 3.0), inst!(OP_CONST, 4.0), inst!(OP_MUL)]
    );
}

#[test]
fn test_compile_sub() {
    let expr = function_call("-", vec![int(5), int(2)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (3, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
        vec![inst!(OP_CONST, 5.0), inst!(OP_CONST, 2.0), inst!(OP_SUB)]
    );
}

#[test]
fn test_compile_div() {
    let expr = function_call("/", vec![int(8), int(2)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (3, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
        vec![inst!(OP_CONST, 8.0), inst!(OP_CONST, 2.0), inst!(OP_DIV)]
    );
}

#[test]
fn test_compile_sin() {
    let expr = function_call("sin", vec![int(0)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (2, PLOT_TYPE_FN_GRAPH));
    assert_eq!(buf, vec![inst!(OP_CONST, 0.0), inst!(OP_SIN)]);
}

#[test]
fn test_compile_cos() {
    let expr = function_call("cos", vec![int(0)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (2, PLOT_TYPE_FN_GRAPH));
    assert_eq!(buf, vec![inst!(OP_CONST, 0.0), inst!(OP_COS)]);
}

#[test]
fn test_compile_pow() {
    let expr = function_call("^", vec![int(2), int(3)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (3, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
        vec![inst!(OP_CONST, 2.0), inst!(OP_CONST, 3.0), inst!(OP_POW)]
    );
}

#[test]
fn test_compile_nested_expression() {
    let inner = function_call("+", vec![int(1), int(2)]);
    let expr = function_call("sin", vec![inner]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (4, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
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
    let inner_add = function_call("+", vec![int(1), int(2)]);
    let inner_sin = function_call("sin", vec![int(0)]);
    let expr = function_call("^", vec![inner_add, inner_sin]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (6, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
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
fn test_compile_function_call_direct() {
    let Expr::FunctionCall(function_call) = function_call("+", vec![int(1), int(2)]) else {
        panic!("Expected Expr::FunctionCall")
    };
    let mut buf = Vec::new();
    let result = compile_s_expr(&function_call, &mut buf).unwrap();
    assert_eq!(result, (3, PLOT_TYPE_FN_GRAPH));
    assert_eq!(
        buf,
        vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 2.0), inst!(OP_ADD)]
    );
}

#[test]
fn test_single_literal_instruction_count() {
    let expr = function_call("sin", vec![int(0)]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (2, PLOT_TYPE_FN_GRAPH));
    assert_eq!(buf[0], inst!(OP_CONST, 0.0));
    assert_eq!(buf[1], inst!(OP_SIN));
}

#[test]
fn test_constant_zero_instruction_count() {
    let expr = int(0);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (1, PLOT_TYPE_FN_GRAPH));
    assert_eq!(buf[0], inst!(OP_CONST, 0.0));
}

#[test]
fn test_complex_expression_instruction_count() {
    let inner = function_call("+", vec![int(1), int(2)]);
    let expr = function_call("sin", vec![inner]);
    let mut buf = Vec::new();
    let result = compile_expr(&expr, &mut buf).unwrap();
    assert_eq!(result, (4, PLOT_TYPE_FN_GRAPH));
}

#[test]
fn test_compile_mapping_with_literal() {
    let mapping = Function {
        name: "a",
        params: vec![],
        body: int(0),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::Function(mapping),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "plot",
                args: vec![Expr::FunctionCall(FunctionCall {
                    name: "a",
                    args: vec![],
                    is_negated: false,
                })],
                is_negated: false,
            })),
        ],
    };

    let (instructions, plot_descs) = crate::compile_module(&module).unwrap();
    assert_eq!(instructions, vec![inst!(OP_CONST, 0.0)]);
    assert_eq!(plot_descs[0].length, 1);
    assert_eq!(plot_descs[0].type_id, PLOT_TYPE_FN_GRAPH);
}

#[test]
fn test_compile_mapping_with_expression() {
    let mapping = Function {
        name: "a",
        params: vec![],
        body: function_call("-", vec![int(1), int(1)]),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::Function(mapping),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "plot",
                args: vec![Expr::FunctionCall(FunctionCall {
                    name: "a",
                    args: vec![],
                    is_negated: false,
                })],
                is_negated: false,
            })),
        ],
    };

    let (instructions, plot_descs) = crate::compile_module(&module).unwrap();
    assert_eq!(
        instructions,
        vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 1.0), inst!(OP_SUB)]
    );
    assert_eq!(plot_descs[0].length, 3);
    assert_eq!(plot_descs[0].type_id, PLOT_TYPE_FN_GRAPH);
}

#[test]
fn test_compile_negated_plot() {
    let mapping = Function {
        name: "a",
        params: vec![],
        body: int(5),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::Function(mapping),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "plot",
                args: vec![Expr::FunctionCall(FunctionCall {
                    name: "a",
                    args: vec![],
                    is_negated: false,
                })],
                is_negated: true,
            })),
        ],
    };

    let (instructions, plot_descs) = crate::compile_module(&module).unwrap();
    assert_eq!(
        instructions,
        vec![inst!(OP_CONST, 5.0), inst!(OP_CONST, -1.0), inst!(OP_MUL)]
    );
    assert_eq!(plot_descs[0].length, 3);
    assert_eq!(plot_descs[0].type_id, PLOT_TYPE_FN_GRAPH);
}

#[test]
fn test_compile_equation() {
    let mapping = Function {
        name: "eq",
        params: vec![],
        body: function_call("<", vec![int(1), int(2)]),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::Function(mapping),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "plot",
                args: vec![Expr::FunctionCall(FunctionCall {
                    name: "eq",
                    args: vec![],
                    is_negated: false,
                })],
                is_negated: false,
            })),
        ],
    };

    let (instructions, plot_descs) = crate::compile_module(&module).unwrap();
    assert_eq!(
        instructions,
        vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 2.0), inst!(OP_LT)]
    );
    assert_eq!(plot_descs[0].length, 3);
    assert_eq!(plot_descs[0].type_id, PLOT_TYPE_EQUATION);
}

#[test]
fn test_compile_multiple_plots() {
    let mapping_a = Function {
        name: "a",
        params: vec![],
        body: int(1),
    };
    let mapping_b = Function {
        name: "b",
        params: vec![],
        body: int(2),
    };

    let module = Module {
        name: None,
        top_level: vec![
            TopLevel::Function(mapping_a),
            TopLevel::Function(mapping_b),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "plot",
                args: vec![Expr::FunctionCall(FunctionCall {
                    name: "a",
                    args: vec![],
                    is_negated: false,
                })],
                is_negated: false,
            })),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "plot",
                args: vec![Expr::FunctionCall(FunctionCall {
                    name: "b",
                    args: vec![],
                    is_negated: false,
                })],
                is_negated: false,
            })),
        ],
    };

    let (instructions, plot_descs) = crate::compile_module(&module).unwrap();
    assert_eq!(
        instructions,
        vec![inst!(OP_CONST, 1.0), inst!(OP_CONST, 2.0)]
    );
    assert_eq!(plot_descs[0].length, 1);
    assert_eq!(plot_descs[0].type_id, PLOT_TYPE_FN_GRAPH);
    assert_eq!(plot_descs[1].length, 1);
    assert_eq!(plot_descs[1].type_id, PLOT_TYPE_FN_GRAPH);
    for i in 2..N_PLOTS {
        assert_eq!(plot_descs[i].length, 0);
        assert_eq!(plot_descs[i].type_id, 0);
    }
}
