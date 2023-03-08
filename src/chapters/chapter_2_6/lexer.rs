use std::collections::HashMap;

use super::errors::LexerError;
use super::*;

pub struct Lexer {
    pub line: u32,
    peek: char,
    words: HashMap<String, Word>,
    input: Vec<char>,
    input_index: usize,
}

impl Lexer {
    fn reserve(&mut self, word: Word) {
        self.words.insert(word.lexeme.clone(), word);
    }

    pub fn new(input: String) -> Self {
        let mut instance = Lexer {
            input: input.chars().collect(),
            input_index: 0,
            line: 1,
            peek: ' ',
            words: HashMap::new(),
        };
        instance.reserve(Word::new(Tag::TRUE, "true".to_string()));
        instance.reserve(Word::new(Tag::FALSE, "false".to_string()));
        instance
    }
    /*
    public Token scan() throws IOException {
        fore ; ; peek = (char)System.in.read() ) {
        if( peek == , , I I peek == '\t' ) continue; else if( peek == '\n' ) line = line + 1; else break;
        }
        if (Character . isDigit (peek) ) { int v = 0;
            do {
            v = 10*v + Character . digit (peek , 10) ;
            peek = (char)System.in.read();
            } while ( Character . isDigit (peek) ) ; return new Num(v) ;
            }
            if( Character.isLetter(peek) ) { StringBuffer b = new StringBuffer () ; do{
            b.append(peek) ;
            peek = (char)System.in.read();
            } while ( Character . isLetterOrDigit (peek) ) ; String s = b.toString();
            Word w = (Word)words.get(s);
            if( w != null ) return W;
            w = new Word(Tag.ID, s);
            words.put(s, w);
            return w;
            Token t = new Token (peek) ; peek = , ' ;
            return t;
            })
             */
    fn read(&mut self) -> Option<char> {
        let ch = self.input.get(self.input_index);
        self.input_index += 1;
        match ch {
            Some(ch) => {
                self.peek = *ch;
                Some(*ch)
            }
            None => None,
        }
    }

    pub fn scan(&mut self) -> Result<Box<dyn Token>, LexerError> {
        self.skip_whitespace();
        match self.peek {
            '/' => {
                self.skip_comment()?;
                self.scan()
            }
            _ if self.peek.is_digit(10) => Ok(Box::new(self.read_number()?)),
            _ if self.peek.is_alphabetic() => Ok(Box::new(self.read_word()?)),
            _ => Ok(Box::new(DefaultToken::new(self.peek))),
        }
    }
    fn skip_whitespace(&mut self) {
        loop {
            let ch = self.read();

            let Some(ch) = ch else {
                break;
            };
            self.peek = ch;
            match ch {
                ' ' | '\t' => {
                    continue;
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {
                    break;
                }
            }
        }
    }
    fn read_number(&mut self) -> Result<Num, LexerError> {
        let mut v: u32 = 0;
        loop {
            v = 10 * v + self.peek.to_digit(10).unwrap();
            let ch = self.read();
            match ch {
                Some(ch) if ch.is_digit(10) => {
                    self.peek = ch;
                }
                Some(ch) if ch.is_whitespace() => {
                    break;
                }
                None => {
                    break;
                }

                _ => return Err(LexerError::UnexpectedEOF),
            }
        }
        Ok(Num::new(v))
    }
    fn read_word(&mut self) -> Result<Word, LexerError> {
        let mut b = String::new();
        loop {
            b.push(self.peek);
            let ch = self.read();
            match ch {
                Some(ch) if ch.is_alphanumeric() => {
                    self.peek = ch;
                }
                _ => {
                    break;
                }
            }
        }
        let w = self.words.get(&b);
        if let Some(w) = w {
            Ok(w.clone())
        } else {
            let w = Word::new(Tag::ID, b);
            self.words.insert(w.lexeme.clone(), w.clone());
            Ok(w)
        }
    }
    fn skip_comment(&mut self) -> Result<(), LexerError> {
        let ch = self.read();
        match ch {
            Some('/') => loop {
                let ch = self.read();
                match ch {
                    Some('\n') => {
                        self.line += 1;
                        break Ok(());
                    }
                    None => break (Err(LexerError::UnexpectedEOF)),
                    _ => continue,
                }
            },
            Some('*') => loop {
                let ch = self.read();
                match ch {
                    Some('\n') => {
                        self.line += 1;
                    }
                    Some('*') => {
                        let ch = self.read();
                        match ch {
                            Some('/') => break Ok(()),
                            None => break Err(LexerError::UnexpectedEOF),
                            _ => continue,
                        }
                    }
                    None => {
                        break Err(LexerError::UnexpectedEOF);
                    }
                    _ => {
                        continue;
                    }
                }
            },
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_empty_lexer() {
        let lexer = Lexer::new("a = 1".to_string());
        assert_eq!(
            lexer.words,
            HashMap::from_iter([
                ("true".to_string(), Word::new(Tag::TRUE, "true".to_string())),
                (
                    "false".to_string(),
                    Word::new(Tag::FALSE, "false".to_string())
                )
            ])
        );
    }
    #[test]
    fn test_first_scan() {
        let mut lexer = Lexer::new("a = 1".to_string());
        let token = lexer.scan();
        assert!(token.is_ok());
        assert_eq!(
            token.unwrap().to_string(),
            Word::new(Tag::ID, "a".to_string()).to_string()
        );
    }
    #[test]
    fn test_white_space() {
        let mut lexer = Lexer::new("\t \n a = 1".to_string());
        let token = lexer.scan();
        assert!(token.is_ok());
        assert_eq!(
            token.unwrap().to_string(),
            Word::new(Tag::ID, "a".to_string()).to_string()
        );
    }
    #[test]
    fn test_single_line_comment() {
        let mut lexer = Lexer::new("//this should be passed\na = 1".to_string());
        let token = lexer.scan();
        assert!(token.is_ok());
        assert_eq!(
            token.unwrap().to_string(),
            Word::new(Tag::ID, "a".to_string()).to_string()
        );
    }
    #[test]
    fn test_single_line_comment_fail() {
        let mut lexer = Lexer::new("//this should not be passed".to_string());
        let token = lexer.scan();
        assert!(token.is_err());
        assert_eq!(token.err(), Some(LexerError::UnexpectedEOF),);
    }
    #[test]
    fn test_multiline_comment() {
        let mut lexer = Lexer::new("/* this should be passed */a=1".to_string());
        let token = lexer.scan();
        assert!(token.is_ok());
        assert_eq!(
            token.unwrap().to_string(),
            Word::new(Tag::ID, "a".to_string()).to_string()
        );
    }
    #[test]
    fn test_multiline_comment_asterisk() {
        let mut lexer = Lexer::new("/* this should be **** passed */a=1".to_string());
        let token = lexer.scan();
        assert!(token.is_ok());
        assert_eq!(
            token.unwrap().to_string(),
            Word::new(Tag::ID, "a".to_string()).to_string()
        );
    }
    #[test]
    fn test_multiline_comment_EOF() {
        let mut lexer = Lexer::new("/* this should not **** pass".to_string());
        let token = lexer.scan();
        assert!(token.is_err());
        assert_eq!(token.err(), Some(LexerError::UnexpectedEOF),);
    }
}
