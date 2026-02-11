use super::*;

pub fn parse_module(src: Cursor) -> PResult<Module> {
    let mut src = src;
    let mut exprs = Vec::new();

    loop {
        match parse_top_level(src.clone()) {
            Ok((new_src, tl)) => {
                src = new_src;
                exprs.push(tl);
            }
            Err(_) => break,
        }
    }

    Ok((
        src,
        Module {
            name: None,
            top_level: exprs,
        },
    ))
}

#[test]
fn golden_test_module() {
    let src = r#"
    a = 1;
    b = 2;

    c :: int;
    c = a;

    add :: int -> int -> int;
    add(x, y) = (x + y);

    add(a, b);
    add(add(a, b), 1);
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
            TopLevel::TypeDecl(TypeDecl {
                name: "add",
                params: vec![Type::Int, Type::Int, Type::Int],
            }),
            TopLevel::MapImpl(Mapping {
                name: "add",
                params: vec![Param("x"), Param("y")],
                body: Expr::FunctionCall(FunctionCall {
                    name: "+",
                    args: vec![varref("x"), varref("y")],
                }),
            }),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "add",
                args: vec![varref("a"), varref("b")],
            })),
            TopLevel::Expr(Expr::FunctionCall(FunctionCall {
                name: "add",
                args: vec![
                    Expr::FunctionCall(FunctionCall {
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
