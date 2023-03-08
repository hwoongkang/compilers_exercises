use super::{Tag, Token};
pub struct Num {
    value: u32,
}

impl Num {
    pub fn new(value: u32) -> Self {
        Num { value }
    }
}

impl Token for Num {
    fn get_tag(&self) -> Tag {
        Tag::NUM
    }
}
