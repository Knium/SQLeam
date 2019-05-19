#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Ident(String),
    Symbol(char),
}
