use crate::{tokens::{Token, TokenVariant, Literal}, location::Location, error::ScanError};

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    position: Location,
    tokens: Vec<Token>,
}


impl Lexer {
    pub fn scan(&mut self) -> Result<Vec<Token>, ScanError> {
        if self.source.is_empty() {
            return Err(ScanError::new("Empty source", self.position.clone()));
        }

        while self.position.char != self.source.len() {
            match self.advance()? {
                ' ' | '\n' => continue,

                '(' => self.tokens.push(Token::new(TokenVariant::OpenParenthesis, "(", self.position.clone())),
                ')' => self.tokens.push(Token::new(TokenVariant::CloseParenthesis, ")", self.position.clone())),
                '.' => self.tokens.push(Token::new(TokenVariant::Dot, ".", self.position.clone())),
                ',' => self.tokens.push(Token::new(TokenVariant::Comma, ",", self.position.clone())),
                '`' => self.tokens.push(Token::new(TokenVariant::Backquote, "`", self.position.clone())),
                ';' => self.tokens.push(Token::new(TokenVariant::Semicolon, ";", self.position.clone())),
                '\'' => self.tokens.push(Token::new(TokenVariant::Quote, "'", self.position.clone())),

                ';' => {
                    while self.advance()? != '\n' {};
                },

                '"' => {
                    let mut string_literal = String::new();
                    while let next_char = self.advance()? {
                        if next_char == '\\' {
                            // Escape character
                            string_literal.push(self.advance()?);
                        } else {
                            string_literal.push(next_char);
                        }
                    }

                    self.tokens.push(Token::new(
                        TokenVariant::Literal(Literal::String(string_literal)),
                        &string_literal,
                        self.position.clone(),
                    ))
                },

                char if char.is_digit(10) => {
                    let mut number_literal_string = String::from(char);

                    while self.peek(None).is_some_and(|char| char.is_digit(10)) {
                        number_literal_string.push(self.advance()?);
                    }

                    self.tokens.push(
                        Token::new(
                            TokenVariant::Literal(Literal::Number(
                                number_literal_string.parse().map_err(
                                    || ScanError::new(

                                    )
                                )
                            )),
                            &number_literal_string,
                            self.position.clone(),
                        )
                    )
                },

                char => {
                    return Err(ScanError::new(
                        &format!("Unexpected character: {}", invalid_char),
                        self.position.clone()
                    ))
                },

            };
        }

        todo!()
    }

    fn advance(&mut self) -> Result<char, ScanError> {
        if let Some(next_char) = self.peek(None) {
            match next_char {
                '\n' => {
                    self.position.line += 1;
                    self.position.column = 0;
                },
                _ => {
                    self.position.column += 1;
                },
            }
            self.position.char += 1;

            Ok(next_char)
        } else {
            Err(ScanError::new("Unexpected end of file", self.position.clone()))
        }
    }

    fn peek(&self, char: Option<usize>) -> Option<char> {
        self.source.chars().nth(
            char.unwrap_or_else(|| self.position.char + 1)
        )
    }
}
