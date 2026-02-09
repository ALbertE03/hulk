use crate::lexer::tokens::Token;
use crate::utils::Position;
use crate::errors::ParseError;

impl super::Parser {
    /// Avanza y retorna el siguiente token junto con su posición.
    /// Convierte errores léxicos en `ParseError::Lex`.
    pub(super) fn advance(&mut self) -> Result<(Token, Position), ParseError> {
        if self.current < self.tokens.len() {
            let res = self.tokens[self.current].clone();
            match &res {
                Ok((t, _)) => {
                    if *t != Token::EOF {
                        self.current += 1;
                    }
                }
                Err(_) => {
                    self.current += 1;
                }
            }
            res.map_err(ParseError::Lex)
        } else {
            Err(ParseError::UnexpectedEOF(self.peek_pos()))
        }
    }

    /// Mira el token actual sin consumirlo.
    pub(super) fn peek(&self) -> Option<&Result<(Token, Position), crate::errors::LexError>> {
        self.tokens.get(self.current)
    }

    /// Comprueba si el token actual coincide con `token`.
    pub(super) fn check(&self, token: &Token) -> bool {
        match self.peek() {
            Some(Ok((t, _))) => t == token,
            _ => false,
        }
    }

    /// Si el token actual coincide con `token`, lo consume y devuelve `true`.
    pub(super) fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance().unwrap();
            true
        } else {
            false
        }
    }

    /// Consume el token esperado o devuelve un `ParseError` con `message`.
    pub(super) fn consume(&mut self, token: &Token, message: &str) -> Result<(Token, Position), ParseError> {
        match self.peek() {
            Some(Ok((t, _))) if t == token => self.advance(),
            Some(Err(_)) => self.advance(),
            _ => Err(ParseError::UnexpectedToken {
                expected: message.to_string(),
                found: format!("{:?}", self.peek().map(|r| match r {
                    Ok((t, _)) => format!("{:?}", t),
                    Err(e) => format!("LexError: {:?}", e),
                })),
                pos: self.peek_pos(),
            })
        }
    }

    /// Devuelve la posición del token actual o la última posición conocida.
    pub(super) fn peek_pos(&self) -> Position {
        self.tokens.get(self.current)
            .and_then(|r| r.as_ref().ok())
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| {
                self.tokens.last()
                    .and_then(|r| match r {
                        Ok((_, p)) => Some(p.clone()),
                        Err(e) => match e {
                            crate::errors::LexError::UnterminatedString(p) => Some(p.clone()),
                            crate::errors::LexError::UnterminatedBlockComment(p) => Some(p.clone()),
                            crate::errors::LexError::UnexpectedCharacter(_, p) => Some(p.clone()),
                        },
                    })
                    .unwrap_or(Position { line: 1, column: 1 })
            })
    }

    /// Devuelve una descripción textual del token actual (o del error léxico).
    pub(super) fn peek_description(&self) -> String {
        match self.peek() {
            Some(Ok((t, _))) => format!("{:?}", t),
            Some(Err(e)) => format!("LexError: {:?}", e),
            None => "EOF".to_string(),
        }
    }

    /// Verifica si estamos al final de los tokens.
    pub(super) fn at_end(&self) -> bool {
        match self.peek() {
            Some(Ok((Token::EOF, _))) => true,
            None => true,
            _ => false,
        }
    }

    /// Verifica si el token actual indica el inicio de una expresión.
    pub(super) fn is_expr_start(&self) -> bool {
        matches!(
            self.peek(),
            Some(Ok((
                Token::Number(_) | Token::StringLiteral(_) | Token::True | Token::False |
                Token::Identifier(_) | Token::Minus | Token::Not | Token::LParen |
                Token::Let | Token::If | Token::While | Token::For | Token::Match |
                Token::LBrace | Token::New | Token::LBracket | Token::Base | Token::Print |
                Token::Concat,
                _
            )))
        )
    }
}
