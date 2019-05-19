use self::AST::*;
use crate::ast::AST;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::collections::VecDeque;

pub struct Parser {
    index: usize,
    tokens: VecDeque<Token>,
}

macro_rules! binop {
    ($left: expr, $op: expr, $right: expr) => {
        BinOP(Box::new($left), $op, Box::new($right))
    };
}

macro_rules! expect {
    ($self: ident, $variant: ident, $value: expr) => {
        let t = $self.get();
        if let Token::$variant($value) = t {
            ()
        } else {
            panic!("Unexpected token! {:?}", t)
        }
    };
}

macro_rules! get {
    ($self: ident, $variant: ident) => {
        if let Token::$variant(value) = $self.get() {
            value
        } else {
            unreachable!()
        }
    };
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        Self { index: 0, tokens }
    }

    fn term(&mut self) -> AST {
        let t = self.get();
        match t {
            Token::Number(n) => Number(n),
            _ => {
                dbg!(t);
                dbg!(&self.tokens);
                panic!();
            }
        }
    }

    // TODO: def_parse_binop!(mul,'*', '/', term)
    fn mul(&mut self) -> AST {
        let mut left = self.term();
        loop {
            let peeked = self.peek();
            if peeked != Some(&Token::Symbol('*')) && peeked != Some(&Token::Symbol('/')) {
                break;
            }
            let op = get!(self, Symbol);
            let right = self.term();
            left = binop!(left, op, right);
        }
        left
    }

    // TODO: def_parse_binop!(add,'*', '/', mul)
    fn add(&mut self) -> AST {
        let mut left = self.mul();
        loop {
            let peeked = self.peek();
            if peeked != Some(&Token::Symbol('+')) && peeked != Some(&Token::Symbol('-')) {
                break;
            }
            let op = get!(self, Symbol);
            let right = self.mul();
            left = binop!(left, op, right);
        }
        left
    }

    fn expr(&mut self) -> AST {
        self.add()
    }

    fn method_call(&mut self) -> AST {
        let table = get!(self, Ident);
        expect!(self, Symbol, '.');
        let name = get!(self, Ident);
        expect!(self, Symbol, '(');
        let args = {
            if let Some(&Token::Symbol(')')) = self.peek() {
                vec![]
            } else {
                let mut v = vec![];
                v.push(self.expr());
                while let Some(&Token::Symbol(',')) = self.peek() {
                    self.get();
                    v.push(self.expr());
                }
                v
            }
        };
        expect!(self, Symbol, ')');
        AST::MethodCall { table, name, args }
    }

    fn peek(&self) -> Option<&Token> {
        if let Some(t) = self.tokens.get(self.index) {
            Some(t)
        } else {
            None
        }
    }

    fn get(&mut self) -> Token {
        if let Some(t) = self.tokens.pop_front() {
            t
        } else {
            panic!();
        }
    }

    pub fn parse(&mut self) -> AST {
        match self.peek().unwrap() {
            Token::Number(_) => self.expr(),
            Token::Ident(_) => self.method_call(),
            _ => unimplemented!(),
        }
    }
}

#[test]

fn new() {
    let tokens = Tokenizer::new("hogefuga").lex_all();
    assert_eq!(Parser::new(tokens).index, 0);
}

#[test]
fn add() {
    let tokens = Tokenizer::new("1 + 2 + 3").lex_all();
    assert_eq!(
        Parser::new(tokens).add(),
        binop!(binop!(Number(1), '+', Number(2)), '+', Number(3))
    );
}

#[test]
fn mul() {
    let tokens = Tokenizer::new("1 + 2 * 3").lex_all();
    assert_eq!(
        Parser::new(tokens).add(),
        binop!(Number(1), '+', binop!(Number(2), '*', Number(3)))
    );
}

#[test]
fn method_call() {
    let tokens = Tokenizer::new("User.select()").lex_all();
    assert_eq!(
        Parser::new(tokens).method_call(),
        AST::MethodCall {
            table: "User".to_string(),
            name: "select".to_string(),
            args: vec![]
        }
    );

    let tokens = Tokenizer::new("User.select(1)").lex_all();
    assert_eq!(
        Parser::new(tokens).method_call(),
        AST::MethodCall {
            table: "User".to_string(),
            name: "select".to_string(),
            args: vec![Number(1)]
        }
    );

    let tokens = Tokenizer::new("User.select(1+1, 2)").lex_all();
    assert_eq!(
        Parser::new(tokens).method_call(),
        AST::MethodCall {
            table: "User".to_string(),
            name: "select".to_string(),
            args: vec![binop!(Number(1), '+', Number(1)), Number(2)]
        }
    );
}
