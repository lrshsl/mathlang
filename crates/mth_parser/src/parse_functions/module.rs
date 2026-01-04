use super::*;

pub fn parse_module(src: &'_ str) -> Module<'_> {
    let mut src = Cursor::new(src);
    let mut exprs = Vec::new();

    loop {
        match parse_top_level(src) {
            Ok((new_src, tl)) => {
                src = new_src;
                exprs.push(tl);
            }
            Err(_) => break,
        }
    }

    Module {
        name: None,
        top_level: exprs,
    }
}
