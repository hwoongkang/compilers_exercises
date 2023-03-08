#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Tag(pub u16);

impl Tag {
    pub const NUM: Tag = Tag(256);
    pub const ID: Tag = Tag(257);
    pub const TRUE: Tag = Tag(258);
    pub const FALSE: Tag = Tag(259);
}
