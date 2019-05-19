use crate::token::Token;

pub struct Tokenizer<'a> {
    src: &'a str,
    index: usize,
    peeked: usize,
    ch: Option<char>,
    result: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            index: 0,
            peeked: 0,
            ch: None,
            result: vec![],
        }
    }

    pub fn lex(&mut self) -> Token {
        match self.peek() {
            // Ident or Keyword?
            Some('a'...'z') | Some('A'...'Z') => {
                let mut buffer = String::new();
                while let Some('a'...'z') | Some('A'...'Z') = self.peek() {
                    buffer.push(self.peek().unwrap());
                    self.read_char();
                }
                Token::Ident(buffer)
            }

            // Number
            Some('0'...'9') => {
                let mut buffer = String::new();
                while let Some('0'...'9') = self.peek() {
                    buffer.push(self.peek().unwrap());
                    self.read_char();
                }
                Token::Number(buffer.parse().unwrap())
            }

            // Only Symbol?
            Some(ch) => match ch {
                '+' | '-' | '*' | '/' | '(' | ')' | '.' | ';' => Token::Symbol(ch),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.index)
    }

    fn read_char(&mut self) {
        self.ch = self.src.chars().nth(self.index);
        self.index += 1;
    }
}

mod tests {
    use super::*;
    #[test]
    fn new() {
        let t = Tokenizer::new("hello");
        assert_eq!(t.src, "hello");
    }

    #[test]
    fn lex() {
        let mut t = Tokenizer::new("hello");
        assert_eq!(t.lex(), Token::Ident("hello".to_string()));

        let mut t = Tokenizer::new("Knium is godlike!");
        assert_eq!(t.lex(), Token::Ident("Knium".to_string()));

        let mut t = Tokenizer::new("42");
        assert_eq!(t.lex(), Token::Number(42));

        let mut t = Tokenizer::new("+");
        assert_eq!(t.lex(), Token::Symbol('+'));
    }
}
