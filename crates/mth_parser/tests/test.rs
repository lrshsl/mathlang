use mth_ast::{Expr, Literal, function_call, int, varref};
use mth_parser::parse_functions::expr;
use parser_lib::cursor::Cursor;

fn assert_expr(input: &str, expected: Expr<'_>, expected_rem: &str) {
    let (next, parsed) = expr(Cursor::new(input)).expect("parse_expr failed");
    assert_eq!(parsed, expected, "remainder: {:?}", next.remainder);
    assert_eq!(next.remainder, expected_rem);
}

fn float(x: f64) -> Expr<'static> {
    Expr::Literal(Literal::Float(x))
}

fn boolean(b: bool) -> Expr<'static> {
    Expr::Literal(Literal::Bool(b))
}

mod literals {
    use super::*;

    #[test]
    fn parse_int() {
        assert_expr("42", int(42), "");
    }

    #[test]
    fn parse_int_with_remainder() {
        assert_expr(
            "42abc",
            function_call("*", vec![int(42), varref("abc")]),
            "",
        );
    }

    #[test]
    fn parse_negative_int() {
        assert_expr("-42", int(-42), "");
    }

    #[test]
    fn parse_float() {
        assert_expr("3.14", float(3.14), "");
    }

    #[test]
    fn parse_negative_float() {
        assert_expr("-3.14", float(-3.14), "");
    }

    #[test]
    fn parse_bool_true() {
        assert_expr("true", boolean(true), "");
    }

    #[test]
    fn parse_bool_false() {
        assert_expr("false", boolean(false), "");
    }
}

mod variables {
    use super::*;

    #[test]
    fn parse_variable() {
        assert_expr("x", varref("x"), "");
    }

    #[test]
    fn parse_variable_with_remainder() {
        assert_expr("xyz+1", function_call("+", vec![varref("xyz"), int(1)]), "");
    }
}

mod unary_operators {
    use super::*;

    #[test]
    fn parse_unary_neg_literal() {
        assert_expr("-5", int(-5), "");
    }

    #[test]
    fn parse_unary_neg_variable() {
        assert_expr("-x", -varref("x"), "");
    }

    #[test]
    fn parse_unary_neg_function_call() {
        assert_expr("-f()", -function_call("f", vec![]), "");
    }

    #[test]
    fn parse_unary_pos_literal() {
        assert_expr("+5", int(5), "");
    }

    #[test]
    fn parse_unary_pos_variable() {
        assert_expr("+x", varref("x"), "");
    }

    #[test]
    fn parse_double_negation() {
        assert_expr("--5", int(5), "");
    }

    #[test]
    fn parse_triple_negation() {
        assert_expr("---x", -varref("x"), "");
    }
}

mod binary_operators {
    use super::*;

    #[test]
    fn parse_addition() {
        assert_expr("1 + 2", function_call("+", vec![int(1), int(2)]), "");
    }

    #[test]
    fn parse_subtraction() {
        assert_expr("5 - 3", function_call("-", vec![int(5), int(3)]), "");
    }

    #[test]
    fn parse_multiplication() {
        assert_expr("2 * 3", function_call("*", vec![int(2), int(3)]), "");
    }

    #[test]
    fn parse_division() {
        assert_expr("6 / 2", function_call("/", vec![int(6), int(2)]), "");
    }

    #[test]
    fn parse_exponentiation() {
        assert_expr("2 ^ 3", function_call("^", vec![int(2), int(3)]), "");
    }

    #[test]
    fn parse_equality() {
        assert_expr("1 == 2", function_call("==", vec![int(1), int(2)]), "");
    }

    #[test]
    fn parse_inequality() {
        assert_expr("1 != 2", function_call("!=", vec![int(1), int(2)]), "");
    }

    #[test]
    fn parse_less_than() {
        assert_expr("1 < 2", function_call("<", vec![int(1), int(2)]), "");
    }

    #[test]
    fn parse_less_than_equal() {
        assert_expr("1 <= 2", function_call("<=", vec![int(1), int(2)]), "");
    }

    #[test]
    fn parse_greater_than() {
        assert_expr("2 > 1", function_call(">", vec![int(2), int(1)]), "");
    }

    #[test]
    fn parse_greater_than_equal() {
        assert_expr("2 >= 1", function_call(">=", vec![int(2), int(1)]), "");
    }

    #[test]
    fn parse_multiple_operators() {
        assert_expr(
            "1 + 2 + 3",
            function_call("+", vec![function_call("+", vec![int(1), int(2)]), int(3)]),
            "",
        );
    }
}

mod operator_precedence {
    use super::*;

    #[test]
    fn precedence_addition_vs_multiplication() {
        assert_expr(
            "1 + 2 * 3",
            function_call("+", vec![int(1), function_call("*", vec![int(2), int(3)])]),
            "",
        );
    }

    #[test]
    fn precedence_multiplication_vs_addition() {
        assert_expr(
            "1 * 2 + 3",
            function_call("+", vec![function_call("*", vec![int(1), int(2)]), int(3)]),
            "",
        );
    }

    #[test]
    fn precedence_exponentiation() {
        assert_expr(
            "2 ^ 3 ^ 2",
            function_call("^", vec![int(2), function_call("^", vec![int(3), int(2)])]),
            "",
        );
    }

    #[test]
    fn precedence_comparison_lowest() {
        assert_expr(
            "1 + 2 < 3 + 4",
            function_call(
                "<",
                vec![
                    function_call("+", vec![int(1), int(2)]),
                    function_call("+", vec![int(3), int(4)]),
                ],
            ),
            "",
        );
    }

