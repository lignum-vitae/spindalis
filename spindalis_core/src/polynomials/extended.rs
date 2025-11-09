use crate::polynomials::ComplexPolyErr;
use crate::polynomials::Term;
use std::collections::HashMap;

static SPECIAL_CHARS: &[char] = &['.', '/', '-'];

pub fn parse_polynomial_extended<S>(expr: S) -> Result<Vec<Term>, ComplexPolyErr>
where
    S: AsRef<str>,
{
    let expr = expr.as_ref();
    let ascii_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let normalized = expr
        .replace(" ", "")
        .replace("^-", "^@")
        .replace("-", "+-")
        .replace("^@", "^-"); // ^- -> ^@ protects negative exponents
    let parts: Vec<&str> = normalized.split('+').filter(|s| !s.is_empty()).collect();

    let mut parsed = Vec::new();
    let mut coeff = String::new();
    let mut vars = Vec::new();
    let mut pow_str = String::new();

    for part in parts {
        coeff.clear();
        vars.clear();
        // Add ability to look at next char without consuming
        let mut chars = part.chars().peekable();

        // Extract coefficient
        while let Some(&ch) = chars.peek() {
            if ch.is_numeric() || ch == '.' || (coeff.is_empty() && ch == '-') {
                coeff.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        // If no explicit coefficient (e.g., "-x"), set it to 1 or -1
        let coeff = if coeff.is_empty() || coeff == "-" {
            if coeff == "-" { -1.0 } else { 1.0 }
        } else {
            let parsed = coeff.parse::<f64>();
            match parsed {
                Ok(x) => x,
                Err(_) => return Err(ComplexPolyErr::InvalidCoefficient { coeff }),
            }
        };

        while let Some(ch) = chars.next() {
            pow_str.clear();
            if ascii_letters.contains(ch) {
                let var = ch.to_string();
                let mut power = 1.0; // default power
                if let Some('^') = chars.peek() {
                    chars.next(); // consume '^'
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_ascii_digit() {
                            pow_str.push(next_char);
                            chars.next(); // consume current digit
                        } else if SPECIAL_CHARS.contains(&next_char) {
                            // Handles fractions and decimals in exponent
                            pow_str.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if pow_str.contains('/') {
                        let fraction: Vec<&str> = pow_str.split('/').collect();
                        if fraction.len() != 2 {
                            return Err(ComplexPolyErr::InvalidFractionalExponent { pow: pow_str });
                        }
                        match (fraction[0].parse::<f64>(), fraction[1].parse::<f64>()) {
                            (Ok(x), Ok(y)) if y != 0.0 => power = x / y,
                            _ => {
                                return Err(ComplexPolyErr::InvalidFractionalExponent {
                                    pow: pow_str,
                                });
                            }
                        }
                    } else if let Ok(pow) = pow_str.parse::<f64>() {
                        power = pow
                    } else {
                        return Err(ComplexPolyErr::InvalidExponent { pow: pow_str });
                    };
                };
                vars.push((var, power));
            }
        }
        parsed.push(Term {
            coefficient: coeff,
            variables: vars.clone(),
        });
    }
    Ok(parsed)
}

pub fn eval_polynomial_extended<I, S, F>(terms: &[Term], vars: &I) -> f64
where
    I: IntoIterator<Item = (S, F)> + std::fmt::Debug + Clone,
    S: AsRef<str>,
    F: Into<f64>,
{
    let vars_map: HashMap<String, f64> = vars
        .clone()
        .into_iter()
        .map(|(k, v)| (k.as_ref().to_string(), v.into()))
        .collect();

    let mut result = 0.0;

    for term in terms {
        let mut term_value = term.coefficient;

        for (var, pow) in &term.variables {
            if let Some(value) = vars_map.get(var) {
                term_value *= value.powf(*pow);
            } else {
                panic!("{var} not in {vars:?}");
            }
        }
        result += term_value;
    }
    result
}
