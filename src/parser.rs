use crate::lexer::{TokenType, Token};


#[derive(Debug, Clone)]
pub enum AstNode {
    Literal {
        token: Token,
    },
    ExprBinaryOp {
        left:     Box<AstNode>,
        operator: Token,
        right:    Box<AstNode>,
    },

}





#[derive(Debug)]
pub struct Parser {
    tokenlist: Vec<Token>,
    position: usize,
}



impl Parser {
    pub fn new(tokenlist: Vec<Token>) -> Self {
        Self {
            tokenlist,
            position: 0usize,
        }
    }

    pub fn parse(&mut self) -> Box<AstNode> {
        self.prod_term()
    }

    fn get_current_token(&self) -> &Token {
        &self.tokenlist[self.position]
    }




    fn prod_primary(&mut self) -> Box<AstNode> {

        self.position += 1;
        Box::new(AstNode::Literal{ token: self.get_current_token().clone() })

    }

    fn prod_term(&mut self) -> Box<AstNode> {
        // term -> primary ( ( "-" | "+" ) primary )*

        let mut expr = self.prod_primary();

        while let
        TokenType::Plus | TokenType::Minus = self.get_current_token().kind {
            let operator = self.get_current_token().clone();
            // let mut operator = self.get_current_token(); // TODO: why does this not work?

            self.position += 1;
            let rhs = self.prod_primary();

            expr = Box::new(AstNode::ExprBinaryOp{
                left: expr,
                operator,
                right: rhs,
            });
        }

        expr

    }






}
