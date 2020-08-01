use crate::{
    ast::{Ast, AstKind, BinOperator, BinOperatorKind, UniOperator, UniOperatorKind},
    token::Annotation,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    DivisionByZero,
}
pub type InterpreterError = Annotation<InterpreterErrorKind>;
pub struct Interpreter<'a> {
    ast: &'a Ast,
}
type InterpreterResult = Result<i64, InterpreterError>;
impl<'a> Interpreter<'a> {
    pub fn new(ast: &'a Ast) -> Interpreter<'a> {
        Self { ast }
    }

    pub fn eval(&self) -> Result<i64, InterpreterError> {
        match &self.ast.value {
            AstKind::Number(num) => Ok(*num as i64),
            AstKind::UniOperator { operator, tree } => {
                let num = Interpreter::new(&tree).eval()?;
                Ok(self.eval_uni_operator(&operator, num))
            }
            AstKind::BinOperator { operator, lhs, rhs } => {
                let left = Interpreter::new(&lhs).eval()?;
                let right = Interpreter::new(&rhs).eval()?;
                self.eval_bin_operator(&operator, left, right)
                    .map_err(|error_kind| InterpreterError::new(error_kind, self.ast.loc.clone()))
            }
        }
    }

    fn eval_uni_operator(&self, uniop: &UniOperator, num: i64) -> i64 {
        match uniop.value {
            UniOperatorKind::Plus => num,
            UniOperatorKind::Minus => -num,
        }
    }
    fn eval_bin_operator(
        &self,
        binop: &BinOperator,
        left: i64,
        right: i64,
    ) -> Result<i64, InterpreterErrorKind> {
        match binop.value {
            BinOperatorKind::Add => Ok(left + right),
            BinOperatorKind::Sub => Ok(left - right),
            BinOperatorKind::Mul => Ok(left * right),
            BinOperatorKind::Div => {
                if right == 0 {
                    Err(InterpreterErrorKind::DivisionByZero)
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}

pub fn eval(ast: &Ast) -> InterpreterResult {
    Interpreter::new(ast).eval()
}
