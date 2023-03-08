use super::Tag;

pub trait Token {
    fn get_tag(&self) -> Tag;
}

pub struct DefaultToken {
    tag: Tag,
}

impl DefaultToken {
    pub fn new(ch: char) -> Self {
        DefaultToken {
            tag: Tag(ch as u16),
        }
    }
}

impl Token for DefaultToken {
    fn get_tag(&self) -> Tag {
        self.tag
    }
}
