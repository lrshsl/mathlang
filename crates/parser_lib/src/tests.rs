use crate::{choice, combinators::*, cursor::Cursor, helpers::*, parse, primitives::*};

#[test]
fn test_satisfy() {
    let src = Cursor::new("abc");
    println!("{}", src.ctx);
    let (src, ch) = satisfy(|c| c == 'a')(src).unwrap();
    assert_eq!(ch, 'a');
    println!("{}", src.ctx);
    println!("{}", src.remainder);
    assert_eq!(src.cur_char, Some('b'));

    let err = satisfy(|c| c == 'x')(src.clone()).unwrap_err();
    assert!(err.msg.contains("Predicate failed"));
    assert_eq!(src.cur_char, Some('b'));
}

#[test]
fn test_satisfy_eof() {
    let src = Cursor::new("");
    let err = satisfy(|c| true)(src).unwrap_err();
    assert!(err.msg.contains("Unexpected EOF"));
}

#[test]
fn test_chr_success_and_fail() {
    let src = Cursor::new("abc");
    let (src, ch) = chr('a')(src).unwrap();
    assert_eq!(ch, 'a');
    assert_eq!(src.cur_char, Some('b'));

    let err = chr('x')(src.clone()).unwrap_err();
    assert!(err.msg.contains("Expected 'x'"));
}

#[test]
fn test_chr_eof() {
    let src = Cursor::new("");
    let err = chr('a')(src).unwrap_err();
    assert!(err.msg.contains("Unexpected EOF"));
}

#[test]
fn test_chr_wrong_char() {
    let src = Cursor::new("b");
    let err = chr('a')(src).unwrap_err();
    assert!(err.msg.contains("Expected 'a'"));
    assert!(err.msg.contains("'b'"));
}

#[test]
fn test_digit_and_some() {
    let src = Cursor::new("123xyz");
    let (src, digits) = some(digit(10))(src).unwrap();
    assert_eq!(String::from_iter(digits), "123");
    assert_eq!(src.cur_char, Some('x'));
}

#[test]
fn test_whitespace_and_tok() {
    let src = Cursor::new("   a");
    let (src, ch) = tok(chr('a'))(src).unwrap();
    assert_eq!(ch, 'a');
    assert_eq!(src.cur_char, None);
}

#[test]
fn test_optional_success_and_none() {
    let src = Cursor::new("x");
    let (src, maybe) = optional(chr('x'))(src).unwrap();
    assert_eq!(maybe, Some('x'));
    assert_eq!(src.cur_char, None);

    let src = Cursor::new("y");
    let (src, maybe) = optional(chr('x'))(src).unwrap();
    assert_eq!(maybe, None);
    assert_eq!(src.cur_char, Some('y'));
}

#[test]
fn test_choice_macro() {
    let src = Cursor::new("a");
    let p = choice!(chr('x'), chr('a'), chr('z'));
    let (_, ch) = p(src).unwrap();
    assert_eq!(ch, 'a');
}

#[test]
fn test_choice_macro_first_match_wins() {
    let src = Cursor::new("z");
    let p = choice!(chr('x'), chr('a'), chr('z'));
    let (_, ch) = p(src).unwrap();
    assert_eq!(ch, 'z');
}

#[test]
fn test_choice_macro_all_fail() {
    let src = Cursor::new("b");
    let p = choice!(chr('x'), chr('a'), chr('z'));
    let err = p(src).unwrap_err();
    assert!(!err.msg.is_empty());
}

#[test]
fn test_many0_and_some() {
    let src = Cursor::new("aaa!");
    let (src, out) = many0(chr('a'))(src).unwrap();
    assert_eq!(out, vec!['a', 'a', 'a']);
    assert_eq!(src.cur_char, Some('!'));

    let src = Cursor::new("!");
    let (src, out) = many0(chr('a'))(src).unwrap();
    assert!(out.is_empty());
    assert_eq!(src.cur_char, Some('!'));

    let src = Cursor::new("bb");
    let (src, out) = some(chr('b'))(src).unwrap();
    assert_eq!(out, vec!['b', 'b']);
    assert_eq!(src.cur_char, None);
}

