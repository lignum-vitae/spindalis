use proc_macro::TokenStream;
use std::str::FromStr;

// Polynomial Parsing Macros

#[proc_macro]
pub fn parse_simple_polynomial(input: TokenStream) -> TokenStream {
    let output =
        match spindalis_core::polynomials::simple::parse_simple_polynomial(input.to_string()) {
            Ok(terms) => terms,
            Err(e) => {
                let error_msg = format!("{:?}", e);
                let error_tokens =
                    format!("compile_error!(\"{}\")", error_msg.replace("\"", "\\\""));
                return TokenStream::from_str(&error_tokens).unwrap();
            }
        };

    let mut tokens = String::from("::spindalis_core::polynomials::structs::SimplePolynomial { ");
    tokens.push_str("coefficients: vec![");
    for coeff in output.coefficients {
        tokens.push_str(&format!("{coeff:?},"));
    }
    tokens.push_str("], ");
    tokens.push_str(&format!("variable: {:?}, ", output.variable));
    tokens.push_str("}");

    TokenStream::from_str(&tokens).unwrap()
}

#[proc_macro]
pub fn parse_intermediate_polynomial(input: TokenStream) -> TokenStream {
    let output = match spindalis_core::polynomials::intermediate::parse_intermediate_polynomial(
        input.to_string(),
    ) {
        Ok(terms) => terms,
        Err(e) => {
            let error_msg = format!("{:?}", e);
            let error_tokens = format!("compile_error!(\"{}\")", error_msg.replace("\"", "\\\""));
            return TokenStream::from_str(&error_tokens).unwrap();
        }
    };

    let mut tokens = String::from("::spindalis_core::polynomials::structs::IntermediatePolynomial { ");
    tokens.push_str("terms: vec![");
    for term in output.terms {
        tokens.push_str(&format!(
            "::spindalis_core::polynomials::Term {{ coefficient: {:?}, variables: vec![",
            term.coefficient,
        ));
        for (var, pow) in term.variables {
            tokens.push_str(&format!("(\"{var}\".to_string(), {pow:?}),"));
        }
        tokens.push_str("] },");
    }
    tokens.push_str("], ");
    tokens.push_str("variables: vec![");
    for var in output.variables {
        tokens.push_str(&format!("\"{var}\".to_string(), "));
    }
    tokens.push_str("], ");
    tokens.push_str("}");

    TokenStream::from_str(&tokens).unwrap()
}
