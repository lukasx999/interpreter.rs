use crate::lexer::{TokenType, Token};


/*
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

// impl AstNode {
//     pub fn interpret_thyself(&self) -> () {
//         match self {
//             Self::Literal { token } => self.eval_literal(),
//             Self::ExprBinaryOp { left, operator, right } => {}
//         }
//     }
//     fn eval_literal(value: Self::Literal) -> () {
//     }
// }
*/


// /*
pub trait Evaluatable {
    fn eval(&self) -> i32;
}

struct Literal {
    token: Token,
}

impl Evaluatable for Literal {
    fn eval(&self) -> i32 {
        match self.token.kind {
            TokenType::LiteralInteger(value) => value,
            _ => {0}
        }
    }
}

struct ExprBinaryOp {
    left:     AstNode,
    operator: Token,
    right:    AstNode,
}

impl Evaluatable for ExprBinaryOp {
    fn eval(&self) -> i32 {
        match self.operator.kind {
            TokenType::Plus     => self.left.node.eval() + self.right.node.eval(),
            TokenType::Minus    => self.left.node.eval() - self.right.node.eval(),
            _ => {0}
        }
    }
}

#[derive(Debug)]
pub enum AstNodeType {
    Literal,
    ExprBinaryOp,
}

pub struct AstNode {
    pub tag: AstNodeType,
    pub node: Box<dyn Evaluatable>,
}







// */






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

    pub fn parse(&mut self) -> AstNode {
        self.prod_term()
    }

    fn get_current_token(&self) -> &Token {
        &self.tokenlist[self.position]
    }




    fn prod_primary(&mut self) -> AstNode {

        let new = AstNode {
            tag: AstNodeType::Literal,
            node: Box::new(Literal { token: self.get_current_token().clone() }),
        };

        self.position += 1;
        new

    }

    fn prod_term(&mut self) -> AstNode {
        // term -> primary ( ( "-" | "+" ) primary )*

        let mut expr: AstNode = self.prod_primary();

        while let
        TokenType::Plus | TokenType::Minus = self.get_current_token().kind {
            let operator: Token = self.get_current_token().clone();
            // self.position turns into slice reference in self.get_current_token() and therefore
            // cannot be reassigned (unless cloned)
            self.position += 1;
            let rhs = self.prod_primary();

            expr = AstNode {
                tag: AstNodeType::ExprBinaryOp,
                node: Box::new(
                    ExprBinaryOp {
                        left: expr,
                        operator,
                        right: rhs,
                    }
                ),
            };
        }

        expr

    }






}
