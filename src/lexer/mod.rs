pub mod tokens;
pub mod errors;

use crate::utils::Position;
use tokens::Token;
use errors::LexError;
use std::str::Chars;
use std::iter::Peekable;


#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn current_pos(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn lex_number(&mut self, first: char) -> Token {
        let mut s = String::from(first);
        
        // integer part
        while let Some(&c) = self.peek_char() {
            if c.is_ascii_digit() {
                s.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        // Check for fractional part
        if let Some(&'.') = self.peek_char() {
            // Need to see if next after dot is digit.
            let mut clone = self.chars.clone();
            clone.next(); // skip dot
            if let Some(&next_c) = clone.peek() {
                if next_c.is_ascii_digit() {
                    s.push('.');
                    self.next_char(); // consume dot
                    while let Some(&c) = self.peek_char() {
                        if c.is_ascii_digit() {
                            s.push(c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        Token::Number(s.parse().unwrap_or(0.0))
    }

    fn lex_string(&mut self, start_pos: Position) -> Result<Token, LexError> {
        // self.chars.next(); // consumed quote before calling
        let mut s = String::new();
        let mut escaped = false;
        
        while let Some(c) = self.next_char() {
            if escaped {
                match c {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    'r' => s.push('\r'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    _ => s.push(c),
                }
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                return Ok(Token::StringLiteral(s));
            } else {
                s.push(c);
            }
        }
        
        Err(LexError::UnterminatedString(start_pos))
    }

    fn lex_identifier_or_keyword(&mut self, first: char) -> Token {
        let mut ident = String::from(first);
        while let Some(&c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.next_char();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "function" => Token::Function,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "type" => Token::Type,
            "new" => Token::New,
            "inherits" => Token::Inherits,
            "protocol" => Token::Protocol,
            "is" => Token::Is,
            "as" => Token::As,
            "print" => Token::Print,
            "true" => Token::True,
            "false" => Token::False,
            "in" => Token::In,
            _ => Token::Identifier(ident),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(Token, Position), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let pos = self.current_pos();
        let c = self.next_char()?;

        // Comments
        if c == '/' {
            // Check next
            if let Some(&next_c) = self.peek_char() {
                if next_c == '/' {
                    // Line comment
                    self.next_char(); // consume /
                    // skip until newline
                    while let Some(x) = self.next_char() {
                        if x == '\n' { break; }
                    }
                    return self.next(); // Recursively call next to get actual token
                } else if next_c == '*' {
                    // Block comment
                    self.next_char(); // consume *
                    let mut terminated = false;
                    while let Some(x) = self.next_char() {
                        if x == '*' {
                            if let Some(&post_star) = self.peek_char() {
                                if post_star == '/' {
                                    self.next_char(); // consume /
                                    terminated = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !terminated {
                        return Some(Err(LexError::UnterminatedBlockComment(pos))); // Or error
                    }
                    return self.next();
                }
            }
        }

        let token_res = match c {
            '0'..='9' => Ok(self.lex_number(c)),
            'a'..='z' | 'A'..='Z' => Ok(self.lex_identifier_or_keyword(c)),
            '"' => self.lex_string(pos),
            '+' => Ok(Token::Plus),
            '-' => {
                if let Some(&'>') = self.peek_char() {
                    self.next_char();
                    Ok(Token::Arrow) // ->
                } else {
                    Ok(Token::Minus)
                }
            },
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash), // Comments handled above, so this is division
            '%' => Ok(Token::Percent),
            '^' => Ok(Token::Power),
            '=' => {
                // = or == or =>
                if let Some(&'=') = self.peek_char() {
                    self.next_char();
                    Ok(Token::Equal)
                } else if let Some(&'>') = self.peek_char() {
                    self.next_char();
                    Ok(Token::Arrow) 
                } else {
                    Ok(Token::Assign)
                }
            },
            ':' => {
                 if let Some(&'=') = self.peek_char() {
                    self.next_char();
                    Ok(Token::DestructAssign) // :=
                } else {
                    Ok(Token::Colon)
                }
            },
            '!' => {
                if let Some(&'=') = self.peek_char() {
                    self.next_char();
                    Ok(Token::NotEqual)
                } else {
                    Ok(Token::Not)
                }
            },
            '<' => {
                if let Some(&'=') = self.peek_char() {
                    self.next_char();
                    Ok(Token::LessThanEq)
                } else {
                    Ok(Token::LessThan)
                }
            },
            '>' => {
                 if let Some(&'=') = self.peek_char() {
                    self.next_char();
                    Ok(Token::GreaterThanEq)
                } else {
                    Ok(Token::GreaterThan)
                }
            },
            '&' => Ok(Token::And),
            '|' => Ok(Token::Or),
            '@' => {
                if let Some(&'@') = self.peek_char() {
                    self.next_char();
                    Ok(Token::ConcatSpace)
                } else {
                    Ok(Token::Concat)
                }
            },
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            ',' => Ok(Token::Comma),
            '.' => Ok(Token::Dot),
            ';' => Ok(Token::Semicolon),
            _ => Err(LexError::UnexpectedCharacter(c, pos)),
        };


        Some(token_res.map(|t| (t, pos)))
    }
}


#[cfg(test)]
mod tests;
