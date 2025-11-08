pub mod gradient_descent;
pub mod least_squares;
pub mod polynomial;

pub trait LinearRegressor {
    fn fit(&self, x: &[f64], y: &[f64]) -> LinearModel;
}

pub struct LinearModel {
    pub coefficients: Vec<f64>,
    pub std_err: f64,
    pub r2: f64,
}

impl LinearModel {
    pub fn intercept(&self) -> f64 {
        self.coefficients[0]
    }

    /// Simple regression slope
    pub fn slope(&self) -> Option<f64> {
        if self.coefficients.len() == 2 {
            Some(self.coefficients[1])
        } else {
            None
        }
    }

    /// Polynomial slope -> order > 2
    pub fn slopes(&self) -> Option<&[f64]> {
        if self.coefficients.len() > 2 {
            Some(&self.coefficients[1..])
        } else {
            None
        }
    }

    pub fn predict(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(pow, &c)| c * x.powi(pow as i32))
            .sum()
    }

    pub fn to_polynomial_string(&self) -> String {
        let mut parts = Vec::new();

        for (pow, &coef) in self.coefficients.iter().enumerate() {
            if coef == 0.0 {
                continue;
            }

            let term = match pow {
                0 => format!("{coef:.5}"),
                1 => match coef {
                    1.0 => "x".to_string(),
                    -1.0 => "-x".to_string(),
                    _ => format!("{coef:.5}x"),
                },
                _ => match coef {
                    1.0 => format!("x^{}", pow),
                    -1.0 => format!("-x^{}", pow),
                    _ => format!("{coef:.5}x^{}", pow),
                },
            };
            parts.push(term);
        }
        if parts.is_empty() {
            return "0".to_string();
        }
        parts.join(" + ").replace("+ -", "- ")
    }
}

impl std::fmt::Display for LinearModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coefficients.len() == 2 {
            write!(
                f,
                "BinomialModel {{ intercept: {:.5}, slope: {:.5}, std_err: {:.5}, r2: {:.5} }}",
                self.intercept(),
                self.slope().unwrap(),
                self.std_err,
                self.r2
            )
        } else {
            write!(
                f,
                "PolynomialModel {{ coefficients: {:?}, std_err: {:.5}, r2: {:.5} }}",
                self.coefficients, self.std_err, self.r2
            )
        }
    }
}
