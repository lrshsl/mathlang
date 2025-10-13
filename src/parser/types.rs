use std::collections::HashMap;

use crate::parser::cursor::Cursor;

type Params = Vec<String>;

#[derive(Debug, Clone)]
pub struct Context {
    functions: HashMap<String, Vec<Params>>,
}

#[derive(Debug, Clone)]
pub struct FileContext {
    pub filename: Option<String>,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for FileContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FileContext {
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

impl Default for FileContext {
    fn default() -> Self {
        Self {
            filename: None,
            line: 1,
            col: 1,
        }
    }
}

#[derive(Debug)]
pub struct PError {
    pub msg: String,
    pub ctx: FileContext,
}

impl std::fmt::Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let PError { msg, ctx } = self;
        write!(f, "Error: {msg}\n   {ctx}")
    }
}

pub type PResult<'s, O> = Result<(Cursor<'s>, O), PError>;

pub type BoxedParser<'s, T> = Box<dyn Fn(crate::parser::cursor::Cursor<'s>) -> PResult<'s, T> + 's>;

#[macro_export]
macro_rules! Parser {
    ($lt:lifetime, $out:ty) => {
        impl Fn(crate::parser::cursor::Cursor<$lt>) -> crate::parser::types::PResult<'_, $out>
    };
}
