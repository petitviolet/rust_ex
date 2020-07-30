use crate::token::{Annotation, Loc, Token};

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
enum ParseError {
    UnexpectedToken(Token),
    NotExpression(Token),
    NotOperator(Token),
    UnclosedOpenParen(Token),
    RedundantExpression(Token),
    EOF,
}
