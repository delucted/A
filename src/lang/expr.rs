#[derive(Clone, Copy)]
pub enum Literal {
    Number(f64),
}

#[derive(Clone)]
pub enum TokenKind {
    LeftParen, RightParen,

    Plus, Minus, Div,
    Mult,
}

#[derive(Clone)]
pub struct Token {
    lexeme: String,
    kind: TokenKind,
    literal: Option<Literal>
}

#[derive(Clone)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    Paren(Box<Expr>),
    Literal(Literal)
}