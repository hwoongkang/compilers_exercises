use std::collections::HashMap;

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
    fn scan(&mut self) -> Box<dyn Token> {
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
        match self.peek {
            _ if self.peek.is_digit(10) => {
                let mut v: u32 = 0;
                loop {
                    v = 10 * v + self.peek.to_digit(10).unwrap();
                    let ch = self.read();
                    match ch {
                        Some(ch) if ch.is_digit(10) => {
                            self.peek = ch;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                Box::new(Num::new(v))
            }
            _ if self.peek.is_alphabetic() => {
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
                    Box::new(w.clone())
                } else {
                    let w = Word::new(Tag::ID, b);
                    self.words.insert(w.lexeme.clone(), w.clone());
                    Box::new(w)
                }
            }
            _ => Box::new(DefaultToken::new(self.peek)),
        }
    }
}
