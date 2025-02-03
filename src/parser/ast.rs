use crate::units::main::Unit;

use crate::parser::lexer::Token;

pub enum AST {
    Value(ASTValue),
    Unit(ASTUnit),
    ConversionOp(ASTConversionOp)
}

pub struct ASTValue {
    token: Token,
    value: f64,
    unit: Box<ASTUnit>,
}

pub struct ASTUnit {
    token: Token,
    unit: Unit
}

pub struct ASTConversionOp {
    left: Box<ASTValue>,
    right: Box<ASTUnit>
}