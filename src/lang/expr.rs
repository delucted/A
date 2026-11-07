pub enum Literal {
    Number(f64),
}

pub enum TokenKind {
    LeftParen, RightParen,

    Plus, Minus, Div,
    Mult,
}

pub struct Token {
    lexeme: String,
    kind: TokenKind,
    literal: Option<Literal>
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Unary { operator: Token, right: Box<Expr> },
    Paren(Box<Expr>),
    Literal(Literal)
}