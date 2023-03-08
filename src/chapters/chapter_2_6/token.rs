use super::Tag;

// to_string
pub trait Token: std::fmt::Display {
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

impl std::fmt::Display for DefaultToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DefaultToken {}", self.tag.0 as u8 as char)
    }
}

impl Token for DefaultToken {
    fn get_tag(&self) -> Tag {
        self.tag
    }
}
