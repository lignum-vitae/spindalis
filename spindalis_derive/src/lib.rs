use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro]
pub fn parse_simple_polynomial(input: TokenStream) -> TokenStream {
    let output =
        spindalis_core::polynomials::simple::parse_simple_polynomial(&input.to_string()).unwrap();

    let mut tokens = String::from("vec![");
    for coeff in output {
        tokens.push_str(&format!("{coeff:?},"));
    }
    tokens.push(']');

    TokenStream::from_str(&tokens).unwrap()
}

#[proc_macro]
pub fn parse_polynomial_extended(input: TokenStream) -> TokenStream {
    let output =
        spindalis_core::polynomials::extended::parse_polynomial_extended(&input.to_string())
            .unwrap();

    let mut tokens = String::from("vec![");
    for term in output {
        tokens.push_str(&format!(
            "::spindalis_core::polynomials::Term {{ coefficient: {:?}, variables: vec![",
            term.coefficient,
        ));
        for (var, pow) in term.variables {
            tokens.push_str(&format!("(\"{var}\".to_string(), {pow:?}),"));
        }
        tokens.push_str("] },");
    }
    tokens.push(']');

    TokenStream::from_str(&tokens).unwrap()
}
