use crate::ast::ParseError;
use crate::{interpreter::InterpreterError, token::{LexErrorKind, LexError}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompileError {
    Lexer(LexError),
    Parser(ParseError),
    Interpret(InterpreterError),
}

impl From<LexError> for CompileError {
    fn from(e: LexError) -> Self {
        CompileError::Lexer(e)
    }
}
impl From<ParseError> for CompileError {
    fn from(e: ParseError) -> Self {
        CompileError::Parser(e)
    }
}


impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value { 
            LexErrorKind::InvalidChar(char) => {
              write!(f, "{}: invalid char '{}'", self.loc, char)
            },
            LexErrorKind::EOF => {
              write!(f, "End of file")
            }
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => {
              write!(f, "{}: {:?} is not expected token", token.loc, token.value)
            }
            ParseError::NotExpression(token) => {
              write!(f, "{}: {:?} is not an expression", token.loc, token.value)
            }
            ParseError::NotOperator(token) => {
              write!(f, "{}: {:?} is not an operator", token.loc, token.value)
            }
            ParseError::UnclosedOpenParen(token) => {
              write!(f, "{}: {:?} is unclosed", token.loc, token.value)
            }
            ParseError::RedundantExpression(token) => {
              write!(f, "{}: {:?} is redundant", token.loc, token.value)
            }
            ParseError::EOF => {
              write!(f, "End of file")
            }
        }
    }
}