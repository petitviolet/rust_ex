use crate::ast::ParseError;
use crate::{interpreter::InterpreterError, token::LexError};

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
