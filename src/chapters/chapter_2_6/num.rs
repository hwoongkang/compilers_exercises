use super::{Tag, Token};
pub struct Num {
    value: u32,
}

impl Num {
    pub fn new(value: u32) -> Self {
        Num { value }
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Num {}", self.value)
    }
}

impl Token for Num {
    fn get_tag(&self) -> Tag {
        Tag::NUM
    }
}
