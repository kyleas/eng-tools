use crate::error::{EquationError, Result};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Var(String),
    Unary {
        op: char,
        rhs: Box<Expr>,
    },
    Binary {
        op: char,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
}

pub fn parse_expression(input: &str) -> Result<Expr> {
    let tokens = tokenize(input)?;
    let mut p = Parser {
        input,
        tokens,
        pos: 0,
    };
    let expr = p.parse_expr(0)?;
    if p.pos != p.tokens.len() {
        return Err(EquationError::ExpressionParse {
            expression: input.to_string(),
            message: "unexpected trailing tokens".to_string(),
        });
    }
    Ok(expr)
}

fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        let t = match c {
            '+' => {
                i += 1;
                Token::Plus
            }
            '-' => {
                i += 1;
                Token::Minus
            }
            '*' => {
                i += 1;
                Token::Star
            }
            '/' => {
                i += 1;
                Token::Slash
            }
            '^' => {
                i += 1;
                Token::Caret
            }
            '(' => {
                i += 1;
                Token::LParen
            }
            ')' => {
                i += 1;
                Token::RParen
            }
            ',' => {
                i += 1;
                Token::Comma
            }
            d if d.is_ascii_digit() || d == '.' => {
                let start = i;
                i += 1;
                while i < chars.len() {
                    let ch = chars[i];
                    let prev = chars[i - 1];
                    let keep = ch.is_ascii_digit()
                        || ch == '.'
                        || ch == 'e'
                        || ch == 'E'
                        || ((ch == '+' || ch == '-') && (prev == 'e' || prev == 'E'));
                    if !keep {
                        break;
                    }
                    i += 1;
                }
                let raw: String = chars[start..i].iter().collect();
                let parsed = raw
                    .parse::<f64>()
                    .map_err(|_| EquationError::ExpressionParse {
                        expression: input.to_string(),
                        message: format!("invalid number '{raw}'"),
                    })?;
                Token::Number(parsed)
            }
            a if a.is_ascii_alphabetic() || a == '_' => {
                let start = i;
                i += 1;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let ident: String = chars[start..i].iter().collect();
                Token::Ident(ident)
            }
            _ => {
                return Err(EquationError::ExpressionParse {
                    expression: input.to_string(),
                    message: format!("unexpected character '{c}'"),
                });
            }
        };
        tokens.push(t);
    }
    Ok(tokens)
}

struct Parser<'a> {
    input: &'a str,
    tokens: Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn parse_expr(&mut self, min_prec: u8) -> Result<Expr> {
        let mut lhs = self.parse_prefix()?;
        loop {
            let Some((op, prec, right_assoc)) = self.peek_infix() else {
                break;
            };
            if prec < min_prec {
                break;
            }
            self.pos += 1;
            let next_prec = if right_assoc { prec } else { prec + 1 };
            let rhs = self.parse_expr(next_prec)?;
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_prefix(&mut self) -> Result<Expr> {
        let token = self
            .tokens
            .get(self.pos)
            .ok_or_else(|| EquationError::ExpressionParse {
                expression: self.input.to_string(),
                message: "unexpected end of expression".to_string(),
            })?;

        match token {
            Token::Number(n) => {
                self.pos += 1;
                Ok(Expr::Number(*n))
            }
            Token::Minus => {
                self.pos += 1;
                let rhs = self.parse_expr(4)?;
                Ok(Expr::Unary {
                    op: '-',
                    rhs: Box::new(rhs),
                })
            }
            Token::Plus => {
                self.pos += 1;
                self.parse_expr(4)
            }
            Token::Ident(name) => {
                let ident = name.clone();
                self.pos += 1;
                if matches!(self.tokens.get(self.pos), Some(Token::LParen)) {
                    self.pos += 1;
                    let mut args = Vec::new();
                    if !matches!(self.tokens.get(self.pos), Some(Token::RParen)) {
                        loop {
                            args.push(self.parse_expr(0)?);
                            if matches!(self.tokens.get(self.pos), Some(Token::Comma)) {
                                self.pos += 1;
                                continue;
                            }
                            break;
                        }
                    }
                    match self.tokens.get(self.pos) {
                        Some(Token::RParen) => self.pos += 1,
                        _ => {
                            return Err(EquationError::ExpressionParse {
                                expression: self.input.to_string(),
                                message: "missing ')' in function call".to_string(),
                            });
                        }
                    }
                    Ok(Expr::Call { name: ident, args })
                } else {
                    Ok(Expr::Var(ident))
                }
            }
            Token::LParen => {
                self.pos += 1;
                let expr = self.parse_expr(0)?;
                match self.tokens.get(self.pos) {
                    Some(Token::RParen) => {
                        self.pos += 1;
                        Ok(expr)
                    }
                    _ => Err(EquationError::ExpressionParse {
                        expression: self.input.to_string(),
                        message: "missing ')'".to_string(),
                    }),
                }
            }
            _ => Err(EquationError::ExpressionParse {
                expression: self.input.to_string(),
                message: "invalid token in expression".to_string(),
            }),
        }
    }

    fn peek_infix(&self) -> Option<(char, u8, bool)> {
        match self.tokens.get(self.pos)? {
            Token::Plus => Some(('+', 1, false)),
            Token::Minus => Some(('-', 1, false)),
            Token::Star => Some(('*', 2, false)),
            Token::Slash => Some(('/', 2, false)),
            Token::Caret => Some(('^', 3, true)),
            _ => None,
        }
    }
}
