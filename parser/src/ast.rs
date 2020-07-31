use crate::token::{Annotation, Loc, Token, TokenKind};
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
    Number(u64),
    UniOperator {
        operator: UniOperator,
        tree: Box<Ast>,
    },
    BinOperator {
        operator: BinOperator,
        lhs: Box<Ast>,
        rhs: Box<Ast>,
    },
}
pub type Ast = Annotation<AstKind>;
impl Ast {
    fn number(num: u64, loc: Loc) -> Self {
        Self::new(AstKind::Number(num), loc)
    }

    fn uni_operator(op: UniOperator, tree: Ast, loc: Loc) -> Self {
        Self::new(
            AstKind::UniOperator {
                operator: op,
                tree: Box::new(tree),
            },
            loc,
        )
    }

    fn bin_operator(op: BinOperator, lhs: Ast, rhs: Ast, loc: Loc) -> Self {
        Self::new(
            AstKind::BinOperator {
                operator: op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            loc,
        )
    }
}

macro_rules! operator_factory {
    ($name: ident, $operator: expr) => {
        pub fn $name(loc: Loc) -> Self {
            Self::new($operator, loc)
        }
    };
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniOperatorKind {
    Plus,
    Minus,
}
pub type UniOperator = Annotation<UniOperatorKind>;
impl UniOperator {
    operator_factory!(plus, UniOperatorKind::Plus);
    operator_factory!(minus, UniOperatorKind::Minus);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOperatorKind {
    Add,
    Sub,
    Mul,
    Div,
}
pub type BinOperator = Annotation<BinOperatorKind>;
impl BinOperator {
    operator_factory!(add, BinOperatorKind::Add);
    operator_factory!(sub, BinOperatorKind::Sub);
    operator_factory!(mul, BinOperatorKind::Mul);
    operator_factory!(div, BinOperatorKind::Div);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnexpectedToken(Token),
    NotExpression(Token),
    NotOperator(Token),
    UnclosedOpenParen(Token),
    RedundantExpression(Token),
    EOF,
}

pub struct Tokens(Vec<Token>);

pub type ParseResult = Result<Ast, ParseError>;

impl Tokens {
    pub fn parse(tokens: Vec<Token>) -> ParseResult {
        let mut tokens = tokens.into_iter().peekable();
        let result = Self::parse_expr(&mut tokens);
        match tokens.next() {
            Some(token) => Err(ParseError::RedundantExpression(token)),
            None => result,
        }
    }

    // expr = add;
    fn parse_expr<T>(tokens: &mut Peekable<T>) -> ParseResult
    where
        T: Iterator<Item = Token>,
    {
        Self::parse_add(tokens)
    }

    // add = mul ("+" mul | "-" mul)* ;
    fn parse_add<T>(tokens: &mut Peekable<T>) -> ParseResult
    where
        T: Iterator<Item = Token>,
    {
        let mut ast = Self::parse_mul(tokens)?;
        loop {
            match tokens.peek().map(|token| &token.value) {
                Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
                    let operator = match tokens.next().unwrap() {
                        Token {
                            value: TokenKind::Plus,
                            loc,
                        } => BinOperator::add(loc),
                        Token {
                            value: TokenKind::Minus,
                            loc,
                        } => BinOperator::sub(loc),
                        _ => unreachable!(),
                    };
                    let rhs = Self::parse_mul(tokens)?;
                    let location = ast.loc.merge(&rhs.loc);
                    ast = Ast::bin_operator(operator, ast, rhs, location);
                }
                _ => return Ok(ast),
            }
        }
    }

    // mul = unary ("*" unary | "/" unary)* ;
    fn parse_mul<T>(tokens: &mut Peekable<T>) -> ParseResult
    where
        T: Iterator<Item = Token>,
    {
        let mut ast = Self::parse_unary(tokens)?;
        loop {
            match tokens.peek().map(|token| &token.value) {
                Some(TokenKind::Asterisk) | Some(TokenKind::Slash) => {
                    let operator = match tokens.next().unwrap() {
                        Token {
                            value: TokenKind::Asterisk,
                            loc,
                        } => BinOperator::mul(loc),
                        Token {
                            value: TokenKind::Slash,
                            loc,
                        } => BinOperator::div(loc),
                        _ => unreachable!(),
                    };
                    let rhs = Self::parse_unary(tokens)?;
                    let location = ast.loc.merge(&rhs.loc);
                    ast = Ast::bin_operator(operator, ast, rhs, location);
                }
                _ => return Ok(ast),
            }
        }
    }

    // unary = ("+" | "-")? atom ;
    fn parse_unary<T>(tokens: &mut Peekable<T>) -> ParseResult
    where
        T: Iterator<Item = Token>,
    {
        match tokens.peek().map(|token| &token.value) {
            Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
                let operator = match tokens.next().unwrap() {
                    Token {
                        value: TokenKind::Plus,
                        loc,
                    } => UniOperator::plus(loc),
                    Token {
                        value: TokenKind::Minus,
                        loc,
                    } => UniOperator::minus(loc),
                    _ => unreachable!(),
                };
                let ast = Self::parse_atom(tokens)?;
                let location = ast.loc.merge(&ast.loc);
                Ok(Ast::uni_operator(operator, ast, location))
            }
            _ => Self::parse_atom(tokens),
        }
    }

    // atom = number | "(" add ")" ;
    // number = digit+
    // digit = "0" | ... | "9" ;
    fn parse_atom<T>(tokens: &mut Peekable<T>) -> ParseResult
    where
        T: Iterator<Item = Token>,
    {
      tokens.next()
            .ok_or(ParseError::EOF)
            .and_then(|token| {
              match token.value {
                TokenKind::Number(num) => {
                  Ok(Ast::number(num, token.loc))
                },
                TokenKind::LParen => {
                  let add = Self::parse_add(tokens)?;
                  match tokens.next().map(|token| token.value) {
                    Some(TokenKind::RParen) => Ok(add),
                    Some(_) => Err(ParseError::RedundantExpression(token)),
                    _ => Err(ParseError::UnclosedOpenParen(token))
                  }
                },
                _ => Err(ParseError::UnexpectedToken(token)),
              }
            })
    }
}
