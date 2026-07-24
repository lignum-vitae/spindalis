use crate::polynomials::PolynomialError;
use crate::polynomials::advanced::{Constants, Functions, Operators, fold_operations};
use crate::polynomials::structs::advanced::{Expr, Polynomial, Visitor};

/* Main derivative function */
pub fn advanced_derivative<S>(poly: &Polynomial, variable: S) -> Result<Polynomial, PolynomialError>
where
    S: AsRef<str>,
{
    let var = variable.as_ref();
    let zeroed = remove_extra_vars(poly.expr.clone(), var);
    let folded = fold_operations(zeroed);
    let filtered = clear_num_var_literal(folded, var);

    let derive = Differentiator { var };
    let Some(result) = filtered.accept(&derive) else {
        return Err(PolynomialError::MissingVariable);
    };
    Ok(Polynomial { expr: result })
    // TODO: Work on error messages
}

/* Input cleanup functions */
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
                        if let Expr::BinaryOp {
                            op:
                                Operators::TempAdd
                                | Operators::TempSub
                                | Operators::TempMul
                                | Operators::TempDiv,
                            ..
                        } = lhs
                        {
                            // Handles 3+y/x
                            replace_temp_operations(Expr::BinaryOp {
                                op,
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                                paren,
                            })
                        } else {
                            // Handles 3x and 3/x
                            Expr::BinaryOp {
                                op,
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                                paren,
                            }
                        }
                    } else if op == Operators::Caret {
                        // Handles e^x and 3^x
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
                        if let Expr::BinaryOp {
                            op:
                                Operators::TempAdd
                                | Operators::TempSub
                                | Operators::TempMul
                                | Operators::TempDiv,
                            ..
                        } = rhs
                        {
                            // Handles x/3+y
                            replace_temp_operations(Expr::BinaryOp {
                                op,
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                                paren,
                            })
                        } else {
                            // Handles 3xy, 3x/y, and 3xy^2
                            Expr::BinaryOp {
                                op,
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                                paren,
                            }
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
                    if op == Operators::Caret {
                        // Handles 3xy^2
                        Expr::BinaryOp {
                            op,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    } else if op == Operators::Add {
                        Expr::BinaryOp {
                            op: Operators::TempAdd,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    } else if op == Operators::Sub {
                        Expr::BinaryOp {
                            op: Operators::TempSub,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    } else if op == Operators::Mul {
                        Expr::BinaryOp {
                            op: Operators::TempMul,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                            paren,
                        }
                    } else if op == Operators::Div {
                        Expr::BinaryOp {
                            op: Operators::TempDiv,
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

fn replace_temp_operations(expr: Expr) -> Expr {
    match expr {
        Expr::Number(_) | Expr::Variable(_) | Expr::Constant(_) => expr,
        Expr::BinaryOp {
            op,
            lhs,
            rhs,
            paren,
        } => {
            let lhs = replace_temp_operations(*lhs);
            let rhs = replace_temp_operations(*rhs);
            match op {
                Operators::TempAdd => Expr::BinaryOp {
                    op: Operators::Add,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    paren,
                },
                Operators::TempSub => Expr::BinaryOp {
                    op: Operators::Sub,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    paren,
                },
                Operators::TempMul => Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    paren,
                },
                Operators::TempDiv => Expr::BinaryOp {
                    op: Operators::Div,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    paren,
                },
                _ => Expr::BinaryOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    paren,
                },
            }
        }
        expr => expr,
    }
}

fn clear_num_var_literal(expr: Expr, var: &str) -> Expr {
    match &expr {
        Expr::Number(..) => Expr::Number(0.),
        Expr::Variable(v) if v != var => Expr::Number(0.),
        _ => expr,
    }
}

pub(crate) struct Differentiator<'a> {
    var: &'a str,
}

/* Actual derivative implementation */
impl<'a> Visitor for Differentiator<'a> {
    type Output = Option<Expr>;

    fn visit_binary_op(&self, op: &Operators, lhs: &Expr, rhs: &Expr, paren: bool) -> Option<Expr> {
        let derived_lhs = lhs.accept(self)?;
        let derived_rhs = rhs.accept(self)?;

        let op = *op;
        match op {
            Operators::Add | Operators::Sub => {
                let res = Expr::BinaryOp {
                    op,
                    lhs: Box::new(derived_lhs),
                    rhs: Box::new(derived_rhs),
                    paren,
                };
                Some(fold_operations(res))
            }
            Operators::Mul => {
                if *lhs == Expr::Constant(Constants::E)
                    || matches!(lhs, Expr::BinaryOp { op: Operators::Caret, lhs: l, .. } if **l == Expr::Constant(Constants::E))
                {
                    return Some(Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: Box::new(lhs.clone()),
                        rhs: Box::new(derived_rhs),
                        paren: false,
                    });
                }
                if *rhs == Expr::Constant(Constants::E)
                    || matches!(rhs, Expr::BinaryOp { op: Operators::Caret, lhs: l, .. } if **l == Expr::Constant(Constants::E))
                {
                    return Some(Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: Box::new(derived_lhs),
                        rhs: Box::new(rhs.clone()),
                        paren: false,
                    });
                }
                let left = Expr::BinaryOp {
                    op,
                    lhs: Box::new(lhs.clone()),
                    rhs: Box::new(derived_rhs),
                    paren,
                };
                let right = Expr::BinaryOp {
                    op,
                    lhs: Box::new(derived_lhs),
                    rhs: Box::new(rhs.clone()),
                    paren,
                };

                let product_rule = Expr::BinaryOp {
                    op: Operators::Add,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                    paren,
                };
                Some(fold_operations(product_rule))
            }
            Operators::Div => {
                let left = Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(derived_lhs),
                    rhs: Box::new(rhs.clone()),
                    paren: true,
                };
                let right = Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(lhs.clone()),
                    rhs: Box::new(derived_rhs),
                    paren,
                };
                let numerator = Expr::BinaryOp {
                    op: Operators::Sub,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                    paren,
                };

                let new_paren = matches!(rhs, Expr::BinaryOp { .. });
                let quotient_rule = Expr::BinaryOp {
                    op: Operators::Div,
                    lhs: Box::new(numerator),
                    rhs: Box::new(Expr::BinaryOp {
                        op: Operators::Caret,
                        lhs: Box::new(rhs.clone()),
                        rhs: Box::new(Expr::Number(2.)),
                        paren: new_paren,
                    }),
                    paren,
                };
                Some(fold_operations(quotient_rule))
            }
            Operators::Caret => {
                let new_exp;
                let exp = rhs;

                if *lhs == Expr::Constant(Constants::E) {
                    return Some(Expr::BinaryOp {
                        op: Operators::Caret,
                        lhs: Box::new(lhs.clone()),
                        rhs: Box::new(exp.clone()),
                        paren,
                    });
                }

                let variable: String = self.var.to_string();
                if *exp == Expr::Variable(variable) {
                    return Some(Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: Box::new(Expr::Function {
                            func: Functions::Ln,
                            inner: Box::new(lhs.clone()),
                        }),
                        rhs: Box::new(Expr::BinaryOp {
                            op: Operators::Caret,
                            lhs: Box::new(lhs.clone()),
                            rhs: Box::new(exp.clone()),
                            paren: false,
                        }),
                        paren: false,
                    });
                }
                if let Expr::Number(val) = exp {
                    new_exp = Expr::Number(val - 1.);
                } else {
                    new_exp = Expr::BinaryOp {
                        op: Operators::Sub,
                        lhs: Box::new(exp.clone()),
                        rhs: Box::new(Expr::Number(1.)),
                        paren: true,
                    }
                };
                let base_pow = Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(lhs.clone()),
                    rhs: Box::new(new_exp),
                    paren: true,
                };
                let power_rule = Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::BinaryOp {
                        op: Operators::Mul,
                        lhs: Box::new(exp.clone()),
                        rhs: Box::new(base_pow),
                        paren: false,
                    }),
                    rhs: Box::new(derived_lhs),
                    paren,
                };
                Some(fold_operations(power_rule))
            }
            _ => Some(Expr::BinaryOp {
                op,
                lhs: Box::new(lhs.clone()),
                rhs: Box::new(rhs.clone()),
                paren,
            }),
        }
    }

    fn visit_function(&self, func: &Functions, value: &Expr) -> Option<Expr> {
        let mut res = match func {
            Functions::Sin => Expr::Function {
                func: Functions::Cos,
                inner: Box::new(value.clone()),
            },
            Functions::Cos => Expr::UnaryOpPrefix {
                op: Operators::Sub,
                value: Box::new(Expr::Function {
                    func: Functions::Sin,
                    inner: Box::new(value.clone()),
                }),
            },
            Functions::Tan => Expr::BinaryOp {
                op: Operators::Caret,
                lhs: Box::new(Expr::Function {
                    func: Functions::Sec,
                    inner: Box::new(value.clone()),
                }),
                rhs: Box::new(Expr::Number(2.)),
                paren: false,
            },
            Functions::Cot => Expr::UnaryOpPrefix {
                op: Operators::Sub,
                value: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(Expr::Function {
                        func: Functions::Csc,
                        inner: Box::new(value.clone()),
                    }),
                    rhs: Box::new(Expr::Number(2.)),
                    paren: false,
                }),
            },
            Functions::Sec => Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Function {
                    func: Functions::Sec,
                    inner: Box::new(value.clone()),
                }),
                rhs: Box::new(Expr::Function {
                    func: Functions::Tan,
                    inner: Box::new(value.clone()),
                }),
                paren: false,
            },
            Functions::Csc => Expr::UnaryOpPrefix {
                op: Operators::Sub,
                value: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(Expr::Function {
                        func: Functions::Csc,
                        inner: Box::new(value.clone()),
                    }),
                    rhs: Box::new(Expr::Function {
                        func: Functions::Cot,
                        inner: Box::new(value.clone()),
                    }),
                    paren: false,
                }),
            },
            /* Log with no arguments is Log_e(var) not Log_10(var)
            Functions::Log => Expr::BinaryOp {
                op: Operators::Div,
                lhs: Box::new(Expr::Number(1.)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Mul,
                    lhs: Box::new(value.clone()),
                    rhs: Box::new(Expr::Function {
                        func: Functions::Ln,
                        inner: Box::new(Expr::Number(BASE OF LOG)),
                    }),
                    paren: true,
                }),
                paren: false,
            },
            */
            Functions::Ln | Functions::Log => Expr::BinaryOp {
                op: Operators::Div,
                lhs: Box::new(Expr::Number(1.)),
                rhs: Box::new(value.clone()),
                paren: false,
            },
            Functions::Sqrt => Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(Expr::Number(0.5)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Operators::Caret,
                    lhs: Box::new(value.clone()),
                    rhs: Box::new(Expr::UnaryOpPrefix {
                        op: Operators::Sub,
                        value: Box::new(Expr::Number(0.5)),
                    }),
                    paren: false,
                }),
                paren: false,
            },
        };

        if contains_var(value, self.var) {
            let derived_val = value.accept(self)?;
            res = Expr::BinaryOp {
                op: Operators::Mul,
                lhs: Box::new(derived_val),
                rhs: Box::new(res),
                paren: false,
            };
        };
        Some(fold_operations(res))
    }

    fn visit_unary_prefix(&self, op: &Operators, value: &Expr) -> Option<Expr> {
        if let Some(value) = value.accept(self) {
            return Some(Expr::UnaryOpPrefix {
                op: *op,
                value: Box::new(value.clone()),
            });
        };
        None
    }

    fn visit_unary_postfix(&self, op: &Operators, value: &Expr) -> Option<Expr> {
        Some(Expr::UnaryOpPostfix {
            op: *op,
            value: Box::new(value.clone()),
        })
    }

    fn visit_constant(&self, cnst: &Constants) -> Option<Expr> {
        Some(Expr::Constant(*cnst))
    }

    fn visit_number(&self, _: f64) -> Option<Expr> {
        Some(Expr::Number(0.))
    }

    fn visit_variable(&self, value: &str) -> Option<Expr> {
        if value == self.var {
            return Some(Expr::Number(1.));
        }
        Some(Expr::Number(0.))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_only() {
        let parsed = Polynomial::parse("3").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let folded = fold_operations(zeroed);
        let result = clear_num_var_literal(folded, "x");
        let expected = Polynomial::parse("0").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn variable_only() {
        let parsed = Polynomial::parse("x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let folded = fold_operations(zeroed);
        let result = clear_num_var_literal(folded, "x");
        let expected = Polynomial::parse("x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_derivative() {
        let parsed = Polynomial::parse("3x").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let folded = fold_operations(zeroed);
        let result = clear_num_var_literal(folded, "x"); // Check that this doesn't effect BinOp
        let expected = Polynomial::parse("3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_derivative_2() {
        let parsed = Polynomial::parse("3x + 2").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("3x").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn univariate_derivative_3() {
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

    #[test]
    fn many_vars() {
        let parsed = Polynomial::parse("4xyz - 7yz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("4xyz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn many_vars_div() {
        let parsed = Polynomial::parse("4xyz/7z - 7yz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("4xyz/7z").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn many_vars_div_2() {
        let parsed = Polynomial::parse("7z/4xyz - 7yz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("7z/4xyz").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn null_return() {
        let parsed = Polynomial::parse("4yz/7z - 7yz").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial {
            expr: Expr::Number(0.),
        };

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn null_return_2() {
        let parsed = Polynomial::parse("y + z").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial {
            expr: Expr::Number(0.),
        };

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn null_return_3() {
        let parsed = Polynomial::parse("y").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let folded = fold_operations(zeroed);
        let result = clear_num_var_literal(folded, "x");
        let expected = Polynomial {
            expr: Expr::Number(0.),
        };

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn power_tower() {
        let parsed = Polynomial::parse("y^2^2^2 - x^3^26^4").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("-x^3^26^4").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn power_tower_div() {
        let parsed = Polynomial::parse("y/x^3^26^4").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("y/x^3^26^4").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn power_tower_div_2() {
        let parsed = Polynomial::parse("(y+3)/x^3^26^4").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("(y+3)/x^3^26^4").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn power_tower_div_3() {
        let parsed = Polynomial::parse("x^3^26^4/y").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("x^3^26^4/y").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn power_tower_div_4() {
        let parsed = Polynomial::parse("x^3^26^4/(y+3)").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("x^3^26^4/(y+3)").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn div_div_deriv() {
        let parsed = Polynomial::parse("x/34/y").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("x/34/y").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn div_add_div_deriv() {
        let parsed = Polynomial::parse("x/(34+y)/y").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("x/(34+y)/y").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }

    #[test]
    fn add_div_div_deriv() {
        let parsed = Polynomial::parse("(x+45)/(34+y)/y").unwrap();
        let zeroed = remove_extra_vars(parsed.expr, "x");
        let result = fold_operations(zeroed);
        let expected = Polynomial::parse("(x+45)/(34+y)/y").unwrap();

        assert_eq!(Polynomial { expr: result }, expected);
    }
}
