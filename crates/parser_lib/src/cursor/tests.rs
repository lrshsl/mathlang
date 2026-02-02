use super::Cursor;

fn make_cursor<'s>(src: &'s str) -> Cursor<'s> {
    Cursor::new(src)
}

#[test]
fn cursor_basic_iteration() {
    let mut c = make_cursor("abc");

    assert_eq!(c.cur_char, Some('a'));
    assert_eq!(c.remainder, "abc");
    assert_eq!(c.ctx.line, 1);
    assert_eq!(c.ctx.col, 1);

    // consume first char
    assert_eq!(c.next(), Some('b'));
    assert_eq!(c.cur_char, Some('b'));
    assert_eq!(c.remainder, "bc");
    assert_eq!(c.ctx.line, 1);
    assert_eq!(c.ctx.col, 2);

    // consume second char
    assert_eq!(c.next(), Some('c'));
    assert_eq!(c.cur_char, Some('c'));
    assert_eq!(c.remainder, "c");
    assert_eq!(c.ctx.line, 1);
    assert_eq!(c.ctx.col, 3);

    // consume last char
    assert_eq!(c.next(), None);
    assert_eq!(c.cur_char, None);
    assert_eq!(c.remainder, "");
    assert_eq!(c.ctx.line, 1);
    assert_eq!(c.ctx.col, 4); // advanced past last char

    // end of input
    assert_eq!(c.next(), None);
    assert_eq!(c.cur_char, None);
    assert_eq!(c.remainder, "");
}

#[test]
fn cursor_newline_handling() {
    let mut c = make_cursor("ab\ncd");

    // consume a, b
    assert_eq!(c.cur_char, Some('a'));
    assert_eq!(c.next(), Some('b'));
    assert_eq!(c.ctx.line, 1);
    assert_eq!(c.ctx.col, 2);

    // consume newline
    c.next();
    assert_eq!(c.cur_char, Some('\n'));
    assert_eq!(c.ctx.line, 2);
    assert_eq!(c.ctx.col, 0); // reset on newline

    // consume c, d
    assert_eq!(c.next(), Some('c'));
    assert_eq!(c.next(), Some('d'));
    assert_eq!(c.ctx.line, 2);
    assert_eq!(c.ctx.col, 2);

    assert_eq!(c.next(), None);
    assert_eq!(c.ctx.line, 2);
    assert_eq!(c.ctx.col, 3);
}

#[test]
fn cursor_advance_function() {
    let mut c = make_cursor("rust");
    c.advance(2);
    assert_eq!(c.cur_char, Some('s'));
    assert_eq!(c.remainder, "st");
    assert_eq!(c.ctx.col, 3);
}

#[test]
fn cursor_empty_input() {
    let mut c = make_cursor("");
    assert_eq!(c.cur_char, None);
    assert_eq!(c.remainder, "");
    assert_eq!(c.ctx.line, 1);
    assert_eq!(c.ctx.col, 1);
    assert_eq!(c.next(), None);
    assert_eq!(c.ctx.line, 1);
    assert_eq!(c.ctx.col, 2);
}
