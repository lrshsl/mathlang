use crate::{
    choice, parse,
    parser::{
        cursor::Cursor,
        parser_lib::{combinators::*, helpers::*, primitives::*},
    },
};

fn make_cursor(s: &str) -> Cursor<'_> {
    Cursor::new(s)
}
#[test]
fn test_satisfy() {
    let src = make_cursor("abc");
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
fn test_chr_success_and_fail() {
    let src = make_cursor("abc");
    let (src, ch) = chr('a')(src).unwrap();
    assert_eq!(ch, 'a');
    assert_eq!(src.cur_char, Some('b'));

    let err = chr('x')(src.clone()).unwrap_err();
    assert!(err.msg.contains("Expected 'x'"));
}

#[test]
fn test_digit_and_some() {
    let src = make_cursor("123xyz");
    let (src, digits) = some(digit(10))(src).unwrap();
    assert_eq!(String::from_iter(digits), "123");
    assert_eq!(src.cur_char, Some('x'));
}

#[test]
fn test_whitespace_and_tok() {
    let src = make_cursor("   a");
    let (src, ch) = tok(chr('a'))(src).unwrap();
    assert_eq!(ch, 'a');
    assert_eq!(src.cur_char, None);
}

#[test]
fn test_optional_success_and_none() {
    let src = make_cursor("x");
    let (src, maybe) = optional(chr('x'))(src).unwrap();
    assert_eq!(maybe, Some('x'));
    assert_eq!(src.cur_char, None);

    let src = make_cursor("y");
    let (src, maybe) = optional(chr('x'))(src).unwrap();
    assert_eq!(maybe, None);
    assert_eq!(src.cur_char, Some('y'));
}

#[test]
fn test_choice_macro() {
    let src = make_cursor("a");
    let p = choice!(chr('x'), chr('a'), chr('z'));
    let (_, ch) = p(src).unwrap();
    assert_eq!(ch, 'a');
}

#[test]
fn test_many0_and_some() {
    let src = make_cursor("aaa!");
    let (src, out) = many0(chr('a'))(src).unwrap();
    assert_eq!(out, vec!['a', 'a', 'a']);
    assert_eq!(src.cur_char, Some('!'));

    let src = make_cursor("!");
    let (src, out) = many0(chr('a'))(src).unwrap();
    assert!(out.is_empty());
    assert_eq!(src.cur_char, Some('!'));

    let src = make_cursor("bb");
    let (src, out) = some(chr('b'))(src).unwrap();
    assert_eq!(out, vec!['b', 'b']);
    assert_eq!(src.cur_char, None);
}

#[test]
fn test_ident() {
    let src = make_cursor("hello123 world");
    let (src, name) = ident(src).unwrap();
    assert_eq!(name, "hello123");
    assert_eq!(src.cur_char, Some(' '));
}

#[test]
fn test_pmap() {
    let src = make_cursor("a");
    let p = pmap(chr('a'), |c| c.to_ascii_uppercase());
    let (_, v) = p(src).unwrap();
    assert_eq!(v, 'A');
}

#[test]
fn test_okparser_and_parse_macro() {
    let src = make_cursor("");
    let parser = okparser(123);
    let (_, v) = parse!(parser, "Failed to run okparser", src).unwrap();
    assert_eq!(v, 123);
}

#[test]
fn test_digit_failure() {
    let src = make_cursor("abc");
    let err = digit(10)(src).unwrap_err();
    assert!(err.msg.contains("Predicate failed"));
}
