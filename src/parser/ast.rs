use crate::units::main::Unit;

use crate::parser::lexer::Token;

#[derive(Debug, Clone)]
pub enum AST {
    Value(ASTValue),
    Unit(ASTUnit),
    ConversionOp(ASTConversionOp)
}

#[derive(Debug, Clone)]
pub struct ASTValue {
    pub token: Token,
    pub value: f64,
    pub unit: ASTUnit,
}

#[derive(Debug, Clone)]
pub struct ASTUnit {
    pub token: Token,
    pub unit: Unit
}

#[derive(Debug, Clone)]
pub struct ASTConversionOp {
    pub left: Box<ASTValue>,
    pub right: ASTUnit
}