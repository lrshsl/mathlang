use super::*;

#[test]
fn s_expr() {
    let s = r#"
    a -> 1;
    b -> 2;

    add :: Int -> Int -> Int
    add x y -> (x + y);

    add a b;
    add (add a b) 1;
    "#;

    let ast = parse_module(s);

    assert_eq!(
        ast,
        Module {
            name: None,
            top_level: Vec::from([
                TopLevel::MapImpl(Mapping {
                    name: "a",
                    params: vec![],
                    body: Expr::Literal(Literal::Int(1)),
                }),
                TopLevel::MapImpl(Mapping {
                    name: "b",
                    params: vec![],
                    body: Expr::Literal(Literal::Int(2)),
                }),
                TopLevel::TypeDecl(TypeDecl {
                    name: "add",
                    params: vec![Type::Int],
                }),
                TopLevel::MapImpl(Mapping {
                    name: "add",
                    params: vec![Param("x"), Param("y"),],
                    body: Expr::SExpr(SExpr {
                        name: "__builtin__add",
                        args: vec![Expr::Ref("x"), Expr::Ref("y")],
                    }),
                }),
                TopLevel::Expr(Expr::SExpr(SExpr {
                    name: "add",
                    args: vec![Expr::Ref("a"), Expr::Ref("b")],
                })),
                TopLevel::Expr(Expr::SExpr(SExpr {
                    name: "add",
                    args: vec![
                        Expr::SExpr(SExpr {
                            name: "add",
                            args: vec![Expr::Ref("a"), Expr::Ref("b")],
                        }),
                        Expr::Literal(Literal::Int(1)),
                    ],
                })),
            ])
        }
    )
}
