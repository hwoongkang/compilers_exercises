use super::*;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Word {
    pub lexeme: String,
    pub tag: Tag,
}

impl Word {
    pub fn new(tag: Tag, lexeme: String) -> Self {
        Word { tag, lexeme }
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Word {}", self.lexeme)
    }
}

impl Token for Word {
    fn get_tag(&self) -> Tag {
        self.tag
    }
}
