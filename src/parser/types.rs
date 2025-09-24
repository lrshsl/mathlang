use crate::parser::cursor::Cursor;

#[derive(Debug, Clone)]
pub struct Context {
    pub filename: Option<String>,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Context {
            filename,
            line,
            col,
        } = self;
        if let Some(fname) = filename {
            write!(f, "{fname}:{line}:{col}")
        } else {
            write!(f, "{line}:{col}")
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            filename: None,
            line: 1,
            col: 0,
        }
    }
}

#[derive(Debug)]
pub struct PError {
    pub msg: String,
    pub ctx: Context,
}

impl std::fmt::Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let PError { msg, ctx } = self;
        write!(f, "Error: {msg}\n   {ctx}")
    }
}

pub type PResult<'s, O> = Result<(Cursor<'s>, O), PError>;

#[macro_export]
macro_rules! Parser {
    ($lt:lifetime, $out:ty) => {
        impl FnOnce(crate::parser::cursor::Cursor<$lt>) -> PResult<$out>
    };
}
