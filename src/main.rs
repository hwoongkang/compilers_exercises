mod chapters;
fn main() {
    let mut lexer = chapters::chapter_2_6::Lexer::new("a=1".to_string());
    lexer.scan().expect("Failed to scan");
}