#[test]
fn test_some_empty_fails() {
    let src = Cursor::new("!");
    let err = some(chr('b'))(src).unwrap_err();
    assert!(err.msg.contains("at least one"));
}

#[test]
fn test_ident() {
    let src = Cursor::new("hello123 world");
    let (src, name) = ident(src).unwrap();
    assert_eq!(name, "hello123");
    assert_eq!(src.cur_char, Some(' '));
}

#[test]
fn test_ident_starts_with_digit_fails() {
    let src = Cursor::new("123abc");
    let err = ident(src).unwrap_err();
    assert!(err.msg.contains("alphabetic"));
}

#[test]
fn test_between_success() {
    let src = Cursor::new("(42)");
    let p = between(
        pmap(
            |s: &str| s.parse::<i32>().unwrap(),
            chr_take_while(|c| c.is_ascii_digit()),
        ),
        chr('('),
        chr(')'),
    );
    let (src, val) = p(src).unwrap();
    assert_eq!(val, 42);
    assert_eq!(src.cur_char, None);
}

#[test]
fn test_ident_with_underscore() {
    let src = Cursor::new("hello_world");
    let (src, name) = ident(src).unwrap();
    assert_eq!(name, "hello_world");
}

#[test]
fn test_pmap() {
    let src = Cursor::new("a");
    let p = pmap(|c| c.to_ascii_uppercase(), chr('a'));
    let (_, v) = p(src).unwrap();
    assert_eq!(v, 'A');
}

#[test]
fn test_okparser_and_parse_macro() {
    let src = Cursor::new("");
    let parser = okparser(123);
    let (_, v) = parse!(parser, "Failed to run okparser", src).unwrap();
    assert_eq!(v, 123);
}

#[test]
fn test_okparser_does_not_consume() {
    let src = Cursor::new("abc");
    let parser = okparser(42);
    let (src2, v) = parser(src.clone()).unwrap();
    assert_eq!(v, 42);
    assert_eq!(src2.remainder, "abc");
}

#[test]
fn test_parse_macro_error_wrapping() {
    let src = Cursor::new("x");
    let err = parse!(chr('y'), "Expected y", src).unwrap_err();
    assert!(err.msg.contains("Expected y"));
}

#[test]
fn test_digit_failure() {
    let src = Cursor::new("abc");
    let err = digit(10)(src).unwrap_err();
    assert!(err.msg.contains("Predicate failed"));
}

#[test]
fn test_keyword() {
    let src = Cursor::new("hello world");
    let (src, kw) = keyword("hello")(src).unwrap();
    assert_eq!(kw, "hello");
    assert_eq!(src.cur_char, Some(' '));
}

#[test]
fn test_keyword_case_sensitive() {
    let src = Cursor::new("Hello");
    let err = keyword("hello")(src).unwrap_err();
    assert!(err.msg.contains("Expected 'hello'"));
}

#[test]
fn test_keyword_partial_match_fails() {
    let src = Cursor::new("hell");
    let err = keyword("hello")(src).unwrap_err();
    assert!(err.msg.contains("EOF") || err.msg.contains("Expected"));
}

#[test]
fn test_keyword_too_short() {
    let src = Cursor::new("x");
    let err = keyword("hello")(src).unwrap_err();
    assert!(err.msg.contains("EOF"));
}

mod combinator_tests {
    use super::*;

