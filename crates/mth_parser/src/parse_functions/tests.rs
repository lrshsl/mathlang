use super::*;

#[test]
fn golden_test_module() {
    let src = r#"
    a -> 1;
    b -> 2;

    c :: int;
    c -> a;

    add x y -> (x + y);

    add a b;
    add (add a b) 1;
    "#;

    let expected = Module {
        name: None,
        top_level: Vec::from([
            TopLevel::MapImpl(Mapping {
                name: "a",
                params: vec![],
                body: int(1),
            }),
            TopLevel::MapImpl(Mapping {
                name: "b",
                params: vec![],
                body: int(2),
            }),
            TopLevel::TypeDecl(TypeDecl {
                name: "c",
                params: vec![Type::Int],
            }),
            TopLevel::MapImpl(Mapping {
                name: "c",
                params: vec![],
                body: varref("a"),
            }),
            TopLevel::MapImpl(Mapping {
                name: "add",
                params: vec![Param("x"), Param("y")],
                body: Expr::SExpr(SExpr {
                    name: "__builtin__add",
                    args: vec![varref("x"), varref("y")],
                }),
            }),
            TopLevel::Expr(Expr::SExpr(SExpr {
                name: "add",
                args: vec![varref("a"), varref("b")],
            })),
            TopLevel::Expr(Expr::SExpr(SExpr {
                name: "add",
                args: vec![
                    Expr::SExpr(SExpr {
                        name: "add",
                        args: vec![varref("a"), varref("b")],
                    }),
                    int(1),
                ],
            })),
        ]),
    };

    let src = Cursor::new(src);
    let parse_result = parse_module(src);

    let Ok((next, ast)) = parse_result else {
        eprintln!("ParseError: {parse_result:?}");
        panic!("Module test failed: Parsing failed");
    };

    for (expr, expected_expr) in ast.top_level.iter().zip(expected.top_level.iter()) {
        if expr != expected_expr {
            eprintln!("Mismatch: {expr:#?} != {expected_expr:#?}");
        }
    }
    if ast != expected {
        eprintln!("Mismatch, got: {ast:#?}\n\nExpected: {expected:#?}\n\nRemainder: {next:?}",);
        panic!("Module test failed: Ast wasn't what as expected");
    }

    assert_eq!(ast, expected);
    assert_eq!(next.remainder.trim(), "");
}

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
fn parse_literal_string() {
    assert_parses(literal, r#""hello""#, Literal::Str("hello".into()), "");
}

#[test]
fn parse_expr_application() {
    assert_parses(
        expr,
        "add x y",
        s_expr("add", vec![varref("x"), varref("y")]),
        "",
    );
}

#[test]
fn parse_mapping_no_params() {
    assert_parses(
        parse_mapping,
        "a -> 1",
        Mapping {
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
        parse_mapping,
        "add x y -> (x + y)",
        Mapping {
            name: "add",
            params: vec![Param("x".into()), Param("y".into())],
            body: s_expr("__builtin__add", vec![varref("x"), varref("y")]),
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
fn parse_top_level_expr() {
    assert_parses(
        parse_top_level,
        "x;",
        TopLevel::Expr(Expr::SExpr(SExpr {
            name: "x",
            args: vec![],
        })),
        "",
    );
}

#[test]
fn parse_module_simple() {
    let src = r#"
        a -> 1;
        b -> 2;
    "#;

    let (_, module) = parse_module(Cursor::new(src)).unwrap();

    assert_eq!(
        module,
        Module {
            name: None,
            top_level: vec![
                TopLevel::MapImpl(Mapping {
                    name: "a",
                    params: vec![],
                    body: int(1),
                }),
                TopLevel::MapImpl(Mapping {
                    name: "b",
                    params: vec![],
                    body: int(2),
                }),
            ],
        }
    );
}
