use super::{ast::{ASTConversionOp, AST}, lexer::{ExpressionError, Lexer, Token, TokenType}};

pub struct Parser {
    lexer: Lexer,
    current_token: Token
}

impl Parser {
    fn new(mut l: Lexer) -> Result<Self, ExpressionError> {
        let c_t = l.get_next_token()?; 
        Ok(Parser {
            lexer: l,
            current_token: c_t
        })
    }

    fn error(&self, m: String) -> ExpressionError {
        ExpressionError::SyntaxError(m)
    }

    fn eat(&mut self, token_type: TokenType) -> Result<(), ExpressionError> {
        if self.current_token.token_type == token_type {
            self.current_token = self.lexer.get_next_token()?
        } else {
            return Err(self.error(format!("Syntax error with {}", token_type)));
        }
        Ok(())
    }

    fn factor(&self) -> Result<AST, ExpressionError> {
        
    }

    fn term(&mut self) -> Result<AST, ExpressionError> {
        let mut node = self.factor()?;

        while vec![TokenType::Arrow].contains(&self.current_token.token_type) {
            self.eat(TokenType::Arrow)?;

            let right = self.factor()?;

            if let (AST::Value(value), AST::Unit(unit)) = (node, right) {
                node = AST::ConversionOp(ASTConversionOp {
                    left: Box::new(value),
                    right: unit
                });
            } else {
                return Err(self.error("Invalid conversion operation".to_string()));
            }
        }

        Ok(node)
    }

    fn expr() {
        todo!("For the future + - operations")
    }


}