    #[test]
    fn precedence_complex() {
        assert_expr(
            "1 + 2 * 3 - 4 / 2",
            function_call(
                "-",
                vec![
                    function_call("+", vec![int(1), function_call("*", vec![int(2), int(3)])]),
                    function_call("/", vec![int(4), int(2)]),
                ],
            ),
            "",
        );
    }
}

mod logical_operators {
    use super::*;

    #[test]
    fn parse_simple_and() {
        assert_expr(
            "1 > 2 and 4 < 2 * 10",
            function_call(
                "and",
                vec![
                    function_call(">", vec![int(1), int(2)]),
                    function_call("<", vec![int(4), function_call("*", vec![int(2), int(10)])]),
                ],
            ),
            "",
        );
    }
}

mod parenthesized_expressions {
    use super::*;

    #[test]
    fn parse_parenthesized_simple() {
        assert_expr("(1)", int(1), "");
    }

    #[test]
    fn parse_parenthesized_addition() {
        assert_expr("(1 + 2)", function_call("+", vec![int(1), int(2)]), "");
    }

    #[test]
    fn parse_parenthesized_precedence() {
        assert_expr(
            "(1 + 2) * 3",
            function_call("*", vec![function_call("+", vec![int(1), int(2)]), int(3)]),
            "",
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        assert_expr("((1 + 2))", function_call("+", vec![int(1), int(2)]), "");
    }
}

mod implicit_multiplication {
    use super::*;

    #[test]
    fn parse_implicit_mult_literal_variable() {
        assert_expr("2 x", function_call("*", vec![int(2), varref("x")]), "");
    }

    #[test]
    fn parse_implicit_mult_variable_literal() {
        assert_expr("x 5", function_call("*", vec![varref("x"), int(5)]), "");
    }

    #[test]
    fn parse_implicit_mult_variable_variable() {
        assert_expr(
            "x y",
            function_call("*", vec![varref("x"), varref("y")]),
            "",
        );
    }

    #[test]
    fn parse_implicit_mult_literal_literal() {
        assert_expr("2 3", function_call("*", vec![int(2), int(3)]), "");
    }

    #[test]
    fn parse_implicit_mult_with_parentheses() {
        assert_expr(
            "2 (3 + 4)",
            function_call("*", vec![int(2), function_call("+", vec![int(3), int(4)])]),
            "",
        );
    }
}

mod function_calls {
    use super::*;

    #[test]
    fn parse_fn_call_no_args() {
        assert_expr("foo()", function_call("foo", vec![]), "");
    }

    #[test]
    fn parse_fn_call_one_arg() {
        assert_expr("foo(x)", function_call("foo", vec![varref("x")]), "");
    }

    #[test]
    fn parse_fn_call_multiple_args() {
        assert_expr(
            "add(x, y)",
            function_call("add", vec![varref("x"), varref("y")]),
            "",
        );
    }

    #[test]
    fn parse_fn_call_with_expression_args() {
        assert_expr(
            "add(1, 2 + 3)",
            function_call(
                "add",
                vec![int(1), function_call("+", vec![int(2), int(3)])],
            ),
            "",
        );
    }
}

mod complex_expressions {
    use super::*;

    #[test]
    fn parse_chained_comparison() {
        assert_expr(
            "1 < 2 <= 3",
            function_call("<=", vec![function_call("<", vec![int(1), int(2)]), int(3)]),
            "",
        );
    }

    #[test]
    fn parse_mixed_operators() {
        assert_expr(
            "a + b * c - d / e",
            function_call(
                "-",
                vec![
                    function_call(
                        "+",
                        vec![
                            varref("a"),
                            function_call("*", vec![varref("b"), varref("c")]),
                        ],
                    ),
                    function_call("/", vec![varref("d"), varref("e")]),
                ],
            ),
            "",
        );
    }

    #[test]
    fn parse_unary_in_expression() {
        assert_expr(
            "-a + b",
            function_call("+", vec![-varref("a"), varref("b")]),
            "",
        );
    }

    #[test]
    fn parse_negated_expression_in_parens() {
        assert_expr(
            "-(a + b)",
            -function_call("+", vec![varref("a"), varref("b")]),
            "",
        );
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn parse_whitespace_handling() {
        assert_expr(
            "  1   +   2  ",
            function_call("+", vec![int(1), int(2)]),
            "",
        );
    }

    #[test]
    fn parse_expression_with_remainder() {
        assert_expr("1 + 2;", function_call("+", vec![int(1), int(2)]), ";");
    }

    #[test]
    fn parse_zero() {
        assert_expr("0", int(0), "");
    }

    #[test]
    fn parse_zero_negation() {
        assert_expr("-0", int(0), "");
    }
}

mod regression_tests {
    use super::*;
    use mth_parser::parse_functions::parse_fn_call;
    use parser_lib::cursor::Cursor;

    #[test]
    fn fn_call_requires_parentheses() {
        // parse_fn_call should FAIL when there's no opening paren
        let src = Cursor::new("xyz+1");
        let result = parse_fn_call(src);
        assert!(result.is_err(), "parse_fn_call should fail without '('");
    }

    #[test]
    fn fn_call_with_parentheses() {
        // parse_fn_call should succeed with parentheses
        let src = Cursor::new("foo(x, y)");
        let (next, fc) = parse_fn_call(src).unwrap();
        assert_eq!(fc.name, "foo");
        assert_eq!(fc.args.len(), 2);
        assert_eq!(next.remainder, "");
    }

    #[test]
    fn expr_with_varref_plus_number() {
        // "xyz+1" should parse as varref("xyz") then binary + with int(1)
        assert_expr("xyz+1", function_call("+", vec![varref("xyz"), int(1)]), "");
    }
}