    #[test]
    fn test_or_first_succeeds() {
        let src = Cursor::new("a");
        let p = or(chr('a'), chr('b'));
        let (src, ch) = p(src).unwrap();
        assert_eq!(ch, 'a');
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_or_second_succeeds() {
        let src = Cursor::new("b");
        let p = or(chr('a'), chr('b'));
        let (src, ch) = p(src).unwrap();
        assert_eq!(ch, 'b');
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_or_both_fail() {
        let src = Cursor::new("c");
        let p = or(chr('a'), chr('b'));
        let err = p(src).unwrap_err();
        assert!(!err.msg.is_empty());
    }

    #[test]
    fn test_preceded_success() {
        let src = Cursor::new("(x)");
        let p = preceded(chr('('), chr('x'));
        let (src, ch) = p(src).unwrap();
        assert_eq!(ch, 'x');
        assert_eq!(src.cur_char, Some(')'));
    }

    #[test]
    fn test_preceded_prefix_missing() {
        let src = Cursor::new("x)");
        let p = preceded(chr('('), chr('x'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("Expected '('"));
    }

    #[test]
    fn test_preceded_parser_fails() {
        let src = Cursor::new("(y)");
        let p = preceded(chr('('), chr('x'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("Expected 'x'"));
    }

    #[test]
    fn test_terminated_success() {
        let src = Cursor::new("x)");
        let p = terminated(chr('x'), chr(')'));
        let (src, ch) = p(src).unwrap();
        assert_eq!(ch, 'x');
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_terminated_suffix_missing() {
        let src = Cursor::new("x");
        let p = terminated(chr('x'), chr(')'));
        let err = p(src).unwrap_err();
        // The error should not be empty (something failed)
        assert!(!err.msg.is_empty());
    }

    #[test]
    fn test_between_success() {
        let src = Cursor::new("(42)");
        let p = between(
            pmap(
                |s: &str| s.parse::<i32>().unwrap(),
                chr_take_while(|c| c.is_ascii_digit()),
            ),
            chr('('),
            chr(')'),
        );
        let (src, val) = p(src).unwrap();
        assert_eq!(val, 42);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_between_open_paren_missing() {
        let src = Cursor::new("42)");
        let p = between(chr('4'), chr('('), chr(')'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("Expected '('"));
    }

    #[test]
    fn test_between_close_paren_missing() {
        let src = Cursor::new("(42");
        let p = between(chr('4'), chr('('), chr(')'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("Expected ')'"));
    }

    #[test]
    fn test_between_with_complex_inner() {
        let src = Cursor::new("(a,b,c)");
        let inner = delimited1(ident, tok(chr(',')));
        let p = between(inner, tok(chr('(')), tok(chr(')')));
        let (src, ids) = p(src).unwrap();
        assert_eq!(ids, vec!["a", "b", "c"]);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_between_fails_on_non_matching_content() {
        let src = Cursor::new("(xyz)"); // content is 'x', not '4'
        let p = between(chr('4'), chr('('), chr(')'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("Expected '4'"));
    }

    #[test]
    fn test_pair_success() {
        let src = Cursor::new("ab");
        let p = pair(chr('a'), chr('b'));
        let (src, (a, b)) = p(src).unwrap();
        assert_eq!(a, 'a');
        assert_eq!(b, 'b');
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_pair_first_fails() {
        let src = Cursor::new("xb");
        let p = pair(chr('a'), chr('b'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("Expected 'a'"));
    }

    #[test]
    fn test_pair_second_fails() {
        let src = Cursor::new("ax");
        let p = pair(chr('a'), chr('b'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("Expected 'b'"));
    }

    #[test]
    fn test_and_then() {
        let src = Cursor::new("ab");
        let p = and_then(chr('a'), chr('b'));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a', 'b']);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_then_append() {
        let src = Cursor::new("ab");
        let p = then_append(many0(chr('a')), chr('b'));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a', 'b']);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_then_append_maybe_second_present() {
        let src = Cursor::new("ab");
        let p = then_append_maybe(many0(chr('a')), chr('b'));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a', 'b']);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_then_append_maybe_second_absent() {
        let src = Cursor::new("a");
        let p = then_append_maybe(many0(chr('a')), chr('b'));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a']);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_delimited0_empty() {
        let src = Cursor::new("");
        let p = delimited0(chr('a'), chr(','));
        let (src, results) = p(src).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_delimited0_single() {
        let src = Cursor::new("a");
        let p = delimited0(chr('a'), chr(','));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a']);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_delimited0_multiple() {
        let src = Cursor::new("a,b,c");
        let p = delimited0(tok(ident), tok(chr(',')));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!["a", "b", "c"]);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_delimited0_with_remainder() {
        let src = Cursor::new("a,b;c");
        let p = delimited0(tok(ident), tok(chr(',')));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!["a", "b"]);
        assert_eq!(src.cur_char, Some(';'));
    }

    #[test]
    fn test_delimited1_single() {
        let src = Cursor::new("a");
        let p = delimited1(tok(ident), tok(chr(',')));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!["a"]);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_delimited1_multiple() {
        let src = Cursor::new("a,b,c");
        let p = delimited1(tok(ident), tok(chr(',')));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!["a", "b", "c"]);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_delimited1_empty_fails() {
        let src = Cursor::new(";x");
        let p = delimited1(tok(ident), tok(chr(',')));
        let err = p(src).unwrap_err();
        // delimited1 uses some() which fails when no match
        assert!(!err.msg.is_empty());
    }

    #[test]
    fn test_delimited1_with_remainder() {
        let src = Cursor::new("a,b;c");
        let p = delimited1(tok(ident), tok(chr(',')));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!["a", "b"]);
        assert_eq!(src.cur_char, Some(';'));
    }

    #[test]
    fn test_many0_zero_matches() {
        let src = Cursor::new("xyz");
        let p = many0(chr('a'));
        let (src, results) = p(src).unwrap();
        assert!(results.is_empty());
        assert_eq!(src.cur_char, Some('x'));
    }

    #[test]
    fn test_many0_multiple_matches() {
        let src = Cursor::new("aaax");
        let p = many0(chr('a'));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a', 'a', 'a']);
        assert_eq!(src.cur_char, Some('x'));
    }

    #[test]
    fn test_many0_empty_input() {
        let src = Cursor::new("");
        let p = many0(chr('a'));
        let (src, results) = p(src).unwrap();
        assert!(results.is_empty());
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_some_one_match() {
        let src = Cursor::new("ax");
        let p = some(chr('a'));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a']);
        assert_eq!(src.cur_char, Some('x'));
    }

    #[test]
    fn test_some_multiple_matches() {
        let src = Cursor::new("aaax");
        let p = some(chr('a'));
        let (src, results) = p(src).unwrap();
        assert_eq!(results, vec!['a', 'a', 'a']);
        assert_eq!(src.cur_char, Some('x'));
    }

    #[test]
    fn test_some_no_match_fails() {
        let src = Cursor::new("x");
        let p = some(chr('a'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("at least one"));
    }

    #[test]
    fn test_some_empty_fails() {
        let src = Cursor::new("");
        let p = some(chr('a'));
        let err = p(src).unwrap_err();
        assert!(err.msg.contains("at least one"));
    }

    #[test]
    fn test_optional_some() {
        let src = Cursor::new("abc");
        let p = optional(chr('a'));
        let (src, result) = p(src).unwrap();
        assert_eq!(result, Some('a'));
        assert_eq!(src.cur_char, Some('b'));
    }

    #[test]
    fn test_optional_none() {
        let src = Cursor::new("bc");
        let p = optional(chr('a'));
        let (src, result) = p(src).unwrap();
        assert_eq!(result, None);
        assert_eq!(src.cur_char, Some('b'));
    }

    #[test]
    fn test_optional_empty() {
        let src = Cursor::new("");
        let p = optional(chr('a'));
        let (src, result) = p(src).unwrap();
        assert_eq!(result, None);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_pmap_transforms_value() {
        let src = Cursor::new("5");
        let p = pmap(|c: char| c.to_digit(10).unwrap() as i32, chr('5'));
        let (src, result) = p(src).unwrap();
        assert_eq!(result, 5);
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_pmap_with_complex_transform() {
        let src = Cursor::new("hello");
        let p = pmap(|s: &str| s.to_uppercase(), ident);
        let (src, result) = p(src).unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_choice_macro_single_parser() {
        let src = Cursor::new("a");
        let p = choice!(chr('a'));
        let (src, ch) = p(src).unwrap();
        assert_eq!(ch, 'a');
    }

    #[test]
    fn test_choice_macro_many_parsers() {
        let src = Cursor::new("e");
        let p = choice!(chr('a'), chr('b'), chr('c'), chr('d'), chr('e'));
        let (src, ch) = p(src).unwrap();
        assert_eq!(ch, 'e');
    }
}

mod whitespace_tests {
    use super::*;

    #[test]
    fn test_whitespace_none() {
        let src = Cursor::new("abc");
        let (src, _) = whitespace(src).unwrap();
        assert_eq!(src.cur_char, Some('a'));
    }

    #[test]
    fn test_whitespace_spaces() {
        let src = Cursor::new("   abc");
        let (src, _) = whitespace(src).unwrap();
        assert_eq!(src.cur_char, Some('a'));
    }

    #[test]
    fn test_whitespace_tabs() {
        let src = Cursor::new("\t\tabc");
        let (src, _) = whitespace(src).unwrap();
        assert_eq!(src.cur_char, Some('a'));
    }

    #[test]
    fn test_whitespace_newlines() {
        let src = Cursor::new("\n\n\nabc");
        let (src, _) = whitespace(src).unwrap();
        assert_eq!(src.cur_char, Some('a'));
    }

    #[test]
    fn test_whitespace_mixed() {
        let src = Cursor::new("  \t\n  abc");
        let (src, _) = whitespace(src).unwrap();
        assert_eq!(src.cur_char, Some('a'));
    }

    #[test]
    fn test_whitespace_empty() {
        let src = Cursor::new("");
        let (src, _) = whitespace(src).unwrap();
        assert_eq!(src.cur_char, None);
    }
}

mod tok_tests {
    use super::*;

    #[test]
    fn test_tok_consumes_whitespace() {
        let src = Cursor::new("   abc");
        let (src, ch) = tok(chr('a'))(src).unwrap();
        assert_eq!(ch, 'a');
        assert_eq!(src.cur_char, Some('b'));
    }

    #[test]
    fn test_tok_no_whitespace() {
        let src = Cursor::new("abc");
        let (src, ch) = tok(chr('a'))(src).unwrap();
        assert_eq!(ch, 'a');
        assert_eq!(src.cur_char, Some('b'));
    }

    #[test]
    fn test_tok_with_ident() {
        let src = Cursor::new("   hello");
        let (src, name) = tok(ident)(src).unwrap();
        assert_eq!(name, "hello");
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_tok_fails_without_match() {
        let src = Cursor::new("   xbc");
        let err = tok(chr('a'))(src).unwrap_err();
        assert!(err.msg.contains("Expected 'a'"));
    }
}

mod regression_tests {
    use super::*;

    #[test]
    fn test_between_does_not_match_without_open_paren() {
        // between(ident, '(', ')') should FAIL when there's no '('
        let src = Cursor::new("xyz+1");
        let p = between(ident, tok(chr('(')), tok(chr(')')));
        let err = p(src).unwrap_err();
        // Should fail because '(' is not present
        assert!(err.msg.contains("Expected '('"));
    }

    #[test]
    fn test_between_matches_with_parens() {
        let src = Cursor::new("(xyz)+1");
        let p = between(ident, tok(chr('(')), tok(chr(')')));
        let (src, name) = p(src).unwrap();
        assert_eq!(name, "xyz");
        // After closing ')', the remainder is '+1', so cur_char is '+'
        assert_eq!(src.cur_char, Some('+'));
    }

    #[test]
    fn test_delimited0_does_not_match_without_delimiter() {
        // delimited0(ident, ',') should succeed with empty vec when no comma
        let src = Cursor::new("abc");
        let p = delimited0(tok(ident), tok(chr(',')));
        let (_, names) = p(src).unwrap();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "abc");
    }

    #[test]
    fn test_delimited0_with_trailing_delimiter() {
        // "a," parses "a" and consumes ",", leaving empty input
        let src = Cursor::new("a,");
        let p = delimited0(tok(ident), tok(chr(',')));
        let (src, names) = p(src).unwrap();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "a");
        assert_eq!(src.cur_char, None);
    }

    #[test]
    fn test_choice_ordermatters() {
        // "abc" should match "ab" before 'c'
        let src = Cursor::new("abc");
        let p = choice!(keyword("abc"), keyword("ab"), keyword("a"));
        let (_, result) = p(src).unwrap();
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_some_stops_on_different_char() {
        let src = Cursor::new("aaabbb");
        let p = some(chr('a'));
        let (src, result) = p(src).unwrap();
        assert_eq!(result, vec!['a', 'a', 'a']);
        assert_eq!(src.cur_char, Some('b'));
    }
}
