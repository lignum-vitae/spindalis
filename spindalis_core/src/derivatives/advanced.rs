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
    let zeroed = remove_extra_vars(poly.expr.clone(), var);
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
fn remove_extra_vars<S>(expr: Expr, variable: S) -> Expr
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
            let lhs = remove_extra_vars(*lhs, var);
            let rhs = remove_extra_vars(*rhs, var);

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
                    if op == Operators::Mul
                        || op == Operators::Div
                        || ((op == Operators::Add || op == Operators::Sub) && paren)
                    {
                        // Handles 3x and 3/x
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
                    if op == Operators::Mul
                        || op == Operators::Div
                        || op == Operators::Caret
                        || ((op == Operators::Add || op == Operators::Sub) && paren)
                    {
                        // Handles 3xy, 3x/y, and 3xy^2
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
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
                (false, false) => {
                    if op == Operators::Caret || op == Operators::Add || op == Operators::Sub {
                        // Handles 3xy^2
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    } else {
                        Expr::Number(0.)
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn univariate_derivative() {
        let parsed = Polynomial::parse("3x + 2").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_derivative_2() {
        let parsed = Polynomial::parse("2 + 3x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn unary_prefix() {
        let parsed = Polynomial::parse("-3x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("-3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_derivative() {
        let parsed = Polynomial::parse("3xy + 2z").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3xy").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_derivative_2() {
        let parsed = Polynomial::parse("2z + 3xy").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3xy").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_with_exponent() {
        let parsed = Polynomial::parse("3x^2 + 3x - 3").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x^2 + 3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_with_exponent_2() {
        let parsed = Polynomial::parse("- 3 + 3x^2 + 3x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x^2 + 3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_with_exponent_3() {
        let parsed = Polynomial::parse("- 3 - 3x^2 + 3x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("-3x^2 + 3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_with_exponent() {
        let parsed = Polynomial::parse("3x^2y + 3xz - 3a + 2d^5").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x^2y + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_with_exponent_2() {
        let parsed = Polynomial::parse("3xy^2 + 3xz - 3a + 2d^5").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3xy^2 + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_with_exponent_3() {
        let parsed = Polynomial::parse("- 3a + 3x^2y + 2d^5 + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x^2y + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_with_exponent_4() {
        let parsed = Polynomial::parse("- 3a + 3xy^2 + 2d^5 + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3xy^2 + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_div_deriv() {
        let parsed = Polynomial::parse("3x/4 + 3x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x/4 + 3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_div_deriv_2() {
        let parsed = Polynomial::parse("3/x + 3x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3/x + 3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_div_deriv() {
        let parsed = Polynomial::parse("3x/y + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x/y + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_div_deriv_2() {
        let parsed = Polynomial::parse("3/x + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3/x + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_div_deriv_3() {
        let parsed = Polynomial::parse("3/xy + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3/xy + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_div_deriv() {
        let parsed = Polynomial::parse("(3+y)/xy + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("(3+y)/xy + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_div_deriv_2() {
        let parsed = Polynomial::parse("xy/(3+y) + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("xy/(3+y) + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_div_deriv_3() {
        let parsed = Polynomial::parse("xy/(3+xy) + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("xy/(3+xy) + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_div_deriv_4() {
        let parsed = Polynomial::parse("xy/(3x+y) + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("xy/(3x+y) + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_div_deriv_5() {
        let parsed = Polynomial::parse("(x+y)/(3+y) + 3xz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("(x+y)/(3+y) + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_div_deriv_6() {
        let parsed = Polynomial::parse("(3+y)/(x+y) + 3xz - 9yf").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("(3+y)/(x+y) + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_div_deriv_exponent() {
        let parsed = Polynomial::parse("xy/(3x+y)^2 + 3xz + 4hi").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("xy/(3x+y)^2 + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn multivariate_add_mul_deriv_exponent() {
        let parsed = Polynomial::parse("xy*(3x+y)^2 + 3xz - 3hy").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("xy*(3x+y)^2 + 3xz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }
}
