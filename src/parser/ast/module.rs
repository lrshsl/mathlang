use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'s> {
    pub name: Option<&'s str>,
    pub top_level: Vec<TopLevel<'s>>,
}
