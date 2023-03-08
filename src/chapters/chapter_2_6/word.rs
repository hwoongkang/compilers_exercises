use super::*;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Word {
    pub lexeme: String,
    pub tag: Tag,
}

impl Word {
    pub fn new(tag: Tag, lexeme: String) -> Self {
        Word { tag, lexeme }
    }
}

impl Token for Word {
    fn get_tag(&self) -> Tag {
        self.tag
    }
}
