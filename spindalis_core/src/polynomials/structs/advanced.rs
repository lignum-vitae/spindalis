use crate::derivatives::advanced::{
    derive_binary_operation, derive_constants, derive_function, derive_numbers,
    derive_postfix_operation, derive_prefix_operation, derive_variables,
};
use crate::polynomials::PolynomialError;
use crate::polynomials::advanced::{Constants, Expr, Functions, Operators, Token, eval_variables};
use crate::polynomials::advanced::{
    eval_advanced_polynomial, eval_binary_operation, eval_constants, eval_function, eval_numbers,
    eval_postfix_operation, eval_prefix_operation, extract_univariate_variable, lexer, parser,
    walk_ast,
};
use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenStream = Peekable<IntoIter<Token>>;

#[derive(Debug, PartialEq)]
pub struct Polynomial {
    pub expr: Expr,
}
impl Polynomial {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}
impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)
    }
}

pub enum PolyResult {
    Float(f64),
    Expression(Expr),
}

#[allow(dead_code)] // Remove after Derive and Integrate are implemented
pub(crate) enum AstOperation {
    Eval,
    Derive,
    Integrate,
}

impl AstOperation {
    pub fn handle_binary_operation(
        &self,
        op: &Operators,
        lhs: &Expr,
        rhs: &Expr,
        paren: &bool,
    ) -> Option<PolyResult> {
        match self {
            AstOperation::Eval => eval_binary_operation(op, lhs, rhs, paren),
            AstOperation::Derive => derive_binary_operation(op, lhs, rhs, paren),
            AstOperation::Integrate => todo!(),
        }
    }
    pub fn handle_constants(&self, cnst: &Constants) -> Option<PolyResult> {
        match self {
            AstOperation::Eval => eval_constants(cnst),
            AstOperation::Derive => derive_constants(cnst),
            AstOperation::Integrate => todo!(),
        }
    }
    pub fn handle_function(&self, func: &Functions, value: &Expr) -> Option<PolyResult> {
        match self {
            AstOperation::Eval => eval_function(func, value),
            AstOperation::Derive => derive_function(func, value),
            AstOperation::Integrate => todo!(),
        }
    }
    pub fn handle_postfix_operation(&self, op: &Operators, value: &Expr) -> Option<PolyResult> {
        match self {
            AstOperation::Eval => eval_postfix_operation(op, value),
            AstOperation::Derive => derive_postfix_operation(op, value),
            AstOperation::Integrate => todo!(),
        }
    }
    pub fn handle_prefix_operation(&self, op: &Operators, value: &Expr) -> Option<PolyResult> {
        match self {
            AstOperation::Eval => eval_prefix_operation(op, value),
            AstOperation::Derive => derive_prefix_operation(op, value),
            AstOperation::Integrate => todo!(),
        }
    }
    pub fn handle_numbers(&self, value: &f64) -> Option<PolyResult> {
        match self {
            AstOperation::Eval => eval_numbers(value),
            AstOperation::Derive => derive_numbers(value),
            AstOperation::Integrate => todo!(),
        }
    }

    pub fn handle_variables(&self, value: &String) -> Option<PolyResult> {
        match self {
            AstOperation::Eval => eval_variables(value),
            AstOperation::Derive => derive_variables(value),
            AstOperation::Integrate => todo!(),
        }
    }
}

impl Polynomial {
    pub fn parse(input: &str) -> Result<Polynomial, PolynomialError> {
        let tokens = lexer(input)?;
        parser(tokens)
    }

    pub fn eval_univariate<F>(&self, point: F) -> Result<f64, PolynomialError>
    where
        F: Into<f64> + std::clone::Clone + std::fmt::Debug,
    {
        let variable = match extract_univariate_variable(&self.expr) {
            Ok(var) => var,
            Err(PolynomialError::MissingVariable) => {
                if let Some(PolyResult::Float(result)) = walk_ast(&self.expr, &AstOperation::Eval) {
                    return Ok(result);
                } else {
                    return Err(PolynomialError::MissingVariable);
                };
            }
            Err(e @ PolynomialError::TooManyVariables { .. }) => return Err(e),
            Err(e) => return Err(e), // Catches any error not explicitly mentioned above
        };
        eval_advanced_polynomial(self, &[(variable, point)])
    }

    pub fn eval_multivariate<V, S, F>(&self, vars: &V) -> Result<f64, PolynomialError>
    where
        V: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
        S: AsRef<str>,
        F: Into<f64>,
    {
        let evaluated = eval_advanced_polynomial(self, vars)?;
        Ok(evaluated)
    }
}
