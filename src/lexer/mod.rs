pub mod tokens;

use tokens::Token;
use std::str::Chars;
use std::iter::Peekable;


#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn lex_number(&mut self, first: char) -> Token {
        let mut s = String::from(first);
        
        // integer part
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        // Check for fractional part
        if let Some(&'.') = self.chars.peek() {
            // Need to see if next after dot is digit.
            let mut clone = self.chars.clone();
            clone.next(); // skip dot
            if let Some(&next_c) = clone.peek() {
                if next_c.is_ascii_digit() {
                    s.push('.');
                    self.chars.next(); // consume dot
                    while let Some(&c) = self.chars.peek() {
                        if c.is_ascii_digit() {
                            s.push(c);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        Token::Number(s.parse().unwrap_or(0.0))
    }

    fn lex_string(&mut self) -> Token {
        // self.chars.next(); // consumed quote before calling
        let mut s = String::new();
        let mut escaped = false;
        
        while let Some(c) = self.chars.next() {
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
                return Token::StringLiteral(s);
            } else {
                s.push(c);
            }
        }
        
        Token::Unknown('"') // Unterminated string
    }

    fn lex_identifier_or_keyword(&mut self, first: char) -> Token {
        let mut ident = String::from(first);
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.chars.next();
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
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let c = self.chars.next()?;

        // Comments
        if c == '/' {
            // Check next
            if let Some(&next_c) = self.chars.peek() {
                if next_c == '/' {
                    // Line comment
                    self.chars.next(); // consume /
                    // skip until newline
                    while let Some(x) = self.chars.next() {
                        if x == '\n' { break; }
                    }
                    return self.next(); // Recursively call next to get actual token
                } else if next_c == '*' {
                    // Block comment
                    self.chars.next(); // consume *
                    let mut terminated = false;
                    while let Some(x) = self.chars.next() {
                        if x == '*' {
                            if let Some(&post_star) = self.chars.peek() {
                                if post_star == '/' {
                                    self.chars.next(); // consume /
                                    terminated = true;
                                    break;
                                }
                            }
                        }
                    }
                    if !terminated {
                        return Some(Token::EOF); // Or error
                    }
                    return self.next();
                }
            }
        }

        let token = match c {
            '0'..='9' => self.lex_number(c),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier_or_keyword(c),
            '"' => self.lex_string(),
            '+' => Token::Plus,
            '-' => {
                if let Some(&'>') = self.chars.peek() {
                    self.chars.next();
                    Token::Arrow // ->
                } else {
                    Token::Minus
                }
            },
            '*' => Token::Star,
            '/' => Token::Slash, // Comments handled above, so this is division
            '%' => Token::Percent,
            '^' => Token::Power,
            '=' => {
                // = or == or =>
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Token::Equal
                } else if let Some(&'>') = self.chars.peek() {
                    self.chars.next();
                    Token::Arrow 
                } else {
                    Token::Assign
                }
            },
            ':' => {
                 if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Token::DestructAssign // :=
                } else {
                    Token::Colon
                }
            },
            '!' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            },
            '<' => {
                if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Token::LessThanEq
                } else {
                    Token::LessThan
                }
            },
            '>' => {
                 if let Some(&'=') = self.chars.peek() {
                    self.chars.next();
                    Token::GreaterThanEq
                } else {
                    Token::GreaterThan
                }
            },
            '&' => Token::And,
            '|' => Token::Or,
            '@' => {
                if let Some(&'@') = self.chars.peek() {
                    self.chars.next();
                    Token::ConcatSpace
                } else {
                    Token::Concat
                }
            },
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ',' => Token::Comma,
            '.' => Token::Dot,
            ';' => Token::Semicolon,
            _ => Token::Unknown(c),
        };


        Some(token)
    }
}


#[cfg(test)]
mod tests;


