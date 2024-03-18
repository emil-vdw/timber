use crate::location::Location;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum TokenVariant {
    OpenParenthesis, CloseParenthesis,
    DoubleQuote, Dot, Comma, Quote, Backquote, Semicolon,

    Literal(Literal)
}

#[derive(Debug, Clone)]
pub struct Token {
    variant:  TokenVariant,
    lexeme: String,
    location: Location,
}

impl Token {
    pub fn new(
        variant: TokenVariant, source: &str, location: Location
    ) -> Self {
        Self {
            variant,
            location,
            lexeme: source.to_owned(),
        }
    }
}
