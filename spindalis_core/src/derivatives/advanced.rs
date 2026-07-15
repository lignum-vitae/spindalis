use crate::polynomials::PolynomialError;
use crate::polynomials::advanced::{
    Constants, Expr, Functions, Operators, fold_operations, walk_ast,
};
use crate::polynomials::structs::advanced::{AstOperation, PolyResult, Polynomial};

pub fn advanced_derivative<S>(poly: &Polynomial, variable: S) -> Result<Polynomial, PolynomialError>
where
    S: AsRef<str>,
{
    let var = variable.as_ref();
    let zeroed = remove_unused_vars(poly.expr.clone(), var);
    let filtered = fold_operations(zeroed);
    let Some(PolyResult::Expression(result)) = walk_ast(&filtered, &AstOperation::Derive) else {
        return Err(PolynomialError::MissingVariable);
    };
    Ok(Polynomial { expr: result })
    // TODO: Work on error messages
}

fn contains_var(expr: &Expr, var: &str) -> bool {
    match expr {
        Expr::Variable(v) => v == var,
        Expr::BinaryOp { lhs, rhs, .. } => contains_var(lhs, var) || contains_var(rhs, var),
        Expr::Function { inner, .. } => contains_var(inner, var),
        Expr::UnaryOpPostfix { value, .. } => contains_var(value, var),
        Expr::UnaryOpPrefix { value, .. } => contains_var(value, var),
        _ => false,
    }
}

// Needs to handle 4xyz; 3x +2; 3xy / 4; 3x; 3
fn remove_unused_vars<S>(expr: Expr, variable: S) -> Expr
where
    S: AsRef<str>,
{
    let var = variable.as_ref();
    match expr {
        Expr::Number(_)
        | Expr::Variable(_)
        | Expr::Constant(_)
        | Expr::Function { func: _, inner: _ }
        | Expr::UnaryOpPostfix { op: _, value: _ }
        | Expr::UnaryOpPrefix { op: _, value: _ } => expr,
        Expr::BinaryOp {
            op,
            lhs,
            rhs,
            paren,
        } => {
            let lhs = remove_unused_vars(*lhs, var);
            let rhs = remove_unused_vars(*rhs, var);

            let lhs_has_var = contains_var(&lhs, var);
            let rhs_has_var = contains_var(&rhs, var);

            match (lhs_has_var, rhs_has_var) {
                // x * x
                (true, true) => Expr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    paren,
                },
                // 2 + (3 * x)
                (false, true) => {
                    if op == Operators::Mul {
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    } else {
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(Expr::Number(0.)),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    }
                }
                // (3 * x) + 2
                (true, false) => {
                    if op == Operators::Mul {
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    } else if op == Operators::Div {
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(Expr::Number(1.)),
                            paren,
                        }
                    } else {
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(Expr::Number(0.)),
                            paren,
                        }
                    }
                }
                // 2 + 2
                (false, false) => Expr::Number(0.),
            }
        }
    }
}

pub(crate) fn derive_binary_operation(
    op: &Operators,
    lhs: &Expr,
    rhs: &Expr,
    paren: &bool,
) -> Option<PolyResult> {
    let Some(PolyResult::Expression(lhs)) = walk_ast(lhs, &AstOperation::Derive) else {
        return None;
    };
    let Some(PolyResult::Expression(rhs)) = walk_ast(rhs, &AstOperation::Derive) else {
        return None;
    };

    // match op <- use operator to determine action
    Some(PolyResult::Expression(Expr::BinaryOp {
        op: *op,
        lhs: Box::new(lhs.clone()),
        rhs: Box::new(rhs.clone()),
        paren: *paren,
    }))
}

pub(crate) fn derive_postfix_operation(op: &Operators, value: &Expr) -> Option<PolyResult> {
    Some(PolyResult::Expression(Expr::UnaryOpPostfix {
        op: *op,
        value: Box::new(value.clone()),
    }))
}

pub(crate) fn derive_prefix_operation(op: &Operators, value: &Expr) -> Option<PolyResult> {
    Some(PolyResult::Expression(Expr::UnaryOpPrefix {
        op: *op,
        value: Box::new(value.clone()),
    }))
}

pub(crate) fn derive_function(func: &Functions, value: &Expr) -> Option<PolyResult> {
    Some(PolyResult::Expression(Expr::Function {
        func: *func,
        inner: Box::new(value.clone()),
    }))
}

pub(crate) fn derive_constants(cnst: &Constants) -> Option<PolyResult> {
    Some(PolyResult::Expression(Expr::Constant(*cnst)))
}

pub(crate) fn derive_numbers(value: &f64) -> Option<PolyResult> {
    Some(PolyResult::Expression(Expr::Number(*value)))
}

pub(crate) fn derive_variables(value: &str) -> Option<PolyResult> {
    Some(PolyResult::Expression(Expr::Variable(value.to_string())))
}
