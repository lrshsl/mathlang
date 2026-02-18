use mth_parser::{parse_expr, expr};
use mth_ast::{int, varref};
use parser_lib::cursor::Cursor;

fn main() {
    // Test basic unary negation
    let test_cases = vec![
        ("5", int(5)),
        ("-5", -int(5)),
        ("x", varref("x")),
        ("-x", -varref("x")),
    ];
    
    for (input, expected) in test_cases {
        match expr(Cursor::new(input)) {
            Ok((remainder, result)) => {
                println!("✓ '{input}' -> {result:?}, remainder: '{}'", remainder.remainder);
                if result != expected {
                    println!("  ❌ Expected: {expected:?}");
                } else {
                    println!("  ✓ Expected: {expected:?}");
                }
            }
            Err(e) => {
                println!("❌ '{input}' failed: {e:?}");
            }
        }
    }
}
