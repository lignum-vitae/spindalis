pub fn parse_polynomial(input: &str) -> Vec<f64> {
    let normalized = input.replace(" ", "").replace("-", "+-");
    let parts: Vec<&str> = normalized.split('+').filter(|s| !s.is_empty()).collect();

    let mut terms: Vec<(f64, usize)> = Vec::new();
    for part in parts {
        let term = {
            if let Some(x) = part.find('x') {
                let coeff_str = &part[..x];
                let coeff = if coeff_str.is_empty() || coeff_str == "+" {
                    1.0
                } else if coeff_str == "-" {
                    -1.0
                } else {
                    coeff_str.parse::<f64>().unwrap()
                };

                if let Some(pow) = part.find('^') {
                    let pow_str = &part[pow + 1..];
                    let power = pow_str.parse::<usize>().unwrap();
                    (coeff, power)
                } else {
                    // x^1 value
                    (coeff, 1)
                }
            } else {
                // No 'x' aka num is constant
                (part.parse::<f64>().unwrap(), 0)
            }
        };
        terms.push(term);
    }

    let max_power = terms.iter().map(|&(_, power)| power).max().unwrap_or(0);
    let mut coeffs = vec![0.0; max_power + 1];
    for (coeff, power) in terms {
        coeffs[power] += coeff;
    }
    coeffs
}

pub fn eval_polynomial(x: f64, coeffs: &[f64]) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(i, &c)| c * x.powi(i as i32))
        .sum()
}
