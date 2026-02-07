use crate::lexer::Lexer;
use crate::lexer::tokens::Token;
use crate::utils::{Position, Spanned};
use crate::errors::ParseError;
use crate::ast::nodes::*;

pub struct Parser {
    tokens: Vec<Result<(Token, Position), crate::errors::LexError>>,
    current: usize,
}

impl Parser {
    /// Crea un nuevo `Parser` a partir de la cadena de entrada.
    /// Inicializa el lexer, recopila tokens y asegura un EOF final.
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(res) = lexer.next() {
            tokens.push(res);
        }
        
        let last_is_eof = matches!(tokens.last(), Some(Ok((Token::EOF, _))));
        if !last_is_eof {
            let last_pos = match tokens.last() {
                Some(Ok((_, p))) => p.clone(),
                Some(Err(e)) => match e {
                    crate::errors::LexError::UnterminatedString(p) => p.clone(),
                    crate::errors::LexError::UnterminatedBlockComment(p) => p.clone(),
                    crate::errors::LexError::UnexpectedCharacter(_, p) => p.clone(),
                },
                None => Position { line: 1, column: 1 },
            };
            tokens.push(Ok((Token::EOF, last_pos)));
        }

        Self {
            tokens,
            current: 0,
        }
    }

    /// Avanza y retorna el siguiente token junto con su posición.
    /// Convierte errores léxicos en `ParseError::Lex`.
    fn advance(&mut self) -> Result<(Token, Position), ParseError> {
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
    fn peek(&self) -> Option<&Result<(Token, Position), crate::errors::LexError>> {
        self.tokens.get(self.current)
    }

    /// Comprueba si el token actual coincide con `token`.
    fn check(&self, token: &Token) -> bool {
        match self.peek() {
            Some(Ok((t, _))) => t == token,
            _ => false,
        }
    }

    /// Si el token actual coincide con `token`, lo consume y devuelve `true`.
    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance().unwrap();
            true
        } else {
            false
        }
    }

    /// Consume el token esperado o devuelve un `ParseError` con `message`.
    fn consume(&mut self, token: &Token, message: &str) -> Result<(Token, Position), ParseError> {
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
    fn peek_pos(&self) -> Position {
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
    fn peek_description(&self) -> String {
        match self.peek() {
            Some(Ok((t, _))) => format!("{:?}", t),
            Some(Err(e)) => format!("LexError: {:?}", e),
            None => "EOF".to_string(),
        }
    }

    // --- Análisis principal ---

    /// Analiza un programa completo y devuelve el `Program` AST.
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = Vec::new();

        while !self.at_end() && !self.is_expr_start() {
            declarations.push(self.parse_declaration()?);
        }

        let expr = if self.at_end() {
            Spanned::new(Expr::Block(Vec::new()), self.peek_pos())
        } else {
            let e = self.parse_spanned_expr(Precedence::Lowest)?;
            self.match_token(&Token::Semicolon);
            e
        };

        if !self.at_end() {
            return Err(ParseError::UnexpectedToken {
                expected: "end of file".to_string(),
                found: self.peek_description(),
                pos: self.peek_pos(),
            });
        }

        Ok(Program {
            declarations,
            expr,
        })
    }

    /// Analiza una declaración (función, tipo o protocolo) y la devuelve.
    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.match_token(&Token::Function) {
            Ok(Declaration::Function(self.parse_function_decl()?))
        } else if self.match_token(&Token::Type) {
            Ok(Declaration::Type(self.parse_type_decl()?))
        } else if self.match_token(&Token::Protocol) {
            Ok(Declaration::Protocol(self.parse_protocol_decl()?))
        } else if self.match_token(&Token::Def) {
            Ok(Declaration::Macro(self.parse_macro_decl()?))
        } else {
            let pos = self.peek_pos();
            Err(ParseError::UnexpectedToken {
                expected: "function, type, protocol, or def".to_string(),
                found: self.peek_description(),
                pos,
            })
        }
    }

    // --- Análisis de expresiones ---

    /// Analiza una expresión respetando la precedencia y devuelve un `Spanned<Expr>`.
    fn parse_spanned_expr(&mut self, precedence: Precedence) -> Result<Spanned<Expr>, ParseError> {
        let mut left = self.parse_prefix()?;

        while !self.at_end() && precedence < self.peek_precedence() {
            left = self.parse_infix(left)?;
        }

        Ok(left)
    }

    /// Analiza expresiones prefijas (literales, identificadores, unarios, etc.).
    fn parse_prefix(&mut self) -> Result<Spanned<Expr>, ParseError> {
        let (token, pos) = self.advance()?;
        match token {
            Token::Number(val) => Ok(Spanned::new(Expr::Number(val), pos)),
            Token::StringLiteral(val) => Ok(Spanned::new(Expr::String(val), pos)),
            Token::True => Ok(Spanned::new(Expr::Boolean(true), pos)),
            Token::False => Ok(Spanned::new(Expr::Boolean(false), pos)),
            Token::Identifier(name) => {
                match name.as_str() {
                    "PI" => Ok(Spanned::new(Expr::PI, pos)),
                    "E" => Ok(Spanned::new(Expr::E, pos)),
                    "rand" if self.check(&Token::LParen) => {
                        self.advance()?; // consumir '('
                        self.consume(&Token::RParen, "Expected ')' after rand")?;
                        Ok(Spanned::new(Expr::Rand, pos))
                    }
                    "sqrt" if self.check(&Token::LParen) => {
                        self.advance()?; // consumir '('
                        let val = self.parse_spanned_expr(Precedence::Lowest)?;
                        self.consume(&Token::RParen, "Expected ')' after sqrt arguments")?;
                        Ok(Spanned::new(Expr::Sqrt(Box::new(val)), pos))
                    }
                    "sin" if self.check(&Token::LParen) => {
                        self.advance()?; // consumir '('
                        let val = self.parse_spanned_expr(Precedence::Lowest)?;
                        self.consume(&Token::RParen, "Expected ')' after sin arguments")?;
                        Ok(Spanned::new(Expr::Sin(Box::new(val)), pos))
                    }
                    "cos" if self.check(&Token::LParen) => {
                        self.advance()?; // consumir '('
                        let val = self.parse_spanned_expr(Precedence::Lowest)?;
                        self.consume(&Token::RParen, "Expected ')' after cos arguments")?;
                        Ok(Spanned::new(Expr::Cos(Box::new(val)), pos))
                    }
                    "exp" if self.check(&Token::LParen) => {
                        self.advance()?; // consumir '('
                        let val = self.parse_spanned_expr(Precedence::Lowest)?;
                        self.consume(&Token::RParen, "Expected ')' after exp arguments")?;
                        Ok(Spanned::new(Expr::Exp(Box::new(val)), pos))
                    }
                    "log" if self.check(&Token::LParen) => {
                        self.advance()?; // consumir '('
                        let base = self.parse_spanned_expr(Precedence::Lowest)?;
                        self.consume(&Token::Comma, "Expected ',' between log arguments")?;
                        let val = self.parse_spanned_expr(Precedence::Lowest)?;
                        self.consume(&Token::RParen, "Expected ')' after log arguments")?;
                        Ok(Spanned::new(Expr::Log(Box::new(base), Box::new(val)), pos))
                    }
                    _ => Ok(Spanned::new(Expr::Identifier(name), pos)),
                }
            }
            Token::Minus => {
                let expr = self.parse_spanned_expr(Precedence::Unary)?;
                Ok(Spanned::new(Expr::Unary(UnOp::Neg, Box::new(expr)), pos))
            }
            Token::Not => {
                let expr = self.parse_spanned_expr(Precedence::Unary)?;
                Ok(Spanned::new(Expr::Unary(UnOp::Not, Box::new(expr)), pos))
            }
            Token::LParen => self.parse_lambda_or_parenthesized(pos),
            Token::Let => self.parse_let_expr(pos),
            Token::If => self.parse_if_expr(pos),
            Token::While => self.parse_while_expr(pos),
            Token::For => self.parse_for_expr(pos),
            Token::Match => {
                let match_expr = self.parse_match_expr()?;
                Ok(Spanned::new(match_expr, pos))
            }
            Token::LBrace => self.parse_block_expr(pos),
            Token::New => self.parse_instantiation_expr(pos),
            Token::LBracket => self.parse_vector_expr(pos),
            Token::Base => {
                self.consume(&Token::LParen, "Expected '(' after base")?;
                let mut args = Vec::new();
                if !self.check(&Token::RParen) {
                    loop {
                        args.push(self.parse_spanned_expr(Precedence::Lowest)?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&Token::RParen, "Expected ')' after base arguments")?;
                Ok(Spanned::new(Expr::BaseCall { args }, pos))
            }
            Token::Print => {
                self.consume(&Token::LParen, "Expected '(' after print")?;
                let mut args = Vec::new();
                if !self.check(&Token::RParen) {
                    loop {
                        args.push(self.parse_spanned_expr(Precedence::Lowest)?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&Token::RParen, "Expected ')' after print arguments")?;
                Ok(Spanned::new(Expr::Call { func: "print".to_string(), args }, pos))
            }
            _ => Err(ParseError::InvalidExpression(pos)),
        }
    }

    /// Determina si el paréntesis abre una lambda o una expresión agrupada y la analiza.
    fn parse_lambda_or_parenthesized(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        let start_index = self.current;
        
        let params = self.parse_params();
        if let Ok(p) = params {
            if self.match_token(&Token::RParen) {
                if self.check(&Token::FuncArrow) || self.check(&Token::Colon) {
                    let return_type = if self.match_token(&Token::Colon) {
                        Some(self.parse_type_annotation()?)
                    } else {
                        None
                    };
                    self.consume(&Token::FuncArrow, "Expected '=>' after lambda signature")?;
                    let body = self.parse_spanned_expr(Precedence::Lowest)?;
                    return Ok(Spanned::new(Expr::Lambda { 
                        params: p, 
                        return_type, 
                        body: Box::new(body) 
                    }, pos));
                }
            }
        }

        self.current = start_index;
        let expr = self.parse_spanned_expr(Precedence::Lowest)?;
        self.consume(&Token::RParen, "Expected ')' after grouping")?;
        Ok(expr)
    }

    /// Analiza operadores infijos y construye expresiones binarias, llamadas, accesos, etc.
    fn parse_infix(&mut self, left: Spanned<Expr>) -> Result<Spanned<Expr>, ParseError> {
        let (token, pos) = self.advance()?;
        
        match token {
            Token::LParen => self.parse_call_expr(left, pos),
            Token::Dot => self.parse_member_access(left, pos),
            Token::LBracket => self.parse_indexing_expr(left, pos),
            Token::Is => {
                let ty = match self.advance()?.0 {
                    Token::Identifier(t) => t,
                    t => return Err(ParseError::UnexpectedToken {
                        expected: "type name".to_string(),
                        found: format!("{:?}", t),
                        pos: self.peek_pos(),
                    }),
                };
                Ok(Spanned::new(Expr::Is(Box::new(left), ty), pos))
            }
            Token::As => {
                let ty = match self.advance()?.0 {
                    Token::Identifier(t) => t,
                    t => return Err(ParseError::UnexpectedToken {
                        expected: "type name".to_string(),
                        found: format!("{:?}", t),
                        pos: self.peek_pos(),
                    }),
                };
                Ok(Spanned::new(Expr::As(Box::new(left), ty), pos))
            }
            Token::DestructAssign => {
                // x := expr
                if let Expr::Identifier(name) = &left.node {
                    let right = self.parse_spanned_expr(Precedence::Assignment)?;
                    let pos_start = left.pos.clone();
                    Ok(Spanned::new(Expr::Assignment { target: name.clone(), value: Box::new(right) }, pos_start))
                } else {
                    Err(ParseError::UnexpectedToken {
                        expected: "identifier".to_string(),
                        found: format!("{:?}", left.node),
                        pos: left.pos,
                    })
                }
            }
            _ => {
                let op = self.token_to_op(&token).ok_or(ParseError::InvalidExpression(pos))?;
                let precedence = self.op_precedence(&op);
                let next_precedence = if op == Op::Pow {
                    Precedence::Unary
                } else {
                    precedence
                };

                let right = self.parse_spanned_expr(next_precedence)?;
                let combined_pos = left.pos.clone(); 
                Ok(Spanned::new(Expr::Binary(Box::new(left), op, Box::new(right)), combined_pos))
            }
        }
    }

    /// Obtiene la precedencia del token actual para decidir asociaciones.
    fn peek_precedence(&self) -> Precedence {
        match self.peek() {
            Some(Ok((t, _))) => self.token_to_precedence(t),
            _ => Precedence::Lowest
        }
    }

    /// Convierte un `Token` en su precedencia correspondiente.
    fn token_to_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::LParen => Precedence::Call,
            Token::Dot => Precedence::Call,
            Token::LBracket => Precedence::Call,
            Token::Is => Precedence::Comparison,
            Token::As => Precedence::Comparison,
            Token::DestructAssign => Precedence::Assignment,
            _ => if let Some(op) = self.token_to_op(token) {
                self.op_precedence(&op)
            } else {
                Precedence::Lowest
            }
        }
    }

    // --- Specific Expression Parsers ---

    /// Analiza una expresión `let` con sus enlaces y el cuerpo asociado.
    fn parse_let_expr(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        let mut bindings = Vec::new();
        loop {
            let name = match self.advance()?.0 {
                Token::Identifier(n) => n,
                t => return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            };

            let type_annotation = if self.match_token(&Token::Colon) {
                Some(self.parse_type_annotation()?)
            } else {
                None
            };

            self.consume(&Token::Assign, "Expected '=' in let binding")?;
            let init = self.parse_spanned_expr(Precedence::Lowest)?;
            bindings.push((name, type_annotation, init));

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        self.consume(&Token::In, "Expected 'in' after let bindings")?;
        let body = self.parse_spanned_expr(Precedence::Lowest)?;

        Ok(Spanned::new(Expr::Let { bindings, body: Box::new(body) }, pos))
    }

    /// Analiza una expresión `if` (condición, then y else/elif).
    fn parse_if_expr(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        self.consume(&Token::LParen, "Expected '(' after if")?;
        let cond = self.parse_spanned_expr(Precedence::Lowest)?;
        self.consume(&Token::RParen, "Expected ')' after if condition")?;
        
        let then_expr = self.parse_spanned_expr(Precedence::Lowest)?;
        
        let else_expr = if self.match_token(&Token::Else) {
            self.parse_spanned_expr(Precedence::Lowest)?
        } else if self.match_token(&Token::Elif) {
            // 'elif' se representa como if anidado
            self.parse_if_expr(self.peek_pos())?
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "else or elif".to_string(),
                found: self.peek_description(),
                pos: self.peek_pos(),
            });
        };

        Ok(Spanned::new(Expr::If {
            cond: Box::new(cond),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        }, pos))
    }

    /// Analiza una expresión `while` con su condición y cuerpo.
    fn parse_while_expr(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        self.consume(&Token::LParen, "Expected '(' after while")?;
        let cond = self.parse_spanned_expr(Precedence::Lowest)?;
        self.consume(&Token::RParen, "Expected ')' after while condition")?;
        
        let body = self.parse_spanned_expr(Precedence::Lowest)?;
        
        Ok(Spanned::new(Expr::While {
            cond: Box::new(cond),
            body: Box::new(body),
        }, pos))
    }

    /// Analiza una expresión `for` (variable, iterable y cuerpo).
    fn parse_for_expr(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        self.consume(&Token::LParen, "Expected '(' after for")?;
        
        let var = match self.advance()?.0 {
            Token::Identifier(n) => n,
            t => return Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: format!("{:?}", t),
                pos: self.peek_pos(),
            }),
        };

        self.consume(&Token::In, "Expected 'in' after for variable")?;
        let iterable = self.parse_spanned_expr(Precedence::Lowest)?;
        self.consume(&Token::RParen, "Expected ')' after for iterable")?;
        
        let body = self.parse_spanned_expr(Precedence::Lowest)?;

        Ok(Spanned::new(Expr::For {
            var,
            iterable: Box::new(iterable),
            body: Box::new(body),
        }, pos))
    }

    /// Analiza un bloque `{ ... }` y devuelve una expresión de bloque.
    fn parse_block_expr(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        let mut exprs = Vec::new();
        while !self.check(&Token::RBrace) && !self.at_end() {
            exprs.push(self.parse_spanned_expr(Precedence::Lowest)?);
            let _ = self.match_token(&Token::Semicolon);
        }
        self.consume(&Token::RBrace, "Expected '}' after block")?;
        Ok(Spanned::new(Expr::Block(exprs), pos))
    }

    /// Analiza la instanciación de un tipo `Type(args...)`.
    fn parse_instantiation_expr(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        let ty = match self.advance()?.0 {
            Token::Identifier(n) => n,
            t => return Err(ParseError::UnexpectedToken {
                expected: "type name".to_string(),
                found: format!("{:?}", t),
                pos: self.peek_pos(),
            }),
        };

        self.consume(&Token::LParen, "Expected '(' in instantiation")?;
        let mut args = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                args.push(self.parse_spanned_expr(Precedence::Lowest)?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.consume(&Token::RParen, "Expected ')' after instantiation arguments")?;

        Ok(Spanned::new(Expr::Instantiation { ty, args }, pos))
    }

    /// Analiza literales de vectores y generadores de vectores.
    fn parse_vector_expr(&mut self, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        if self.check(&Token::RBracket) {
            self.advance()?;
            return Ok(Spanned::new(Expr::VectorLiteral(Vec::new()), pos));
        }

        let first = self.parse_spanned_expr(Precedence::Or)?;

        if self.match_token(&Token::Or) {
            let var = match self.advance()?.0 {
                Token::Identifier(n) => n,
                t => return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            };

            self.consume(&Token::In, "Expected 'in' after variable in generator")?;
            let iterable = self.parse_spanned_expr(Precedence::Lowest)?;
            self.consume(&Token::RBracket, "Expected ']' after generator")?;

            Ok(Spanned::new(Expr::VectorGenerator {
                expr: Box::new(first),
                var,
                iterable: Box::new(iterable),
            }, pos))
        } else {
            let mut exprs = vec![first];
            while self.match_token(&Token::Comma) {
                exprs.push(self.parse_spanned_expr(Precedence::Lowest)?);
            }
            self.consume(&Token::RBracket, "Expected ']' after vector")?;
            Ok(Spanned::new(Expr::VectorLiteral(exprs), pos))
        }
    }

    /// Analiza una llamada de función a partir de la expresión izquierda.
    fn parse_call_expr(&mut self, left: Spanned<Expr>, _pos: Position) -> Result<Spanned<Expr>, ParseError> {
        let func_name = match left.node {
            Expr::Identifier(n) => n,
            _ => return Err(ParseError::UnexpectedToken {
                expected: "function name".to_string(),
                found: format!("{:?}", left.node),
                pos: left.pos,
            }),
        };

        let mut args = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                args.push(self.parse_spanned_expr(Precedence::Lowest)?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.consume(&Token::RParen, "Expected ')' after call arguments")?;

        Ok(Spanned::new(Expr::Call { func: func_name, args }, left.pos))
    }

    /// Analiza acceso a miembro o llamada de método (`obj.member` o `obj.member(...)`).
    fn parse_member_access(&mut self, left: Spanned<Expr>, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        let name = match self.advance()?.0 {
            Token::Identifier(n) => n,
            t => return Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: format!("{:?}", t),
                pos,
            }),
        };

        if self.match_token(&Token::LParen) {
            let mut args = Vec::new();
            if !self.check(&Token::RParen) {
                loop {
                    args.push(self.parse_spanned_expr(Precedence::Lowest)?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.consume(&Token::RParen, "Expected ')' after method arguments")?;
            Ok(Spanned::new(Expr::MethodCall { obj: Box::new(left), method: name, args }, pos))
        } else {
            Ok(Spanned::new(Expr::AttributeAccess { obj: Box::new(left), attribute: name }, pos))
        }
    }

    /// Analiza indexación `obj[index]` y devuelve la expresión correspondiente.
    fn parse_indexing_expr(&mut self, left: Spanned<Expr>, pos: Position) -> Result<Spanned<Expr>, ParseError> {
        let index = self.parse_spanned_expr(Precedence::Lowest)?;
        self.consume(&Token::RBracket, "Expected ']' after index")?;
        Ok(Spanned::new(Expr::Indexing { obj: Box::new(left), index: Box::new(index) }, pos))
    }

    /// Indica si se ha alcanzado el final de los tokens (EOF).
    fn at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.tokens[self.current], Ok((Token::EOF, _)))
    }

    /// Determina si la posición actual pertenece al inicio de una expresión.
    fn is_expr_start(&self) -> bool {
        !self.check(&Token::Function) && !self.check(&Token::Type) && !self.check(&Token::Protocol) && !self.check(&Token::Def)
    }

    /// Convierte un `Token` de operador en el enum `Op` correspondiente.
    fn token_to_op(&self, token: &Token) -> Option<Op> {
        match token {
            Token::Plus => Some(Op::Add),
            Token::Minus => Some(Op::Sub),
            Token::Star => Some(Op::Mul),
            Token::Slash => Some(Op::Div),
            Token::Percent => Some(Op::Mod),
            Token::Power => Some(Op::Pow),
            Token::Equal => Some(Op::Eq),
            Token::NotEqual => Some(Op::Neq),
            Token::LessThan => Some(Op::Lt),
            Token::GreaterThan => Some(Op::Gt),
            Token::LessThanEq => Some(Op::Le),
            Token::GreaterThanEq => Some(Op::Ge),
            Token::And => Some(Op::And),
            Token::Or => Some(Op::Or),
            Token::Concat => Some(Op::Concat),
            Token::ConcatSpace => Some(Op::ConcatSpace),
            _ => None,
        }
    }

    /// Devuelve la precedencia asociada a una operación `Op`.
    fn op_precedence(&self, op: &Op) -> Precedence {
        match op {
            Op::Or => Precedence::Or,
            Op::And => Precedence::And,
            Op::Eq | Op::Neq | Op::Lt | Op::Gt | Op::Le | Op::Ge => Precedence::Comparison,
            Op::Concat | Op::ConcatSpace => Precedence::Concat,
            Op::Add | Op::Sub => Precedence::Sum,
            Op::Mul | Op::Div | Op::Mod => Precedence::Product,
            Op::Pow => Precedence::Power,
        }
    }

    // --- Declaration Parsers ---

    /// Analiza la declaración de una función y devuelve su `FunctionDecl`.
    fn parse_function_decl(&mut self) -> Result<FunctionDecl, ParseError> {
        let name = match self.advance()?.0 {
            Token::Identifier(n) => n,
            t => return Err(ParseError::UnexpectedToken {
                expected: "function name".to_string(),
                found: format!("{:?}", t),
                pos: self.peek_pos(),
            }),
        };

        self.consume(&Token::LParen, "Expected '(' after function name")?;
        let params = self.parse_params()?;
        self.consume(&Token::RParen, "Expected ')' after function parameters")?;

        let return_type = if self.match_token(&Token::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        let body = if self.match_token(&Token::FuncArrow) {
            let e = self.parse_spanned_expr(Precedence::Lowest)?;
            self.consume(&Token::Semicolon, "Expected ';' after inline function body")?;
            e
        } else {
            let pos = self.peek_pos();
            self.consume(&Token::LBrace, "Expected '=>' or '{' for function body")?;
            let b = self.parse_block_expr(pos)?;
            self.match_token(&Token::Semicolon); 
            b
        };

        Ok(FunctionDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    /// Analiza la declaración de un tipo (atributos y métodos) y la devuelve.
    fn parse_type_decl(&mut self) -> Result<TypeDecl, ParseError> {
        let name = match self.advance()?.0 {
            Token::Identifier(n) => n,
            t => return Err(ParseError::UnexpectedToken {
                expected: "type name".to_string(),
                found: format!("{:?}", t),
                pos: self.peek_pos(),
            }),
        };

        let params = if self.match_token(&Token::LParen) {
            let p = self.parse_params()?;
            self.consume(&Token::RParen, "Expected ')' after type parameters")?;
            p
        } else {
            Vec::new()
        };

        let parent = if self.match_token(&Token::Inherits) {
            let parent_name = match self.advance()?.0 {
                Token::Identifier(n) => n,
                t => return Err(ParseError::UnexpectedToken {
                    expected: "parent type name".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            };

            let args = if self.match_token(&Token::LParen) {
                let mut a = Vec::new();
                if !self.check(&Token::RParen) {
                    loop {
                        a.push(self.parse_spanned_expr(Precedence::Lowest)?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&Token::RParen, "Expected ')' after parent constructor arguments")?;
                a
            } else {
                Vec::new()
            };

            Some(TypeInit { name: parent_name, args })
        } else {
            None
        };

        self.consume(&Token::LBrace, "Expected '{' to start type body")?;
        
        let mut attributes = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&Token::RBrace) && !self.at_end() {
            if self.match_token(&Token::Function) {
                methods.push(self.parse_function_decl()?);
            } else {
                let is_method = if let Some(Ok((Token::Identifier(_), _))) = self.peek() {
                    if let Some(Ok((Token::LParen, _))) = self.tokens.get(self.current + 1) {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

                if is_method {
                    methods.push(self.parse_function_decl()?);
                } else {
                    // Attribute
                    let attr_name = match self.advance()?.0 {
                        Token::Identifier(n) => n,
                        t => return Err(ParseError::UnexpectedToken {
                            expected: "attribute or method".to_string(),
                            found: format!("{:?}", t),
                            pos: self.peek_pos(),
                        }),
                    };

                    let type_annotation = if self.match_token(&Token::Colon) {
                        Some(self.parse_type_annotation()?)
                    } else {
                        None
                    };

                    self.consume(&Token::Assign, "Expected '=' after attribute name")?;
                    let init = self.parse_spanned_expr(Precedence::Lowest)?;
                    self.consume(&Token::Semicolon, "Expected ';' after attribute")?;
                    
                    attributes.push(Attribute { name: attr_name, type_annotation, init });
                }
            }
        }

        self.consume(&Token::RBrace, "Expected '}' after type body")?;

        Ok(TypeDecl {
            name,
            params,
            parent,
            attributes,
            methods,
        })
    }

    /// Analiza la declaración de un protocolo y devuelve su firma.
    fn parse_protocol_decl(&mut self) -> Result<ProtocolDecl, ParseError> {
        let name = match self.advance()?.0 {
            Token::Identifier(n) => n,
            t => return Err(ParseError::UnexpectedToken {
                expected: "protocol name".to_string(),
                found: format!("{:?}", t),
                pos: self.peek_pos(),
            }),
        };

        let parent = if self.match_token(&Token::Extends) {
            match self.advance()?.0 {
                Token::Identifier(n) => Some(n),
                t => return Err(ParseError::UnexpectedToken {
                    expected: "parent protocol name".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            }
        } else {
            None
        };

        self.consume(&Token::LBrace, "Expected '{' to start protocol body")?;
        
        let mut methods = Vec::new();
        while !self.check(&Token::RBrace) && !self.at_end() {
            let method_name = match self.advance()?.0 {
                Token::Identifier(n) => n,
                t => return Err(ParseError::UnexpectedToken {
                    expected: "method name".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            };

            self.consume(&Token::LParen, "Expected '(' after method name")?;
            let params = self.parse_params()?;
            self.consume(&Token::RParen, "Expected ')' after method parameters")?;
            self.consume(&Token::Colon, "Protocols methods must have a return type")?;
            
            let return_type = self.parse_type_annotation()?;

            self.consume(&Token::Semicolon, "Expected ';' after method signature")?;
            methods.push(MethodSignature { name: method_name, params, return_type });
        }

        self.consume(&Token::RBrace, "Expected '}' after protocol body")?;

        Ok(ProtocolDecl {
            name,
            parent,
            methods,
        })
    }

    /// Analiza anotaciones de tipo (nombres, funciones y iterables).
    fn parse_type_annotation(&mut self) -> Result<TypeAnnotation, ParseError> {
        if self.match_token(&Token::LParen) {
            let mut params = Vec::new();
            if !self.check(&Token::RParen) {
                loop {
                    params.push(self.parse_type_annotation()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.consume(&Token::RParen, "Expected ')' after type params")?;
            self.consume(&Token::TypeArrow, "Expected '->' for function type")?;
            let return_type = self.parse_type_annotation()?;
            Ok(TypeAnnotation::Function {
                params,
                return_type: Box::new(return_type),
            })
        } else {
            let name = match self.advance()?.0 {
                Token::Identifier(n) => n,
                t => return Err(ParseError::UnexpectedToken {
                    expected: "type name".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            };
            
            let mut ty = TypeAnnotation::Name(name);
            while self.match_token(&Token::Star) {
                ty = TypeAnnotation::Iterable(Box::new(ty));
            }
            Ok(ty)
        }
    }

    /// Parsea type annotation sin consumir * (para usar en patterns donde * es multiplicación)
    fn parse_type_annotation_no_star(&mut self) -> Result<TypeAnnotation, ParseError> {
        if self.match_token(&Token::LParen) {
            let mut params = Vec::new();
            if !self.check(&Token::RParen) {
                loop {
                    params.push(self.parse_type_annotation_no_star()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.consume(&Token::RParen, "Expected ')' after type params")?;
            self.consume(&Token::TypeArrow, "Expected '->' for function type")?;
            let return_type = self.parse_type_annotation_no_star()?;
            Ok(TypeAnnotation::Function {
                params,
                return_type: Box::new(return_type),
            })
        } else {
            let name = match self.advance()?.0 {
                Token::Identifier(n) => n,
                t => return Err(ParseError::UnexpectedToken {
                    expected: "type name".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            };
            
            // NO consumimos * aquí porque en patterns puede ser multiplicación
            Ok(TypeAnnotation::Name(name))
        }
    }

    /// Analiza una lista de parámetros dentro de paréntesis y devuelve `Vec<Param>`.
    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                let name = match self.advance()?.0 {
                    Token::Identifier(n) => n,
                    t => return Err(ParseError::UnexpectedToken {
                        expected: "parameter name".to_string(),
                        found: format!("{:?}", t),
                        pos: self.peek_pos(),
                    }),
                };

                let type_annotation = if self.match_token(&Token::Colon) {
                    Some(self.parse_type_annotation()?)
                } else {
                    None
                };

                params.push(Param { name, type_annotation });

                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        Ok(params)
    }

    // --- Parsing de Macros ---

    /// Analiza una declaración de macro: def name(params) => body
    fn parse_macro_decl(&mut self) -> Result<MacroDecl, ParseError> {
        // Consumir 'def' ya está hecho en parse_declaration
        
        // Nombre de la macro
        let name = match self.advance()?.0 {
            Token::Identifier(n) => n,
            t => return Err(ParseError::UnexpectedToken {
                expected: "macro name".to_string(),
                found: format!("{:?}", t),
                pos: self.peek_pos(),
            }),
        };

        // Parámetros
        self.consume(&Token::LParen, "Expected '(' after macro name")?;
        let params = self.parse_macro_params()?;
        self.consume(&Token::RParen, "Expected ')' after macro parameters")?;

        // Tipo de retorno opcional
        let return_type = if self.match_token(&Token::Colon) {
            Some(self.parse_type_annotation()?)
        } else {
            None
        };

        // Cuerpo de la macro: puede ser => expr o un bloque { ... }
        let body = if self.match_token(&Token::FuncArrow) {
            // Forma corta: => expr
            self.parse_spanned_expr(Precedence::Lowest)?
        } else {
            // Forma de bloque: { ... }
            let pos = self.peek_pos();  // Guardar posición antes de consumir {
            if !self.match_token(&Token::LBrace) {
                return Err(ParseError::UnexpectedToken {
                    expected: "'=>' or '{' before macro body".to_string(),
                    found: self.peek_description(),
                    pos: self.peek_pos(),
                });
            }
            self.parse_block_expr(pos)?
        };

        // Semicolon opcional
        self.match_token(&Token::Semicolon);

        Ok(MacroDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    /// Analiza parámetros de macro con prefijos especiales (@, $, *)
    fn parse_macro_params(&mut self) -> Result<Vec<MacroParam>, ParseError> {
        let mut params = Vec::new();

        if self.check(&Token::RParen) {
            return Ok(params);
        }

        loop {
            // Detectar prefijo
            let prefix = if self.match_token(&Token::Concat) { // @ usado para symbolic args
                Some("@")
            } else if self.match_token(&Token::Dollar) {
                Some("$")
            } else if self.match_token(&Token::Star) {
                Some("*")
            } else {
                None
            };

            // Nombre del parámetro
            let name = match self.advance()?.0 {
                Token::Identifier(n) => n,
                t => return Err(ParseError::UnexpectedToken {
                    expected: "parameter name".to_string(),
                    found: format!("{:?}", t),
                    pos: self.peek_pos(),
                }),
            };

            // Tipo (obligatorio)
            self.consume(&Token::Colon, "Expected ':' after parameter name")?;
            let type_annotation = self.parse_type_annotation()?;

            // Crear parámetro según prefijo
            let param = match prefix {
                Some("@") => MacroParam::Symbolic { name, type_annotation },
                Some("$") => MacroParam::Placeholder { name, type_annotation },
                Some("*") => MacroParam::Body { name, type_annotation },
                None => MacroParam::Normal { name, type_annotation },
                _ => unreachable!(),
            };

            params.push(param);

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(params)
    }

    /// Analiza una expresión match con pattern matching
    fn parse_match_expr(&mut self) -> Result<Expr, ParseError> {
        // Consumir 'match' ya está hecho
        self.consume(&Token::LParen, "Expected '(' after 'match'")?;
        
        let expr = Box::new(self.parse_spanned_expr(Precedence::Lowest)?);

        self.consume(&Token::RParen, "Expected ')' after match expression")?;
        self.consume(&Token::LBrace, "Expected '{' to start match body")?;

        // Parsear casos
        let mut cases = Vec::new();
        let mut default = None;

        while !self.check(&Token::RBrace) && !self.at_end() {
            if self.match_token(&Token::Case) {
                // case pattern => expr
                let pattern = self.parse_pattern()?;
                self.consume(&Token::FuncArrow, "Expected '=>' after case pattern")?;
                let case_expr = self.parse_spanned_expr(Precedence::Lowest)?;
                self.consume(&Token::Semicolon, "Expected ';' after case expression")?;

                cases.push(MatchCase {
                    pattern,
                    expr: case_expr,
                });
            } else if self.match_token(&Token::Default) {
                // default => expr
                self.consume(&Token::FuncArrow, "Expected '=>' after 'default'")?;
                let default_expr = Box::new(self.parse_spanned_expr(Precedence::Lowest)?);
                self.consume(&Token::Semicolon, "Expected ';' after default expression")?;

                default = Some(default_expr);
                break; // default debe ser el último
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "'case' or 'default'".to_string(),
                    found: self.peek_description(),
                    pos: self.peek_pos(),
                });
            }
        }

        self.consume(&Token::RBrace, "Expected '}' to close match body")?;

        Ok(Expr::Match {
            expr,
            cases,
            default,
        })
    }

    /// Analiza un patrón para pattern matching
    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        // Por simplicidad, empezar con paréntesis para patrones complejos
        if self.match_token(&Token::LParen) {
            let pattern = self.parse_pattern_expr()?;
            self.consume(&Token::RParen, "Expected ')' after pattern")?;
            Ok(pattern)
        } else {
            self.parse_pattern_expr()
        }
    }

    /// Analiza expresión de patrón (puede ser binaria, literal, variable, etc.)
    fn parse_pattern_expr(&mut self) -> Result<Pattern, ParseError> {
        // Intentar parsear como expresión binaria con recursión
        self.parse_pattern_binary(Precedence::Lowest)
    }

    /// Analiza patrón binario con precedencia
    fn parse_pattern_binary(&mut self, precedence: Precedence) -> Result<Pattern, ParseError> {
        let mut left = self.parse_pattern_primary()?;

        while !self.at_end() && precedence < self.peek_precedence() {
            let op = self.peek_binary_op();
            if op.is_none() {
                break;
            }

            let (_, _pos) = self.advance()?; // consumir operador
            let op = op.unwrap();

            let next_prec = self.peek_precedence();
            let right = self.parse_pattern_binary(next_prec)?;

            left = Pattern::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Analiza patrón primario (literal, variable, wildcard, unario)
    fn parse_pattern_primary(&mut self) -> Result<Pattern, ParseError> {
        let (token, pos) = self.advance()?;

        match token {
            // Literales
            Token::Number(n) => Ok(Pattern::Literal(Expr::Number(n))),
            Token::StringLiteral(s) => Ok(Pattern::Literal(Expr::String(s))),
            Token::True => Ok(Pattern::Literal(Expr::Boolean(true))),
            Token::False => Ok(Pattern::Literal(Expr::Boolean(false))),

            // Identifier: puede ser variable con tipo o sin tipo
            Token::Identifier(name) => {
                // Si hay :, es variable tipada
                let type_annotation = if self.match_token(&Token::Colon) {
                    // En patterns, no consumimos * después del tipo
                    // porque * puede ser parte del patrón binario (multiplicación)
                    Some(self.parse_type_annotation_no_star()?)
                } else {
                    None
                };

                Ok(Pattern::Variable {
                    name,
                    type_annotation,
                })
            }

            // Operador unario
            Token::Minus => {
                let operand = self.parse_pattern_primary()?;
                Ok(Pattern::Unary {
                    op: UnOp::Neg,
                    operand: Box::new(operand),
                })
            }

            Token::Not => {
                let operand = self.parse_pattern_primary()?;
                Ok(Pattern::Unary {
                    op: UnOp::Not,
                    operand: Box::new(operand),
                })
            }

            // Paréntesis
            Token::LParen => {
                let pattern = self.parse_pattern_expr()?;
                self.consume(&Token::RParen, "Expected ')' after pattern")?;
                Ok(pattern)
            }

            _ => Err(ParseError::UnexpectedToken {
                expected: "pattern (literal, variable, or unary)".to_string(),
                found: format!("{:?}", token),
                pos,
            }),
        }
    }

    /// Obtiene el operador binario del token actual (si existe)
    fn peek_binary_op(&self) -> Option<Op> {
        match self.peek() {
            Some(Ok((Token::Plus, _))) => Some(Op::Add),
            Some(Ok((Token::Minus, _))) => Some(Op::Sub),
            Some(Ok((Token::Star, _))) => Some(Op::Mul),
            Some(Ok((Token::Slash, _))) => Some(Op::Div),
            Some(Ok((Token::Percent, _))) => Some(Op::Mod),
            Some(Ok((Token::Power, _))) => Some(Op::Pow),
            Some(Ok((Token::Equal, _))) => Some(Op::Eq),
            Some(Ok((Token::NotEqual, _))) => Some(Op::Neq),
            Some(Ok((Token::LessThan, _))) => Some(Op::Lt),
            Some(Ok((Token::GreaterThan, _))) => Some(Op::Gt),
            Some(Ok((Token::LessThanEq, _))) => Some(Op::Le),
            Some(Ok((Token::GreaterThanEq, _))) => Some(Op::Ge),
            Some(Ok((Token::And, _))) => Some(Op::And),
            Some(Ok((Token::Or, _))) => Some(Op::Or),
            Some(Ok((Token::Concat, _))) => Some(Op::Concat),
            Some(Ok((Token::ConcatSpace, _))) => Some(Op::ConcatSpace),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
    Lowest,
    Assignment,
    Or,
    And,
    Comparison,
    Concat,
    Sum,
    Product,
    Unary,
    Power,
    Call,
}


#[cfg(test)]
mod tests;
