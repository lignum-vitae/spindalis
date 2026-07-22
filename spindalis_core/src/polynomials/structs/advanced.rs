use crate::polynomials::PolynomialError;
use crate::polynomials::advanced::{Constants, Evaluator, Functions, Operators, Token};
use crate::polynomials::advanced::{
    eval_advanced_polynomial, extract_univariate_variable, lexer, parser,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Constant(Constants),
    Function {
        func: Functions,
        inner: Box<Self>,
    },
    UnaryOpPrefix {
        op: Operators,
        value: Box<Self>,
    },
    UnaryOpPostfix {
        op: Operators,
        value: Box<Self>,
    },
    BinaryOp {
        op: Operators,
        lhs: Box<Self>,
        rhs: Box<Self>,
        paren: bool,
    },
}

pub trait Visitor {
    type Output;

    fn visit_binary_op(&self, op: &Operators, lhs: &Expr, rhs: &Expr, paren: bool) -> Self::Output;
    fn visit_unary_prefix(&self, op: &Operators, value: &Expr) -> Self::Output;
    fn visit_unary_postfix(&self, op: &Operators, value: &Expr) -> Self::Output;
    fn visit_function(&self, func: &Functions, inner: &Expr) -> Self::Output;
    fn visit_constant(&self, c: &Constants) -> Self::Output;
    fn visit_number(&self, val: f64) -> Self::Output;
    fn visit_variable(&self, var: &str) -> Self::Output;
}

impl Expr {
    pub fn map(
        &self,
        f: &mut impl FnMut(&Expr) -> Result<Expr, PolynomialError>,
    ) -> Result<Expr, PolynomialError> {
        match self {
            // recursively walks down the polynomial for operators
            Expr::BinaryOp {
                op,
                lhs,
                rhs,
                paren,
            } => Ok(Expr::BinaryOp {
                op: *op,
                lhs: Box::new(lhs.map(f)?),
                rhs: Box::new(rhs.map(f)?),
                paren: *paren,
            }),
            Expr::UnaryOpPrefix { op, value } => Ok(Expr::UnaryOpPrefix {
                op: *op,
                value: Box::new(value.map(f)?),
            }),
            Expr::UnaryOpPostfix { op, value } => Ok(Expr::UnaryOpPostfix {
                op: *op,
                value: Box::new(value.map(f)?),
            }),
            Expr::Function { func, inner } => Ok(Expr::Function {
                func: *func,
                inner: Box::new(inner.map(f)?),
            }),
            // This allows for extension of variants with f.
            x => f(x),
        }
    }
    pub fn visit(&self, f: &mut impl FnMut(&Expr)) {
        f(self);
        match self {
            Expr::BinaryOp { lhs, rhs, .. } => {
                lhs.visit(f);
                rhs.visit(f);
            }
            Expr::UnaryOpPrefix { value, .. }
            | Expr::UnaryOpPostfix { value, .. }
            | Expr::Function { inner: value, .. } => {
                value.visit(f);
            }
            _ => {}
        }
    }

    pub fn accept<V: Visitor>(&self, visitor: &V) -> V::Output {
        match self {
            Expr::BinaryOp {
                op,
                lhs,
                rhs,
                paren,
            } => visitor.visit_binary_op(op, lhs, rhs, *paren),
            Expr::UnaryOpPrefix { op, value } => visitor.visit_unary_prefix(op, value),
            Expr::UnaryOpPostfix { op, value } => visitor.visit_unary_postfix(op, value),
            Expr::Function { func, inner } => visitor.visit_function(func, inner),
            Expr::Constant(c) => visitor.visit_constant(c),
            Expr::Number(val) => visitor.visit_number(*val),
            Expr::Variable(var) => visitor.visit_variable(var),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Variable(v) => write!(f, "{v}"),
            Self::Constant(c) => write!(f, "{c}"),
            Self::Function { func, inner } => write!(f, "{func}({inner})"),
            Self::UnaryOpPrefix { op, value } => write!(f, "{op}{value}"),
            Self::UnaryOpPostfix { op, value } => write!(f, "{value}{op}"),
            Self::BinaryOp {
                op,
                lhs,
                rhs,
                paren,
            } => {
                // 4 * x   -> 4x
                // 5 * x^2 -> 5x^2
                // 2 * π   -> 2π
                let mut implied: Option<String> = None;

                if *op == Operators::Mul {
                    implied = match (&**lhs, &**rhs) {
                        (Self::Number(n), Self::Variable(v)) => Some(format!("{n}{v}")),
                        (Self::Number(n), Self::Constant(c)) => Some(format!("{n}{c}")),
                        (
                            Self::Number(n),
                            Self::BinaryOp {
                                op: Operators::Caret,
                                ..
                            },
                        ) => Some(format!("{n}{rhs}")),
                        (Self::Variable(v), Self::Number(n)) => Some(format!("{v}{n}")),
                        (Self::Constant(c), Self::Number(n)) => Some(format!("{c}{n}")),
                        (Self::Variable(v1), Self::Variable(v2)) => Some(format!("{v1}{v2}")),
                        _ => None,
                    };
                } else if *op == Operators::Caret {
                    let wrap_paren = |node: &Self| {
                        if *paren {
                            format!("({node})")
                        } else {
                            node.to_string()
                        }
                    };

                    let (l, r) = (&**lhs, &**rhs);
                    implied = Some(format!("{}^{r}", wrap_paren(l)));

                    if let Some(s) = implied {
                        return write!(f, "{s}");
                    }
                } else if *op == Operators::Div {
                    implied = match (&**lhs, &**rhs) {
                        (l, r @ Expr::BinaryOp { op: rhs_op, .. })
                            if *rhs_op != Operators::Caret =>
                        {
                            Some(format!("{l} / ({r})"))
                        }
                        (l, r) => Some(format!("{l} / {r}")),
                    };
                }

                if let Some(s) = implied {
                    if *paren {
                        return write!(f, "({s})");
                    }
                    return write!(f, "{s}");
                }

                if *paren {
                    write!(f, "({lhs} {op} {rhs})")
                } else {
                    write!(f, "{lhs} {op} {rhs}")
                }
            }
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
                let eval = Evaluator;
                let Some(result) = &self.expr.accept(&eval) else {
                    return Err(PolynomialError::MissingVariable);
                };
                return Ok(*result);
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
