use super::*;

pub struct Module<'s> {
    pub name: Option<&'s str>,
    pub top_level: Vec<TopLevel<'s>>,
}
