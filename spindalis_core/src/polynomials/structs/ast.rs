use crate::polynomials::ast::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenStream = Peekable<IntoIter<Token>>;
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Ast {
    cval: Token,            // Value of the node
    lval: Option<Box<Ast>>, // left node of AST
    rval: Option<Box<Ast>>, // right node of AST
}

impl Ast {
    pub(crate) fn new(cval: Token, lval: Option<Ast>, rval: Option<Ast>) -> Self {
        return Self {
            cval,
            lval: lval.map(|l| Box::new(l)),
            rval: rval.map(|r| Box::new(r)),
        };
    }
}
