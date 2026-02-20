use super::*;

/// Helper to assert parser output and remaining input
fn assert_parses<'s, T: std::fmt::Debug + PartialEq>(
    parser: impl Fn(Cursor<'s>) -> PResult<'s, T>,
    input: &'s str,
    expected_val: T,
    expected_rem: &'s str,
) {
    let (next, val) = parser(Cursor::new(input)).expect("parse failed");
    assert_eq!(val, expected_val, "remainder: {}", next.remainder);
    assert_eq!(next.remainder, expected_rem);
}

#[test]
fn parse_literal_int() {
    assert_parses(literal, "42", Literal::Int(42), "");
}

#[test]
fn parse_literal_partial() {
    assert_parses(literal, "42abc", Literal::Int(42), "abc");
}

#[test]
fn parse_expr_application() {
    assert_parses(
        expr,
        "add(x, y)",
        function_call("add", vec![varref("x"), varref("y")]),
        "",
    );
}

#[test]
fn parse_mapping_no_params() {
    assert_parses(
        parse_fn_decl,
        "a = 1",
        Function {
            name: "a",
            params: vec![],
            body: int(1),
        },
        "",
    );
}

#[test]
fn parse_mapping_with_params() {
    assert_parses(
        parse_fn_decl,
        "add(x, y) = (x + y)",
        Function {
            name: "add",
            params: vec![Param("x".into()), Param("y".into())],
            body: function_call("+", vec![varref("x"), varref("y")]),
        },
        "",
    );
}

#[test]
fn parse_type_decl_simple() {
    assert_parses(
        parse_type_decl,
        "x :: int",
        TypeDecl {
            name: "x",
            params: vec![Type::Int],
        },
        "",
    );
}

#[test]
fn parse_type_decl_fn() {
    assert_parses(
        parse_type_decl,
        "eq :: int -> int -> bool",
        TypeDecl {
            name: "eq",
            params: vec![Type::Int, Type::Int, Type::Bool],
        },
        "",
    );
}

#[test]
fn parse_top_level_expr() {
    assert_parses(parse_top_level, "x();", TopLevel::Expr(varref("x")), "");
}

#[test]
fn parse_module_simple() {
    let src = r#"
        a = 1;
        b = 2;
    "#;

    let (_, module) = parse_module(Cursor::new(src)).unwrap();

    assert_eq!(
        module,
        Module {
            name: None,
            top_level: vec![
                TopLevel::Function(Function {
                    name: "a",
                    params: vec![],
                    body: int(1),
                }),
                TopLevel::Function(Function {
                    name: "b",
                    params: vec![],
                    body: int(2),
                }),
            ],
        }
    );
}
